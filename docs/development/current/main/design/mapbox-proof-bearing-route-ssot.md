---
Status: SSOT
Date: 2026-06-09
Scope: MapBox lowering route boundaries for the Hako mimalloc optimization lane.
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/abi-export-inventory.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/mir/map_lookup_fusion_plan.rs
  - src/llvm_py/instructions/mir_call/collection_method_call.py
---

# MapBox Proof-Bearing Route SSOT

## Decision

Do not make generic `MapBox` a default `DirectMap`.

Use a proof-bearing Map route model instead:

```text
MapBox:
  dynamic hash map semantic/runtime owner
  hides hash / collision / resize / lifetime details
  generic path is runtime direct-ABI helper

MapAccessSite / GenericMethodRoute facts:
  MIR records semantic get/has/set/delete sites and receiver/key/value facts

MapLookupPlan / MapReprPlan / MapRouteDecision:
  Analyzer/Planner promotes only proven subsets

RouteDecision / Verifier:
  selected_route, fallback_policy, and proof_ids are fixed before backend use

LLVM:
  consumes selected proof-bearing routes only
  does not reclassify raw MapBox layout or helper names
```

Short rule:

```text
DirectMap default: no
proof-bearing Map route default: yes
```

## Current Mismatch

`map_lookup_fusion_routes` currently represents a same-key MapGet/MapHas
preflight. The Rust-side contract says it is metadata-only and does not lower
or call backend helpers.

That row must not change backend behavior by itself.

The Python LLVM producer may still load the metadata for diagnostics and
indexing, but it must not fold or fuse MapBox behavior unless an accepted
backend-active route decision selects that lowering route.

Required stop line:

```text
map_lookup_fusion_routes alone:
  metadata/preflight only
  lowering_effect=none

RouteDecision selected_route:
  backend-active lowering contract
```

The critical evidence counter is:

```text
map_metadata_only_backend_consumed_count=0
```

## Layer Model

### 1. MapBox Semantic Layer

`MapBox` remains the general dynamic container:

```text
hash(key)
bucket probe
collision handling
resize
delete/tombstone policy
ownership/lifetime boundary
```

Generic `MapBox` lowering uses stable direct ABI helpers such as
`nyash.map.slot_load_hh`, `nyash.map.slot_store_hhh`, and
`nyash.map.probe_hh`. This is direct ABI, not direct layout access.

### 2. Fact Layer

MIRBuilder records canonical call/method sites and facts only.

Allowed:

```text
receiver value
key value
value demand
method semantic op
source span
candidate same-key window
```

Forbidden:

```text
DirectMap selection
hash table layout selection
bucket pointer classification
fixed/enum/interned route selection
backend helper name reclassification
```

### 3. Plan And Decision Layer

Analyzer/Planner may promote proven subsets into backend-active routes.

Initial route kinds:

```text
GenericRuntimeGet
GenericRuntimeHas
GenericRuntimeSet
GenericRuntimeDelete

SameKeyConstFold
SameKeyProbeOnce

FixedStaticLookup
FixedSmallLinearLookup
FixedOpenAddressLookup

EnumKeyDenseLoad
EnumKeyDenseStore

InternedKeyDirectAbiLookup
```

Every selected route carries proof ids and a fallback policy:

```text
source_plan_kind=MapLookupFusionRoute
semantic_op=MapLookupPair
selected_route=map_lookup_const_fold | map_lookup_probe_once | generic_map_helpers
fallback_policy=opportunistic | report_if_slow | fail_fast
proof_ids=[...]
```

### 4. Backend Consumer Layer

LLVM consumes selected routes only.

Allowed:

```text
read RouteDecision / backend-active route
emit selected_route
use generic helper when fallback policy allows it
fail-fast when a required selected route cannot be emitted
```

Forbidden:

```text
consume metadata-only rows as lowering contracts
infer route from method/helper strings
read MapBox bucket layout without a MapReprPlan
silently fall back from required direct route
```

## Route Families

### Generic MapBox

```text
surface=MapBox
repr=dynamic_hash_runtime
route=generic direct-ABI helper
layout_direct_access=0
```

This keeps generic MapBox semantics stable while avoiding broad method dispatch.

### Same-Key Fusion

Same-key fusion is an optimization subset of generic MapBox.

`SameKeyConstFold` requires a dominating stored-value proof and no intervening
escape/mutation in the selected window.

`SameKeyProbeOnce` uses one helper probe and produces both value and presence.
The probe token is backend-private and must not enter ordinary MIR/LLVM value
maps as a raw pointer.

Sketch ABI:

```c
int64_t nyash_map_lookup_i64_present_hh_out(
    int64_t map_h,
    int64_t key_h,
    int64_t* present_out
);
```

### Fixed / Enum / Interned Maps

These are not generic MapBox direct layout reads. They belong to a separate
`MapReprPlan` family:

```text
GenericHashRuntime
FixedStatic
FixedSmallLinear
FixedOpenAddress
EnumKeyDense
InternedKeyHash
InternedKeyFixed
```

`EnumKeyDense` is the family closest to DirectArray:

```text
tag = enum_discriminant(key)
value = values[tag]
present = bitmap[tag]
```

`InternedKey*` requires an interned-key fact or const intern id. Dynamic string
keys must not enter a direct route without proof.

## Report And Check Fields

Required evidence:

```text
mapbox_generic_helper_get_count
mapbox_generic_helper_has_count
map_lookup_fusion_candidate_count
map_lookup_fusion_selected_count
map_lookup_const_fold_get_count
map_lookup_const_fold_has_count
map_lookup_probe_once_count
map_route_decision_missing_count
map_metadata_only_backend_consumed_count=0
map_backend_redecide_count=0
map_silent_fallback_count=0
map_required_route_failfast_count
```

The first hard gate is:

```text
map_metadata_only_backend_consumed_count=0
```

## Task Ladder

```text
MAPBOX-SSOT-000:
  document map_lookup_fusion_routes as metadata/preflight only
  backend behavior requires selected RouteDecision

MAPBOX-ROUTE-DECISION-001:
  add MapLookupFusionRoute as a RouteDecision source_plan_kind
  select map_lookup_const_fold / map_lookup_probe_once / helper fallback

LLVM-MAPBOX-GUARD-001:
  update Python LLVM map lookup fusion consumer
  consume only selected proof-bearing map routes
  report metadata-only backend consumption as an error

MAPBOX-PROBEONCE-001:
  add probe-once direct ABI helper and backend side table
  produce value and presence from one helper call

MAPREPR-001:
  introduce MapReprPlan family for fixed, enum-key, and interned-key maps
  keep generic MapBox separate from direct repr families
```

## Acceptance

```text
DirectArray and MapBox are not treated as the same direct-lowering family.
MapBox remains a dynamic semantic container by default.
Generic MapBox uses direct ABI helpers, not bucket layout reads.
map_lookup_fusion_routes without RouteDecision do not affect lowering.
LLVM consumes selected proof-bearing map routes only.
Required selected routes fail fast when unsupported.
No provider activation, allocator replacement, hook, global allocator, or
winner claim is opened by this row.
```
