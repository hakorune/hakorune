# llvm_codegen

Thin Rust bridge for backend object emission.

## Responsibility split

- `route.rs`
  - legacy compare/archive selection only
  - does not re-decide MIR acceptance
- `ll_emit_bridge.rs`
  - compare-only bridge orchestration
  - keeps the orchestration thin while `ll_emit_compare_driver.rs` owns compare/debug orchestration and `ll_emit_compare_source.rs` owns compare source materialization residue
  - delegates `.ll -> verify -> llc -> .o` to `ll_tool_driver.rs`
- `ll_emit_compare_source.rs`
  - archive-later compare source materialization
  - MIR(JSON) to compare-driver `.hako` source rendering only
- `ll_emit_compare_driver.rs`
  - archive-later compare/debug orchestration
  - VM execution / stdout contract parse / `.ll` extraction
- `ll_tool_driver.rs`
  - thin LLVM tool seam
  - `.ll` text or file -> verifier -> `llc` -> `.o`
- `transport.rs`
  - legacy C ABI transport
  - explicit provider keep lanes (`ny-llvmc`, `llvmlite`)
- `legacy_json.rs`
  - legacy MIR(JSON) front door for compare/archive callers
  - routes through `route.rs` and stays out of the daily root-first tool seam
  - direct runtime callers have been moved onto a helper alias; keep this module archive-later until the compare bridge itself is thinned further
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
- compare/debug residue is now split: `ll_emit_compare_source.rs` owns source materialization, `ll_emit_compare_driver.rs` owns orchestration / VM / stdout parse, `ll_emit_bridge.rs` stays orchestration-only and the separate `hako_ll_driver.rs` helper has been retired
- legacy JSON wrapper residue now lives in `legacy_json.rs`; the root facade stays thin and daily code only stops at `compile_ll_text(...)` / `ll_text_to_object(...)`
- direct runtime caller retirement for `mir_json_to_object(...)` is landed; the remaining thin task is compare source materialization / compare driver residue retirement
