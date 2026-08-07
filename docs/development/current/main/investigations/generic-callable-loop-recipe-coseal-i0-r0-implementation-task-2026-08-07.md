# Callable single-loop Recipe co-seal I0/R0

Status: `Decision: bounded caller-zero implementation task; production selection and physicalization are not authorized`

Parent: `RECIPE-COSEAL-D0-r1`

Design authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Change

Consume `VerifiedCallableSingleLoopSourceMapV1` exactly once and publish one
non-Clone caller-zero result with disjoint components:

```text
VerifiedLoopRecipeCoSealV1
  = existing Core/Recipe/JoinSig
  + operation/input source relations
  + semantic context
  + VerifiedLoopContinuationContractV1

VerifiedCallablePreludeV1
VerifiedCallableTailV1
```

Old authority: none. Do not issue `VerifiedLoopAfterTailEnvelopeV1`, a second
Recipe/Core/JoinSig owner, or a callable-specific physical plan.

## Contract

The selected `StringHelpers.int_to_str/1` row maps the existing MAP roles to
the one recursive `LoopRecipeV1` algebra exactly as fixed by the parent and
`docs/reference/mir/loop-recipe-contract.md`. InitialCarrier retains an exact
preheader input relation; PrefixBoundary stays Prelude; TailReturnRead stays
`VerifiedCallableTailV1`; logical Loop After stays the separate continuation.

This row has no authority to issue an exact return ABI or
`VerifiedFunctionCompletionV1`. Existing ABI/Completion issuers remain
unchanged until `LOOP-PHYSICAL-PREPARE-I0-R0`. AST/name/path/ordinal rematch,
Builder, ValueId, BasicBlockId, CFG, PHI, retry, fallback, and production
selection are forbidden. The 19 legacy labels remain ingress coverage only.

## Done

The positive source-map fixture publishes the move-only result and survives
source-view drop. Focused negatives cover missing/duplicate/foreign/unconsumed
rows, owner/frame/Scope/Region and binding mismatches, unsupported policy,
continuation/Tail fusion, and Recipe/JoinSig/relation cross-product mismatch.

`cargo test --lib callable_single_loop --no-fail-fast`, `cargo check --lib`,
the existing reusable Loop/Recipe guard, pointer guard, line guard, and
`git diff --check` are green. The same implementation commit updates
`docs/reference/mir/loop-recipe-contract.md`,
`docs/reference/mir/generic-loop-stage-matrix.md`, the owning README, current
pointer/card, and any immutable receipt changed by the row.

## Stop

Return `NoSafeSlice` before effects if exact mapping requires copied source
truth, a new ABI/Completion issuer, a second continuation/Recipe owner, or a
callable-specific Recipe/physicalizer. After closeout, stop and ask before
`CANONICAL-FUNCTION-FINISH-TERMINAL-R0`; Recipe completion does not claim
physical completion, production activation, or legacy retirement.
