---
Status: Active
Date: 2026-07-28
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
| Scope | `active_lane_status` in `CURRENT_STATE.toml` |

## Current Read

1. `docs/development/current/main/CURRENT_STATE.toml`
2. the file named by `latest_workstream_card`
3. the file named by `latest_card_path`
4. the file named by `method_anchor`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Immediate Sequence

1. Run `bash tools/checks/current_state_pointer_guard.sh`.
2. Work only on the exact current production replacement cell.
3. Switch the named production caller and delete the selected old path.
4. Prove parity after cutover; do not add a disconnected route chain.

## Parked Resume

```text
Stage-B / Ownership / Language v1 / selfhost:
  parked until CURRENT_STATE.toml explicitly reopens them
```

Optimization, allocator replacement, provider activation, and broad selfhost
authority remain parked unless `CURRENT_STATE.toml` explicitly reopens them.
