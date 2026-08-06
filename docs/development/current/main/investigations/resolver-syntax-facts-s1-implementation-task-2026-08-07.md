# Resolver syntax facts S1 implementation

Status: `caller-zero implementation; production route remains closed`

Parent: `RESOLVER-SYNTAX-FACTS-D0`

## Change

Implement one compiler-side source observer/product for the selected
`StringHelpers.int_to_str/1` single-loop fixture. The observer is the only
boundary allowed to inspect the exact source view; its sealed output contains
no AST, source lifetime, names-as-identity, `ValueId`, Recipe, CFG, PHI, or
Builder route.

## Authority and shape

Input:

```text
ResolvedFunctionLoweringInputV1
  + FunctionSourceViewV1 / LocatedStmtV1 / LocatedExprV1
  + CallableSemanticSourceLedgerView
  + resolver-issued loop context (source + frame + Scope/Region)
```

Output: `VerifiedSourceSyntaxFactsV1`, containing owner/origin/source-kind,
the fixed nine syntax rows, and one separate prefix boundary:

```text
9 syntax rows:
  InitialCarrier
  Condition Lhs / Rhs / Operator
  Step Lhs / Rhs / Operator / AssignmentTarget
  TerminalTail

separate envelope:
  PrefixBoundary
```

Rows retain exact typed source sites and neutral as-written shapes only.
Operator, literal, call-boundary, and return-expression shapes are observer
vocabulary. Type/range/overflow/monotonicity policy remains downstream.
BindingRef, direct-call target, exit identity, owner/frame, and Scope/Region
remain resolver facts for the later MAP-S1 join; the observer never resolves
them from names or AST.

## Contract

- Reuse `FunctionSourceViewV1` navigation and the shared `ExprChildRoleV1` /
  `BodyChildRoleV1` vocabulary. Do not call raw AST projection or reconstruct
  path suffixes in the new box.
- Add only the minimal resolver ledger accessor needed to co-seal one loop
  context. Do not add a second resolver or an AST-bearing resolver schema.
- The product is immutable and sealed. Dropping the source unit after issue
  must not affect product reads.
- Caller-zero only: no Builder/MIR/Recipe/ValueId/CFG/PHI/route caller.
- Typed rejects cover foreign owner/context, missing or extra loop/body rows,
  unsupported operator/literal/call/tail shape, non-terminal/void/extra tail,
  and source navigation failure. Binding/direct-call mismatch is reserved for
  MAP-S1 join tests, not inferred by this observer.

## Acceptance

- Positive fixture proves all nine syntax rows plus the separate prefix
  boundary and exact site uniqueness.
- Neutral shape enums do not store frontend AST enums or `GenericOperandFact`.
- Resolver context includes source/frame/Scope/Region from one sealed owner.
- Focused tests prove foreign/mismatched context rejection and that the
  product remains readable after the borrowed syntax owner is dropped.
- No production caller or legacy route changes are introduced.
- This task's implementation commit updates the exact `docs/reference/**`
  row, current pointer, workstream, and task status together.

## Next

After this row is green, reopen `MAP-S1` directly. Do not add another D0
suffix. `RECIPE-S2`, physicalization, production selection, retry/fallback
retirement, and legacy corpus deletion remain closed.
