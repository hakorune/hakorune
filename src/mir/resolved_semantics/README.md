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

SA0 adds only the closed canonical schema and publication boundary. SA1 adds
a physically separate, disconnected shadow resolver. The shadow resolver may
read canonical syntax, but it owns only `Shadow*V0` handles and can publish
only `ShadowResolvedFunctionV0` for tests and deterministic inspection. It
cannot populate the canonical draft, allocate `BindingId`, plan control flow,
or lower MIR.

SA2 installs the real publication boundary. A compilation-scoped issuer with
a process-unique compilation brand is the only owner-brand constructor;
`ResolvedFunctionDraftV1::seal` verifies owner membership, graph
roots/ancestry, binding accounting, supplied source-index integrity, and exact
RegionId control targets before publishing. The sealed product also owns a
deterministic origin-keyed normalized graph for parity. Raw owner, binding,
scope, and region numbers never enter that graph. SA2 still has no canonical
AST resolver producer and no Planner or Lower connection; those are later
authority-cutover slices.

Because the canonical AST is intentionally not stored here, SA2 cannot prove
that a caller omitted no syntax site. Canonical-AST site totality becomes
checkable only when the SA3 resolver co-constructs the closed indexes. SA2
claims referential integrity of supplied indexes, not syntax-independent
omniscience.

## SA3 transport boundary

SA3-A adds a behavior-neutral Lower transport in
`builder/vars/resolved_binding_state.rs`. It can hold one sealed product,
claim an exact `SourceBindingSiteV1` once, and publish `BindingId -> ValueId`.
Resolved parameter/local publication APIs exist but have no production caller
until the atomic SA3-B switch. During SA3-A the legacy Lower allocator remains
the single active canonical BindingId owner; installing a canonical product is
still zero.

The canonical `FunctionSemanticResolverSessionV1` also exists as a
disconnected producer. It borrows `FunctionSyntaxViewV1`, uses
construction-local draft indices during one traversal, converts them once to
owner-scoped canonical IDs, and immediately verifies/seals. Draft indices
never enter Lower or the normalized parity graph. Production installation
remains zero until the accepted syntax inventory and all declaration callers
can switch together.

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

- canonical arena files must not own cloned/normalized AST payloads;
  `FunctionSyntaxViewV1` and the construction resolver may only borrow syntax;
- Planner, Recipe, JoinIR ownership, or Lower modules;
- `ValueId` or `BasicBlockId`;
- `MirBuilder`, `CoreContext`, or any BindingId allocator;
- the GC/debug `mir::region::RegionId`;
- the private `join_ir::ownership` BindingId/ScopeId family.

Mutable drafts remain crate-private. Only a verified sealed product may become
a public consumer input. Unsupported resolution never retries a legacy path.

SA1's `ShadowBindingOrdinalV0` is now also the construction-local draft index
used inside canonical resolution. It is converted and discarded before seal;
it never enters Planner or Lower and is not semantic identity. Unsupported
syntax returns a typed resolver error and never retries an old resolver. There
is no test-only unverified publication bypass.
