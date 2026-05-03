---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: promote remaining guard-candidate tools/dev scripts to tools/checks
Related:
  - tools/checks/mir_builder_layer_dependency_guard.sh
  - tools/checks/loop_pattern_context_zero_guard.sh
  - tools/dev/README.md
  - docs/tools/check-scripts-index.md
---

# P364A: Dev Guard Promotion

## Intent

Move the remaining guard-candidate scripts out of `tools/dev`.

After P363A, the only files in `tools/dev` that were not helpers/probes were:

- `check_builder_layers.sh`
- `check_loop_pattern_context_allowlist.sh`

Both are lightweight fail-fast checks, so `tools/checks` is the correct owner.

## Boundary

Allowed:

- promote the two guard scripts to `tools/checks`
- rename them to describe their current contracts
- wire them into quick gate
- update the `tools/dev` surface inventory

Not allowed:

- change MIR builder layering policy
- change LoopPatternContext removal policy
- archive or modify explicit proof probes

## Guard

The promoted guards are:

- `tools/checks/mir_builder_layer_dependency_guard.sh`
- `tools/checks/loop_pattern_context_zero_guard.sh`

`tools/checks/tools_dev_surface_inventory_guard.sh` now keeps the smaller
`tools/dev` file set fixed.

## Acceptance

```bash
bash tools/checks/mir_builder_layer_dependency_guard.sh
bash tools/checks/loop_pattern_context_zero_guard.sh
bash tools/checks/tools_dev_surface_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
