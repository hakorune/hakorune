# WASM Backend Unsupported Inventory (WSM-01)

## Last Updated
- 2026-02-26

## Scope
This document tracks current unsupported surface for the Rust WASM backend based on actual source status.

- Entry: `src/backend/wasm/mod.rs`
- Instruction lowering: `src/backend/wasm/codegen/instructions.rs`
- BoxCall lowering: `src/backend/wasm/codegen/builtins.rs`
- Runtime imports: `src/backend/wasm/runtime.rs`

## Current Implementation Snapshot

### 1. Extern call support (partial)
Supported extern names in `instructions.rs`:
- `env.console.log`
- `env.canvas.fillRect`
- `env.canvas.fillText`

Unsupported extern calls fail-fast with:
- `Unsupported extern call: <name> (supported: ...)`

### 2. BoxCall support (partial)
Supported methods in `builtins.rs`:
- `toString`
- `print`
- `equals`
- `clone`
- `log`

Unsupported methods fail-fast with:
- `Unsupported BoxCall method: <name> (supported: ...)`

### 3. Executor status
- `src/backend/wasm/executor.rs` is not currently active in mainline.
- `src/backend/wasm/mod.rs` exports compiler/codegen/runtime only.

## WSM-01 Decision (accepted)
- Do not add broad fallback behavior.
- Keep unsupported paths fail-fast with explicit supported-list diagnostics.
- Keep this inventory synchronized to actual source files.

## Next Candidates (WSM-02+)
- Expand extern-call coverage beyond current 3 names.
- Expand BoxCall coverage for core methods used by selfhost fixtures.
- Add wasm-focused gate fixtures that assert supported/unsupported boundaries.
