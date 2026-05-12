# 293x-211: C203c Record Local Scalar Metadata

Status: Complete

## Purpose

Expose concrete record layouts in the folded local-aggregate metadata inventory.

C203b derives `record_layout_plans`. C203c makes those plans visible as
`record_local_layout` rows in `agg_local_scalarization_routes` and folded
placement/effect metadata. This prepares the compiler for later record
construction/read lowering without installing a rewrite yet.

## Metadata Lane

`record_local_layout` rows are:

- source `agg_local_scalarization`
- decision `local_aggregate` when folded into placement/effect
- keyed by record name
- linked to the `record_layout_plans[].layout_id`

The row name is intentionally record-specific. It must not reuse
`user_box_local_body` or typed-object seed route names.

## Stop Line

C203c does not add:

- record constructor lowering
- record field access lowering
- MIR scalar rewrite
- packed `ArrayBox` storage
- `userbox_local_scalar_seed_route` reuse
- LLVM/Python user-box local aggregate route changes
- `.inc` record matchers

## Acceptance

- `agg_local_scalarization_routes` emits `record_local_layout` from
  `record_layout_plans`.
- placement/effect metadata folds `record_local_layout` as a local aggregate.
- MIR JSON emits the record-specific agg-local row.
- Existing typed-slot routes remain agg-local only.
- No record-local matcher leaks into `.inc` shims.
