# Local Assistant 2.0 — Open-Source Integration Catalog

Last reviewed: 2026-11-23

This is a living research register for technologies that may improve the
desktop assistant, shared core, and eventual AI-integrated operating system.
It exists to prevent useful discoveries from being forgotten or the project
from drifting into ad-hoc dependency choices.

**Listing is not approval to adopt.** Every candidate must be checked again
when its implementation stage arrives. Verify current maintenance, exact
release, license of code and bundled assets/models/grammars, platform support,
security posture, resource use, packaging, and distribution obligations.

## Decision vocabulary

- **Approved:** the user explicitly approved the integration approach.
- **Evaluate:** promising candidate; benchmark and inspect before adoption.
- **Use existing platform service:** prefer the operating system's standard
  interface instead of replacing it.
- **Later:** relevant to a later milestone or OS product, not current scope.
- **Do not assume adoption:** an entry is a research lead, not a dependency.

## A. Current core and immediate development

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Project text search | [ripgrep](https://github.com/BurntSushi/ripgrep) | Fast recursive search through a controlled executable adapter; no shell invocation, workspace confinement, bounded output/time | **Approved**; next tool expansion |
| Syntax-aware code parsing | [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) | Incremental parsing, syntax trees, structural code navigation | **Approved**; after basic search |
| External tool interoperability | [Model Context Protocol](https://modelcontextprotocol.io/) | Adapter from external MCP servers into the native Tool Registry | **Approved**; after request-aware execution context |
| Code structure search | [ast-grep](https://github.com/ast-grep/ast-grep) | Structural search and transformations complementary to Tree-sitter and ripgrep | Evaluate alongside Tree-sitter; avoid duplicate features |
| Language intelligence | [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) and servers such as [rust-analyzer](https://github.com/rust-lang/rust-analyzer) and [clangd](https://clangd.llvm.org/) | Reuse compiler-grade diagnostics, symbol lookup, references, completion and navigation | Evaluate per language when Coding Agent needs it |
| Cross-language code index | [SCIP](https://github.com/scip-code/scip) | Portable symbol/index format for definitions, references and implementations | Later; evaluate after code navigation needs are clear |
| Code intelligence toolkit | [deagle](https://github.com/dirmacs/deagle) | Research reference for a Rust-native combination of Tree-sitter, SQLite/FTS, structural search and ripgrep | Research only; compare capabilities and maturity, do not add blindly |
| Local persistence | [SQLite](https://www.sqlite.org/) with FTS5 | Keep conversations, task records, metadata and initial keyword search in the existing embedded database | Existing foundation; extend only where justified |

## B. Inference and model execution

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Current local model service | [Ollama](https://github.com/ollama/ollama) | Current model runtime and local model management | Existing choice; retain unless a concrete gap appears |
| Alternative inference engine | [llama.cpp](https://github.com/ggml-org/llama.cpp) | Potential direct GGUF inference/server backend, quantization and hardware-backend comparison | Evaluate only if Ollama limits a required feature or performance |
| Cross-platform model runtime | [ONNX Runtime](https://github.com/microsoft/onnxruntime) | Run small specialist models for OCR, speech, embeddings or vision with execution providers | Evaluate for non-chat models; not a replacement for Ollama by default |
| Model format and runtime ecosystem | [GGUF](https://github.com/ggml-org/ggml) / GGML | Model packaging and efficient quantized inference ecosystem | Track compatibility and licensing of each model separately |
| Model download and lifecycle | Ollama model APIs; later compare runtime-native model managers | Pull, list, update, validate and remove models with clear user controls | Current product milestone; do not create a parallel manager without need |

**Guardrail:** model weights, tokenizer files, datasets, adapters and voice
assets may have terms different from the runtime's source-code license.
Review each model card and redistribution permission independently.

## C. Agent, tool, workflow and execution infrastructure

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Agent orchestration | [LangGraph](https://github.com/langchain-ai/langgraph) | Reference/evaluation for durable graphs, checkpoints, retries and human approval | Evaluate against lightweight native Rust coordinator; major adoption needs approval |
| Inter-agent/tool protocol | MCP | External server interoperability only; native registry remains policy authority | **Approved adapter approach** |
| Workflow automation | [n8n](https://github.com/n8n-io/n8n) | Reference for visual workflow authoring and integrations | Later evaluation; likely too large as a core runtime dependency initially |
| Process sandbox | [bubblewrap](https://github.com/containers/bubblewrap) | Linux namespace/filesystem isolation for untrusted or modifying worker processes | High-priority security evaluation before coding tools can execute arbitrary project commands |
| OS security primitives | Linux namespaces, seccomp and Landlock | Defense-in-depth restrictions for worker processes and filesystem access | Evaluate as platform primitives; avoid claiming any one mechanism is a complete sandbox |
| Worker lifecycle | [systemd user services/scopes](https://systemd.io/) | Supervise long-running user-level components and optionally scope resource use | Evaluate for daemon/worker lifecycle; retain application-level task state |
| Browser automation | [Playwright](https://github.com/microsoft/playwright) | Controlled browser automation for research workflows if/when browser actions are required | Later; explicit network/domain and user-approval policy required |
| Permission policy | Native Tool Registry and capability broker | Authoritative principal, capability, workspace, approval, limits and audit decisions | Product-owned core; do not delegate authority to external frameworks |

**Security rule:** no external agent framework, MCP server, workflow engine or
worker process may bypass the native permission broker. Do not expose a
general unrestricted shell tool by default.

## D. Search, knowledge and memory

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Keyword search | SQLite FTS5 | Local BM25-style full-text search for conversations, notes and indexed documents | Preferred first step; already compatible with SQLite foundation |
| Embedded full-text engine | [Tantivy](https://github.com/quickwit-oss/tantivy) | Higher-scale Rust-native indexing if SQLite FTS5 becomes insufficient | Evaluate only after measured limits |
| Vector search extension | [sqlite-vec](https://github.com/asg017/sqlite-vec) | Embedded local vector similarity within SQLite | Evaluate when semantic retrieval is needed; verify extension packaging and performance |
| Alternative SQLite vector extension | [sqlite-vector](https://github.com/sqliteai/sqlite-vector) | Another embedded vector-search option | Compare with sqlite-vec; do not adopt both without a clear reason |
| Dedicated vector service | [Qdrant](https://github.com/qdrant/qdrant) | Larger-scale vector retrieval and filtering | Later/conditional; server and operational overhead likely unjustified for first desktop release |
| Local embeddings | [FastEmbed](https://github.com/Anush008/fastembed-rs) and ONNX Runtime ecosystem | Local embedding generation for semantic search | Evaluate model size, CPU/GPU acceleration, language coverage and model terms |
| Hybrid retrieval | SQLite FTS5 + vector candidate | Combine keyword and semantic retrieval with explicit ranking/fusion | Later, after real memory/document corpus exists |
| Knowledge graph | SQLite relational tables first; graph DB only if required | Relationships between people, projects, files, facts and tasks | Avoid graph infrastructure until query patterns justify it |

**Storage principle:** SQLite remains canonical state initially. Search indexes,
embeddings and caches are derived data that can be rebuilt. Do not introduce
a separate database service merely because a project is popular.

## E. Documents, files and OCR

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| General document conversion | [Docling](https://github.com/docling-project/docling) | Local PDF, Office, HTML and image conversion into structured document output for agents | High-value evaluation for Research Agent and knowledge ingestion |
| Broad file extraction | [Apache Tika](https://github.com/apache/tika) | Format detection, metadata and text extraction across many file types | Evaluate as a broad extraction layer; JVM/runtime footprint matters |
| OCR engine | [Tesseract](https://github.com/tesseract-ocr/tesseract) | Lightweight local OCR fallback and text extraction | Evaluate for screen/document OCR; language packs and accuracy matter |
| Multilingual OCR/document AI | [PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR) | OCR, layout, tables and structured document extraction | Evaluate as optional isolated worker; larger model/runtime cost |
| Searchable scanned PDFs | [OCRmyPDF](https://github.com/ocrmypdf/OCRmyPDF) | Add OCR text layers to scanned PDFs | Evaluate for document workflows; inspect dependencies and distribution license |
| PDF rendering | [PDF.js](https://github.com/mozilla/pdf.js) | Embedded PDF viewing in a future document interface | Later, if the desktop needs a built-in document viewer |
| File type detection | [libmagic](https://github.com/file/file) | Identify actual file formats before parsing | Evaluate for secure document ingestion; never trust filename extensions alone |

Document parsing must be isolated and resource-bounded because user-provided
documents are untrusted inputs. Avoid automatically executing embedded
content or following document links.

## F. Voice and audio

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Speech-to-text | [whisper.cpp](https://github.com/ggml-org/whisper.cpp) | Local transcription, streaming microphone input and voice commands | High-value evaluation for a future Voice Mode |
| Unified speech toolkit | [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) | Offline ASR, TTS, VAD, diarization and audio processing across platforms | Compare with whisper.cpp; possible cross-platform audio subsystem |
| Text-to-speech | [Piper](https://github.com/OHF-Voice/piper1-gpl) | Local speech synthesis | Evaluate engine and every voice model's license separately |
| Audio/video graph | [PipeWire](https://pipewire.org/) | Linux microphone, speaker and media routing/capture | Use existing OS service on Linux; abstract provider for Windows |
| Voice activity detection | sherpa-onnx and compatible VAD models | Avoid sending silence to ASR and manage turn boundaries | Evaluate with speech pipeline |
| Audio device abstraction | Qt Multimedia / PipeWire APIs | Device selection, capture/playback and audio state | Reuse platform APIs; avoid direct device management unless required |

Voice activation, microphone capture and any background listening require
clear opt-in, visible state, immediate mute/stop controls and local processing
by default. Model and voice-data terms must be checked separately.

## G. Screen awareness, desktop and accessibility

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Secure screen capture | [XDG Desktop Portal](https://flatpak.github.io/xdg-desktop-portal/docs/) | User-mediated capture and permission flow on Linux/Wayland | High-priority; preferred Linux interface |
| Media transport | [PipeWire](https://docs.pipewire.org/) | Receive screen/audio streams from portals and route media | High-priority Linux provider |
| KDE desktop integration | [KDE/KWin](https://invent.kde.org/plasma/kwin) APIs and protocols | Window metadata and KDE-specific context | Evaluate behind replaceable provider |
| Accessibility tree | [AT-SPI](https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/) | Read UI roles, labels and accessible structure instead of relying only on pixels | High-value evaluation; respect application and user privacy |
| Desktop IPC | [D-Bus](https://www.freedesktop.org/wiki/Software/dbus/) | Communicate with desktop services and system components | Use standard Linux IPC where appropriate |
| Visual processing | [OpenCV](https://github.com/opencv/opencv) | Image transforms, regions, matching and preprocessing | Evaluate only for concrete screen/image tasks |
| OCR | Tesseract / PaddleOCR | Extract visible text when accessibility metadata is unavailable | Use as a fallback, not continuous full-screen OCR by default |
| Windowing/platform boundary | Provider interfaces in our Rust core | Keep KDE/KWin implementation replaceable by Windows providers | Product-owned abstraction |

Prefer event-driven window/accessibility context over constant screenshots.
Screen awareness must have visible active state, pause/disable, app/window
exclusions, sensitive-content protections and no capture while inactive.

## H. Linux OS integration and eventual OS product

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| User services and supervision | [systemd](https://systemd.io/) | Daemon lifecycle, startup, restart policy and resource controls | Use existing Linux service manager; evaluate product unit design |
| Desktop/system IPC | D-Bus | Desktop services and controlled system service communication | Use platform standard |
| Privileged actions | [polkit](https://www.freedesktop.org/software/polkit/) | Authorization boundary for narrowly scoped privileged system actions | Evaluate for OS product; never replace with ad-hoc sudo calls |
| Secure desktop access | XDG Desktop Portal | File chooser, screen capture and user-mediated desktop permissions | Prefer standard portal interfaces |
| Application packaging | [Flatpak](https://flatpak.org/) | Optional desktop distribution and sandboxing | Evaluate alongside native packages; sandbox limitations may affect daemon/system access |
| Application sandbox runtime | bubblewrap | Process isolation and filesystem namespace policy | Evaluate with Flatpak and direct worker isolation |
| Linux installer | [Calamares](https://github.com/calamares/calamares) | Reusable installer framework for a future distribution image | OS-stage evaluation only |
| Package management | [pacman/libalpm](https://gitlab.archlinux.org/pacman/pacman) | Package database and package transactions for a CachyOS-derived OS | OS-stage; reuse Arch ecosystem rather than inventing package manager |
| Firmware updates | [fwupd](https://github.com/fwupd/fwupd) | Standard firmware update integration | OS-stage; only through supported hardware/vendor flows |
| Boot and initramfs | Existing CachyOS/Arch boot stack (systemd-boot/GRUB, mkinitcpio as selected by distro) | Boot, kernel and initramfs lifecycle | OS-stage; preserve upstream-supported defaults |
| Desktop shell | KDE Plasma / KWin | Desktop shell and compositor foundation | Existing environment; consider customization only after desktop products |
| Cross-desktop compatibility | XDG specifications | File locations, desktop entries, MIME associations and portals | Prefer standards over KDE-only assumptions where practical |

The OS product should derive from and contribute to mature Linux infrastructure.
Do not fork or replace core system components without a strong product reason.
Any privileged integration requires explicit authorization, least privilege,
auditability, rollback and recovery design.

## I. Development productivity and project quality

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Rust test runner | [cargo-nextest](https://github.com/nextest-rs/nextest) | Faster, more controllable Rust test execution | Evaluate for larger test suite/CI |
| Dependency/license policy | [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | Check licenses, duplicate dependencies, advisories and sources | High-value before third-party dependency growth |
| Rust security advisories | [cargo-audit](https://github.com/RustSec/rustsec) | Audit Cargo.lock against RustSec advisory database | High-value periodic development check |
| Build cache | [sccache](https://github.com/mozilla/sccache) | Cache compiler artifacts and reduce rebuild times | Evaluate if build times become a bottleneck |
| Faster linker | [mold](https://github.com/rui314/mold) | Reduce native link time on supported Linux toolchains | Optional developer-machine optimization |
| Task runner | [just](https://github.com/casey/just) | Reproducible project commands and developer workflows | Evaluate when command set grows |
| Python tooling | [uv](https://github.com/astral-sh/uv) | Reproducible, fast Python tooling for optional AI/document workers | Evaluate only if Python services are introduced |
| Structured logs/tracing | Rust `tracing` ecosystem | Correlate request, agent, tool and worker lifecycles | Use for local diagnostics; no automatic telemetry export |
| Observability standard | [OpenTelemetry](https://opentelemetry.io/) | Optional local traces/metrics export for debugging complex workflows | Evaluate only if built-in structured logs are insufficient |
| C/C++ language support | [clangd](https://clangd.llvm.org/) | Diagnostics/navigation for desktop C++ development and Coding Agent | Evaluate as LSP provider |
| Build system | Existing CMake + Cargo | Build Qt/C++ desktop and Rust core | Keep current toolchains; don't add a meta-build framework without need |

Development tools should improve iteration without turning routine work into
a large CI/observability project. Diagnostics remain local and opt-in; do not
add hosted telemetry.

## J. Cross-platform and future capabilities

| Capability | Candidate | Intended role | Status / timing |
|---|---|---|---|
| Browser rendering/automation | Playwright | Controlled web workflows and testing | Later; separate browser profile and network policy |
| Computer vision | OpenCV / ONNX Runtime | Visual matching, image preprocessing and small vision models | Later; benchmark on target hardware |
| Windows screen/accessibility | Windows Graphics Capture and UI Automation | Replace Linux portal/AT-SPI providers | Product B |
| Windows IPC | Named pipes | Native Windows local IPC provider | Product B |
| Distribution and updates | Native package repositories, Flatpak, and OS-native update infrastructure | Install, update, rollback and dependency management | Product A packaging; revisit for OS |
| Diagnostics | Local support bundle with user review | User-triggered troubleshooting without automatic uploads | Product A completion; explicit review before export |

## K. Evaluation checklist for every candidate

Before adopting a component, record:

1. Exact project, upstream repository and version/release.
2. Maintenance activity, release cadence, issue health and security policy.
3. Code license and licenses of bundled dependencies.
4. Separate licenses/terms for model weights, grammars, voice data and datasets.
5. Linux/CachyOS/KDE compatibility and Windows portability where relevant.
6. CPU, RAM, GPU/VRAM, disk and background-service costs.
7. Local/offline behaviour and any network or telemetry defaults.
8. Security boundary, input trust assumptions and permission model.
9. Integration shape: library, binary, service, protocol or OS API.
10. Packaging/update burden and ability to replace or remove it.
11. Measured benefit against the simplest existing implementation.
12. User approval for major dependencies, architecture changes or licensing risks.

## L. Current sequencing

1. Finish workspace-aware request/tool execution and security hardening.
2. Expand the native Tool Registry and integrate ripgrep.
3. Add Tree-sitter after text search works.
4. Stabilize execution context, permissions and lifecycle before MCP.
5. Build the real Agent Coordinator and useful specialist workflows.
6. Evaluate document ingestion (Docling/Tika/Tesseract) when Research Agent needs it.
7. Evaluate local retrieval (SQLite FTS5 first; embeddings/vector search only when needed) during memory development.
8. Evaluate screen-awareness platform services (XDG Portal, PipeWire, KDE/KWin, AT-SPI) during Screen Awareness.
9. Evaluate speech stack (whisper.cpp/sherpa-onnx/Piper) when Voice Mode is scheduled.
10. Complete Linux desktop product, packaging, dependency and license review.
11. Build Windows providers and product.
12. Evaluate OS-stage components (Calamares, pacman/libalpm, polkit, fwupd and system integration) only after both desktop products are complete.

## M. Open decisions requiring future approval

Do not silently decide these from the candidate list:

- ripgrep installation/distribution strategy.
- MCP initial transports and server onboarding policy.
- Exact Tree-sitter languages and grammar distribution/licensing.
- Whether/when to adopt a major agent orchestration framework.
- File write/delete and command-execution permission model.
- Document parser and OCR runtime footprint.
- Voice activation/background microphone policy and speech model licenses.
- Semantic memory/vector-search dependency.
- Flatpak versus native package distribution strategy.
- OS base, installer, package/update and privileged-service architecture.
- Any cloud processing, remote MCP endpoint or telemetry export.

