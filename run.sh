#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"
CORE="$ROOT/target/debug/assistantd"
GUI="$ROOT/build/desktop/local-assistant"
RUN_DIR="$ROOT/.run"
PID_FILE="$RUN_DIR/assistantd.pid"
LOG_FILE="$RUN_DIR/assistantd.log"

MODEL="\${ASSISTANT_MODEL:-qwen3.5:2b-q4_K_M}"
OLLAMA_HOST="\${OLLAMA_HOST:-http://127.0.0.1:11434}"
OLLAMA_URL="$OLLAMA_HOST/api/chat"
CONTEXT="\${ASSISTANT_CONTEXT_LENGTH:-8192}"
PYTHON="$(command -v python3 || true)"

usage() {
    printf '%s\n' \
        "Local Assistant 2.0 launcher" \
        "Usage: ./run.sh [run|build|status|stop|clean|help]" \
        "  run     Build, start the local core, and launch the GUI (default)" \
        "  build   Build the Rust core and Qt desktop client" \
        "  status  Show core and model endpoint status" \
        "  stop    Stop the daemon started by this launcher" \
        "  clean   Stop the daemon and remove generated desktop build files"
}

install_dependencies_if_needed() {
    local missing=0
    for command_name in cargo cmake ninja python3 curl; do
        if ! command -v "$command_name" >/dev/null 2>&1; then
            printf 'Missing command: %s\n' "$command_name"
            missing=1
        fi
    done
    if ! pacman -Q qt6-base qt6-declarative >/dev/null 2>&1; then
        printf 'Qt 6 development packages are missing.\n'
        missing=1
    fi

    if (( missing )); then
        if ! command -v pacman >/dev/null 2>&1; then
            printf 'Install Rust, CMake, Ninja, Python 3, curl and Qt 6 development packages first.\n' >&2
            exit 1
        fi
        printf 'Required CachyOS packages: base-devel cmake ninja qt6-base qt6-declarative python curl\n'
        read -r -p 'Install missing build dependencies with pacman? [y/N] ' answer
        case "$answer" in
            y|Y|yes|YES)
                sudo pacman -S --needed base-devel cmake ninja qt6-base qt6-declarative python curl
                ;;
            *)
                printf 'Dependency installation cancelled.\n' >&2
                exit 1
                ;;
        esac
    fi

    PYTHON="$(command -v python3)"
    command -v cargo >/dev/null 2>&1 || {
        printf 'Rust is not installed. Install rustup and run rustup default stable.\n' >&2
        exit 1
    }
}

check_context() {
    if ! [[ "$CONTEXT" =~ ^[0-9]+$ ]] || (( CONTEXT < 4096 || CONTEXT > 16384 || CONTEXT % 1024 != 0 )); then
        printf 'ASSISTANT_CONTEXT_LENGTH must be 4096-16384 in 1024-token steps.\n' >&2
        exit 1
    fi
}

check_ollama() {
    if ! curl --fail --silent --show-error --max-time 4 "$OLLAMA_HOST/api/tags" >/dev/null; then
        printf '\nCannot reach Ollama at %s.\n' "$OLLAMA_HOST" >&2
        printf 'Start your existing Ollama server separately, then run this script again.\n' >&2
        exit 1
    fi

    if ! python3 - "$MODEL" "$OLLAMA_HOST/api/tags" <<'PY'
import json
import sys
import urllib.request

wanted, url = sys.argv[1], sys.argv[2]
with urllib.request.urlopen(url, timeout=4) as response:
    models = json.load(response).get("models", [])
names = {item.get("name", "") for item in models}
raise SystemExit(0 if wanted in names else 1)
PY
    then
        printf 'Model %s is not present in the local Ollama cache.\n' "$MODEL"
        read -r -p 'Download it now with ollama pull? [y/N] ' answer
        case "$answer" in
            y|Y|yes|YES)
                command -v ollama >/dev/null 2>&1 || {
                    printf 'The ollama command is not installed.\n' >&2
                    exit 1
                }
                ollama pull "$MODEL"
                ;;
            *)
                printf 'Model download cancelled.\n' >&2
                exit 1
                ;;
        esac
    fi
}

build_project() {
    printf '\n==> Building Rust core\n'
    (cd "$ROOT" && cargo build -p assistantd)
    printf '\n==> Building Qt 6 desktop client\n'
    cmake -S "$ROOT/desktop" -B "$ROOT/build/desktop" -G Ninja -DCMAKE_BUILD_TYPE=Release
    cmake --build "$ROOT/build/desktop" --parallel
}

stop_core() {
    if [[ -f "$PID_FILE" ]]; then
        local pid actual_exe expected_exe
        pid="$(<"$PID_FILE")"
        actual_exe=""
        expected_exe="$(readlink -f "$CORE")"
        if [[ "$pid" =~ ^[0-9]+$ && -e "/proc/$pid/exe" ]]; then
            actual_exe="$(readlink -f "/proc/$pid/exe" || true)"
        fi
        if [[ "$actual_exe" == "$expected_exe" ]]; then
            printf 'Stopping assistantd (PID %s)...\n' "$pid"
            kill -INT "$pid"
            for _ in $(seq 1 50); do
                kill -0 "$pid" 2>/dev/null || break
                sleep 0.1
            done
            if kill -0 "$pid" 2>/dev/null; then
                kill -TERM "$pid" 2>/dev/null || true
            fi
        else
            printf 'Stored daemon PID no longer identifies this assistantd binary; not signalling it.\n'
        fi
        rm -f "$PID_FILE"
    fi
}

start_core() {
    mkdir -p "$RUN_DIR"
    chmod 700 "$RUN_DIR"

    if "$CORE" health >/dev/null 2>&1; then
        if [[ -f "$PID_FILE" ]]; then
            stop_core
        else
            printf 'An assistantd instance is already active but is not managed by this launcher.\n' >&2
            printf 'Stop that development instance first, then rerun ./run.sh.\n' >&2
            exit 1
        fi
    fi

    printf '\n==> Starting the per-user assistant core\n'
    nohup env \
        ASSISTANT_WORKER_PATH="$ROOT/workers/agent/worker.py" \
        ASSISTANT_PYTHON="$PYTHON" \
        ASSISTANT_MODEL="$MODEL" \
        ASSISTANT_OLLAMA_URL="$OLLAMA_URL" \
        ASSISTANT_CONTEXT_LENGTH="$CONTEXT" \
        "$CORE" serve >>"$LOG_FILE" 2>&1 </dev/null &
    echo "$!" > "$PID_FILE"

    for _ in $(seq 1 50); do
        if "$CORE" health >/dev/null 2>&1; then
            printf 'assistantd is ready.\n'
            return
        fi
        if ! kill -0 "$(<"$PID_FILE")" 2>/dev/null; then
            printf 'assistantd exited during startup. Recent log:\n' >&2
            tail -n 50 "$LOG_FILE" >&2 || true
            rm -f "$PID_FILE"
            exit 1
        fi
        sleep 0.1
    done

    printf 'assistantd did not become ready. Recent log:\n' >&2
    tail -n 50 "$LOG_FILE" >&2 || true
    stop_core
    exit 1
}

run_app() {
    if [[ -z "\${XDG_RUNTIME_DIR:-}" || ! -d "$XDG_RUNTIME_DIR" ]]; then
        printf 'XDG_RUNTIME_DIR is unavailable. Launch from your normal desktop session.\n' >&2
        exit 1
    fi
    check_context
    install_dependencies_if_needed
    check_ollama
    build_project
    start_core
    printf '\nLaunching Local Assistant 2.0...\n'
    "$GUI"
}

case "\${1:-run}" in
    run|start) run_app ;;
    build)
        install_dependencies_if_needed
        build_project
        ;;
    status)
        if "$CORE" status; then
            printf 'Ollama endpoint: %s\n' "$OLLAMA_HOST"
            printf 'Selected model: %s\n' "$MODEL"
        else
            printf 'assistantd is not running.\n'
            exit 1
        fi
        ;;
    stop) stop_core ;;
    clean)
        stop_core
        rm -rf "$ROOT/build/desktop"
        printf 'Generated desktop build files removed.\n'
        ;;
    help|-h|--help) usage ;;
    *)
        usage >&2
        exit 2
        ;;
esac
