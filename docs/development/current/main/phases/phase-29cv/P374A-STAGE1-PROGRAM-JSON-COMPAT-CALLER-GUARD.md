---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: lock active shell callers of the Stage1 Program(JSON) compat execution helper
Related:
  - tools/selfhost/lib/stage1_contract.sh
  - tools/selfhost/compat/run_stage1_cli.sh
  - tools/dev/phase29ch_program_json_compat_route_probe.sh
  - tools/checks/stage1_program_json_compat_caller_guard.sh
---

# P374A: Stage1 Program JSON Compat Caller Guard

## Intent

Prevent the Stage1 Program(JSON) compat execution helper from becoming a
general shell route again.

`stage1_contract_exec_program_json_compat` stays live only as the exact helper
behind `tools/dev/phase29ch_program_json_compat_route_probe.sh`.

## Boundary

Allowed:

- add a no-growth guard for active shell callers of
  `stage1_contract_exec_program_json_compat`
- remove direct helper-name guidance from the retired `run_stage1_cli.sh`
  `--from-program-json` path
- wire the guard into quick gate

Not allowed:

- delete `stage1_contract.sh`
- change Stage1 contract execution behavior
- add a new Program(JSON) shell route
- retire `phase29ch_program_json_compat_route_probe.sh` without MIR-first
  replacement proof

## Acceptance

```bash
bash tools/checks/stage1_program_json_compat_caller_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
