# Phase 197x: optimization pointer inventory sync

Status: Landed

Purpose
- inventory current optimization docs after `phase196x`
- sync current-pointer wording without changing optimization semantics

Scope
- update `CURRENT_TASK.md`, `10-Now.md`, `15-Workstream-Map.md`, `phases/README.md`, and the optimization roadmap immediate-read pointer
- keep this cut docs-only

Non-goals
- no DCE widening
- no string / sum / user-box semantic change
- no roadmap layer reorder beyond pointer sync

Result
- current docs agree that the immediate next cut is `semantic simplification bundle lane B0`
- the one-screen work map now uses layer/lane wording for `Front` and `Blocker`
- phase index duplicates and stale timestamps are cleaned up
