Status: selected production caller; handoff design stop; no implementation authorized
Task: MIR-LOOP-COMPARE-CONNECT0-D0
Date: 2026-08-22
Priority: caller census and atomic handoff before production connection
Parent: MIR-LOOP-COMPARE-STRICT-WRITER-P0
PreviousCard: MIR-LOOP-COMPARE-STRICT-WRITER-P0
NextCard: MIR-LOOP-COMPARE-I9-HANDOFF-PREPARE-D0 (same rolling card)
---

# Loop Compare CONNECT0 design stop

## Six-line brief

```text
Decision: select the active Selected Dynamic I9 normal-landing Compare as the one named non-test production caller candidate; keep CONNECT0 at design_stop until its Dynamic-to-canonical handoff is co-sealed and atomic. The handoff uses the Dynamic value ledger as the sole I9 publication ledger; it does not project into the Loop ledger.
Source authority + canonical issuer: `DynamicV2CompareI64CapabilityDemandV1`/the prepared I9 row and the session-owned `DynamicV2PhysicalValueLedgerV1` own I9 facts; canonical CFG/SSA owns target and definition witnesses; one private `SelectedDynamicI9CompareHandoffIssuerV1` co-seals them before any Compare effect.
Non-authority: the test-only Generic G0 dispatcher, focused canaries, old emit_compare_i64_at, Dynamic brand alone, Dynamic value views alone, raw ValueId/state.get, current_block, Builder cursor, operation enum alone, and a fallback retry.
Fail-fast boundary: I9 row/target/operand provenance, Dynamic-brand-to-function-owner binding, canonical same-block witnesses, destination/Bool preparation, and the Dynamic ledger's V13 reservation must all be complete before the first Compare append; a rejected strict row cannot return to the old leaf.
Smallest next slice: `MIR-LOOP-COMPARE-I9-HANDOFF-PREPARE-D0`, fixing owner binding, one-shot Dynamic-to-canonical operand rebind, Dynamic V13 reservation, and the one-append/infallible-commit suffix; no production switch or fallback.
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
while the strict writer requires a canonical open-target witness, canonical
same-block Integer operands, a fresh destination, and an infallible
writer-to-ledger commit. The writer does not require a Loop ledger. A generic
Dynamic-to-Loop adapter or a second dispatcher would create a competing
authority. The Dynamic result publication is also fallible after the current
legacy append, so it needs its own prepared reservation/commit in the same
atomic handoff.

## Authority and handoff table

| Handoff fact | Existing owner | Required connection proof |
| --- | --- | --- |
| Compare operation, item, result key, and schedule order | verified Recipe/operation row | row identity and result key are passed once; no reclassification |
| logical target to physical block | `DynamicV2PhysicalTargetSetV1` plus canonical CFG session | exact Dynamic brand/owner/normal landing; no Builder cursor lookup |
| lhs/rhs source values | `DynamicV2PhysicalValueLedgerV1` | exact V11/V12 producer/result/representation/block views are re-bound once by the private I9 issuer to canonical same-block Integer witnesses; no Loop-ledger projection |
| Compare destination | `CanonicalSsaFunctionSessionV2` | fresh wrapped destination capability; no raw allocator in dispatcher |
| Bool result fact | prepared Compare type plan | all type conflict checks before append; commit after the writer definition |
| result publication | `DynamicV2PhysicalValueLedgerV1` | reserve V13 before append and commit only from the writer-owned definition source; no second I9 publication ledger |
| physical mutation | strict writer/shared append core | exactly one append; no legacy retry or post-append fallible check |

The Loop operation ledger remains a separate generic caller-zero lane. It is not
an authority or transport requirement for this selected Dynamic row. Re-issuing
V11/V12 into it would duplicate publication state without adding a proof that
the strict writer needs.

## Worker audit and selected next task

The read-only worker confirmed the following decision:

```text
implementation state: NoSafeSlice
design shape: B, conditionally accepted
```

The selected shape is:

```text
Dynamic demand + Dynamic V11/V12 views
  -> private I9 handoff
  -> canonical owner/target and unique same-block Integer witnesses
  -> strict writer
  -> Dynamic V13 commit
```

The alternative of projecting V11/V12 into
`LoopOperationValueLedgerV1` and publishing V13 into both ledgers is rejected
for this row. It would create two publication authorities for one selected
physical value, while `CanonicalLoopCompareI64WriterV1` already accepts the
canonical operand witnesses directly and does not require a Loop ledger.

The next bounded design task is:

```text
MIR-LOOP-COMPARE-I9-HANDOFF-PREPARE-D0
```

It has exactly four deliverables:

1. Bind `DynamicV2PhysicalSessionBrandV1` to the same
   `FunctionOwnerIdV1` used to construct `CanonicalSsaFunctionSessionV2`.
2. Rebind exact Dynamic V11/V12 views once through
   `prepare_existing_same_block_integer`; do not add a generic Dynamic-to-Loop
   adapter or let `canonical_ssa` depend on the Dynamic ledger.
3. Add a private Dynamic V13 reservation/commit pair. Reservation is the last
   fallible step; commit consumes the strict writer's definition source and is
   infallible.
4. Specify the handoff states and focused acceptance before enabling the named
   production edge.

The required order is:

```text
I9 demand/row and Dynamic brand-owner check
-> exact Dynamic V11/V12 views
-> canonical same-block witnesses and open target
-> fresh destination and Bool plan
-> Dynamic V13 reservation
-> strict writer one-shot append
-> Bool commit
-> Dynamic V13 commit
```

If owner binding or Dynamic reservation cannot be made non-forgeable without
raw-value reconstruction, this task returns to `NoSafeSlice` rather than
introducing a second ledger or a post-append repair.

The dispatcher may sequence these existing authorities, but it may not become
a second CFG/SSA/ledger/source authority. In particular, `state.get()` and the
old `emit_compare_i64_at` result path cannot be the canonical handoff contract.

## Finite design states

| State | Meaning | Effect | Next |
| --- | --- | ---: | --- |
| `CallerSelected` | Selected Dynamic I9 normal-landing route and exact Compare row are named | none | handoff design |
| `HandoffUnresolved` | I9 is named, but Dynamic target/value and canonical/owner-bound receipts are not co-sealed | none | design only |
| `HandoffPrepared` | Dynamic owner/target, exact operand views, same-block witnesses, destination, Bool plan, and Dynamic V13 reservation are co-sealed | none | strict writer |
| `AppendedPendingCommit` | strict writer appended the one Compare and returned its definition source | one Compare | Bool/Dynamic commits only |
| `Committed` | writer definition, Bool fact, and Dynamic V13 publication completed | one Compare | caller-specific postcondition |
| `RejectedBeforeEffect` | typed relation or preparation failure | none | outer unpublished discard |
| `Poisoned` | a pending Dynamic reservation was dropped before commit | one attempted append at most | outer unpublished discard only |
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
3. Show the same-session owner binding for the Dynamic brand and the exact
   normal landing used by V11/V12.
4. Fix the last fallible step as Dynamic V13 reservation, followed by strict
   append, Bool commit, and Dynamic ledger commit only.
5. Prove rejected preparation leaves instruction count, type context, and
   Dynamic ledger state unchanged and cannot call the old Compare leaf.
6. Record the production caller count, strict writer count, old canonical edge
   count, and fallback count in a reusable guard before implementation.

## NoSafeSlice

Keep this card at `design_stop` if I9 cannot receive a canonical open-target
witness, if either Dynamic operand lacks a unique same-block definition, if the
Dynamic brand cannot be bound to the canonical function owner, if the Dynamic
result slot cannot be reserved before append, if the active route needs
cross-block/parameter operands outside C-prime, if a legacy leaf is needed
after strict rejection, or if connection requires a second target/value/ledger
authority. The Generic dispatcher remains caller-zero and cannot be promoted
by test evidence. Do not solve these gaps with a Builder-cursor adapter, a
default/empty receipt, a Loop-ledger re-publication, or a post-append repair.
