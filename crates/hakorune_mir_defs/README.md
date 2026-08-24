# hakorune_mir_defs

Shared MIR call-shape substrate extracted during the crate split preparation
lane.

## Scope

- `call_unified.rs`

## Boundaries

- This crate only holds unified call definitions and related call-shape helpers.
- It depends on `hakorune_mir_core` for the pure substrate types.
- It does not own MIR lowering, builder policy, or bridge routing.

`Callee::for_each_value_operand` and `Callee::rewrite_value_operands` are the
single structural owners for embedded `ValueId` reads and rewrites. Both visit
`Method.receiver`, `Value`, and `Closure` captures in stored order followed by
`me_capture`; target-less variants are explicit no-ops and duplicate
occurrences are preserved. `MirInstruction::used_values` delegates its typed
Call target projection here, then appends Call args. Consumers may apply their
own policy, but must not reimplement this variant match.
