---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive completed phase29ck pre-perf diagnostics probes from active tools/dev
Related:
  - docs/development/current/main/phases/phase-29ck/P7-PRE-PERF-RUNWAY-TASK-PACK.md
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/tools/check-scripts-index.md
  - tools/checks/phase29ck_preperf_probe_surface_guard.sh
  - tools/archive/legacy-selfhost/engineering/README.md
---

# P355A: Phase29ck Pre-Perf Probe Archive

## Intent

Continue the active dev-surface cleanup after P354A.

`phase29ck_backend_recipe_profile_probe.sh` and
`phase29ck_boundary_fallback_inventory_probe.sh` were completion evidence for
the phase29ck pre-perf runway. P7 now says there is no active pre-perf runway
front, and these two scripts have no active smoke/tool callers outside docs.

## Archived

- `tools/archive/legacy-selfhost/engineering/phase29ck_backend_recipe_profile_probe.sh`
- `tools/archive/legacy-selfhost/engineering/phase29ck_boundary_fallback_inventory_probe.sh`

Both archived scripts keep runnable repo-root resolution from the archive
bucket.

## Boundary

Allowed:

- move completed pre-perf diagnostics probes to archived engineering evidence
- update P7 acceptance references to archived paths
- add a no-regrowth guard for the archived active paths

Not allowed:

- archive `phase29ck_boundary_explicit_compat_probe.sh`
- archive `phase29ck_boundary_historical_alias_probe.sh`
- archive `phase29ck_stage1_mir_dialect_probe.sh`
- change boundary route behavior or Stage1 MIR dialect behavior

## Replacement

Live proof remains in:

- `tools/dev/phase29ck_boundary_explicit_compat_probe.sh`
- `tools/dev/phase29ck_boundary_historical_alias_probe.sh`
- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh`
- phase29ck boundary smokes under `tools/smokes/v2/profiles/integration/phase29ck_boundary/`

## Guard

`tools/checks/phase29ck_preperf_probe_surface_guard.sh` fails if the archived
pre-perf diagnostics probes return to active `tools/dev`, and confirms the live
compat/dialect keepers still exist.

## Acceptance

```bash
tools/checks/phase29ck_preperf_probe_surface_guard.sh
bash -n tools/checks/phase29ck_preperf_probe_surface_guard.sh tools/checks/dev_gate.sh \
  tools/archive/legacy-selfhost/engineering/phase29ck_backend_recipe_profile_probe.sh \
  tools/archive/legacy-selfhost/engineering/phase29ck_boundary_fallback_inventory_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
