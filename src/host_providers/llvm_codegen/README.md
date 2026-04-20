# llvm_codegen

Thin Rust bridge for backend object emission.

## Responsibility split

- `route.rs`
  - legacy compare/archive selection only
  - does not re-decide MIR acceptance
- `ll_emit_compare_source.rs`
  - archive-later compare source rendering
  - MIR(JSON) to compare-driver `.hako` source rendering only; temp-path file materialization is handled by `transport_paths.rs` / `transport_io.rs` and orchestration stays in the driver
- `ll_emit_compare_driver.rs`
  - archive-later compare/debug orchestration only
  - owns VM spawn plus stdout/LL extraction local
- `provider_keep.rs`
  - archive-later explicit provider keep lanes
  - `ny-llvmc` / `llvmlite` path resolution and object emission helpers only
- `mir_json_text_object.rs`
  - explicit `MIR(JSON text) -> object path` backend boundary
  - Rust-side text object emission chokepoint for monitor-only proof lanes
  - normalizes input once, then delegates route selection without owning MIR acceptance
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
- daily object emit owner is `ny-llvmc --emit obj`
  - current mainline object emit does not require `tools/llvmlite_harness.py`
- llvmlite object emit is explicit compat/probe keep only
  - compat helper writes a temp MIR JSON file and spawns `tools/llvmlite_harness.py --in <mir.json> --out <obj.o>`
  - no Rust-side MIR JSON reparse or legacy front-door round-trip
- `normalize.rs`
  - backend input validation / JSON normalization
- `defaults.rs`
  - transport defaults only

## Current policy

- mainline owner is being cut over shape-by-shape from legacy C `.inc` to `.hako ll emitter`
- route policy ownership lives in `BackendRecipeBox` and the LLVM boundary-lock SSOT; `llvm_codegen` stays transport-only and must not be read as the daily policy owner
- compare lane is explicit bridge evidence, not a permanent default route; the proof smoke now runs from `phase29x-derust-archive.txt`
- canonical seam stays MIR; do not reopen `AST -> LLVM` direct lowering here
- current tool seam is now `.ll` text
- `compile_json_path` has been retired from code; flipped `.hako ll emitter` daily profiles stop at `ll_text_to_object(...)`
- launcher/mainline transport cut is landed; `route.rs` is now compare/archive-only; `transport_paths.rs` and `transport_io.rs` own the remaining temp-path helpers; `provider_keep.rs` owns explicit provider keep lanes plus provider path resolution; `capi_transport.rs` owns explicit CAPI helpers
- compare/debug residue is now split: `ll_emit_compare_source.rs` owns source rendering, `ll_emit_compare_driver.rs` owns orchestration plus VM spawn and stdout/LL extraction, `provider_keep.rs` owns explicit provider keep lanes plus provider path resolution, and the separate `hako_ll_driver.rs` / `ll_emit_bridge.rs` helpers have been retired
- explicit legacy helper deletion is landed; the root facade stays thin and daily code only stops at `compile_ll_text(...)` / `ll_text_to_object(...)`
- mainline object emit now goes through `ny-llvmc --emit obj`; the llvmlite keep lane stays explicit only
- direct runtime caller retirement for the file-based `mir_json_file_to_object(...)` front door is landed; remaining text object emission is carried by `mir_json_text_object.rs`
- the Rust compat chokepoint and compiled-stage1 surrogate both use the shared MIR JSON text object boundary; there is no remaining explicit legacy helper module
