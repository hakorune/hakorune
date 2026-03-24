# Host Provider MIR Builder

Scope: Rust-side current authority façade for `source / Program(JSON) -> MIR(JSON)` under [mir_builder.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder.rs); owner-local handoff objects now live in `handoff.rs` and `user_box_decls` shaping lives in `decls.rs`.

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
- `mir_builder.rs`
  - public façade
  - shared `module_to_mir_json(...)` stop-line
- `handoff.rs`
  - source / Program(JSON) handoff objects
  - typed Program(JSON) input/value seam and source-route authority/output split above the shared stop-line
  - strict source authority and output projection split
  - module-handoff and finalized-module emit split
- `decls.rs`
  - explicit payload shaping from parsed Program(JSON) values
  - compat fallback from defs/body
  - metadata projection for `MirModule`
- `Stage1ProgramJsonInput`
  - raw Program(JSON) text owner for the live explicit route
  - parses typed Program(JSON) value and module handoff above `module_to_mir_json(...)`
- `Stage1ProgramJsonValue`
  - typed parsed Program(JSON) value
  - resolves `user_box_decls` without reopening the live string-parse seam in `decls.rs`
- `Stage1ProgramJsonModuleHandoff`
  - combine parsed `MirModule` + typed `user_box_decls`
  - delegate metadata finalize / guarded emit
- `Stage1FinalizedMirModule`
  - materialize final `MirModule.metadata.user_box_decls`
  - own plain/guarded MIR JSON emission
- `SourceProgramJsonAuthority`
  - strict source -> Program(JSON) authority
  - yields the output-projection owner instead of mixing emit projection inline
- `SourceProgramJsonOutputHandoff`
  - owns the strict source route's `program_json` payload
  - delegates Program(JSON) -> MIR(JSON) to `Stage1ProgramJsonModuleHandoff`
- `Stage1UserBoxDecls`
  - explicit payload parse
  - compat fallback from defs/body
  - metadata projection for `MirModule`
- `module_to_mir_json(module)`
  - shared MIR JSON stop-line
  - implemented through `runner::mir_json_emit`

## Guardrails

- `mir_builder.rs` is the live authority façade for source/explicit Program(JSON) handoff.
- `handoff.rs` keeps the owner-local handoff objects; `decls.rs` keeps `user_box_decls` shaping.
- the live explicit route now enters `decls.rs` through `Stage1ProgramJsonInput` / `Stage1ProgramJsonValue`; do not move the primary string-parse seam back into `decls.rs`.
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
