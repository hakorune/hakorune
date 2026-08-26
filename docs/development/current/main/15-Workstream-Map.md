---
Status: Active
Date: 2026-08-26
Scope: one-screen current lane and parked-resume map.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Workstream Map

## Current Lane

| Item | Source |
| --- | --- |
| Active lane | `active_lane` in `CURRENT_STATE.toml` |
| Workstream | `latest_workstream_card` in `CURRENT_STATE.toml` |
| Front | `latest_card_path` in `CURRENT_STATE.toml` |
| Blocker | `current_blocker_token` in `CURRENT_STATE.toml` |
| Scope | `latest_card_summary` in `CURRENT_STATE.toml` |
| Exact task order | file named by `latest_workstream_card` |

## Current Read

1. `docs/development/current/main/CURRENT_STATE.toml`
2. the file named by `latest_workstream_card`
3. the file named by `latest_card_path`
4. the file named by `method_anchor`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Immediate Sequence

1. Run `bash tools/checks/current_state_pointer_guard.sh`.
2. Read `work_mode` before choosing an action.
3. When `work_mode = "design_stop"`, perform only the selected census/design
   row and stop before code, fixtures, caller switch, or old-path deletion.
4. When `work_mode = "fast"`, implement only the named bounded production
   cell, switch its caller, delete its selected old edge, and prove parity.
5. When `work_mode = "closeout"`, classify evidence and synchronize the
   owning docs/guard/commit without widening the implementation.

## Parked Resume

```text
Stage-B / Ownership / Language v1 / selfhost:
  parked until CURRENT_STATE.toml explicitly reopens them
```

Optimization, allocator replacement, provider activation, and broad selfhost
authority remain parked unless `CURRENT_STATE.toml` explicitly reopens them.
