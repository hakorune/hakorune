---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: lock active shell callers of the Stage-B Program(JSON) stdout capture helper
Related:
  - tools/selfhost/lib/stageb_program_json_capture.sh
  - tools/hakorune_emit_mir.sh
  - tools/selfhost_exe_stageb.sh
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/checks/stageb_program_json_capture_caller_guard.sh
---

# P373A: Stage-B Program JSON Capture Caller Guard

## Intent

Prevent the Stage-B Program(JSON) stdout capture capsule from gaining new active
shell callers while Program(JSON v0) compatibility keepers are still being
closed out.

`tools/selfhost/lib/stageb_program_json_capture.sh` stays live, but active use is
limited to the central MIR emit / Stage-B helper surfaces:

- `tools/hakorune_emit_mir.sh`
- `tools/selfhost_exe_stageb.sh`
- `tools/smokes/v2/lib/stageb_helpers.sh`

## Boundary

Allowed:

- add a no-growth guard for active shell callers of
  `stageb_program_json_extract_from_stdin`
- prune dead helper sourcing from a smoke that exits before running the canary
- allow archived engineering evidence outside active tools
- wire the guard into quick gate

Not allowed:

- delete `stageb_program_json_capture.sh`
- change capture parsing behavior
- add new Stage-B Program(JSON) callers outside the central helper surfaces
- replace remaining Program(JSON v0) keepers without their MIR-first proof

## Acceptance

```bash
bash tools/checks/stageb_program_json_capture_caller_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
