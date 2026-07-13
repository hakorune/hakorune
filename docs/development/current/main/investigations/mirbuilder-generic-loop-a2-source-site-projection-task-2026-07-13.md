# Generic Loop A2-C2 — Source Site Projection Taskboard

Status: Superseded by Generic Loop Baseline V1; source projection continues as G0 coverage infrastructure.
Date: 2026-07-13
Decision: hybrid (`A + E-light + sealed paired provenance`).
Classification: BoxShape only; acceptance-neutral until C2-I4.

## Purpose

Superseding taskboard:

```text
docs/development/current/main/investigations/
  mirbuilder-generic-loop-baseline-v1-task-2026-07-13.md
```

Repair the missing source/Recipe identity owner without contaminating generic
Recipe vocabulary with source provenance:

```text
canonical source tree
  -> SealedSourceProjectionV0
  -> CandidateRecipeDraftV0
       generic Recipe tree
       sealed paired positional provenance
       optional exact CanonicalStepWitnessV0
  -> GenericRecipeVerifier
  -> SourceRecipeBijectionVerifierV0
  -> VerifiedCandidateLoopRecipeV0
  -> Unique / Zero / Multiple
  -> Lower
```

The first slice is only the source projection.  Recipe, candidate selection,
Lower, parser/source-carrier P1, and acceptance remain unchanged.

## Decision

Adopt:

```text
A:
  SourceStmtSiteV0 is canonical source identity

E-light:
  replace the generic-loop untyped flatten route with an
  identity-preserving typed projection

sealed paired provenance:
  Recipe and provenance remain separate types but are co-constructed,
  co-sealed, and positionally bound
```

Reject:

```text
RecipeItem / StmtRef source fields
mutable path-keyed sidecar maps
canonical-step shape filtering
full rewrite of every Recipe builder
ProgramV0 or parser P1 as identity authority
```

## Authority

```text
source statement identity:
  immutable canonical source tree + SourceStmtSiteV0

transparent-container policy:
  TransparentSourceContainerPolicyV0

source projection:
  SourceProjectionBuilderV0 + SourceProjectionSealerV0

source/Recipe correspondence construction:
  CandidateRecipeDraftBuilderV0

exact coverage:
  SourceRecipeBijectionVerifierV0

generic Recipe contract:
  existing Recipe verifier

canonical step identity:
  exact SourceStmtSiteV0 + CanonicalStepWitnessV0

final acceptance:
  LoopProgressionSelectionV0
```

`VerifiedRecipeBlock` keeps its current meaning.  Composite source-aware proof
uses a distinct type:

```text
VerifiedCandidateLoopRecipeV0 {
  recipe: VerifiedRecipeBlock,
  projection: VerifiedSourceRecipeProjectionV0,
  progression: LoopProgressionProofV0,
  step: StepDispositionV0,
}
```

## Non-authority

```text
AST equality or pointer identity
source span / token offset
candidate-local preorder_index
flattened top_level_stmt_index
BodyId or StmtRef alone
Recipe/Lower success
ShapeId / numeric rank / names
ProgramV0
mutable path-keyed maps
```

## Source site schema

```text
SourceStmtSiteV0 {
  root_kind: SourceRootKindV0,
  segments: NonEmptyVec<SourceStmtPathSegmentV0>,
}

SourceRootKindV0 =
  Program | FunctionBody | MethodBody | LambdaBody | BlockBody

SourceStmtPathSegmentV0 =
  RootBody(index)
  | ProgramBody(index)
  | BlockBody(index)
  | IfThen(index)
  | IfElse(index)
  | LoopBody(index)
  | ScopeBody(index)
```

Diagnostic grammar:

```text
$.(program|function|method|lambda|block).body[Index]
  (.program.body[Index]
   |.block.body[Index]
   |.if.then[Index]
   |.if.else[Index]
   |.loop.body[Index]
   |.scope.body[Index])*
```

Paths are deterministic only within the same immutable source root.  They are
not persistent IDs across source edits.

## ScopeBox policy

`ScopeBox` is a transparent structural container, not an executable statement.
It is still accounted exactly once in a separate container domain:

```text
TransparentContainerExpansionV0 {
  container_site,
  child_projected_sites_in_order,
}
```

Children retain `ScopeBody(index)` in their source paths while appearing in
effective projected order.  If ScopeBox gains independent lifetime, fini,
visibility, allocation, exit, or other semantic effects, stop and classify it
as Unsupported or open a separate Recipe-vocabulary BoxShape.

## Projection model

```text
SealedSourceProjectionV0 {
  bodies: Vec<ProjectedBodyV0>,
  statements: Vec<ProjectedStmtV0>,
  transparent_expansions: Vec<TransparentContainerExpansionV0>,
}

ProjectedBodyV0 {
  id,
  origin,
  statements_in_effective_order,
}

ProjectedStmtV0 {
  id,
  source_site,
  node,
  child_body_roles,
}
```

`If` preserves then-body identity and exact else absence/presence.  `Loop`
preserves its child-body identity.  Arena IDs are invocation-local handles;
only structural source paths are identity authority.

## Paired Recipe provenance

Later slices add:

```text
RecipeBodyBindingV0 {
  recipe_body_id,
  projected_body_id,
  slots: Vec<ProjectedStmtIdV0>,
}
```

Recipe AST slots and provenance slots must be produced by one builder call.
Post-build attachment and arbitrary source-path injection APIs are forbidden.

The ledger is a checked witness, not authority.

## Coverage laws

```text
E = executable source statement multiset
T = transparent container multiset
R = reachable Recipe-accounted source multiset
S = canonical-step witness multiset
C = transparent-container certificate multiset

R ⊎ S = E
multiplicity(E site) = 1
R ∩ S = empty

C = T
multiplicity(T site) = 1
```

Every reachable RecipeBlock must also reference each local RecipeBody slot
exactly once.  Bounds, omission, and duplicate are separate failures.

Open construction may contain scratch bodies.  A sealed draft may not contain
unreachable/orphan bodies; seal should defensively reconstruct the published
arena from reachable bodies.

## Canonical step

```text
StepDispositionV0 =
  BodyManaged
  | CanonicalExternal(CanonicalStepWitnessV0)
```

Canonical construction omits exactly one `ProjectedStmtIdV0`, never every AST
matching a predicate.  Identical statements at other sites remain in Recipe.
BodyManaged omits nothing and adds no synthetic or implicit step.

Unsupported placement remains Unsupported; C2 adds no RecipeItem or CFG
wiring.

## Failure domains

```text
Unsupported:
  valid source outside transparent/projection/Recipe vocabulary

Ambiguous:
  multiple E-proven and R-verified candidates only

InternalProjectionContractViolation:
  invalid/duplicate source path, broken child provenance, projection drift

InternalRecipeContractViolation:
  Recipe omission/duplicate/OOB, double accounting, orphan sealed body,
  generic Recipe contract failure
```

Internal failures must not be rounded to Unsupported or Ambiguous.

## Task order

### C2-I0 — identity-preserving projection (active)

1. Add physically separate schema/path/model/builder/sealer modules.
2. Add `SourceRootKindV0`, path segments, and `SourceStmtSiteV0`.
3. Add projected body/statement IDs and immutable projected records.
4. Implement transparent ScopeBox expansion with exact source paths.
5. Preserve typed If/Loop child-body identities and else presence.
6. Seal only complete, internally consistent projections.
7. Compare projected AST sequence with old flatten output on the accepted
   corpus; the old flatten is compatibility evidence, never authority.
8. Keep generic-loop product path and acceptance unchanged.

### C2-I1 — paired body registration and Recipe-local coverage

Co-construct RecipeBody and positional provenance, then add omission,
duplicate, OOB, and reachable-body verification.  No candidate selection.

### C2-I2 — nested provenance and sealed arena

Verify exact If then/else and Loop child-body correspondence, absence versus
present-empty, transparent certificates, and orphan-free publication.

### C2-I3 — exact canonical-step witness

Carry one exact projected source site, retire shape-based body filtering, keep
all other same-shaped statements, and preserve every BodyManaged write.

### C2-I4 — candidate integration

Build `VerifiedCandidateLoopRecipeV0`, then apply E + R and
Unique/Zero/Multiple.  Lower consumes only the composite verified result.

## C2-I0 gates

```text
same immutable source tree -> exact repeated projection equality
identical statements at different indices -> distinct source sites
equal/unknown spans -> distinct source sites
If then/else -> distinct typed paths
nested Loop -> every LoopBody boundary retained
nested ScopeBox -> every ScopeBody boundary retained
old flattened AST sequence == projected effective AST sequence
invalid segment/node role -> InternalProjectionContractViolation
missing/duplicate Scope expansion -> InternalProjectionContractViolation
acceptance widening = 0
Recipe/Lower/product-path connection = 0
ProgramV0 outcome unchanged
all files < 800 lines
```

## Isolation guards

Forbidden in the projection family:

```text
source span or AST pointer as identity
path-keyed HashMap sidecar
ProgramV0
parser/source-carrier P1
Recipe/Lower/backend/runtime
ShapeId / rank / names / environment toggle
```

## May claim after C2-I0

```text
generic-loop source statement identity survives transparent projection
ScopeBox expansion preserves structural paths
If/Loop child body identities and optional presence are typed
identity is independent of source spans and AST pointers
acceptance behavior is unchanged
```

## Must not claim after C2-I0

```text
Recipe source coverage is verified
canonical filtering is retired
candidate selection is connected
all Recipe users carry provenance
ScopeBox is universally transparent
Hako parity is implemented
parser/source-carrier P1 is connected
```

## Retirement

After I4, retire generic-loop `flatten_scope_boxes`, shape-based step filtering,
identity uses of candidate preorder/top-level indices, dummy body-managed step
sentinels, and raw `loop_increment: ASTNode` ownership.  Keep Recipe core
source-independent.  Promote provenance into Recipe core only under a future
cross-family decision.

## Stop conditions

1. Paths are generated from flattened indices rather than the source tree.
2. ScopeBox segments disappear.
3. Pointer/span/token identity is introduced.
4. Provenance can be attached or mutated after Recipe construction.
5. Canonical filtering still uses AST shape equality.
6. Same-shaped steps cannot remain distinct.
7. BodyManaged drops any write.
8. Child bodies are validated only by bounds.
9. Absent and present-empty children are merged.
10. Sealed drafts retain orphan BodyIds.
11. Internal failures are rounded to Unsupported/Ambiguous.
12. Lower rediscovers role from AST/source path.
13. New RecipeItem or CFG wiring is required.
14. ProgramV0 or parser P1 enters this lane.
15. Any source file reaches 800 lines.
