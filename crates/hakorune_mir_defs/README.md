# hakorune_mir_defs

Shared MIR call/object-reference substrate extracted during the crate split preparation
lane.

## Scope

- `call_unified.rs`
- `object_ref.rs`: module-local object and declaration-order field coordinates.

## Boundaries

- This crate holds structural call definitions and module-local object references.
- It depends on `hakorune_mir_core` for the pure substrate types.
- It does not own MIR lowering, builder policy, or bridge routing.

`Callee::for_each_value_operand` and `Callee::rewrite_value_operands` are the
single structural owners for embedded `ValueId` reads and rewrites. Both visit
`Method.receiver`, `Value`, and `Closure` captures in stored order followed by
`me_capture`; target-less variants are explicit no-ops and duplicate
occurrences are preserved. `MirInstruction::used_values` delegates its typed
Call target projection here, then appends Call args. Consumers may apply their
own policy, but must not reimplement this variant match.

`Callee::SameModuleInstance` is the source-backed instance-call carrier. It
keeps an already-issued `CanonicalSameModuleCallableKeyV1::InstanceBoxMethod`
and a mandatory receiver separate from source arguments. The receiver is a
physical operand only through this typed field; no consumer may recover it from
`args[0]`, a name, or a numeric sentinel. The final `Call` schema cutover is a
later row; legacy `Method { ..., receiver: Option<_> }` remains a compatibility
surface until its caller-zero proof is complete.

`Callee::BirthConstructor` uses the same operand visitors but a distinct Birth
key namespace. It invokes the constructor hook on a fresh receiver; it does not
allocate (`Constructor`/`NewBox`) or masquerade as Global. Source args exclude
the receiver, including when an argument has the same ValueId. Effect/result
admission belongs to the source recipe, not this structural crate.

`CanonicalObjectIdV1` and `CanonicalFieldRefV1` do not prove source membership.
The existing semantic batch assigns IDs once and retains the exact source
correspondence; publication must validate the module brand, definition and field
range. Equal numeric IDs across modules are unrelated. Runtime type IDs are a
separate physical layout projection, never a source identity substitute.
