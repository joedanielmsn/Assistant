#!/usr/bin/env python3
"""One-request Local Assistant worker. stdout is one JSON protocol response."""

from __future__ import annotations

import json
import os
import sys
import urllib.error
import urllib.request

SYSTEM_PROMPT = """You are Local Assistant 2.0, a private, local-first AI assistant.
Be helpful, accurate, and clear. Ask a concise clarifying question when needed.
This prototype has no tools. Never claim to have run commands, changed files,
browsed the web, inspected the computer, or performed an external action.
Model output is conversational text, not authorization to perform actions."""


def main() -> int:
    try:
        request = json.load(sys.stdin)
        messages = request.get("messages")
        if not isinstance(messages, list) or not messages:
            raise ValueError("messages must be a non-empty array")

        safe_messages = []
        for item in messages:
            if not isinstance(item, dict):
                raise ValueError("invalid message")
            role = item.get("role")
            content = item.get("content")
            if role not in ("user", "assistant") or not isinstance(content, str):
                raise ValueError("invalid message fields")
            if not content.strip():
                raise ValueError("empty message")
            safe_messages.append({"role": role, "content": content})

        model = os.environ.get("ASSISTANT_MODEL", "qwen3.5:2b-q4_K_M")
        endpoint = os.environ.get("ASSISTANT_OLLAMA_URL", "http://127.0.0.1:11434/api/chat")
        context = int(os.environ.get("ASSISTANT_CONTEXT_LENGTH", "8192"))
        if context < 4096 or context > 16384 or context % 1024:
            raise ValueError("context length must be 4096-16384 in 1024-token steps")

        body = {
            "model": model,
            "messages": [{"role": "system", "content": SYSTEM_PROMPT}, *safe_messages],
            "stream": False,
            "think": False,
            "options": {"num_ctx": context},
        }
        encoded = json.dumps(body, ensure_ascii=False).encode("utf-8")
        http_request = urllib.request.Request(
            endpoint, data=encoded,
            headers={"Content-Type": "application/json"}, method="POST"
        )
        with urllib.request.urlopen(http_request, timeout=170) as response:
            result = json.load(response)

        content = result.get("message", {}).get("content")
        if not isinstance(content, str) or not content.strip():
            raise RuntimeError("model returned no message content")

        sys.stdout.write(json.dumps(
            {"message": content, "model": model},
            ensure_ascii=False, separators=(",", ":")
        ) + "\n")
        return 0
    except urllib.error.HTTPError as error:
        detail = error.read(512).decode("utf-8", errors="replace")
        print(f"Ollama HTTP error {error.code}: {detail}", file=sys.stderr)
    except urllib.error.URLError as error:
        print(f"Cannot reach the local Ollama endpoint: {error.reason}", file=sys.stderr)
    except Exception as error:
        print(f"Local agent worker failed: {error}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
