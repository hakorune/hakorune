JoinIR → VM bridge layer  

Responsibilities:
- Convert normalized JoinIR modules into MIR for the Rust VM without changing semantics.
- Provide a thin runner helper that executes a JoinIR entry via the VM.
- Host experimental metadata-aware paths (Phase 40-1) behind clearly marked helpers.

Boundaries:
- No new control-flow semantics or heuristics here; this layer only maps structures already normalized by JoinIR.
- Keep type information minimal (MirType::Unknown) and avoid adding inference or guessing.
- Debug/diagnostic output must stay behind `NYASH_JOINIR_VM_BRIDGE_DEBUG=1`.

File layout:
- `mod.rs`: public surface + shared helpers (naming, error, logging)
- `convert.rs`: JoinIR→MIR lowering (functions/blocks/instructions)
- `runner.rs`: VM execution entry (`run_joinir_via_vm`)
- `meta.rs`: experimental metadata-aware conversion hooks
- `tests.rs`: bridge-specific unit tests (kept local to avoid cross-layer leakage)
