---
Status: Landed
Date: 2026-04-26
Scope: Retire the residual metadata-absent direct MapBox.has sentinel and prune the final MIR-call method/box classifier rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-272-mir-call-maphas-surface-fallback-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-286-mir-call-maphas-residual-seam-cleanup-card.md
  - apps/tests/mir_shape_guard/map_has_no_metadata_min_v1.mir.json
  - tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-287 MIR-Call MapHas Sentinel Retirement Card

## Goal

Close the last CoreMethodContract `.inc` method/box classifier baseline by
retiring the supported metadata-absent direct `MapBox.has` boundary.

This card keeps the supported direct `MapBox.has` seed, but moves it to the
same MIR-owned route metadata contract used by the rest of the has family.

## Decision

Metadata-absent direct `MapBox.has` is no longer a supported phase-291x
boundary.

Supported direct `MapBox.has` must carry:

```text
generic_method.has
core_method.op = MapHas
route_kind = map_contains_i64 | map_contains_any
lowering_tier = warm_direct_abi
```

This matches generated MIR JSON producers, where `refresh_module_semantic_metadata`
populates `generic_method_routes` before JSON emission. Hand-written boundary
fixtures must also carry the same metadata when they are treated as supported
compiler inputs.

## Cleanup

- Converted the legacy `map_has_no_metadata_min_v1.mir.json` fixture to carry
  `generic_method.has` / `MapHas` metadata. The filename is historical; the
  current payload is metadata-bearing.
- Updated the boundary smoke to assert the metadata route trace instead of the
  old MIR-surface fallback trace.
- Removed the final `MapBox` / `has` string classifiers from the MIR-call route
  policy.
- Emptied the no-growth allowlist. The guard now expects:

```text
classifiers=0
rows=0
```

## Boundary

- Do not reintroduce metadata-absent direct `MapBox.has` as supported input.
- Do not add a replacement method-name or box-name fallback.
- Metadata-first route selection remains the only supported path for direct
  `MapBox.has`.
- Historical cards may still describe the rejected/blocked metadata-absent era;
  do not edit those ledgers in place.

## Next

Select the next compiler-clean BoxShape cleanup from phase-291x inventory or
close the CoreMethodContract `.inc` mirror-pruning lane as zero-baseline.

## Acceptance

```bash
python3 -m json.tool apps/tests/mir_shape_guard/map_has_no_metadata_min_v1.mir.json >/dev/null
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
