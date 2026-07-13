# MIR compiler ingress boundary

This directory owns module-level route selection before `MirBuilder` creates
module, entry-block, or FunctionRegion state.

## Typed ingress contract

- `LegacyModuleLoweringInputV1` owns a bare AST plus an explicit legacy origin.
- `ResolvedModuleLoweringInputV1` can only borrow an opaque
  `VerifiedResolvedSourceUnitV1`.
- The verified unit is canonical syntax plus its sealed semantic owner forest;
  it is not a rewritten or resolved AST.
- A private request enum is matched once in `MirCompiler` and never reaches
  recursive Lower.
- Canonical failure never retries through the legacy route.
- `VerifiedResolvedSourceUnitV1::resolve_function` is the sole production
  constructor. It owns one AST and resolves/seals its forest and exact source
  projection in the same call.

Exact child-site navigation belongs to B0-L2b. Function transaction cleanup
belongs to B0-L2c. BindingId adoption and production semantic activation belong
to atomic SA3-B.

## B0-L2b source projection boundary

`VerifiedSourceProjectionV1` is sealed beside the canonical syntax and owner
forest. It contains only structural owner locators; it never stores AST
pointers, Spans, names, traversal ordinals, or cloned nodes. A
`FunctionSourceViewV1` borrows one exact function/Lambda root through that
projection and is the only factory for `LocatedBodyV1`, `LocatedStmtV1`, and
`LocatedExprV1`.

Child navigation is parent-relative and immutable. Closed child-role enums
select an AST field and the existing `SourcePathSegmentV1` together. The
physical AST-field projection is implemented once in `source_projection.rs`;
recursive Lower must consume located carriers instead of rebuilding paths.

B0-L2b landed source views as disconnected transport. SA3-B now has exactly
one production consumer under `builder/resolved_lowering/`; Planner suffix
transport remains disconnected.

Forbidden identity sources are AST pointer, Span, name, traversal order,
producer path, and ProgramV0 reconstruction.

## SA3-B first-family activation

`CanonicalLoweringPreflightV1` accepts exactly one non-main static/free
function owner with a straight-line closed grammar. It runs before a candidate
Builder is created. `CanonicalModuleLoweringSessionV1` discards that candidate
on any error and commits it only after compiler post-processing succeeds.

The function input is derived only from the verified unit. Recursive Lower is
owned by `CanonicalFunctionLowererV1` and receives only located carriers. Its
value environment is `BindingRefV1 -> ValueId`; names are diagnostic
cross-checks and never lookup keys. Declaration adoption, variable-use sites,
assignment-target sites, and Return sites must all finish coverage before the
unpublished draft may commit. The Builder's legacy BindingId allocator is
fallibly vetoed for the entire installed-owner interval.

The first capability also admits straight-line lexical BlockExpr. Each
expression consumes its exact sealed scope/region pair, retires only inner
BindingRefs, and returns the tail ValueId after balanced leave. The default
bare-AST route, ProgramV0, REPL, Main, instance methods, Lambda,
If/Loop/CorePlan, and Planner remain outside it. Canonical failure never
retries legacy.
