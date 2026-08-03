---
Status: accepted design boundary; D0-B0/B1 and D0-B2-A/B/C facts/mapper gates landed; D0-B2-D firewall classified; D0-B3-A logical JoinSig, D0-B3-B physical-input seal, and D0-B3-C guard/boundary gates landed; D0-C producer/consumer design and D0-C1/D0-C2 implementation landed; D0-D/E physical adoption and selected-edge cutover remain design-gated
Date: 2026-08-04
Decision: JOINIR-IF-RECIPE-CONTRACT-V1
Scope: portable semantic contract for the first resolved-trivial If shape
Related:
  - docs/development/current/main/investigations/joinir-if-recipe-ssa-adoption0-d0-design-stop-2026-08-04.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
---

# If Recipe Contract

## Authority split

`CanonicalSsaFunctionSessionV2` (`CanonicalCfgSessionV1` + function-owned
`BindingSsaBuilderV1` + `PhiTxn`) is the physical SSA/CFG/PHI owner for the
resolved-trivial lane. It is not yet the sole If writer in the repository.

The portable If recipe is a semantic product between observation/facts and
verified lowering. Raw `IfForm`, resolved A+ `IfCfgSessionV1`, CorePlan
`apply_if_joins`, JoinIR converter writers, and JSON-v0 bridge writers remain
parity or compatibility authorities until their shape-scoped caller-zero rows
close. They must not be silently treated as portable recipe consumers.

## Selected V1 shape

The first production candidate is intentionally narrow:

```text
resolved-trivial profile
explicit else
then and else fall through normally
one outer BindingRef is assigned exactly once in each branch
both values have one admitted homogeneous class (i64/Bool)
condition is Bool from the admitted i64-comparison profile
the same BindingRef is read after the merge
no nested If/Loop/BlockExpr, return/throw, short-circuit, Call, Record, Match,
or hidden fallback/reselection
```

An implicit fallthrough is a later shape. It is not equivalent to an explicit
empty else because its predecessor and PHI input obligations differ.

## Contract shape

The eventual `IfRecipeArtifactV1` owns a schema version, structural source
provenance/binding, and one semantic `IfRecipeV1`. D0-B1 uses a fixed
four-block shell (`condition`, `then`, `else`, `continuation`) for the selected
shape. Recursive/nested If items are a later shape; the shell is intentionally
not a synthetic Loop and does not import Loop carrier/exit semantics. The
semantic product owns:

```text
  typed Bool condition value from the admitted i64-comparison profile
then block
explicit else block for the selected shape
ElseDisposition::Explicit(block) | ImplicitFallthrough (later shape)
ordered leaf operations plus recipe-local binding/value keys
branch-transfer obligations
post-merge read obligation
logical JoinSig merge/predecessor/value-edge obligations
recipe-local canonical keys only
```

`IfJoinRowV1.entry_value` is a logical pre-If input, not a condition-expression
shortcut. The B0 facts product currently lacks this witness. D0-B2 must close
the gap with a non-`Clone` entry-witness capability backed by the same-pass
pre-branch environment or an already sealed definition-origin ledger. If that
ledger is insufficient, extend the facts owner; never synthesize the value
from a `ReadBinding` or change the portable row to carry `BindingRefV1`.

The schema has no AST nodes, `LocatedStmt`, `MirBuilder`, `CorePlan`, physical
`ValueId`, physical `BasicBlockId`, callbacks, route retry, or emission command.
The producer-side `BindingRefV1` correspondence remains outside the artifact;
the wire carries only `IfBindingKeyV1`/`IfValueKeyV1`-style local identities.
Control flow stays in the recursive block algebra. Operations are leaves; a
nested If or Loop is a block item, never an opaque operation payload.

The semantic contract must remain shape-scoped until a separate design row
proves a shared control vocabulary with the Loop artifact. Do not create a
second universal control algebra merely to remove a few duplicate structs.

## Verification and elaboration

The existing resolved topology/effect verifiers remain authorities for their
own products. A same-pass shape projection supplies the additional condition,
branch-operation, assignment-cardinality, and continuation facts; it must not
re-scan the source after the recipe is sealed. `IfRecipeVerifierV1` owns only
the portable structural obligations:

- canonical local key order and unique definitions;
- exact condition/value classes;
- explicit-else versus implicit-fallthrough distinction;
- exactly one write to the same outer BindingRef per branch;
- equal admitted branch value class;
- post-merge read names the written binding;
- no unsupported nested/exit/effect shape in this row.

`IfJoinSigElaboratorV1` owns logical predecessor and value-edge proof. It must
prove two distinct branch predecessors and the two incoming values before any
physical ID or Builder effect. The verifier and JoinSig elaborator remain
separate owners, as in the Loop contract.

The source-side producer cannot be `VerifiedResolvedIfFlowV1` alone: that flow
does not carry enough condition/assignment/cardinality information. D0-B must
add a builder-free shape projection (`VerifiedTrivialIfRecipeFactsV1`) from the
same pre-Builder traversal. The D0-C adapter then consumes the verified recipe
and supplies the existing canonical trivial lowerer. The lowerer may borrow an
immutable source view for already-admitted leaf emission, but it must not
re-scan source to choose a route, repair JoinSig, or reinterpret the recipe.

## Cutover boundary

```text
one selected trivial profile
  -> builder-free IfRecipe producer
  -> IfRecipeVerifier
  -> IfJoinSig elaboration
  -> CanonicalSsaFunctionSessionV2
  -> existing canonical If physicalization
  -> unpublished compile candidate
```

The first cutover retires only the selected trivial shape's competing route
edge. It does not retire raw IfForm, A+ IfCfgSession, CorePlan If, JoinIR
converter writers, or JSON-v0. A physicalizer may return success or Freeze;
it may not return `Option`, retry, or a different route.

## D0-C producer/consumer decision

The first production seam is the resolved-trivial canonical ingress, not raw
`IfForm`, A+ `IfCfgSessionV1`, CorePlan/JoinIR, or JSON-v0. The central
`lower_resolved_trivial_function_draft_retaining_failure_v1` entry is the sole
producer call site. It consumes the already sealed
`VerifiedTrivialCanonicalOwnerV1::recipe_facts()` and performs exactly:

```text
map_trivial_if_recipe_v1(profile, input.function())
  -> VerifiedIfRecipeArtifactV1
  -> VerifiedIfPhysicalInputV1::from_artifact
```

`recipe_facts()==None` is pre-effect `NotThisShape`; it must never become a
physicalizer `Option`, post-effect retry, or route reselection. Mapper, JoinSig,
or physical-input rejection is a typed Freeze before Builder mutation.

The consumer is a one-shot admission bridge into the existing
`CanonicalTrivialSsaLowererV1` and `CanonicalSsaFunctionSessionV2`. It checks
source-claim/root correspondence and the logical JoinSig, then delegates
physical CFG/SSA/PHI work to the existing canonical session. It may borrow an
immutable source view for admitted leaf emission, but may not rescan source to
choose or repair a route. The existing `lower_if` remains the parity/physical
oracle until D0-D; D0-C does not claim selected-edge retirement.

D0-C1 promotes the mapper to one production caller. D0-C2 threads and
consumes the non-Clone physical input once; focused resolved-lowering parity,
candidate lifecycle, and structural guards are green. D0-D is the later
physical adoption row; D0-E is the selected old-edge cutover row.

## Non-claims

The fixed-shell schema, source-claim verifier, structural verifier,
deterministic normalizers, same-pass entry witness, caller-zero facts mapper,
and reachable mapper rejection matrix now exist in
`src/mir/if_recipe_contract/` and `src/mir/resolved_value_profile/` (commits
`8999950faf`, `a907874551`, `1bd50829c5`, `f2afec934d`, and
`1fd0e5ab70`, `46a4ccfcf8`, and `1d9b8aa78d`). Defensive-only mapper variants are not ordinary-input
coverage; D0-B2-D retains them as a documented sealed-facts firewall without
synthetic malformed facts. D0-C1/D0-C2 now define the first named production
producer/consumer seam. They do not make repository-wide PHI/CFG ownership
sole or retire the selected old writer; those claims require D0-D/E evidence.
