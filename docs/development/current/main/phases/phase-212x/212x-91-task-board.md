# 212x-91 Task Board

Status: Closed

## Tasks

- [x] fold `sum_local_layout(...)` agg-local routes into `placement_effect_routes`
- [x] fold `user_box_local_body(...)` agg-local routes into `placement_effect_routes`
- [x] keep `typed_slot_storage(...)` outside the placement/effect fold-up
- [x] add focused MIR regression coverage
- [x] add MIR JSON regression coverage
- [x] sync current pointers

## Non-Goals

- no lowering widening
- no storage-only route fold-up
- no generic placement/effect proving slice yet

## Exit

- `placement_effect_routes` reads placement-relevant agg-local routes
- storage-only agg-local routes remain under `agg_local_scalarization_routes`
