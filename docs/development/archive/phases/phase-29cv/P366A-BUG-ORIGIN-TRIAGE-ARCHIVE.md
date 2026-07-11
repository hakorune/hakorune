---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive internal bug-origin triage helper from active tools/dev
Related:
  - tools/archive/legacy-selfhost/engineering/bug_origin_triage.sh
  - tools/checks/legacy_dev_utility_surface_guard.sh
  - tools/dev/README.md
---

# P366A: Bug Origin Triage Archive

## Intent

Remove the last docs/manual-only vm-family triage helper from active
`tools/dev`.

`bug_origin_triage.sh` was already classified by phase119/120/121 docs as
candidate-thin / retire-first. It has no smoke or check caller, and it is not a
front-door runtime surface.

The script remains available as archived engineering evidence under
`tools/archive/legacy-selfhost/engineering/`.

## Boundary

Allowed:

- archive the helper
- update docs that presented it as an active tool
- extend the legacy utility no-regrowth guard
- keep the script runnable from its archive path

Not allowed:

- change vm-family route semantics
- delete route-observability smokes
- change `route_env_probe` or direct-route sweep behavior

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/bug_origin_triage.sh
bash tools/checks/legacy_dev_utility_surface_guard.sh
bash tools/checks/tools_dev_surface_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
