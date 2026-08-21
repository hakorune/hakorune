Status: selected design stop; no implementation authorized
Task: MIR-LOOP-COMPARE-CONNECT0-D0
Date: 2026-08-22
Priority: caller census and atomic handoff before production connection
Parent: MIR-LOOP-COMPARE-STRICT-WRITER-P0
PreviousCard: MIR-LOOP-COMPARE-STRICT-WRITER-P0
NextCard: none until the named caller and handoff are accepted
---

# Loop Compare CONNECT0 design stop

## Six-line brief

```text
Decision: keep CONNECT0 at design_stop until one named non-test production caller is selected; the strict writer remains a caller-zero physical contract.
Source authority + canonical issuer: the existing Recipe-order operation row, exact Loop target receipt, full Published operand receipts, owner-bound result ledger, and canonical CFG/SSA session must be co-sealed by one dispatcher handoff issuer before any Compare effect.
Non-authority: focused canary callers, test-only Generic G0 session, old emit_compare_i64_at, raw ValueId/state.get, current_block, Builder cursor, operation enum alone, and a fallback retry.
Fail-fast boundary: caller selection, owner/target/operand/result relation, strict destination/Bool preparation, and result-slot reservation must complete before the first Compare append; a rejected strict row cannot return to the old leaf.
Smallest next slice: read-only census of the named non-test caller and a finite dispatcher handoff table; no code, fixture, fallback, or production switch until that table is accepted.
Non-claims: no caller selection by test convenience, no general dominance, no cross-block operands, no Const/Binary migration, no A/C/Recipe redesign, no old-leaf retirement, and no production I0/R0.
```

## Current census

The strict writer P0 is intentionally caller-zero:

```text
CanonicalLoopCompareI64WriterV1::emit production callers = 0
CanonicalLoopCompareI64WriterV1::emit focused test callers = 3
emit_loop_segment_operation_dispatch_v1 non-test callers = 0
emit_prepared_pure_operation_v1 non-test callers = 0
```

The old `emit_compare_i64_at` remains a shared legacy leaf with callers in
`pure_operation_emitter.rs` and other canonical canary/compatibility areas.
Those callers are evidence of an existing physical route, not permission to
connect the strict writer. The current Loop physicalizer dispatcher and
Generic G0 physical-emitter session are test-only/caller-zero surfaces, so a
production caller must be named from an actual active route before CONNECT0
can become an implementation row.

## Authority and handoff table

| Handoff fact | Existing owner | Required connection proof |
| --- | --- | --- |
| Compare operation, item, result key, and schedule order | verified Recipe/operation row | row identity and result key are passed once; no reclassification |
| logical target to physical block | `VerifiedLoopOperationTargetBlockV1` plus canonical CFG session | exact owner/item/role/session target; no Builder cursor lookup |
| lhs/rhs source values | full `LoopOperationValueReceiptV1` | Published, owner-bound, exact target, then C-prime same-block Integer witnesses |
| Compare destination | `CanonicalSsaFunctionSessionV2` | fresh wrapped destination capability; no raw allocator in dispatcher |
| Bool result fact | prepared Compare type plan | all type conflict checks before append; commit after the writer definition |
| result publication | owner-bound `LoopOperationValueLedgerV1` | reserve before append and commit only from the writer-owned definition source |
| physical mutation | strict writer/shared append core | exactly one append; no legacy retry or post-append fallible check |

The dispatcher may sequence these existing authorities, but it may not become
a second CFG/SSA/ledger/source authority. In particular, `state.get()` and the
old `emit_compare_i64_at` result path cannot be the canonical handoff contract.

## Finite design states

| State | Meaning | Effect | Next |
| --- | --- | ---: | --- |
| `CallerUnselected` | no named non-test production edge is proven | none | census only |
| `CallerSelected` | one active route and exact Compare row are named | none | handoff design |
| `HandoffPrepared` | target, full receipts, destination, Bool plan, and result reservation are co-sealed | none | strict writer |
| `Committed` | writer definition and ledger publication completed | one Compare | caller-specific postcondition |
| `RejectedBeforeEffect` | typed relation or preparation failure | none | outer unpublished discard |
| `NoSafeSlice` | a required fact comes only from old leaf/cursor/test evidence | none | return to design |

`CallerUnselected` is not a runtime disposition and must not be converted to
`NonCandidate`, `Declined`, or a legacy fallback. It is the current SSOT
development state.

## Required design evidence before P0

1. Count the named non-test caller and identify its active backend/route from
   `CURRENT_STATE.toml` and the active card; test-only canaries cannot satisfy
   this row.
2. Show the exact row handoff from the caller's verified operation/target to
   the C-prime operand witnesses without re-pairing by name, ordinal, or raw
   `ValueId`.
3. Show where the owner-bound ledger is created and prove that Const/Binary or
   read producers publish compatible full receipts before Compare reservation.
4. Fix the last fallible step as result-slot reservation, followed by strict
   append, Bool commit, and ledger commit only.
5. Prove rejected preparation leaves instruction count, type context, and
   ledger state unchanged and cannot call the old Compare leaf.
6. Record the production caller count, strict writer count, old canonical edge
   count, and fallback count in a reusable guard before implementation.

## NoSafeSlice

Keep this card at `design_stop` if the only available caller is a test canary,
if the active route needs cross-block/parameter operands outside C-prime, if
the owner-bound ledger cannot surround the whole schedule, if a legacy leaf is
needed after strict rejection, or if dispatcher connection requires a second
target/value/ledger authority. Do not solve those gaps with an adapter from
the Builder cursor or with a default/empty receipt.
