# Source instance-result contract

## Disposition

Decision (2026-08-08): retire this entire caller-zero family before the new
declaration-first instance target is implemented. Repository census found no
non-test caller. This module derives an exact-I64 result from body proof and
resolves a target from the current call site/name/arity catalog, which is the
opposite direction from the accepted declaration -> contract -> target ->
source-bound call pipeline.

`SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-R0` deletes this module, its rebind and
pre-loop adapters, tests, and `src/mir/mod.rs` declaration in one BoxShape
slice. It does not delete the general exact source-site primitives in
`source_call_target` or unrelated production result-representation owners.
No new resolver target may coexist with this authority.

This module owns one disconnected source-only result contract for an exact
current-owner instance `MethodCall`.

It may seal `me` plus an instance caller into one same-owner instance target,
consume an opaque catalog-issued body proof, and publish an Integer result only
for `ExactI64([])`.

It must not borrow or name the static result catalog type; infer from names,
annotations, MIR, runtime, or metadata; emit a Call; write a `ValueId` or
`MirType`; change GenericLoop; retry; fall back; or retain a source-site map.

## Pre-loop association

`PreparedPreloopNestedResultAssociationV1` is the source-only bridge for the
selected pre-loop occurrence. It consumes the sealed Integer contract and the
catalog-backed `RawLocatedMethodCallInputV1`, then co-seals the same catalog
allocation, declaration row, caller, structural site, and borrowed MethodCall
node. The parked loop-refresh occurrence is rejected before lowering.

This bridge is not a Builder or Call owner. It does not convert to a legacy
input, select a route, emit a Call, retain a destination, or publish a type.
The later physical-receipt row is the only allowed consumer toward the actual
unified Call success boundary.

## Owned rebind witness

`OwnedNestedInstanceResultRebindWitnessV1` is the sole owned projection of an
already-sealed nested Integer contract. It retains only the original catalog
allocation identity, caller, source site, target key, and a private
unconditional-Integer seal.

Its one consuming rebind terminal accepts the retained shared catalog and an
exact `VerifiedSourceMethodCallSiteV1`. It rechecks catalog allocation, caller,
the existing same-owner instance relation, target key, and site before
reissuing the existing borrowed contract. It does not rebuild callable-result
evidence, inspect Builder state, or expose retry/resume authority.

## Located outer argument

`PreparedPreloopLocatedArgumentV1` adds the structural outer
`CallArgument(1)` relation to that association. The Raw source view issues the
relation through the shared source-path/projector vocabulary; the co-seal then
requires the same source view, child site, and child syntax pointer. It retains
both source owners on rejection and exposes inspection plus `discard` only.

This is still source-only. It does not own ordered argument descent, route
selection, a Builder, a physical Call receipt, a `ValueId`, or a `MirType`.
