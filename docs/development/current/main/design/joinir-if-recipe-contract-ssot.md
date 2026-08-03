---
Status: accepted design boundary; D0-B schema/verification work pending
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

The eventual `IfRecipeArtifactV1` owns a schema version, owned source
provenance/binding, and one semantic `IfRecipeV1`. The semantic product owns:

```text
  typed Bool condition value from the admitted i64-comparison profile
then block
explicit else block for the selected shape
ElseDisposition::Explicit(block) | ImplicitFallthrough (later shape)
ordered leaf operations and BindingRef/value facts
branch-transfer obligations
post-merge read obligation
logical JoinSig merge/predecessor/value-edge obligations
recipe-local canonical keys only
```

The schema has no AST nodes, `LocatedStmt`, `MirBuilder`, `CorePlan`, physical
`ValueId`, physical `BasicBlockId`, callbacks, route retry, or emission command.
Control flow stays in the recursive block algebra. Operations are leaves; a
nested If or Loop is a block item, never an opaque operation payload.

The semantic contract must remain shape-scoped until a separate design row
proves a shared control vocabulary with the Loop artifact. Do not create a
second universal control algebra merely to remove a few duplicate structs.

## Verification and elaboration

`IfRecipeVerifierV1` owns structural obligations:

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
add a builder-free shape projection or exact preflight facts. The D0-C adapter
then consumes the verified recipe and supplies the existing canonical trivial
lowerer without re-reading source to make route decisions.

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

## Non-claims

This document does not claim that the schema exists in Rust, that a production
If recipe consumer exists, or that repository-wide PHI/CFG ownership is
unified. Those claims require D0-B implementation and D0-C/D0-D evidence.
