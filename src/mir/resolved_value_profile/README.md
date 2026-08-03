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
- a direct-call row co-sealed from the source-unit callable
  index, with exact target projection, ordered argument sites, result, effect,
  and one `DirectCall(site)` coverage subject;
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
R0a-I1 connects this witness through the route-local callable-ABI facade. Only
the co-sealed exact `i64` row reaches production; the terminal remains return-
value authority and resolved Lower does not reread the raw annotation.

The D0-B2 If mapper is a disconnected elaboration owned by this directory. It
consumes one sealed `VerifiedTrivialIfRecipeFactsV1` plus the function-origin
receipt, creates recipe-local keys, and immediately invokes the portable
IfRecipe verifier. It may not rescan source, infer missing entry values, select
routes, return `Option`, or emit Builder/SSA effects. A missing pre-If entry
witness is a typed rejection; JoinSig and canonical PHI remain later owners.

Profile rejection is data, not fallback. A later compiler route may select the
existing canonical A+ path from a sealed rejection before Builder effects, but
it must never retry A+ after a trivial-profile lowering failure or mix the two
authorities inside one source unit.

P0c-S0b introduced the row disconnected. The final callable Program authority
uses the same generic row for every finite one-or-more exact call set, including
singleton self calls, sibling edges, nested calls, and recursive SCCs. The
profile assumes neither catalog cardinality nor target-equals-caller identity.
Its call result is represented
only by `DirectCall(site)`, never by a duplicate generic `Value(site)` row. The
consumption API returns the whole row so Lower cannot pair target and ABI from
separate authorities.

The finite analyzer records nested calls in execution postorder (argument calls
before their enclosing call) and uses checked cardinality. Only two admission
modes remain: body-only `Forbidden` and callable-Program `FiniteOneOrMore`.
There is no exact-one analyzer or retry path. Non-VM execution remains rejected
before effects.

SSA-I1-T consumes an admitted profile exactly once in the dedicated trivial
Binding-SSA lowerer. A non-admitted profile selects the whole-unit A+ route
before Builder effects; a lowering failure never retries another route.
