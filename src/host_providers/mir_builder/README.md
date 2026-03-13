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
- `mir_builder/lowering.rs`
  - thin lowering facade + shared parse/emit helpers
- `mir_builder/user_box_decls.rs`
  - shared `user_box_decls` injection owner for source authority and explicit Program(JSON) kernel routes
- `mir_builder/lowering/program_json.rs`
  - current `Program(JSON v0) -> MIR(JSON)` lowering owner
  - this is the real pure-`.hako` blocker inside the lowering half
- `mir_builder/lowering/ast_json.rs`
  - legacy AST JSON compat route owner
  - treat this as compat keep, not as the primary pure-`.hako` blocker

## Guardrails

- treat `authority.rs` as the current primary de-Rust blocker
- treat `lowering/program_json.rs` as the current Rust-owned Program(JSON)->MIR lowering owner
- keep `source_to_program_and_mir_json(...)` test-only; cross-crate source surfaces should stay on `source_to_mir_json(...)`
- do not widen `.hako` workaround contracts here
- keep fail-fast tags and temp-path policy owner-local to this cluster
