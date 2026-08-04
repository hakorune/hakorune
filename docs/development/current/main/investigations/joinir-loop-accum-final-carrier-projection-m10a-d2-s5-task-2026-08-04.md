---
Status: Closed implementation boundary
Date: 2026-08-04
Decision: accepted — `JOINIR-LOOP-ACCUM-FINAL-CARRIER-PROJECTION-M10A-D2-S5`
Scope: make the resolved DirectAccum production candidate publish its final
       role-keyed carrier values through the existing After/Binding-SSA owner
Related:
  - joinir-loop-accum-production-bridge-m10a-n2-design-stop-2026-08-03.md
  - joinir-loop-accum-physicalizer-candidate0-m10a-d1-design-stop-2026-08-03.md
  - joinir-loop-accum-verified-recipe-consumer-p1-design-2026-08-03.md
  - joinir-loop-accum-mir-physical-snapshot0-m5-p4-s1-task-2026-08-04.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../../../../reference/mir/phi_invariants.md
  - ../../../../reference/mir/phi_policy.md

# DirectAccum final-carrier projection: M10a D2-S5

## Why this task exists

The first resolved DirectAccum production candidate now reaches the shared
physicalizer through one canonical CFG/Binding-SSA/PhiTxn owner. A test-only
observer of the actual candidate exposed a real contract gap: the candidate
has the standard `Preheader -> Header -> Body -> Step -> Header` shape and an
`After` return, but it does not yet materialize the final `i`/`sum` carrier
reads required by the P1/D1 binding-publication contract. The P4-S1 observer
must not synthesize those rows from header PHIs, so P4-S1 pauses here.

This is a production lowerer contract fix, not a P4 observer workaround. It is
taskized separately because the P4-S1 slice is test-only after physicalization
and must not silently widen its authority.

## Accepted ownership and order

The resolved lowerer remains the sole caller-side owner. The exact order is:

```text
physicalizer emits P/H/B/S/A and leaves A selected
  -> caller derives the VerifiedPredecessorsV1 for A
  -> caller seals A through CanonicalDirectAccumBindingPort::seal
  -> caller reads keys 0 and 1 at sealed A through
     CanonicalDirectAccumBindingPort::read_entry_for_key
  -> caller stores an owned role-keyed final-carrier receipt
  -> caller verifies effect claims without consuming the port
  -> existing semantics/identity/PhiTxn/completion finish and commit
```

The read occurs before `ResolvedSsaIdentityStateV2::finish` and
`PhiTxn::commit`, after the existing `finish_after` predecessor/SSA seal. The
receipt is keyed only by the already verified `LoopBindingKeyV1` carrier rows;
it must not introduce names, raw MIR IDs as authority, or a second binding map.
The physicalizer's generic continuation receipt remains a continuation/result
handoff; the resolved caller owns the final-carrier receipt because it owns the
After seal and function-wide identity session.

The receipt must reject, before finish/commit, missing, duplicate, unexpected,
or out-of-order carrier keys and owner/frame mismatches. The accepted
DirectAccum shape has exactly the two verified carrier keys (`i`, `sum`); no
header-PHI projection or observer-side fabrication is allowed.

The implementation must split the current `finish_after` helper at this
boundary so the port remains alive for the sealed-After reads. The receipt
must contain the two rows in verified JoinSig carrier order plus the existing
`Unit` result disposition and must leave no pending carrier PHI. The existing
caller-zero physicalizer's closed-After `capture_final_values` path is not
changed by this task.

The resolved lowerer must return or otherwise hand off this
`DirectAccumFinalBindingReceiptV1` to its candidate helper; a bare local that
is immediately discarded is not sufficient evidence. Do not widen generic
`ReadyFunctionCompletionV1` or MIR metadata with loop-specific fields. A
`cfg(test)` candidate adapter may retain the receipt for P4 observation, while
the production outer path may consume it after the existing completion
contract has verified it.

## Implementation slice

1. Add one small caller-owned final-carrier receipt/helper in the resolved
   DirectAccum lowerer. Keep the touched Rust files below 800 lines.
2. Read the two verified carrier bindings at the sealed `After` block through
   the existing `CanonicalDirectAccumBindingPort` API. Do not add a PHI writer,
   CFG owner, route branch, retry, fallback, or name-based lookup.
   Change only the effect-claim check's ownership mode if needed so final
   reads happen before the check; the adapter remains the sole read/PHI owner.
3. Preserve the existing finish order and candidate transaction boundary:
   late failure drops the unpublished candidate, and the live Builder remains
   reusable for a fresh candidate/session.
4. Add focused tests proving the actual resolved candidate contains the final
   `i`/`sum` After-carrier evidence and that missing/duplicate carrier rows
   fail fast without commit.
5. Resume P4-S1 only after these tests are green; the P4 observer then reads
   the production evidence and compares it with the legacy alpha snapshot.

## Required gates

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
focused resolved DirectAccum lowerer / Binding-SSA / PhiTxn tests
candidate abort + fresh-session reuse tests (P4-S1 integration follows)
RUSTFLAGS='-Awarnings' cargo check -q
```

Record exact commands, expected/actual counts, caller census, and every
touched-file line count in the closeout.

## Mandatory reference-document closeout

The implementation is incomplete until the implementation commit or its
immediately-following closeout commit updates the applicable reference and
design authorities:

```text
this D2-S5 task and the M10a bridge card
P1/D1 final-carrier wording and the P4-S1 task/card
joinir-loop-selfhost-recipe-pipeline-ssot.md
phi-lifecycle-ssot.md
binding-ssa-first-control-lowering-ssot.md
docs/reference/mir/phi_invariants.md
docs/reference/mir/phi_policy.md
src/mir/builder/README.md
CURRENT_STATE.toml and 10-Now.md when the active pointer/status changes
```

The closeout must explicitly record that no grammar, IR, route, Generic,
Retry, fallback, or selfhost authority changed. Reference-document
synchronization is an acceptance condition, not optional cleanup.

## Explicit non-claims

This task does not claim all Loop families, Generic policy, Retry deletion,
legacy-edge retirement, default `compile_with_source` activation, raw MIR ID
parity, printer-text parity, M10b scheduler cutover, grammar changes, IR
changes, or selfhost ownership. It only closes the missing final-carrier
publication evidence for the resolved DirectAccum candidate.

## Implementation closeout (2026-08-04)

The resolved lowerer now keeps the generic open-After continuation receipt
unchanged, derives the sealed predecessor witness, seals `After` through the
existing `CanonicalDirectAccumBindingPort`, reads verified carrier keys 0/1,
and hands an owned `DirectAccumFinalBindingReceiptV1` to the candidate helper
before effect/identity/PhiTxn/completion finish. The receipt rejects missing,
duplicate, reordered, or non-Unit rows. No open-After read or header-PHI
synthesis is used.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib loop_accum -- --test-threads=1
  6 passed, 0 failed
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_candidate_ -- --test-threads=1
  3 passed, 0 failed
RUSTFLAGS='-Awarnings' cargo test -q --lib final_binding_receipt_rejects_missing_duplicate_and_non_unit_rows -- --test-threads=1
  1 passed, 0 failed
RUSTFLAGS='-Awarnings' cargo check -q
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

The non-test physicalizer caller census remains exactly one at
`resolved_lowering/direct_accum_lowerer.rs`; route_loop, Retry/Option,
fallback, Generic, and legacy physicalizer callers remain zero for this
branch. `wc -l` at closeout reports 360 lines for the production lowerer,
340 for its adapter, and 563 for resolved lowering; every touched Rust file
remains below the 800-line budget. No grammar, IR, route, Generic, Retry,
fallback, or selfhost authority changed.

Reference-document closeout completed in the same implementation series:
the M10a/P4/P1 cards, Loop pipeline, PHI lifecycle, Binding-SSA SSOT,
`docs/reference/mir/phi_invariants.md`, `docs/reference/mir/phi_policy.md`,
`src/mir/builder/README.md`, and current mirrors now describe this receipt and
its sealed-After order.
