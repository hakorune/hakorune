---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive unused MirBuilder dev env shell helper from active tools/dev
Related:
  - tools/smokes/v2/lib/test_runner.sh
  - tools/checks/legacy_dev_utility_surface_guard.sh
  - tools/archive/legacy-selfhost/engineering/README.md
  - docs/development/current/main/phases/phase-29cv/README.md
---

# P362A: MirBuilder Dev Env Shell Archive

## Intent

Remove a confusing duplicate active entry.

`tools/dev/enable_mirbuilder_dev_env.sh` had no active callers. The current
MirBuilder smoke profile owner is the shell function
`enable_mirbuilder_dev_env()` in `tools/smokes/v2/lib/test_runner.sh`, which is
called directly by current smoke profiles.

The unused shell helper now lives under
`tools/archive/legacy-selfhost/engineering/` as historical evidence.

## Boundary

Allowed:

- archive the unused shell helper
- extend the legacy dev utility no-regrowth guard
- document that the smoke-library function remains the active owner

Not allowed:

- change the smoke-library `enable_mirbuilder_dev_env()` function
- change smoke profile environment semantics
- archive active MirBuilder or phase2160 smoke fixtures

## Guard

`tools/checks/legacy_dev_utility_surface_guard.sh` now also fails if
`tools/dev/enable_mirbuilder_dev_env.sh` returns to active `tools/dev`, and
confirms the archived evidence remains present.

## Acceptance

```bash
bash tools/checks/legacy_dev_utility_surface_guard.sh
bash -n tools/checks/legacy_dev_utility_surface_guard.sh \
  tools/archive/legacy-selfhost/engineering/enable_mirbuilder_dev_env.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
