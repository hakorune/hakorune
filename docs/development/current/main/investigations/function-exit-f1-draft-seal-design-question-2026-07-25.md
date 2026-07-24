# FUNCTION-EXIT-F1-DRAFT-SEAL-D0 consultation

Decision authority: `FUNCTION-EXIT-SEMANTICS-prime-r1`

This is a design question, not an implementation authorization. The current
MATERIALIZE0 row is paused until the finalizer ownership boundary is decided.

## Observed seam

The current physical path is:

```text
VerifiedFunctionCompletionV1
  -> ResolvedFunctionCompletionConsumptionV1
  -> emit/observe Return
  -> MirBuilder::finalize_function_draft
```

`finalize_function_draft` still owns fallible type propagation, stale-fact
handling, type-hint metadata, completed-draft verification, function-state
extraction, and Return-operand signature inference. The canonical and trivial
SSA lowerers also do not currently share one synthetic-Return writer.

The accepted F1 law requires:

```text
borrow-only prepare
-> one unpublished prepared owner
-> private infallible commit
```

No Return/signature mutation may be followed by a fallible operation in the
claimed materialization commit.

## Authority lock

Authority:

```text
SealedFunctionExitContractV1
VerifiedFunctionCompletionV1
function-owned type context and current function session
```

Not authority:

```text
last lowered ValueId position
MIR Return scan as source semantics
finalize_function_draft fallback inference
postprocess return repair
Legacy build_module fallback
```

## Questions to close

### Q1 — What exactly is the owner boundary?

Choose one:

```text
A: MATERIALIZE0 only owns Return/signature materialization; the existing
   draft finalizer remains a later fallible boundary, and the card is amended
   to remove the stronger post-commit-infallible claim.

B: introduce PreparedFunctionDraftSealV1. Draft sealing and exit
   materialization are prepared together, then one private commit performs all
   mutation and extraction infallibly.
```

Recommendation: B. The accepted F1 decision currently claims the stronger
law, so A would be a semantic weakening rather than an implementation fix.

### Q2 — Which operations are borrow-only preparation?

For the selected boundary, specify how these are planned without mutation:

```text
TypePropagationPipeline
stale-fact preparation and commit inputs
type-hint metadata
PHI input materialization
completed-draft verification
current_function/current_module extraction
```

Each plan must have a typed error and must not consume the live owner before
the complete prepared product is issued.

### Q3 — Where is the sole synthetic Return writer?

Choose the one owner for implicit Unit materialization and explain how the
canonical lowerer and trivial-SSA lowerer converge. In particular:

```text
implicit fallthrough / empty body
explicit Unit
already-preterminated exact Return
```

must not create two physical Return writers or silently accept a second
terminator.

### Q4 — What is the prepared and committed product?

Define the minimum non-Clone typestate, for example:

```rust
PreparedFunctionDraftSealV1
  -> commit(self) -> CompletedFunctionDraftV1
```

The product must retain the exact function owner, type context plan,
completion contract, Return operand relation, metadata plan, and cleanup
evidence needed by the infallible commit. No bare mutable `MirFunction` may
escape to a second owner.

### Q5 — What is retained on failure?

Define the discard-only rejection owner for failures before preparation:

```text
open Builder session
current function and blocks
type context / stale facts
completion contract and consumption state
route and source-site evidence
typed stage and nested cause
```

The design must say whether a failure after any internal planning step keeps
the original owner, a moved unpublished owner, or a dedicated rejected owner.
Clone rollback, retry, fallback, and post-commit repair are forbidden.

### Q6 — What is the exact exit/type contract?

Fix the relation for the current 0-or-1 slice:

```text
ExplicitValue -> existing exact operand + exact Builder type
ExplicitUnit  -> one Void operand
ImplicitUnit  -> one synthetic Void operand
```

The source contract, not a Return scan, selects the relation. The design must
also state how `: void`, unannotated explicit values, existing preterminated
returns, and unsupported/dynamic types fail before mutation.

### Q7 — What is the smallest executable row and its gate?

Define the first implementation slice and its non-claims. It should remain
limited to the existing exact 0/1 root-terminal routes and cover:

```text
empty / implicit Unit
explicit Unit
explicit value
canonical lowerer
trivial-SSA lowerer
success and typed pre-commit failure retention
```

The gate must prove one Return writer, no Return-based signature inference,
no fallible edge after the prepared commit starts, no source re-observation,
and all modified source/check files below 800 lines.

## Recommendation

Select B and open a strict draft-seal design row. Do not edit
`completion_consumption.rs`, `module_lifecycle.rs`, or either lowerer until
Q1-Q7 are answered. MATERIALIZE0 remains paused and no implementation claim
is made by this card.

## Non-claims

```text
nested/multiple/all-path completion
Script result tail classification
physical Main and process-exit projection
App compatibility execution
public ingress, JSON, executor, normal-entry cutover
old Raw-chain retirement
```
