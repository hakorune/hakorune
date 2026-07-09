---
Status: Active
Date: 2026-07-10
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
2. `docs/development/current/main/workstreams/language-v1-convergence-current.md`
3. the file named by `latest_card_path`
4. `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`
5. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Immediate Sequence

1. Run `bash tools/checks/current_state_pointer_guard.sh`.
2. Work only on the current macro row.
3. Keep parser/runtime behavior unchanged during the Constitution row.
4. Advance through the ordered language-v1 workstream without creating
   inventory, rerun, or consultation cards.

## Parked Resume

```text
MirBuilder resume:
  MIRBUILDER-MAPSTORE-ROUTE-POLICY-KEY-VALUE-DOMAIN-BOXSHAPE-001

resume gate:
  LANGV1-CONFORMANCE-CLOSEOUT-001
```

Optimization, allocator replacement, provider activation, and broad selfhost
authority remain parked unless `CURRENT_STATE.toml` explicitly reopens them.
