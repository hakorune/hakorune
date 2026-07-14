# Resolved Value Profile

This directory owns immutable, pre-Builder executable representation proofs for
canonical resolved source.

## Boundary

Inputs are exact located source carriers plus the sealed resolved-semantic
product. Outputs may contain only:

- `FunctionOwnerIdV1`, `BindingRefV1`, and exact source sites;
- closed representation vocabulary owned by this directory;
- exact value/definition/join/terminal coverage.

This layer must not import or infer from `MirBuilder`, `ValueId`,
`BasicBlockId`, `MirType`, `StorageClass`, runtime values, spans, pointers, or
names. It decides no CFG layout and emits no MIR.

## SSA-I0-PROFILE contract

`VerifiedTrivialCanonicalOwnerV1` is a pre-Builder whole-owner proof. It
admits exact `InlineI64`, `InlineBool`, `InlineF64`, and local-flow
`NullSentinel` values and proves their propagation through locals, reads,
rebinds, binary expressions, BlockExpr results, fallthrough If merge profiles,
and terminal disposition. `NullSentinel` preserves exact source Null identity
while materializing through the existing MIR/runtime no-value representation;
it is not an ownership-managed value. Null itself is not yet a return ABI.
Merge-profile rows prove representation homogeneity only; they never decide
whether a PHI is needed or placed. Function-owned Binding SSA retains that
authority.

Profile rejection is data, not fallback. A later compiler route may select the
existing canonical A+ path from a sealed rejection before Builder effects, but
it must never retry A+ after a trivial-profile lowering failure or mix the two
authorities inside one source unit.

SSA-I1-T consumes an admitted profile exactly once in the dedicated trivial
Binding-SSA lowerer. A non-admitted profile selects the whole-unit A+ route
before Builder effects; a lowering failure never retries another route.
