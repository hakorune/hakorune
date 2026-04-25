---
Status: Landed
Date: 2026-04-26
Scope: Close out the remaining MIR-call MapBox+has surface fallback rows.
Related:
  - docs/development/current/main/phases/phase-291x/291x-257-mir-call-mapbox-receiver-surface-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-271-generic-has-emit-kind-fallback-prune-card.md
  - apps/tests/mir_shape_guard/map_has_no_metadata_min_v1.mir.json
  - tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-272 MIR-Call MapHas Surface Fallback Closeout Card

## Goal

Decide whether the final two no-growth rows are removable:

- `classify_mir_call_receiver_surface box MapBox`
- `classify_mir_call_method_surface method has`

## Evidence

291x-271 added an exact metadata-absent boundary fixture:

```text
apps/tests/mir_shape_guard/map_has_no_metadata_min_v1.mir.json
```

The fixture intentionally has:

```text
generic_method_routes = []
MapBox.has(key)
```

The canonical smoke confirms that this shape still compiles through the
MIR-call surface fallback:

```bash
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
```

The route trace pins the active fallback state:

```text
stage=mir_call_method ... bname=MapBox mname=has ... map_has:1
```

## Decision

No safe prune.

The final two rows are a paired fallback contract for metadata-absent direct
`MapBox.has`. Removing either side independently breaks the only remaining
surface fallback path for that boundary.

## Boundary

- Keep `classify_mir_call_receiver_surface(... "MapBox" ...)`.
- Keep `classify_mir_call_method_surface(... "has" ...)`.
- Keep the two no-growth allowlist rows.
- Do not reintroduce generic method-name `has` fallback.
- Do not reintroduce direct `ArrayBox` / `RuntimeDataBox` has route fallbacks.

## Result

The has cleanup series is closed with an intentional two-row MIR-surface
fallback baseline:

```text
[core-method-contract-inc-no-growth-guard] ok classifiers=2 rows=2
```

## Validated

```bash
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Pick the next compiler-clean cleanup card outside the closed has-route fallback
series. The remaining two rows should only be revisited if
metadata-absent direct `MapBox.has` is retired or replaced by an explicit
metadata-only contract.
