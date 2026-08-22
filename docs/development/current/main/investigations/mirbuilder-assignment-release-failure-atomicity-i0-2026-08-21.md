Status: Gate 1 implementation complete; Gate 2 structural guard is the next independent slice
Date: 2026-08-21
Priority: High
Owner: `src/mir/builder/assignment_lowering.rs`
NextCard: MIR-RESULT-DISCARD-GUARD-I0
---

# MIRBuilder assignment ReleaseStrong failure atomicity I0

## Decision

Treat the previous-strong-reference release as a required physical emission
inside assignment publication. An assignment must not publish its new local
value when `ReleaseStrong` fails.

The current code does this:

```rust
let _ = self.emit_instruction(MirInstruction::ReleaseStrong { .. });
variable_map.insert(var_name, published_value);
Ok(published_value)
```

That permits physical emission failure and semantic/local publication to
diverge. The ignored result is a real failure-atomicity hole, not a style
warning.

Gate 1 now propagates the `ReleaseStrong` result with `?`. The existing
function-owned session boundary remains responsible for discarding an
unpublished child draft; this slice does not add a second transaction or
physical writer.

The broader fallible-result policy is documented separately in
[`mirbuilder-result-discard-policy-d0-2026-08-21.md`](./mirbuilder-result-discard-policy-d0-2026-08-21.md).
This card is the first concrete consumer of that policy; it must not wait for
a repo-wide lint cleanup and must not hide behind a blanket allow.

## Authority and boundary

Source authority is the already-evaluated assignment value and the existing
function-owned binding/local-contract state. `MirBuilder::emit_instruction`
remains the sole physical writer. `variable_ctx.variable_map` remains the
sole local publication owner.

The fail-fast boundary is immediately before `variable_map` publication:

```text
validate declaration and local contract
  -> prepare required LocalContractWrite / ReleaseStrong effects
  -> commit physical effects through the sole writer
  -> publish variable_map only after all required emits succeed
```

An error must reach the enclosing function/session discard owner. No fallback,
second emitter, silent `ReleaseStrong`, or partial local publication is
allowed.

## Smallest implementation slice

`MIR-ASSIGNMENT-RELEASE-FAILFAST-I0` has one responsibility:

1. replace the ignored `ReleaseStrong` result with typed propagation;
2. preserve the existing outer function-session discard contract for earlier
   MIR/metadata changes;
3. if the existing session boundary cannot provide that guarantee, introduce
   the smallest private prepare/commit receipt around this assignment only;
4. add focused success/failure tests and a reusable structural guard.

The first implementation choice is to use `?` only if the existing enclosing
session demonstrably discards the preceding `LocalContractWrite`, metadata,
and type state on failure. Otherwise split the assignment tail into a private
prepare/commit seam so `variable_map` is published only after the required
physical effects have committed. Do not redesign all emission in this card.

## Gate 1 implementation checkpoint — 2026-08-22

The session audit confirmed the smallest safe implementation boundary:

- `FunctionOwnedStateTransactionV1` moves the caller-owned function, local
  maps, binding/type state, pending PHI state, metadata-bearing compilation
  state, and related transient state into the child lowering attempt;
- `CanonicalFunctionLoweringSessionV1` restores that state on an error and
  does not publish the failed child draft;
- the existing session tests cover every fallible checkpoint plus the case
  where both the primary and cleanup errors must be preserved.

Therefore the production change is exactly:

```rust
self.emit_instruction(MirInstruction::ReleaseStrong { values: vec![prev] })?;
```

The focused negative fixture removes the current function immediately before
the reassignment. The sole physical writer then fails, while the old
`variable_map` value remains and no `ReleaseStrong` instruction is appended.
The fixture does not alter the emitter's block-creation behavior or introduce
a test-only failure hook.

Evidence:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  variable_assignment_descent --lib       -> 6 passed, 0 failed

function_session_tests::every_fallible_checkpoint_restores_caller_and_publishes_nothing
  -> passed
function_session_tests::primary_and_cleanup_errors_are_both_preserved
  -> passed

ignored `emit_instruction` result under src/mir/builder -> 0
ignored `ReleaseStrong` result under src/mir/builder -> 0
rustfmt check for touched Rust files -> passed
git diff --check -> passed
```

The reusable machine guard is intentionally the next card, so this Gate 1
commit does not widen into a workspace Clippy rollout or classify the other
MIRBuilder discard families.

## Non-authority

The following must not become new assignment authorities in this slice:

- a second physical emitter or direct instruction append;
- `variable_map` as proof that `ReleaseStrong` succeeded;
- debug output, test-only flags, fallback assignment, or compatibility retry;
- `ValueId`/metadata snapshots used as a semantic repair authority;
- a repository-wide `emit_instruction` rewrite;
- `builder.rs` barrel cleanup or directory reorganization;
- Script A/C capability, Recipe/Join, publication, or production-switch work.

## Acceptance evidence

Positive:

- successful reassignment emits one required `ReleaseStrong` and publishes the
  new value exactly once;
- pins and assignments without a prior strong value retain their current
  behavior;
- a terminated block does not emit a release and keeps the existing terminal
  contract.

Negative:

- an injected `ReleaseStrong` emission failure is returned to the caller;
- `variable_map` does not contain the new value after that failure;
- the enclosing function/session can be discarded and reused without the
  failed assignment being observable;
- no later assignment or compatibility path repairs the failed publication.

Structural:

```text
ignored ReleaseStrong result                    = 0
variable_map publication before release success = 0
second physical writer                          = 0
assignment fallback/retry                       = 0
assignment_lowering.rs                          < 760 lines
```

Required focused commands and results belong in this card at closeout. A
baseline red elsewhere in MirBuilder must be classified separately and may
not be used as evidence for this task.

## Follow-up boundaries

After this task, a separate design/cleanup row may audit strict canonical
emission versus legacy repair inside `builder_emit.rs`. That later row may
introduce a named prepared-emission receipt or strict/legacy API split while
keeping one final physical writer. It is not part of this assignment fix.

The `builder.rs` barrel and `builder_init.rs` responsibility cleanup are also
caller-zero/retirement follow-ups. They must wait until their production
owners and cross-import boundary are documented.
