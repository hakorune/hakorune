---
Status: Landed
Date: 2026-06-09
Scope: compact the restart surface so current pointers are easy to find.
Blocker: DOCS-POINTER-CLEANUP-AND-MIRROR-SLIM-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
---

# 296x-647 Docs Pointer Cleanup and Mirror Slim

## Purpose

Make the restart surface readable again. The active docs are too wide, and the
pointer chain is expensive to follow. This row does not open a new behavior
lane. It trims the current mirrors down to a thin, searchable surface and
points long history to archive notes.

## Decision

```text
keep thin:
  CURRENT_STATE.toml
  CURRENT_TASK.md
  05-Restart-Quick-Resume.md
  10-Now.md
  phase-296x/README.md
  296x-90-mimalloc-benchmark-taskboard.md
  workstreams/mimalloc-current.md

archive:
  long landed history
  retired lane detail
  pointer-heavy queue prose

inventory:
  use docs-pointer-inventory-2026-06-09.md as the compact map
```

## Required Outputs

- update `CURRENT_STATE.toml` to the cleanup lane
- slim the root/current restart docs to one-screen pointers
- keep the taskboard and workstream as thin mirrors with archive links
- keep long chronology in investigation/archive notes

## Acceptance

```text
current_state_pointer_guard=pass
restart_surface_thin=1
pointer_inventory_linked=1
long_history_copied_into_restart_docs=0
new_behavior_lane_opened=0
summary=ok
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

```text
next: resume the actual migration lane from the archived queue after the docs
cleanup mirrors are slim again
```
