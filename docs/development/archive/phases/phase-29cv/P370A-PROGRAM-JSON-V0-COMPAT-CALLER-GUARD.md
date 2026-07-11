---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: lock active shell callers of the raw Program(JSON v0) compat emit helper
Related:
  - tools/lib/program_json_v0_compat.sh
  - tools/selfhost/lib/stage1_contract.sh
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/checks/program_json_v0_compat_caller_guard.sh
---

# P370A: Program JSON v0 Compat Caller Guard

## Intent

Prevent raw Program(JSON v0) compat emit from growing again while the remaining
keeper buckets are being replaced or archived.

`tools/lib/program_json_v0_compat.sh` remains live, but active shell access is
limited to the two current owner helpers:

- `tools/selfhost/lib/stage1_contract.sh`
- `tools/smokes/v2/lib/stageb_helpers.sh`

## Boundary

Allowed:

- add a no-growth guard for active shell callers of
  `program_json_v0_compat_emit_to_file`
- allow the owner helper implementation file itself
- allow archived engineering evidence outside active tools
- wire the guard into quick gate

Not allowed:

- delete `tools/lib/program_json_v0_compat.sh`
- delete Stage1 contract compat helper paths
- delete Stage-B fixture helper paths
- change `--emit-program-json-v0` Rust behavior

## Acceptance

```bash
bash tools/checks/program_json_v0_compat_caller_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
