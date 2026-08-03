# JOINIR-LOOP-NESTED-PREDICATE-CLOSURE0-D2-D-RECIPE-PRODUCER

Status: producer implementation landed; parity/negative closure is active.
Date: 2026-08-03

## Decision

Authorize one caller-zero `NestedLoopMinimal` producer that consumes the
sealed `VerifiedNestedLoopSourceProjectionV1` by value and emits exactly one
portable semantic product:

```text
VerifiedNestedLoopSourceProjectionV1
  -> canonical two-loop LoopRecipeArtifactV1
  -> source-bound verifier terminal
  -> VerifiedLoopRecipeV1
  -> VerifiedLoopJoinSigV1
```

The producer is the first consumer of D2-C/D2-C1a. It is not a route selector,
legacy fallback, physicalizer, PHI writer, or Builder caller.

## Source-to-recipe boundary

- The projection's forest is consumed exactly once. Its two ordered members
  map to Recipe loop keys `0` (root) and `1` (child); the existing parent index
  remains the sole nesting proof.
- The three resolver BindingRefs map by sealed projection position to the
  canonical Recipe bindings `0` (root recurrence), `1` (ancestor recurrence),
  and `2` (child recurrence). Recipe labels are producer-local diagnostics;
  source names are not reread or dispatched by name.
- D2-C1a root initializer evidence must bind to the two root recurrence
  bindings and carry exact integer values `0`. The portable Recipe keeps those
  root values as opaque pre-loop `inputs`/carrier entry values; the producer
  must not synthesize extra root `ConstI64` operations or claim numeric input
  parity.
- The child initializer evidence must be integer `0` and is emitted as the
  child-local `ConstI64` entry operation. Predicate bounds and update deltas
  come only from the sealed source shape; missing or inconsistent evidence is
  a typed reject.
- The source forest is converted to `LoopRecipeSourceBindingV1` only after
  semantic verification, so its member count and parent indexes are checked
  against the canonical Recipe.

## Verifier seam

`LoopRecipeVerifierV1::verify_artifact` currently returns a private artifact
capability. Add one crate-visible terminal that consumes the artifact and
returns only `VerifiedLoopRecipeV1`; do not expose provenance or the private
source-claim capability to the producer. This is an API visibility seam, not
new verification logic.

## Non-goals and owner boundary

- No AST, `FunctionSourceViewV1`, `Located*`, or resolver reread in the
  producer.
- No new route selection, retry, `None`, Generic fallback, or legacy JoinIR
  caller.
- No `MirBuilder`, `ValueId`, `BasicBlockId`, CFG, PHI, SSA, physicalizer, or
  candidate publish. Existing PHI/SSA SSOT remains
  `CanonicalSsaFunctionSessionV2` -> `CanonicalCfgSessionV1` +
  `BindingSsaBuilderV1` + `PhiTxn`.
- D2-D must not change the Recipe schema or broaden the nested Predicate
  grammar. The existing bounded D2-B JoinSig elaborator is the only logical
  consumer.

## Acceptance gates

1. A positive resolver fixture produces a verified two-loop Recipe and
   deterministic JoinSig. The source forest is consumed once and the source
   claim contains both loop paths with parent `0 -> 1`.
2. Root initializer values are validated as `0` without adding root constants;
   child initializer emits exactly one local zero constant.
3. Typed negatives reject missing/mismatched root initializer evidence,
   nonzero child initialization, forest/Recipe parent mismatch, and any
   unsupported shape before a Recipe or JoinSig escapes.
4. Producer module imports no AST/Builder/physical/PHI/SSA/route scheduler
   symbols. Caller count is exactly one under `cfg(test)` and zero in
   production.
5. D2-C1a, D2-B, `loop_recipe_contract`, targeted `phi_lifecycle`, source
   guards, and `cargo check --lib` remain green. Touched files stay below 800
   lines.

After this card closes, the next slice is parity fixtures for the producer;
physical M6 remains a separate design/implementation lane.
