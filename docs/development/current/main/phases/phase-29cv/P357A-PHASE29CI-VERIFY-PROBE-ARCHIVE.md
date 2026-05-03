---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive phase29ci W17 verify proof from active tools/dev
Related:
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - docs/development/current/main/phases/archive/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29cv/README.md
  - tools/checks/phase29ci_verify_probe_surface_guard.sh
  - tools/archive/legacy-selfhost/engineering/README.md
---

# P357A: Phase29ci Verify Probe Archive

## Intent

Move the landed W17 exact verify proof out of active `tools/dev`.

`phase29ci_verify_primary_core_route_probe.sh` is referenced only as the landed
W17 proof in phase29ci docs. There are no active smoke/tool callers. It should
remain runnable as archived engineering evidence, not as an active dev entry.

## Archived

- `tools/archive/legacy-selfhost/engineering/phase29ci_verify_primary_core_route_probe.sh`

The script already resolves repo root through `git rev-parse`, so no archive
root rewrite is required.

## Boundary

Allowed:

- move the W17 proof script to archived engineering evidence
- update phase29ci docs to the archived path
- add a no-regrowth guard for the old active path

Not allowed:

- change `tools/smokes/v2/lib/test_runner.sh`
- change verify route behavior
- change phase2044/phase2160 monitor canaries

## Guard

`tools/checks/phase29ci_verify_probe_surface_guard.sh` fails if the archived
W17 verify proof returns to active `tools/dev`.

## Acceptance

```bash
tools/checks/phase29ci_verify_probe_surface_guard.sh
bash -n tools/checks/phase29ci_verify_probe_surface_guard.sh tools/checks/dev_gate.sh \
  tools/archive/legacy-selfhost/engineering/phase29ci_verify_primary_core_route_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
