# Phase 212x: placement/effect agg-local fold-up

Status: Landed

Purpose
- widen the folded generic `placement_effect_routes` owner seam without changing lowering behavior
- read placement-relevant `agg_local` routes from the landed owner seam while keeping storage-only routes under `agg_local scalarization`

Scope
- fold `sum_local_layout(...)` and `user_box_local_body(...)` agg-local routes into `placement_effect_routes`
- keep `typed_slot_storage(...)` inside `agg_local_scalarization_routes` only
- keep semantic refresh order and MIR JSON export behavior-preserving

Follow-on
- first proving slice under `generic placement / effect`

Non-goals
- no MIR rewrite
- no lowering / codegen widening
- no deletion of `agg_local_scalarization_routes`
- no storage-only route fold-up into `placement_effect_routes`

Acceptance
- `placement_effect_routes` includes placement-relevant agg-local routes
- storage-only agg-local routes stay out of the placement/effect fold-up
- `git diff --check`
