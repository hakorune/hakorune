# Portable If Recipe Contract

This directory owns the Builder-free, fixed-shell semantic contract for the
first If migration slice. It is intentionally disconnected from production
lowering; the caller-zero logical JoinSig is accepted, while physical input and
production consumption remain later rows.

## Authority

- `IfRecipeArtifactV1` owns schema version, provenance receipt, source claims,
  and one `IfRecipeV1`.
- `IfRecipeV1` owns only the fixed four-block shell: condition, then, explicit
  else, and continuation. Keys are recipe-local and deterministic.
- `IfRecipeVerifierV1` owns structural checks: canonical keys, defined-before-
  use, value classes, one merge target, one write per branch, branch/join
  correspondence, and the required post-merge read.
- `IfRecipeSourceClaimVerifierV1` owns only the claim's structural path shape.
  It does not prove that the named AST/function exists or produced the recipe.
- `IfRecipeNormalizerV1` exposes semantic-only and source-bound views. Source
  and provenance are never part of semantic normalization.
- `IfJoinSigElaboratorV1` consumes only `VerifiedIfRecipeV1` and seals the
  fixed logical transfer edges and join values. `VerifiedIfJoinSigV1` is a
  non-`Clone` caller-zero proof, not a physical CFG or PHI owner.
- `VerifiedIfPhysicalInputV1::from_artifact` is the caller-zero one-shot
  pairing boundary. It consumes one verified artifact and internally
  elaborates the matching JoinSig, so callers cannot supply an unrelated
  artifact/signature pair. This wrapper still has no Builder, MIR IDs, CFG,
  PHI, or production consumer.

## Forbidden dependencies

This subtree must not import AST nodes, `MirBuilder`, `CorePlan`, physical
`ValueId`/`BasicBlockId`, `BindingRef`, route selection/retry, PHI sessions,
or legacy mutation policy. Facts mapping is owned by the resolved profile;
PHI physicalization and production caller wiring belong to later task rows.

`IfRecipeProvenanceV1` is a profile receipt, not route authority. The portable
recipe is not a synthetic Loop and does not reuse Loop carrier/exit ownership.

## Extension boundary

The first profile admits only explicit-else, fallthrough-only Ifs with scalar
`i64`/`bool` operations. Add a new operation, recursive block shape, or
implicit-else profile only with a counterexample fixture and a separate
design/acceptance row. Do not widen this contract to hide an unsupported
production path.

## Current non-claims

- No production If lowerer consumes this artifact, logical JoinSig, or
  physical-input wrapper yet.
- No canonical PHI writer or physical block projection is created here.
- Verification proves internal artifact structure, not AST provenance or MIR
  parity.
