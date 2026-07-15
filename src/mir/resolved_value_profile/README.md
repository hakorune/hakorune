# Resolved Value Profile

This directory owns immutable, pre-Builder executable representation proofs for
canonical resolved source.

## Boundary

Inputs are exact located source carriers plus the sealed resolved-semantic
product. Outputs may contain only:

- `FunctionOwnerIdV1`, `BindingRefV1`, and exact source sites;
- closed representation vocabulary owned by this directory;
- sealed parameter ABI rows whose source names are transport/diagnostic data,
  never lookup or binding identity authority;
- a sealed return ABI witness that refers to the existing exact terminal and
  never owns another return value or terminal analysis;
- exact value/definition/join/terminal coverage.

This layer must not import or infer from `MirBuilder`, `ValueId`,
`BasicBlockId`, `MirType`, `StorageClass`, runtime values, spans, pointers, or
names. It decides no CFG layout and emits no MIR.

## SSA-I0-PROFILE contract

`VerifiedTrivialCanonicalOwnerV1` is a pre-Builder whole-owner proof. It
admits exact `InlineI64`, `InlineBool`, `InlineF64`, and local-flow
`ExplicitVoidValue`, and local-flow `NullSentinel` values and proves their
propagation through locals, reads, rebinds, binary expressions, BlockExpr
results, fallthrough If merge profiles, and terminal disposition.
`ExplicitVoidValue` preserves exact source `void` as a value and remains
distinct from `return;` and implicit completion. `NullSentinel` preserves exact
source Null identity. Both reuse the existing MIR/runtime no-value
representation and are not ownership-managed values. Null itself is not yet a
return ABI.
Merge-profile rows prove representation homogeneity only; they never decide
whether a PHI is needed or placed. Function-owned Binding SSA retains that
authority.

Exact parameter rows are ABI sidecars. Their declaration `Definition` row is
the sole exact-once coverage subject, and parameter names never replace
`BindingRefV1` identity. The first row accepts only exact source `i64`; it
allocates no `ValueId` and has no production Builder connection until P0a-I1.

The first return witness accepts only exact source `i64` co-sealed with the
existing final explicit `InlineI64` terminal, completion, and coverage row.
R0a-S0 keeps this witness disconnected: no Builder consumer or production
typed-return admission exists until the atomic R0a-I1 slice.

Profile rejection is data, not fallback. A later compiler route may select the
existing canonical A+ path from a sealed rejection before Builder effects, but
it must never retry A+ after a trivial-profile lowering failure or mix the two
authorities inside one source unit.

SSA-I1-T consumes an admitted profile exactly once in the dedicated trivial
Binding-SSA lowerer. A non-admitted profile selects the whole-unit A+ route
before Builder effects; a lowering failure never retries another route.
