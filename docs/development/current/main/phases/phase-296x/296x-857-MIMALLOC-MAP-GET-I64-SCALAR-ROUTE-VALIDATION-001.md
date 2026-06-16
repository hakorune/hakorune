# 296x-857 MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-VALIDATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Validate that the implemented `MapLoadScalarI64` route is not only present in
Rust route metadata, but is consumed by the exact-AOT `ny-llvmc` boundary and
emits the scalar no-publication helper.

This row is validation only. It does not change benchmark sources and does not
claim a Hako-vs-C win from the invalid old C map-missing pair.

## Evidence

output_contract=hako-mimalloc-map-get-i64-scalar-route-validation-v0
source_evidence=296x-856
row_kind=validation

validation_fixture=apps/tests/mir_shape_guard/lowering_plan_map_get_scalar_i64_directabi_min_v1.mir.json
validation_guard=tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_validation_guard.sh

c_shim_rebuild_required=1
c_shim_contains_map_load_scalar_i64=1
c_shim_contains_scalar_helper=1

scalar_validation_object_emitted=1
scalar_validation_symbol_present=nyash.map.scalar_load_hi
scalar_validation_map_birth_symbol_present=nyash.map.birth_h
scalar_validation_runtime_data_get_hh_symbol_present=0
scalar_validation_slot_load_hh_symbol_present=0
scalar_validation_slot_load_hi_symbol_present=0

mixed_get_fixture_still_runtime_data_get_hh=1
direct_mapbox_get_fixture_still_slot_load_hh=1

map_get_scalar_i64_route_kind_present=1
map_get_scalar_i64_route_tag=map_load_scalar_i64
map_get_scalar_i64_helper=nyash.map.scalar_load_hi

benchmark_source_changed=0
product_default_changed=0
stored_value_constant_emission_enabled=0
typed_i64_key_map_storage_enabled=0
string_key_map_storage_changed=0
winner_claim=0

selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-MEASUREMENT-001
summary=ok

## Stop Lines

- do not change C benchmark source
- do not claim Hako-vs-C map_get win from the invalid old C pair
- do not route unproven get calls to MapLoadScalarI64
- do not use nyash.map.slot_load_hi as scalar helper
- do not change mixed RuntimeDataBox.get fallback
- do not change direct MapBox.get handle return contract
- do not add typed i64-key map storage
- do not emit stored_value constants in this helper route
- do not skip C shim rebuild in validation

## Notes

The validation fixture is intentionally lower-plan driven. It verifies the
backend consumer for the route contract without inferring from helper names or
benchmark names.

`nyash.map.scalar_load_hi` still uses the current String-key map storage
substrate internally. Removing key String conversion is a separate storage
representation task, not part of this row.
