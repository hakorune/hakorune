# Portable If Recipe Contract

This directory owns the Builder-free, fixed-shell semantic contract for the
first If migration slice. The named D0-C adapter is its only production seam;
the contract remains Builder-free while physical CFG/SSA/PHI work stays in the
canonical resolved lowerer.

## Authority

- `IfRecipeArtifactV1` owns schema version, provenance receipt, source claims,
  and one `IfRecipeV1`.
- `IfRecipeV1` owns only the fixed four-block shell: condition, then, explicit
  else, and continuation. Keys are recipe-local and deterministic.
- `IfRecipeVerifierV1` owns structural checks: canonical keys, defined-before-
  use, value classes, one merge target, one write per branch, branch/join
  correspondence, direct-call operation/claim parity, and the required
  post-merge read.
- `IfRecipeSourceClaimVerifierV1` owns only the claim's structural path shape.
  It does not prove that the named AST/function exists or produced the recipe.
- `IfRecipeNormalizerV1` exposes semantic-only and source-bound views. Source
  and provenance are never part of semantic normalization.
- `IfJoinSigElaboratorV1` consumes only `VerifiedIfRecipeV1` and seals the
  fixed logical transfer edges and join values. `VerifiedIfJoinSigV1` is a
  non-`Clone` logical proof, not a physical CFG or PHI owner.
- `VerifiedIfPhysicalInputV1::from_artifact` is the one-shot
  pairing boundary. It consumes one verified artifact and internally
  elaborates the matching JoinSig, so callers cannot supply an unrelated
  artifact/signature pair. This wrapper still has no Builder, MIR IDs, CFG,
  or PHI authority; only the named D0-C adapter may consume it.

## Forbidden dependencies

This subtree must not import AST nodes, `MirBuilder`, `CorePlan`, physical
`ValueId`/`BasicBlockId`, `BindingRef`, route selection/retry, PHI sessions,
or legacy mutation policy. Facts mapping is owned by the resolved profile;
PHI physicalization and production caller wiring belong to later task rows.

`IfRecipeProvenanceV1` is a profile receipt, not route authority. The portable
recipe is not a synthetic Loop and does not reuse Loop carrier/exit ownership.

## Extension boundary

The first profile admits only explicit-else, fallthrough-only Ifs with scalar
`i64`/`bool` operations. A direct static `i64` call is admitted only as an
assignment RHS: zero or one call for the existing no/one-call shapes, or one
call in each explicit branch for the two-call D0 shape. The latter emits six
ordered source claims (`IfNode`, `Condition`, `ThenAssignment`,
`ElseAssignment`, then-call, else-call); branch identity is carried by the
exact source path and claim position, while the portable operation remains
identity-free. Calls in conditions, arguments, continuations, nested shapes,
implicit fallthrough, or any third/duplicate arm call remain rejected.

Add a new operation, recursive block shape, or implicit-else profile only with
a counterexample fixture and a separate design/acceptance row. Do not widen
this contract to hide an unsupported production path.

## Nested depth-one profile (D0)

`nested_schema.rs`, `nested_verify.rs`, and `nested_join_sig.rs` own a
separate, disconnected profile for exactly one outer explicit-`else` If with
one explicit-`else` child in the outer `then` branch. The profile is
`ResolvedTrivialExplicitElseDepthOne` and contains two deterministic node
keys, one shared `i64` binding, portable source claims, and a composition row
that transfers the child merge into the outer `then` edge.

This profile is not a recursive extension of `IfRecipeV1`. The original
four-block schema remains immutable and continues to reject nested structure.
The nested profile has no production physicalizer, route selection, retry,
Builder access, or PHI/SSA ownership. Its D0 consumer is the resolved-value
profile mapper and focused contract tests only. A future production consumer
must first satisfy the nested execution card's D1 owner census and D2
candidate-abort/parity gates.

Depth greater than one, implicit `else`, multiple carried bindings,
unsupported operations, and missing transfer/continuation evidence are typed
producer or verifier rejections. They must not be widened through the
original one-If contract or silently sent to a retry route.

## Current non-claims

- Only the named D0-C adapter consumes this artifact, logical JoinSig, and
  physical-input wrapper; no other production route may construct or consume
  them.
- No canonical PHI writer or physical block projection is created here.
- Verification proves internal artifact structure, not AST provenance or MIR
  parity.
