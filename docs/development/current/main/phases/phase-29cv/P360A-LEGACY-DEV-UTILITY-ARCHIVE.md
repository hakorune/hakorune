---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive old one-shot/dev-env utilities from active tools/dev
Related:
  - tools/checks/legacy_dev_utility_surface_guard.sh
  - tools/archive/legacy-selfhost/engineering/README.md
  - docs/development/current/main/phases/phase-29cv/README.md
---

# P360A: Legacy Dev Utility Archive

## Intent

Continue the active `tools/dev` surface inventory with a no-behavior-change
cleanup slice.

Two utilities were active only because they still lived under `tools/dev`, not
because current smokes or developer gates call them:

- `enable_phase216_env.sh`: old Phase 21.6 Hakorune-only env helper
- `rename_nyash_to_hako.sh`: one-shot `.nyash` to `.hako` migration helper

Both now live under `tools/archive/legacy-selfhost/engineering/` as historical
evidence.

## Boundary

Allowed:

- move the old utilities out of active `tools/dev`
- keep archived scripts runnable from the archive bucket when practical
- add a no-regrowth guard for the old active paths

Not allowed:

- archive current dev sugar helpers
- archive `hako_preinclude.sh`, `hako_debug_run.sh`, or current Program(JSON)
  compat keepers
- change smoke env semantics or migration behavior
- touch the smoke-library `enable_mirbuilder_dev_env()` function

## Guard

`tools/checks/legacy_dev_utility_surface_guard.sh` fails if the archived
utilities return to active `tools/dev`, and confirms the archived evidence files
exist.

## Acceptance

```bash
bash tools/checks/legacy_dev_utility_surface_guard.sh
bash -n tools/checks/legacy_dev_utility_surface_guard.sh \
  tools/archive/legacy-selfhost/engineering/enable_phase216_env.sh \
  tools/archive/legacy-selfhost/engineering/rename_nyash_to_hako.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
