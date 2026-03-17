# Host Provider MIR Builder

Scope: Rust-side current authority owner for `source / Program(JSON) -> MIR(JSON)` under [mir_builder.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder.rs).

Related SSOT:
- [CURRENT_TASK.md](/home/tomoaki/git/hakorune-selfhost/CURRENT_TASK.md)
- [selfhost-bootstrap-route-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/selfhost-bootstrap-route-ssot.md)
- [frontend-owner-proof-index.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/frontend-owner-proof-index.md)

## Current Owner Graph

Public entries:
- `program_json_to_mir_json_with_user_box_decls(program_json)`
- `source_to_mir_json(source_text)`

Test-only evidence seams:
- `program_json_to_mir_json(program_json)`
- `program_json_to_mir_json_with_imports(program_json, imports)`
- `source_to_program_and_mir_json(source_text)`

Current owner split:
- `Stage1ProgramJsonModuleHandoff`
  - parse `Program(JSON)` into `MirModule`
  - parse `user_box_decls`
  - materialize final `MirModule.metadata.user_box_decls`
  - emit guarded MIR JSON
- `SourceProgramJsonHandoff`
  - strict source -> Program(JSON) authority
  - delegates Program(JSON) -> MIR(JSON) to `Stage1ProgramJsonModuleHandoff`
- `Stage1UserBoxDecls`
  - explicit payload parse
  - compat fallback from defs/body
  - metadata projection for `MirModule`
- `module_to_mir_json(module)`
  - shared MIR JSON stop-line
  - implemented through `runner::mir_json_emit`

## Guardrails

- `mir_builder.rs` is the live authority owner for source/explicit Program(JSON) handoff.
- `lowering.rs` is test-only evidence; do not reopen it as the daily source of MIR emission.
- `module_to_mir_json(...)` is the shared Rust stop-line; push caller ownership above it, not MIR emitter ownership back outward.
- `user_box_decls` shaping stays owner-local here; do not duplicate it in bridge or runner compat lanes.
- fail-fast tags and temporary env guards stay same-owner here.

## Proofs

Primary proofs:
- `cargo test mir_builder -- --nocapture`
- `cargo test user_box_decls -- --nocapture`
- `cargo test program_json_to_mir_file -- --nocapture`
- `bash tools/dev/phase29ch_program_json_cold_compat_probe.sh`

See also:
- [frontend-owner-proof-index.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/frontend-owner-proof-index.md)
