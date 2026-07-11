---
Status: Landed
Date: 2026-04-25
Scope: Repair the dev-gate MapLookup fusion const-fold smoke.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - apps/tests/mir_shape_guard/maplookup_fusion_const_fold_min_v1.mir.json
  - tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 291x-236 MapLookup Fusion Smoke Repair Card

## Goal

Make the `dev_gate quick` MapLookup fusion const-fold smoke runnable again.

The old smoke generated MIR from `bench_kilo_leaf_map_getset_has.hako` and then
compiled the whole benchmark. That source-derived MIR now contains setup shapes
outside the current pure-first backend recipe, so the smoke stopped before the
MapLookup fusion contract was exercised:

```text
unsupported pure shape for current backend recipe
```

## Boundary

- Do not change MapLookup fusion lowering.
- Do not weaken the const-fold contract.
- Do not re-enable harness fallback.
- Do not use source-benchmark shape as a hidden dependency for this structural
  guard.

## Repair

- Add a minimal MIR fixture that contains exactly the MapLookupSameKey get/has
  pair plus required route metadata.
- Point the smoke at that fixture.
- Keep the lowered-entry IR checks that reject runtime get/has/probe calls.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
bash tools/checks/map_lookup_fusion_reader_boundary_guard.sh
bash tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed in 2026-04-25 smoke repair slice.

- Added `maplookup_fusion_const_fold_min_v1.mir.json` as a minimal structural
  fixture for the MapLookupSameKey const-fold contract.
- Updated the smoke to compile the fixture instead of the full source
  benchmark.
- Kept the lowered-entry IR checks for `nyash.runtime_data.get_hh`,
  `nyash.map.has_h`, and `nyash.map.probe_hi`.
- Added a direct check that both get/has results were emitted as folded
  constant `1` values.
