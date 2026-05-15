# 293x-385 MIRBUILDER-DIET-001 Boundary SSOT

Status: landed
Date: 2026-05-15

## Decision

Open a temporary MIR builder diet / FlowPlanner cleanup sidecar before
returning to `MIMAP-021C`.

This row is docs-first. It pins the ownership vocabulary and current pointers
so the next cleanup rows do not mix with allocator behavior.

## Scope

- Add `docs/development/current/main/design/mir-builder-diet-flowplanner-boundary-ssot.md`.
- Add `docs/development/current/main/phases/phase-293x/293x-mir-builder-diet-taskboard.md`.
- Park `MIMAP-021C` as the post-cleanup return row.
- Select `FLOWPLANNER-ENTRY-001` as the next active cleanup row.

## Stop Lines

- No source behavior changes.
- No physical crate split or directory move.
- No new control-flow acceptance shape.
- No allocator behavior row.
- No backend route or `.inc` matcher change.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
