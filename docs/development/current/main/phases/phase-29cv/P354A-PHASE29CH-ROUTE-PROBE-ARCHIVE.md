---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive historical phase29ch route diagnostics probes from active tools/dev
Related:
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - tools/checks/phase29ch_route_probe_surface_guard.sh
  - tools/archive/legacy-selfhost/engineering/README.md
---

# P354A: Phase29ch Route Probe Archive

## Intent

Close the historical phase29ch route diagnostics surface in active `tools/dev`.

After P353A, active Program(JSON) dev surface is guarded. A broader route-probe
inventory shows the remaining phase29ch route probes are referenced by docs as
historical evidence only. They do not have active tool/smoke callers outside
docs, and they should not remain beside live dev keepers.

## Archived

- `phase29ch_bridge_bypass_probe.sh`
- `phase29ch_fixed_program_mir_repeat_probe.sh`
- `phase29ch_impossible_gate_probe.sh`
- `phase29ch_raw_mir_diff_probe.sh`
- `phase29ch_route_mode_matrix.sh`
- `phase29ch_same_route_repeat_probe.sh`
- `phase29ch_selfhost_source_route_bisect_probe.sh`
- `phase29ch_selfhost_source_route_helper_probe.sh`
- `phase29ch_source_route_direct_probe.sh`
- `phase29ch_source_route_materialize_probe.sh`
- `phase29ch_transient_boundary_probe.sh`

## Boundary

Allowed:

- move diagnostics-only phase29ch route probes to archived engineering evidence
- update phase evidence docs to point at archive paths
- add a no-regrowth guard for active `tools/dev/phase29ch_*.sh`

Not allowed:

- archive `tools/dev/phase29ch_program_json_compat_route_probe.sh`
- change Stage1 contract behavior
- change source/MIR compare policy
- change Stage2 bootstrap PHI proof

## Replacement

Live proof remains:

- `tools/dev/phase29ch_program_json_compat_route_probe.sh` for explicit
  Program(JSON) compat proof.
- `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` for the bridge/PHI
  capsule.
- `tools/selfhost/lib/stage1_contract.sh` and selfhost smokes for active
  Stage1 CLI contract behavior.

The archived route probes remain runnable as historical engineering evidence
from `tools/archive/legacy-selfhost/engineering/`.

## Guard

`tools/checks/phase29ch_route_probe_surface_guard.sh` fails if a new active
`tools/dev/phase29ch_*.sh` file appears without being the live compat keeper.

## Acceptance

```bash
tools/checks/phase29ch_route_probe_surface_guard.sh
bash -n tools/checks/phase29ch_route_probe_surface_guard.sh tools/checks/dev_gate.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
