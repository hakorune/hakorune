# llvm_codegen

Thin Rust bridge for backend object emission.

## Responsibility split

- `route.rs`
  - owner selection only
  - does not re-decide MIR acceptance
- `ll_emit_bridge.rs`
  - `.hako ll emitter` bridge only
  - embeds MIR for the internal driver, extracts `.ll`, runs verifier, then `llc`
- `transport.rs`
  - legacy C ABI transport
  - explicit provider keep lanes (`ny-llvmc`, `llvmlite`)
- `normalize.rs`
  - backend input validation / JSON normalization
- `defaults.rs`
  - transport defaults only

## Current policy

- mainline owner is being cut over shape-by-shape from legacy C `.inc` to `.hako ll emitter`
- compare lane is explicit bridge evidence, not a permanent default route
- canonical seam stays MIR; do not reopen `AST -> LLVM` direct lowering here
