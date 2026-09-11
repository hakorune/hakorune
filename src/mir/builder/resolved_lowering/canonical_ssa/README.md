# canonical_ssa

This directory owns the one function-scoped mutable SSA/CFG/PHI session used
by canonical lowering profiles.

## Authority

- `CanonicalSsaFunctionSessionV2` is the sole mutable session owner and keeps
  the existing lifecycle methods that issue physical state.
- `ResolvedSsaIdentityStateV2` owns binding/identity evidence.
- `CanonicalCfgSessionV1` owns physical CFG writes.
- `session/s6c_state.rs` stores only the existing S6C cursor-seal and
  pinned-Text lifecycle state; it does not infer meaning or publish products.

The existing V1/V2 names are intentional: V1 types are receipts/state records,
while V2 names the canonical function session. They are not alternate owners.

## Caller boundary

Existing callers remain the callable, direct-accumulation, trivial-SSA, nested-
predicate, generic-G0, selected-dynamic, and common-V2/S6C lowering routes.
`physical_entry_boundary.rs` and `residence_lifecycle.rs` remain the only
mutation boundaries for the S6C state child.

## Must-not

The child must not create a second SSA/CFG/PHI owner, issue `ValueId` or MIR
products, select a caller, reinterpret source semantics, or introduce a
fallback/legacy route. Physical layout code must not re-infer edge, merge, or
exit meaning. Any semantic or caller expansion requires a new Recipe/authority
decision before implementation.

`tools/checks/common_v2_s6c_structure_guard.sh` protects this boundary and the
below-800-line rule.
