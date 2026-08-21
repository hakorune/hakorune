Status: selected production caller; handoff design stop; no implementation authorized
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
Decision: select the active Selected Dynamic I9 normal-landing Compare as the one named non-test production caller candidate; keep CONNECT0 at design_stop until its Dynamic-to-canonical handoff is co-sealed and atomic.
Source authority + canonical issuer: `DynamicV2CompareI64CapabilityDemandV1`/the prepared I9 row, the session-owned Dynamic target/value ledger, and the canonical CFG/SSA sessions must be co-sealed by one private `SelectedDynamicI9CompareHandoffIssuerV1` before any Compare effect.
Non-authority: the test-only Generic G0 dispatcher, focused canaries, old emit_compare_i64_at, Dynamic brand alone, Dynamic value views alone, raw ValueId/state.get, current_block, Builder cursor, operation enum alone, and a fallback retry.
Fail-fast boundary: I9 row/target/operand provenance, canonical same-block witnesses, owner-bound result reservation, and the Dynamic ledger's result publication reservation must all be prepared before the first Compare append; a rejected strict row cannot return to the old leaf.
Smallest next slice: design-only I9 handoff table covering target issuance, full operand receipts, both result ledgers, and one atomic commit; no code, fixture, fallback, or production switch until that table is accepted.
Non-claims: no I7 header Compare, no generic dispatcher connection, no general dominance, no cross-block operands, no Const/Binary migration, no A/C/Recipe redesign, no old-leaf retirement, and no production I0/R0.
```

## Current census

The strict writer P0 remains intentionally caller-zero:

```text
CanonicalLoopCompareI64WriterV1::emit production callers = 0
CanonicalLoopCompareI64WriterV1::emit focused test callers = 3
emit_loop_segment_operation_dispatch_v1 non-test callers = 0
emit_prepared_pure_operation_v1 non-test callers = 0
```

The old `emit_compare_i64_at` remains a shared legacy leaf with callers in
`pure_operation_emitter.rs` and other canonical canary/compatibility areas.
Those callers are evidence of existing physical routes, not permission to
connect the strict writer. The generic Loop segment dispatcher is still
caller-zero outside `#[cfg(test)]`; it is not the production caller for this
row.

## Named production caller decision

The active production edge is:

```text
normal_callable_semantic_loan_port.rs:lower_cataloged_static_box_method
  -> assemble_unpublished_selected_dynamic_w6
  -> callout_corridor::emit
  -> i8_i9_control::emit
  -> I9 normal-landing Compare
```

The exact I9 row is co-checked against `I9`, `V11`, `V12`, and `V13` by
`DynamicV2CompareI64CapabilityDemandV1`; the physical normal landing is created
by the canonical CFG session, `V11` is the I7 normal-result definition, and
`V12` is emitted immediately before the Compare in that same landing. This is
enough to select I9 as the named caller candidate, but it is not yet a
canonical same-block proof accepted by the strict writer.

The I7 header Compare is explicitly excluded: its current/formal operands
cross the formal/header relation and are outside the C-prime same-block slice.

The required handoff is still missing. The active route has a
`DynamicV2PhysicalValueLedgerV1` and post-append `values.publish(I9, V13, ...)`,
while the strict writer requires a canonical open-target witness, full
`LoopOperationValueReceiptV1` operands, an owner-bound
`LoopOperationValueLedgerV1`, and an infallible writer-to-ledger commit. A
simple adapter or a second generic dispatcher would create a competing
authority. The Dynamic result publication is also fallible after the current
legacy append, so it needs its own prepared reservation/commit in the same
atomic handoff.

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
| `CallerSelected` | Selected Dynamic I9 normal-landing route and exact Compare row are named | none | handoff design |
| `HandoffUnresolved` | I9 is named, but Dynamic target/value and canonical/owner-bound receipts are not co-sealed | none | design only |
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

Keep this card at `design_stop` if I9 cannot receive a canonical open-target
witness, if either Dynamic operand lacks a unique same-block definition, if
the Dynamic result slot cannot be reserved before append, if the owner-bound
ledger cannot be the one handoff publication owner, if the active route needs
cross-block/parameter operands outside C-prime, if a legacy leaf is needed
after strict rejection, or if connection requires a second
target/value/ledger authority. The Generic dispatcher remains caller-zero and
cannot be promoted by test evidence. Do not solve these gaps with an adapter
from the Builder cursor, a default/empty receipt, or a post-append repair.
