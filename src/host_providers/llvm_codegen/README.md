# llvm_codegen

Thin Rust bridge for backend object emission.

## Responsibility split

- `route.rs`
  - legacy compare/archive selection only
  - does not re-decide MIR acceptance
- `ll_emit_compare_source.rs`
  - archive-later compare source materialization
  - MIR(JSON) to compare-driver `.hako` source rendering only
- `ll_emit_compare_driver.rs`
  - archive-later compare/debug orchestration only
  - delegates VM execution to `ll_emit_compare_vm.rs` and stdout extraction to `ll_emit_compare_stdout.rs`
- `ll_emit_compare_vm.rs`
  - archive-later compare VM helper
  - `NYASH_BIN` / current exe resolution and VM spawn only
- `ll_emit_compare_stdout.rs`
  - archive-later compare stdout helper
  - contract-line / `.ll` extraction only
- `provider_keep.rs`
  - archive-later explicit provider keep lanes
  - `ny-llvmc` / `llvmlite` object emission helpers only
- `capi_transport.rs`
  - explicit CAPI helper split from the legacy transport helper surface
  - compile/link CAPI helpers only
- `transport_paths.rs`
  - temp-path path resolution helpers only
- `transport_io.rs`
  - temp-path file I/O helpers only
- `ll_tool_driver.rs`
  - thin LLVM tool seam
  - `.ll` text or file -> verifier -> `llc` -> `.o`
- `legacy_json.rs`
  - legacy MIR(JSON) front door for compare/archive callers
  - routes through `route.rs` and stays out of the daily root-first tool seam
  - direct runtime callers have moved to the string-based `emit_object_from_mir_json(...)` front door; keep this module archive-later until the compare bridge itself is thinned further
- stage0 harness object emit is direct llvmlite keep lane
  - Rust helper writes a temp MIR JSON file and spawns `tools/llvmlite_harness.py --in <mir.json> --out <obj.o>`
  - no Rust-side MIR JSON reparse or legacy front-door round-trip
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
- launcher/mainline transport cut is landed; `route.rs` is now compare/archive-only; `transport_paths.rs` and `transport_io.rs` own the remaining temp-path helpers; `provider_keep.rs` owns explicit provider keep lanes; `capi_transport.rs` owns explicit CAPI helpers
- compare/debug residue is now split: `ll_emit_compare_source.rs` owns source materialization, `ll_emit_compare_driver.rs` owns orchestration, `ll_emit_compare_vm.rs` owns VM spawn, `ll_emit_compare_stdout.rs` owns stdout extraction, `provider_keep.rs` owns explicit provider keep lanes, and the separate `hako_ll_driver.rs` / `ll_emit_bridge.rs` helpers have been retired
- legacy JSON wrapper residue now lives in `legacy_json.rs`; the root facade stays thin and daily code only stops at `compile_ll_text(...)` / `ll_text_to_object(...)`
- stage0 object emit now goes straight from the Rust helper to `tools/llvmlite_harness.py`; the old Rust-side object-emit JSON round-trip is retired
- direct runtime caller retirement for the file-based `mir_json_file_to_object(...)` front door is landed; the remaining wrapper is the string-based `emit_object_from_mir_json(...)` compare/archive helper
