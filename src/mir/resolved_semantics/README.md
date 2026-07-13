# Resolved Semantics

This module family owns the passive, function-scoped semantic product that is
constructed before Planner and Lower.

```text
canonical function AST
  -> FunctionSemanticResolverV1       (SA1+)
  -> ResolvedFunctionDraftV1
  -> ResolvedFunctionVerifierV1       (SA2+)
  -> VerifiedResolvedFunctionV1
```

SA0 adds only the closed schema and publication boundary. It does not resolve
syntax, allocate identities, plan control flow, or lower MIR.

## Authority

- The canonical AST owns syntax and source execution order.
- A sealed `VerifiedResolvedFunctionV1` owns resolved binding, scope, region,
  assignment-target, and control-target facts for one function.
- `BindingOriginV1`, `RegionOriginV1`, and source sites are provenance. They
  are not lexical/control equality authorities.
- `BindingRefV1`, `ScopeId`, and `RegionId` carry an invocation-local function
  owner brand. The brand detects cross-function mixing but is not source or
  parity identity. `BindingRefV1` wraps the canonical `BindingId`; it does not
  replace it.
- Lower owns `BindingId -> ValueId` and `RegionId -> BasicBlockId`
  materialization only.

## Forbidden dependencies

This module family must not import or call:

- `ASTNode` or any cloned/normalized AST payload;
- Planner, Recipe, JoinIR ownership, or Lower modules;
- `ValueId` or `BasicBlockId`;
- `MirBuilder`, `CoreContext`, or any BindingId allocator;
- the GC/debug `mir::region::RegionId`;
- the private `join_ir::ownership` BindingId/ScopeId family.

Mutable drafts remain crate-private. Only a verified sealed product may become
a public consumer input. Unsupported resolution never retries a legacy path.

SA1's `ShadowBindingOrdinalV0` must use a separate test-only shadow product.
It must never populate this canonical BindingId product or enter Planner or
Lower. SA2 removes the schema-test verifier bypass once the real verifier is
available.
