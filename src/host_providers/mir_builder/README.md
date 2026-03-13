# Host Provider MIR Builder

Scope: Rust-side current authority / lowering owner under `src/host_providers/mir_builder.rs`.

## Responsibility Split

- `mir_builder.rs`
  - thin public facade for the current Rust-owned provider surface
  - keeps shared fail-fast / trace / temp-path helpers
  - now also owns the shared `user_box_decls` shaping for the source and explicit Program(JSON) routes
  - keeps plain `program_json_to_mir_json(...)` and imports-bearing lowering as test-only seams; live cross-crate callers stay on `source_to_mir_json(...)` and `program_json_to_mir_json_with_user_box_decls(...)`
  - keeps `program_json_to_mir_json_with_imports(...)` test-only; live cross-crate callers should not depend on imports-bearing Program(JSON) lowering here
- `mir_builder/lowering.rs`
  - thin lowering facade + shared parse/emit helpers
  - now also owns the exact `Program(JSON v0) -> MIR(JSON)` lowering leaf directly
  - keeps imports-bearing lowering as a test-only seam; live lowering is the plain `program_json_to_mir_json(...)` path
- `mir_builder.rs::module_to_mir_json(...)`
  - shared MIR(JSON) emission seam
  - runtime/plugin imports route reuses this seam without staying a live caller of `lowering.rs`
- `mir_builder/lowering/ast_json.rs`
  - legacy AST JSON compat route owner
  - treat this as compat keep, not as the primary pure-`.hako` blocker

## Guardrails

- treat `mir_builder.rs` as the current source-route handoff + shared `user_box_decls` shaping owner
- treat `lowering.rs` as the current Rust-owned Program(JSON)->MIR lowering owner
- treat runtime/plugin `env.mirbuilder.emit` as a separate keep that now bypasses `lowering.rs`
- keep `source_to_program_and_mir_json(...)` test-only in the façade; cross-crate source surfaces should stay on `source_to_mir_json(...)`
- do not widen `.hako` workaround contracts here
- keep fail-fast tags and temp-path policy owner-local to this cluster
