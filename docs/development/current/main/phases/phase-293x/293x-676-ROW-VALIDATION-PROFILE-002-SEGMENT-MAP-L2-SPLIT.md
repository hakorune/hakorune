---
Status: landed
Decision: accepted
Date: 2026-05-18
---

# 293x-676 ROW-VALIDATION-PROFILE-002 Segment Map L2 Split

## Decision

Make the segment-map readiness family the first manifest-selectable validation
split. Public guard entrypoints remain full L3 by default, while manifest
`--level L2` can run static checks, VM proof, MIR JSON assertions, and route
preflight without building/running the EXE.

## Owner

```text
tools/checks/lib/manifest_runner.py
tools/checks/lib/pure_first_exe_guard.sh
tools/checks/proof_apps.toml
tools/checks/k2_wide_hako_alloc_segment_allocation_blocked_substrate_matrix_guard.sh
tools/checks/k2_wide_hako_alloc_segment_map_scalar_lookup_boundary_inventory_guard.sh
tools/checks/k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh
docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md
```

## Scope

- Add manifest runner `--level` command selection.
- Add optional `cmd_l2` manifest commands.
- Add shared pure-first guard level helpers.
- Split `MIMAP-149A`, `MIMAP-151A`, and `MIMAP-153A` guards:
  - no args: existing full L3 behavior;
  - `--level L2`: static + VM + MIR JSON + route preflight, no EXE.

## Stop Lines

- Do not change default public guard behavior.
- Do not remove pure-first EXE evidence from any existing no-arg guard.
- Do not add level selection to `dev_gate.sh` or allocator-wide.
- Do not bulk-split unrelated guards.
- Do not change allocator behavior, compiler acceptance, route vocabulary, or
  backend lowering.

## Evidence

```text
bash tools/checks/run_proof_app.sh --validation-profile scalar-mir --level L2 --dry-run
bash tools/checks/run_proof_app.sh --row-kind inventory --level L2 --dry-run
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_map_scalar_lookup_boundary_inventory_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

`MIMAP-154A` remains current. Further guard splitting should happen one family
at a time, only after the family has manifest metadata and a closeout pack.
