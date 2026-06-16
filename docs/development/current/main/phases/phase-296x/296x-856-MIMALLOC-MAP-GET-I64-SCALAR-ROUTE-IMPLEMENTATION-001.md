# 296x-856 MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Implement the proof-backed `MapLoadScalarI64` route selected by 296x-854 and
authorized by 296x-855.

This row is intentionally narrow. It removes the `runtime_data.get_hh` mixed
facade from scalar-proof-positive `RuntimeDataBox.get` on a `MapBox` receiver,
but it does not change MapBox storage, benchmark source, or mixed get
semantics.

## Implemented Slice

```text
Rust route vocabulary:
  GenericMethodRouteKind::MapLoadScalarI64
  tag=map_load_scalar_i64
  helper=nyash.map.scalar_load_hi

Rust route producer:
  scalar-proof-positive RuntimeDataBox.get from MapBox with i64 key
  proof=map_set_scalar_i64_same_key_no_escape
  proof=map_set_scalar_i64_dominates_no_escape
  tier=WarmDirectAbi

Runtime substrate:
  nyash.map.scalar_load_hi(handle, key_i64) -> scalar i64 / missing zero
  no mixed handle publication

LLVM C shim consumer:
  generic_method.get route_kind=map_load_scalar_i64
  helper validation and emit path for nyash.map.scalar_load_hi
  declaration need for nyash.map.scalar_load_hi
```

## Preserved Boundaries

```text
mixed RuntimeDataBox.get:
  RuntimeDataLoadAny -> nyash.runtime_data.get_hh

direct MapBox.get:
  MapLoadAny -> nyash.map.slot_load_hh

Map i64-key substrate:
  nyash.map.slot_load_hi remains handle-return and is not scalar route

storage:
  String-key HashMap storage unchanged
  typed i64-key MapBox storage disabled

constant emission:
  stored-value constant emission disabled

benchmark:
  benchmark sources unchanged
```

## Result

```text
output_contract=hako-mimalloc-map-get-i64-scalar-route-implementation-v0
source_evidence=296x-854,296x-855
row_kind=implementation

map_get_scalar_i64_route_kind_present=1
map_get_scalar_i64_route_tag=map_load_scalar_i64
map_get_scalar_i64_helper=nyash.map.scalar_load_hi
scalar_proof_runtime_data_get_route_kind=MapLoadScalarI64
scalar_proof_lowering_tier=WarmDirectAbi
scalar_proof_publication_policy=NoPublication

mixed_runtime_data_get_route_kind=RuntimeDataLoadAny
mixed_runtime_data_get_helper=nyash.runtime_data.get_hh
direct_mapbox_get_route_kind=MapLoadAny
direct_mapbox_get_helper=nyash.map.slot_load_hh

slot_load_hi_scalar_route_usage=0
benchmark_source_changed=0
product_default_changed=0
stored_value_constant_emission_enabled=0
typed_i64_key_map_storage_enabled=0
string_key_map_storage_changed=0

selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-VALIDATION-001
summary=ok
```

## Verification

```text
cargo_test_lib_scalar_proof=pass
cargo_test_lib_map_get_scalar=pass
cargo_test_lib_generic_method_route_plan=pass
cargo_check_release_bin_hakorune=pass
cargo_fmt_check=pass
nyash_kernel_scalar_load_hi_test=blocked_by_unrelated_existing_kernel_test_build_failures
```

The `nyash_kernel` package-level test target currently fails to build in
unrelated test modules before the new map helper test can run. The scalar helper
contract is still fixed in code and should be covered by the package test once
the unrelated kernel test build failures are cleared.

## Stop Line

```text
do not route unproven get calls to MapLoadScalarI64
do not use nyash.map.slot_load_hi as scalar helper
do not change C benchmark source
do not add typed i64-key map storage
do not emit stored_value constants in this helper route
do not change RuntimeDataBox.get mixed return contract
do not change direct MapBox.get handle return contract
do not claim String-key conversion is removed
do not claim Hako-vs-C map_get win from the invalid old C pair
```

## Proof Bundle

```bash
bash tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_implementation_guard.sh
cargo test --lib scalar_proof -- --nocapture
cargo test --lib map_get_scalar -- --nocapture
cargo test --lib generic_method_route_plan -- --nocapture
cargo check --release --bin hakorune
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
```
