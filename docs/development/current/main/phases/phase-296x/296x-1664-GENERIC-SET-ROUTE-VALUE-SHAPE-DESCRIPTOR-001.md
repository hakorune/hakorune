---
Status: Complete
Date: 2026-06-24
Scope: Move generic-method set-route value-shape selection into descriptor data.
---

# GENERIC-SET-ROUTE-VALUE-SHAPE-DESCRIPTOR-001

## Decision

Select this before same-module / extern descriptor generation. This closes the
generic-method descriptor family first, then expands to other route families.

## Target

Move the handwritten `LoweringPlanSetRouteRule` tuple data out of
`generic_method_match.inc` and into descriptor-owned manifest data.

Conceptual manifest shape:

```text
routes.c_set_routes:
  value_shape
  helper_variant
  result
```

Existing route descriptor data already owns `route_id`, `route_kind`,
`helper_symbol`, `tier`, `emit_kind`, `c_need_kind`, and `c_helper_variants`.
This slice adds the remaining value-shape to set-route-result mapping.

## Acceptance

```text
MapSet:
  map_store_i64 + any
  map_store_any + any

ArraySet:
  array_store_any + i64
  array_store_any + non_string_handle
  array_store_any + string_handle

Manifest contains symbolic value_shape and result names, not C enum integers.
Generator rejects unknown value_shape, helper_variant, or set-route result.
Generated C registry gains set_value_shape and set_route_result fields.
LoweringPlanSetRouteRule handwritten struct/table is removed.
Missing/malformed descriptor fails closed instead of receiver/helper fallback.
```

## Non-Claims

```text
value-shape fact producer redesign = 0
plain-i64 / string-origin classification change = 0
same-module descriptor generation = 0
extern descriptor generation = 0
new backend route = 0
runtime fallback = 0
```

## Verification

```text
python3 tools/generic_method_route_descriptor_codegen.py --check
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```
