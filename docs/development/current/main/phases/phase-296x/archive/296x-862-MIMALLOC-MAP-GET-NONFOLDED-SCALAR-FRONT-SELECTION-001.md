# 296x-862 MIMALLOC-MAP-GET-NONFOLDED-SCALAR-FRONT-SELECTION-001

Status: Landed
Date: 2026-06-16

## Purpose

Select a real repeated map-get front after `kilo_leaf_map_getset_has` proved
route reachability but folded to a single scalar load. This row is front
selection only: it does not add a new lowering rule, does not claim a C winner,
and does not treat invalid C volatile comparisons as map lookup evidence.

## Evidence

```text
output_contract=hako-mimalloc-map-get-nonfolded-scalar-front-selection-v0
source_evidence=296x-861
row_kind=front_selection

selected_front=kilo_leaf_map_get_dynamic_covered_i64
selected_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako
selected_front_is_hako_only=1
c_pair_comparison_valid=0
c_pair_measurement_used=0
winner_claim=0

selected_front_shape=preseed_const_i64_values_dynamic_i64_key_get_loop
loop_key_shape=i_mod_3
preseeded_key_count=3
loop_repeated_map_get_present=1
loop_repeated_map_get_helper_current=nyash.runtime_data.get_hh
loop_repeated_map_get_scalar_route_current=0
final_const_key_get_helper_current=nyash.map.slot_load_hh

rejected_front=kilo_leaf_map_getset_has
rejected_reason=folded_single_store_single_scalar_load
rejected_body_loop_repeated_map_get_measurement_available=0

scratch_set_get_const_value_route_current=map_load_any_or_constant_eliminated
scratch_set_get_dynamic_value_route_current=map_load_any_or_value_eliminated
scratch_env_guard_route_current=map_load_scalar_i64_hoisted_out_of_loop

selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-DESIGN-001
implementation_started=0
summary=ok
```

## Interpretation

The selected front keeps real repeated `map.get(k)` work in the body because
`k = i % 3` is dynamic. The map is pre-seeded with all possible keys, so the
next proof row can ask a narrow semantic question:

```text
Can a covered dynamic i64 key MapGet use scalar load semantics?
```

The answer must come from route proof / map representation facts, not from
benchmark-name or helper-name branches.

## Stop Lines

- do not claim Hako-vs-C winner from this Hako-only front
- do not use invalid C volatile comparison pairs as map lookup evidence
- do not add benchmark-specific route branches
- do not infer scalar legality from `nyash.runtime_data.get_hh` alone
- do not treat preseeded key coverage as proven until the next design row
- do not change product MapBox semantics
- do not change MapBox storage representation in this row

## Next

`MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-DESIGN-001` should define the
proof boundary for covered dynamic i64 keys. It should decide whether the proof
belongs in map route facts, map representation facts, or a small dedicated
coverage proof surface before any implementation.
