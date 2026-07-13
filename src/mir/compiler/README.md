# MIR compiler ingress boundary

This directory owns module-level route selection before `MirBuilder` creates
module, entry-block, or FunctionRegion state.

## B0-L2 contract

- `LegacyModuleLoweringInputV1` owns a bare AST plus an explicit legacy origin.
- `ResolvedModuleLoweringInputV1` can only borrow an opaque
  `VerifiedResolvedSourceUnitV1`.
- The verified unit is canonical syntax plus its sealed semantic owner forest;
  it is not a rewritten or resolved AST.
- A private request enum is matched once in `MirCompiler` and never reaches
  recursive Lower.
- Canonical failure never retries through the legacy route.
- B0-L2a has no production constructor for a verified source unit and no
  canonical Lower activation. Its resolved entry fails before Builder effects.

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

During B0-L2b all source views remain disconnected test/transport
infrastructure. There is no production verified-unit constructor,
`compile_resolved` caller, Builder consumer, Planner suffix connection, or
semantic activation. The temporary disconnected-code allowance ends when
SA3-B installs the first closed canonical function family.

Forbidden identity sources are AST pointer, Span, name, traversal order,
producer path, and ProgramV0 reconstruction.
