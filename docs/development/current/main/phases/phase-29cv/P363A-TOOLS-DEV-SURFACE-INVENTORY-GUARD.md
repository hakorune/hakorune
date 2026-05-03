---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: lock remaining tools/dev active surface after archive/promote cleanup
Related:
  - tools/dev/README.md
  - tools/checks/tools_dev_surface_inventory_guard.sh
  - docs/tools/check-scripts-index.md
  - docs/development/current/main/phases/phase-29cv/README.md
---

# P363A: Tools Dev Surface Inventory Guard

## Intent

Stop `tools/dev` from growing back after the P353A-P362A cleanup series.

At this point, remaining files are mostly explicit keepers or active developer
helpers. The next useful cleanup is not another blind archive pass; it is an
executable inventory that makes each remaining file justify its owner and
removal path.

## Boundary

Allowed:

- add `tools/dev/README.md` as the active surface inventory
- add a quick-gate guard that fails on unclassified file-set drift
- classify remaining files as active helpers, guard candidates, paired helpers,
  or explicit keepers

Not allowed:

- archive remaining explicit keeper probes without replacement proof
- change helper/probe behavior
- move guard-candidate scripts in the same card

## Guard

`tools/checks/tools_dev_surface_inventory_guard.sh` compares the top-level
`tools/dev` file set against the manifest and requires every file to appear in
`tools/dev/README.md`.

## Acceptance

```bash
bash tools/checks/tools_dev_surface_inventory_guard.sh
bash -n tools/checks/tools_dev_surface_inventory_guard.sh tools/checks/dev_gate.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
