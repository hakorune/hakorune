---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: archive old phase216/217 normalization bring-up canaries from active tools/dev
Related:
  - docs/development/normalization/ownership.md
  - docs/archive/roadmap/phases/phase-21.6-solidification/README.md
  - docs/archive/roadmap/phases/phase-21.6-solidification/CHECKLIST.md
  - docs/archive/roadmap/phases/phase-21.7-normalization/CHECKLIST.md
  - docs/development/current/main/phases/phase-29cv/README.md
  - tools/checks/phase216217_normalization_canary_surface_guard.sh
  - tools/archive/legacy-selfhost/engineering/README.md
---

# P358A: Phase216/217 Normalization Canary Archive

## Intent

Move old phase216/217 normalization bring-up canaries out of active `tools/dev`.

These scripts are historical evidence for the old Hakorune-only chain and
methodization bring-up. They have no active smoke/tool callers, while the
current dehang proof still lives in `phase2160_mirbuilder_module_load_probe.sh`.

## Archived

- `phase216_chain_canary.sh`
- `phase216_chain_canary_binop.sh`
- `phase216_chain_canary_binop_precedence_block.sh`
- `phase216_chain_canary_call.sh`
- `phase216_chain_canary_loop_undefined_block.sh`
- `phase216_chain_canary_return.sh`
- `phase216_direct_loop_progression_canary.sh`
- `phase217_method_norm_canary.sh`
- `phase217_methodize_canary.sh`
- `phase217_methodize_json_canary.sh`
- `phase217_methodize_json_strict.sh`

All scripts now live under `tools/archive/legacy-selfhost/engineering/` and keep
runnable repo-root resolution from that archive bucket.

## Boundary

Allowed:

- move historical phase216/217 bring-up canaries to archived engineering evidence
- update normalization/phase archive docs to archived paths
- add a no-regrowth guard for the old active paths

Not allowed:

- archive `tools/dev/phase2160_mirbuilder_module_load_probe.sh`
- change methodization or direct-route behavior
- change current phase29ci dehang proof semantics

## Guard

`tools/checks/phase216217_normalization_canary_surface_guard.sh` fails if the
archived phase216/217 canaries return to active `tools/dev`, and confirms the
current phase2160 dehang proof still exists.

## Acceptance

```bash
tools/checks/phase216217_normalization_canary_surface_guard.sh
bash -n tools/checks/phase216217_normalization_canary_surface_guard.sh tools/checks/dev_gate.sh \
  tools/archive/legacy-selfhost/engineering/phase216_chain_canary.sh \
  tools/archive/legacy-selfhost/engineering/phase217_methodize_json_strict.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
