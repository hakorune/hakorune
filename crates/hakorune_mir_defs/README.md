# hakorune_mir_defs

Shared MIR call-shape substrate extracted during the crate split preparation
lane.

## Scope

- `call_unified.rs`

## Boundaries

- This crate only holds unified call definitions and related call-shape helpers.
- It depends on `hakorune_mir_core` for the pure substrate types.
- It does not own MIR lowering, builder policy, or bridge routing.

`Callee::rewrite_value_operands` is the single structural owner for embedded
`ValueId` rewrites. It visits `Method.receiver`, `Value`, and `Closure` captures
in stored order followed by `me_capture`; target-less variants are explicit
no-ops and duplicate occurrences are preserved. Consumers may apply their own
policy, but must not reimplement this variant match.
