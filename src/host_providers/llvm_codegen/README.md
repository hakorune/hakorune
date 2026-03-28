# llvm_codegen

Thin Rust bridge for backend object emission.

## Responsibility split

- `route.rs`
  - owner selection only
  - does not re-decide MIR acceptance
- `ll_emit_bridge.rs`
  - `.hako ll emitter` bridge orchestration only
  - delegates compare-driver render / VM execution / `.ll` extraction to `hako_ll_driver.rs`
  - delegates `.ll -> verify -> llc -> .o` to `ll_tool_driver.rs`
- `hako_ll_driver.rs`
  - compare/debug bridge helper for MIR(JSON) embed, driver VM execution, and stdout extraction only
- `ll_tool_driver.rs`
  - thin LLVM tool seam
  - `.ll` text or file -> verifier -> `llc` -> `.o`
- `transport.rs`
  - legacy C ABI transport
  - explicit provider keep lanes (`ny-llvmc`, `llvmlite`)
- `normalize.rs`
  - backend input validation / JSON normalization
- `defaults.rs`
  - transport defaults only

## Current policy

- mainline owner is being cut over shape-by-shape from legacy C `.inc` to `.hako ll emitter`
- compare lane is explicit bridge evidence, not a permanent default route; the proof smoke now runs from `phase29x-derust-archive.txt`
- canonical seam stays MIR; do not reopen `AST -> LLVM` direct lowering here
- current tool seam is now `.ll` text; full daily transport cut still needs a `.hako` MIR root entry before `compile_json_path` can leave mainline
