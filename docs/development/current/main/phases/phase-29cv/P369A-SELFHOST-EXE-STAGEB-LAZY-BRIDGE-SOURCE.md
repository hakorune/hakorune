---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: narrow Program(JSON) bridge helper loading in `tools/selfhost_exe_stageb.sh`
Related:
  - tools/selfhost_exe_stageb.sh
  - tools/selfhost/lib/program_json_mir_bridge.sh
  - tools/selfhost/lib/stageb_program_json_capture.sh
---

# P369A: Selfhost EXE Stage-B Lazy Bridge Source

## Intent

Keep the default `tools/selfhost_exe_stageb.sh` route MIR-first at load time.

The script defaults to `HAKORUNE_STAGE1_EMIT_ROUTE=direct`, but it still loaded
the Program(JSON)->MIR bridge and Stage-B Program(JSON) capture helpers before
route dispatch. Those helpers are needed only by the explicit
`stageb-delegate` compat capsule.

## Boundary

Allowed:

- move Program(JSON) bridge/capture `source` calls behind a
  `stageb-delegate`-only loader
- preserve `direct` and `stageb-delegate` route behavior
- leave the bridge helper files live

Not allowed:

- delete `program_json_mir_bridge.sh`
- delete `stageb_program_json_capture.sh`
- change direct MIR emit options
- make `stageb-delegate` a default route

## Acceptance

```bash
bash -n tools/selfhost_exe_stageb.sh
HAKORUNE_STAGE1_EMIT_ROUTE=direct bash tools/selfhost_exe_stageb.sh apps/tests/hello_simple_llvm.hako -o /tmp/p369a_direct --run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
