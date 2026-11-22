# SP-06 — Wasmtime capability sandbox: Linux prototype report

**Date:** 2026-11-22  
**Platform:** CachyOS, Linux x86_64  
**Kernel:** 7.3.5-2-cachyos  
**Rust:** 1.100.0  
**Cargo:** 1.100.0  
**Wasmtime crate:** 51.0.0 (pinned in the disposable prototype manifest)  
**Status:** Linux embedded-runtime prototype checks passed; full spike remains in progress.

## Objective

Validate the initial Wasmtime-as-an-embedded-Rust-library direction for WebAssembly plugins. The Rust host remains authoritative and exposes only explicitly registered host capabilities. Plugins do not receive ambient WASI filesystem or network interfaces.

## Environment

- Rust target installed: `x86_64-unknown-linux-gnu`.
- Wasmtime CLI, `wasm-tools`, `wit-bindgen` and `cargo-component` were not installed before the spike.
- No Wasmtime or supporting crates were present in the Cargo source cache.
- The prototype therefore used a disposable Cargo project and fetched the pinned crate dependency graph through Cargo.

## Prototype checks reported by the user

The user reported that the integrated prototype completed successfully with all checks passing:

1. **Explicit host capability:** a Wasm module imported and called the one deliberately registered host function.
2. **Memory limit:** a module's attempt to grow memory beyond the configured 128 KiB store limit was rejected.
3. **Undeclared capability denial:** instantiation failed when the module imported an unregistered host function.
4. **No ambient WASI:** instantiation failed for an unprovided `wasi_snapshot_preview1` import.
5. **Fuel exhaustion:** an infinite-loop module trapped after exhausting its configured fuel budget.

The prototype used Wasmtime 51.0.0, fuel consumption, a store resource limiter, and a minimal host linker. No WASI implementation was linked into the test host.

## Result

**Linux prototype: PASS (user-reported).** The initial embedded Wasmtime runtime and capability-registration model worked on the development system.

## Limitations — not production acceptance

- The exact terminal output and compiler transcript were not retained; the pass status is based on the user's report that all prototype checks passed.
- This was a minimal Core WebAssembly module test, not a Component Model/WIT ABI conformance test.
- Filesystem and network denial were demonstrated by withholding WASI imports, not by testing a complete production plugin interface or every possible host capability.
- The host function was a demonstration capability, not a connection to the production permission broker.
- Identity propagation, broker-mediated privileged operations, capability revocation, cancellation, plugin lifecycle supervision, ABI compatibility policy and adversarial module validation remain untested.
- The prototype does not establish Windows compatibility.
- Wasmtime's resource limiter and fuel controls need production policy values, accounting, interruption behaviour and regression tests before release.

## Provisional engineering direction

Retain Wasmtime embedded in the Rust core/host process as the initial implementation candidate. Expose a deliberately small, versioned host interface; provide no ambient filesystem or network access; mediate privileged host operations through the central permission broker; and enforce per-instance memory, fuel, time and lifecycle limits. Plugin identity must be assigned by the host and cannot be asserted by the module.

Use the Component Model/WIT for the eventual stable plugin contract after a separate compatibility and versioning spike. Do not treat this Core-Wasm prototype as validation of that ABI.

## Remaining acceptance work

- Component Model/WIT contract and version mismatch tests.
- Explicit capability grants and revocation through a broker-backed host interface.
- Bounded execution interruption/cancellation and instance teardown.
- Filesystem preopens and network capability tests once such capabilities are intentionally added.
- Malformed/hostile module validation and resource exhaustion regression tests.
- Windows validation after the Linux-first implementation sequence reaches that stage.

**Decision:** The embedded Wasmtime direction is provisionally supported by the Linux prototype. SP-06 remains **in progress** until the remaining acceptance criteria are tested and the Linux production integration is reviewed.
