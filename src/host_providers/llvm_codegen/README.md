# llvm_codegen

Thin Rust bridge for backend object emission.

## Responsibility split

- `route.rs`
  - legacy compare/archive selection only
  - does not re-decide MIR acceptance
- `ll_emit_bridge.rs`
  - compare-only bridge orchestration
  - folds compare-driver render / VM execution / `.ll` extraction into one archive-later wrapper surface
  - delegates `.ll -> verify -> llc -> .o` to `ll_tool_driver.rs`
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
- current tool seam is now `.ll` text
- `compile_json_path` has been retired from code; flipped `.hako ll emitter` daily profiles stop at `ll_text_to_object(...)`
- launcher/mainline transport cut is landed; `route.rs` is now compare/archive-only and `transport.rs` keeps only legacy C ABI / explicit provider keep lanes
- compare/debug residue is thin enough that it now lives only in `ll_emit_bridge.rs`; the separate `hako_ll_driver.rs` helper has been retired
