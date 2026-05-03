---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: lock active shell callers of the Program(JSON)->MIR bridge helper
Related:
  - tools/selfhost/lib/program_json_mir_bridge.sh
  - tools/selfhost_exe_stageb.sh
  - tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
  - tools/checks/program_json_mir_bridge_caller_guard.sh
---

# P372A: Program JSON MIR Bridge Caller Guard

## Intent

Prevent the Program(JSON)->MIR bridge capsule from gaining new active shell
callers while the remaining replacement proofs are pending.

`tools/selfhost/lib/program_json_mir_bridge.sh` stays live, but active use is
limited to:

- `tools/selfhost_exe_stageb.sh` explicit `stageb-delegate` route
- `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` bridge proof

## Boundary

Allowed:

- add a no-growth guard for active shell callers of
  `program_json_mir_bridge_emit`
- allow archived engineering evidence outside active tools
- wire the guard into quick gate

Not allowed:

- delete `program_json_mir_bridge.sh`
- change bridge helper behavior
- change the `stageb-delegate` route
- replace `phase29cg` proof without the P106 MIR-first replacement going green

## Acceptance

```bash
bash tools/checks/program_json_mir_bridge_caller_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
