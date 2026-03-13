# Host Provider MIR Builder

Scope: Rust-side current authority / lowering owner under `src/host_providers/mir_builder.rs`.

## Responsibility Split

- `mir_builder.rs`
  - thin public facade for the current Rust-owned provider surface
  - keeps shared fail-fast / trace / temp-path helpers
- `mir_builder/authority.rs`
  - current Rust authority path
  - `source -> Program(JSON v0) -> MIR(JSON)` owner-local chain
  - source-route authority still lives here, but `user_box_decls` injection is now shared through `mir_builder/user_box_decls.rs`
  - no longer owns the transient Program(JSON) helper; it exposes only the live `source_to_mir_json(...)` adapter
- `mir_builder/lowering.rs`
  - thin lowering facade + shared parse/emit helpers
- `mir_builder/user_box_decls.rs`
  - shared `user_box_decls` owner for source authority and explicit Program(JSON) kernel routes
  - also owns the transient Program(JSON) materialization handoff for the live source route
- `mir_builder/lowering/program_json.rs`
  - current `Program(JSON v0) -> MIR(JSON)` lowering owner
  - this is the real pure-`.hako` blocker inside the lowering half
- `mir_builder.rs::module_to_mir_json(...)`
  - shared MIR(JSON) emission seam
  - runtime/plugin imports route reuses this seam without staying a live caller of `lowering/program_json.rs`
- `mir_builder/lowering/ast_json.rs`
  - legacy AST JSON compat route owner
  - treat this as compat keep, not as the primary pure-`.hako` blocker

## Guardrails

- treat `authority.rs` as the current primary de-Rust blocker
- treat `lowering/program_json.rs` as the current Rust-owned Program(JSON)->MIR lowering owner
- treat runtime/plugin `env.mirbuilder.emit` as a separate keep that now bypasses `lowering/program_json.rs`
- keep `source_to_program_and_mir_json(...)` test-only in the façade; cross-crate source surfaces should stay on `source_to_mir_json(...)`
- do not widen `.hako` workaround contracts here
- keep fail-fast tags and temp-path policy owner-local to this cluster
