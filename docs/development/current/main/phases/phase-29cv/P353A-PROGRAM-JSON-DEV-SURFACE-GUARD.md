---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: executable no-regrowth guard for active Program(JSON v0) dev surface
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/tools/check-scripts-index.md
  - tools/checks/program_json_dev_surface_guard.sh
---

# P353A: Program(JSON) Dev Surface Guard

## Intent

Lock the cleanup frontier after P347A-P352A.

The diagnostics-only Program(JSON) probes and the empty `tools/dev/program_json_v0/`
capsule marker are now archived. The remaining active `tools/dev` Program(JSON)
surface is limited to explicit keepers:

- `tools/dev/phase29ch_program_json_compat_route_probe.sh`
- `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`

This card adds a lightweight guard so archived diagnostics probes do not return
to active `tools/dev` without a keeper card.

## Boundary

Allowed:

- add an executable no-regrowth guard for archived Program(JSON) dev probes
- wire the guard into `tools/checks/dev_gate.sh quick`
- update check-script docs and current phase pointers

Not allowed:

- change Stage1 contract behavior
- change compat route probe behavior
- change bridge or fixture keepers
- delete Rust/public Program(JSON v0) surface

## Implementation

- Added `tools/checks/program_json_dev_surface_guard.sh`.
- The guard fails if:
  - `tools/dev/program_json_v0/` exists again
  - any archived Program(JSON) diagnostics probe returns under `tools/dev`
- The guard intentionally allows the two remaining active keepers.

## Acceptance

```bash
tools/checks/program_json_dev_surface_guard.sh
bash -n tools/checks/program_json_dev_surface_guard.sh tools/checks/dev_gate.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
