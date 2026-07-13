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

Forbidden identity sources are AST pointer, Span, name, traversal order,
producer path, and ProgramV0 reconstruction.
