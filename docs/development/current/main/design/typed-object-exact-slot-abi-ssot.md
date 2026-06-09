---
Status: SSOT
Decision: accepted
Date: 2026-06-09
Scope: typed-object exact slot ABI split for C-speed user-box field access.
Related:
  - docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
  - docs/development/current/main/design/pinned-typed-object-arena-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
---

# Typed Object Exact Slot ABI SSOT

## Naming Policy

Preferred names for new exact-slot work use the `hako.*` / `hakorune.*`
namespace family. Existing `nyash.*` exports remain legacy aliases until the
runtime migration is complete.

## Decision

Typed-object exact lane ABI is separate from the compat/public object route.

```text
compat / public object route:
  field_get_hii
  hako.instance.get_field_h
  hako.instance.get_i64_field_h
  name lookup, object compatibility, debug/reflection, materialization fallback

exact slot route:
  hako.object.exact_slot_get_i64_hii
  hako.object.exact_slot_set_i64_hii
  hako.object.exact_slot_get_u64_hii
  hako.object.exact_slot_set_u64_hiu
  hako.object.exact_slot_get_handle_hii
  hako.object.exact_slot_set_handle_hii
  no name lookup, no compat rediscovery, no silent fallback after selection

final NativeDirect route:
  selected exact slot route -> inline direct load/store
```

`field_get_hii` may remain as a compat / legacy adapter. It is not the keeper
ABI for C-speed exact-lane field access.

## Why

Recent `kilo_micro_userbox_counter_step_chain` measurements moved the hot owner
from compat extraction into `field_get_hii_direct`. That means the remaining
cost is no longer broad runtime compatibility work; it is the exact-lane ABI
boundary itself.

Keeping `field_get_hii` as a unified helper with internal route dispatch leaves
these costs in the hot path:

```text
receiver / backend route branch
compat-vs-exact rediscovery
legacy object-world fallback shape
runtime legality/provenance checks
```

This violates the optimization owner rule for hot scalar/storage operations:
the selected hot route must not keep a cross-ABI helper boundary merely because
the helper can dispatch quickly.

## Naming

Use route names for MIR / planner truth and ABI symbol names only as backend
artifacts.

```text
MIR / RouteDecision route names:
  hako.typed_object.slot_load_i64
  hako.typed_object.slot_store_i64
  hako.typed_object.slot_load_u64
  hako.typed_object.slot_store_u64
  hako.typed_object.slot_load_bool
  hako.typed_object.slot_store_bool
  hako.typed_object.slot_load_f64
  hako.typed_object.slot_store_f64
  hako.typed_object.slot_load_handle
  hako.typed_object.slot_store_handle

C ABI symbol names:
  hako.object.exact_slot_get_i64_hii
  hako.object.exact_slot_set_i64_hii
  hako.object.exact_slot_get_u64_hii
  hako.object.exact_slot_set_u64_hiu
  hako.object.exact_slot_get_handle_hii
  hako.object.exact_slot_set_handle_hii

Legacy alias names:
  nyash.object.exact_slot_get_i64_hii
  nyash.object.exact_slot_set_i64_hii
  nyash.object.exact_slot_get_u64_hii
  nyash.object.exact_slot_set_u64_hiu
  nyash.object.exact_slot_get_handle_hii
  nyash.object.exact_slot_set_handle_hii
```

Do not make compact call-shape suffixes such as `hii` the semantic SSOT. They
are ABI artifacts. The semantic SSOT is the route decision plus value class.

`hako.object.exact_slot_get_handle_hii` means the field storage / return class
is a handle. It is not the right primary route for an i64 field benchmark.
i64 field access must use `hako.typed_object.slot_load_i64` and, while
helper-backed, the `hako.object.exact_slot_get_i64_hii` ABI row.

## Route Decision Contract

Example selected route:

```json
{
  "route_id": "route.decision",
  "semantic_op": "FieldGet",
  "access_kind": "hako_typed_object_slot_load_i64",
  "preferred_route": "hako.typed_object.slot_load_i64",
  "selected_route": "hako.typed_object.slot_load_i64",
  "fallback_route": "compat_field_get_i64",
  "fallback_policy": "fail_fast",
  "source_plan_kind": "TypedObjectSlotAccessPlan",
  "proof_ids": [
    "field_decls_authority",
    "typed_object_plan",
    "receiver_exact_type_id",
    "slot_in_bounds",
    "storage_i64",
    "non_weak_field",
    "materialization_boundary_known"
  ]
}
```

TYPEDOBJ-ABI-002 uses one semantic route with an explicit lowering form:

```json
{
  "selected_route": "hako.typed_object.slot_load_i64",
  "lowering_form": "exact_helper_bridge",
  "bridge_symbol": "hako.object.exact_slot_get_i64_hii",
  "native_direct_ready": false
}
```

NativeDirect later changes the lowering form, not the semantic route:

```json
{
  "selected_route": "hako.typed_object.slot_load_i64",
  "lowering_form": "native_direct",
  "bridge_symbol": null,
  "native_direct_ready": true
}
```

Do not split the semantic route into helper and inline variants. The split is:

```text
semantic route:
  hako.typed_object.slot_load_i64

lowering forms:
  exact_helper_bridge
  native_direct
```

Rules:

```text
no selected exact route:
  compat fallback is allowed when the source semantics allow it

selected exact route + fallback_policy=fail_fast:
  compat fallback is forbidden
  backend unsupported is fail-fast
  helper internal dispatch is not keeper evidence

selected NativeDirect route:
  lowerer consumes verified plan only
  LLVM emits direct load/store
  runtime helper must not remain inside the selected hot region
```

## Exact Helper Contract

Exact helper calls are a transition step between compat helpers and inline
NativeDirect lowering. They are allowed only behind a proof-bearing selected
route.

TYPEDOBJ-ABI-002 keeps this helper bridge by design. Its purpose is to move the
i64 benchmark away from compat `field_get_hii`, make exact route evidence
visible, and measure whether the helper call is the remaining owner. The helper
bridge is not keeper evidence for the final C-speed route.

Required proof for a selected exact load/store:

```text
object_handle_positive_or_proven_valid=1
receiver_exact_type_id_known=1
layout_version_known=1
slot_in_bounds=1
slot_storage_class_matches_route=1
field_is_not_weak=1
materialization_boundary_known=1
backend_capability_supports_exact_slot=1
```

Forbidden inside exact route:

```text
field_name_lookup
dynamic field table search
get_compat_i64
generic method dispatch
object materialization fallback
silent return-zero fallback after selected route
runtime route rediscovery as keeper evidence
```

During transition, existing helpers may keep compatibility-shaped return values
for legacy callers. The planner/verifier must ensure selected exact routes do
not rely on those values as semantic fallback.

Legacy `nyash.object.*` exact-slot exports remain migration aliases only and
must not become the preferred naming in new docs or routes.

Implementation migration may choose one of these two forms:

```text
preferred:
  add hako.object.exact_slot_get_i64_hii as the preferred export
  keep nyash.object.exact_slot_get_i64_hii as a legacy alias

minimal bridge:
  call the existing nyash.object.exact_slot_get_i64_hii export
  report migration_alias_used=1
  keep RouteDecision selected_route=hako.typed_object.slot_load_i64
```

Both forms must keep the semantic route in the `hako.*` namespace.

## Lowering Ladder

```text
PublicObject:
  compat helper route
  object identity / reflection / fallback surface

ExactSlotObject:
  selected typed-object slot route
  exact helper call allowed as a bridge

NativeDirect:
  selected typed-object slot route
  direct GEP/load/store or equivalent backend-native access
  no runtime helper in the selected hot region
```

NativeDirect is not part of TYPEDOBJ-ABI-002. It requires storage/slot/address
facts from the pinned typed-object arena contract:

```text
object_storage_pinned_required=1
field_address_stable_required=1
object_generation_required=1
slot_layout_stable_required=1
handle_generation_validation_required=1
lease_region_required=1
lease_barrier_policy_required=1
```

Do not reopen broad `field_get_hii` helper optimization once the owner is the
exact ABI boundary. The next keeper must either move the benchmark route to
`hako.object.exact_slot_get_i64_hii` or move the selected route to inline
direct load/store.

## Report / Check Fields

Minimum evidence:

```text
typed_object_exact_slot_get_i64_count
typed_object_exact_slot_set_i64_count
typed_object_exact_helper_call_count
typed_object_exact_lowering_form=exact_helper_bridge|native_direct|none
typed_object_exact_bridge_symbol=hako.object.exact_slot_get_i64_hii|none
typed_object_migration_alias_used=0|1
typed_object_inline_slot_load_count
typed_object_inline_slot_store_count
typed_object_compat_field_get_count
typed_object_get_compat_i64_count=0
typed_object_exact_name_lookup_count=0
typed_object_exact_internal_dispatch_count=0
typed_object_exact_silent_fallback_count=0
typed_object_required_route_failfast_count
```

Keeper gates:

```text
selected_exact_route_present=1
typed_object_exact_lowering_form=exact_helper_bridge
typed_object_get_compat_i64_count=0
typed_object_exact_internal_dispatch_count=0
typed_object_exact_silent_fallback_count=0
hot_symbol_is_compat_field_get_hii=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

## Task Ladder

```text
TYPEDOBJ-ABI-000:
  Land this SSOT and the phase card.

TYPEDOBJ-ABI-001:
  Add route/report vocabulary for typed_object.slot_load/store_* and exact
  versus compat counters.

TYPEDOBJ-ABI-002:
  Route i64 typed field benchmark through selected
  hako.typed_object.slot_load_i64 and helper-backed
  hako.object.exact_slot_get_i64_hii.
  Keep field_get_hii as compat/legacy.
  Keep lowering_form=exact_helper_bridge.
  Do not open NativeDirect.

TYPEDOBJ-ABI-003:
  Guard exact selected routes so helper internal dispatch is not counted as a
  keeper route.

TYPEDOBJ-ABI-004:
  Select the first NativeDirect typed-object slot load/store pilot.
  Lower selected route to inline direct load/store when storage/address facts
  are proven.
```

## Stop Line

```text
do_not_make_field_get_hii_the_exact_ssot=1
do_not_route_i64_benchmark_through_hako_object_exact_slot_get_handle_hii=1
do_not_add_benchmark_name_special_cases=1
do_not_silently_fallback_after_selected_exact_route=1
do_not_claim_allocator_replacement_or_winner=1
```
