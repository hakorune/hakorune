---
Status: SSOT mirror
Date: 2026-08-10
Scope: one-screen restart pointer. Do not store current values or landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Now

This file is a thin mirror. Read the machine-readable authority directly:

```text
docs/development/current/main/CURRENT_STATE.toml
  -> active_lane
  -> work_mode
  -> current_execution_row
  -> current_blocker_token
  -> latest_workstream_card
  -> latest_card_path
  -> current_design_stop
  -> current_execution_design
```

Then follow `CURRENT_TASK.md` and the exact active card. If
`work_mode = "design_stop"`, stop before code, fixtures, route activation, or
fallback and close only the authority question named by
`current_blocker_token`.

Before editing, run `git status -sb` and preserve unrelated user changes.
