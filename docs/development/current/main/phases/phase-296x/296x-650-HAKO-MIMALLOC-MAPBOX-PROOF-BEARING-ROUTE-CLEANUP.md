---
Status: Active
Date: 2026-06-09
Scope: separate MapBox metadata-only fusion candidates from backend-active proof-bearing routes before further MapBox optimization.
Blocker: HAKO-MIMALLOC-MAPBOX-PROOF-BEARING-ROUTE-CLEANUP-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mapbox-proof-bearing-route-ssot.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/design/abi-export-inventory.md
  - src/mir/map_lookup_fusion_plan.rs
  - src/llvm_py/instructions/mir_call/collection_method_call.py
---

# 296x-650 Hako Mimalloc MapBox Proof-Bearing Route Cleanup

## Purpose

Stop MapBox optimization from using metadata-only rows as backend behavior.

The current same-key MapBox fusion metadata is useful preflight evidence, but
it must not directly change Python LLVM lowering until a selected
proof-bearing route exists. This card fixes the route boundary before the next
MapBox exact-front optimization work.

## Decision

```text
DirectMap default:
  rejected

Proof-bearing Map route default:
  accepted

Generic MapBox:
  dynamic semantic container
  direct ABI helper path
  no raw bucket layout reads

MapLookupFusionRoute:
  candidate/preflight metadata until promoted by RouteDecision

LLVM:
  selected RouteDecision only
```

## Required Output

```text
output_contract=hako-mimalloc-mapbox-proof-bearing-route-cleanup-v0
mapbox_directmap_default=0
mapbox_proof_bearing_route_default=1
map_lookup_fusion_routes_metadata_only=1
map_metadata_only_backend_consumed_count=0
map_backend_redecide_count=0
map_silent_fallback_count=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Task Ladder

```text
MAPBOX-SSOT-000:
  Land the MapBox proof-bearing route SSOT.
  Record that map_lookup_fusion_routes are metadata/preflight only.

MAPBOX-ROUTE-DECISION-001:
  Add MapLookupFusionRoute to RouteDecision source_plan_kind.
  Selected routes: map_lookup_const_fold, map_lookup_probe_once, helper fallback.

LLVM-MAPBOX-GUARD-001:
  Make Python LLVM consume map lookup fusion only when selected RouteDecision
  exists.
  Add evidence for map_metadata_only_backend_consumed_count=0.

MAPBOX-PROBEONCE-001:
  Add one-probe MapBox helper route for value + present.
  Keep probe token backend-private.

MAPREPR-001:
  Split fixed, enum-key, and interned-key maps into MapReprPlan.
  Do not call generic MapBox a DirectMap.
```

## First Implementation Slice

```text
target=LLVM-MAPBOX-GUARD-001
behavior_change=guard existing Python LLVM map lookup fusion
must_not_change=generic MapBox helper semantics
must_not_add=MapBox bucket direct layout reads
```

## First Commands

```bash
bash tools/checks/current_state_pointer_guard.sh
python3 -m unittest src.llvm_py.tests.test_fastmem_metadata_loader src.llvm_py.tests.test_collection_method_call
git diff --check
```

## Stop Line

- do not promote generic `MapBox` into a default `DirectMap`
- do not consume `map_lookup_fusion_routes` as lowering behavior without
  selected `RouteDecision`
- do not infer map routes from helper names or method strings in the backend
- do not read raw MapBox bucket layout without a `MapReprPlan`
- do not reopen provider activation, hooks, global allocator claims, or winner
  claims

## Next

```text
HAKO-MIMALLOC-MAPBOX-PROOF-BEARING-ROUTE-CLEANUP-296X-001:
  guard metadata-only MapBox fusion rows from backend consumption

After green:
  return to 296x-649 userbox/counter-heavy or select kilo_leaf_map_getset_has
  with proof-bearing route evidence fixed.
```
