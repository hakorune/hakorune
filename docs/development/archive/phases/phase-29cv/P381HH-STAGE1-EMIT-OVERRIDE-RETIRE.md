# P381HH Stage1 Emit Override Retire

Date: 2026-05-07
Scope: retire the stale `BuildBox._emit_program_json_from_scan_src/1`
Stage1 emit override seam after active source ownership moved to the public
BuildBox/Stage1 authority wrappers.

## Context

`src/mir/global_call_route_plan.rs` still carried a
`GlobalCallLoweringOverride::Stage1EmitProgramJson` branch for
`BuildBox._emit_program_json_from_scan_src/1`.

That symbol no longer exists in active source ownership:

- current Hako ownership uses `BuildBox.emit_program_json_v0/2`
- the active Stage1 raw wrapper is
  `Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1`
- repo searches found the old symbol only in Rust route/canonicalize tests and
  the stale override branch itself

So the old `_emit_program_json_from_scan_src/1` route had become
override-only residue rather than a live authority seam.

## Change

- Removed the stale Stage1 emit override branch for
  `BuildBox._emit_program_json_from_scan_src/1`.
- Repointed directly-related tests to the active Stage1 raw wrapper/public seam.
- Updated the Program(JSON v0) lowering-plan SSOT note so
  `program_json_emit_body` refers only to the live Stage1 raw wrapper that calls
  `BuildBox.emit_program_json_v0(source, null)`.

## Result

The Stage1 Program(JSON v0) override path now matches current source ownership:

- live authority stays on `BuildBox.emit_program_json_v0/2`
- live wrapper proof stays on
  `Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1`
- the old scan-src override name no longer lingers as an active route-plan seam

## Validation

```bash
cargo test -q refresh_module_global_call_routes_marks_program_json_emit_body_direct_target
cargo test -q build_mir_json_root_emits_direct_plan_for_program_json_emit_body
cargo test -q stage1_buildbox_emit_program_json_null_opts_stays_global_call
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
