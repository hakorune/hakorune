Status: parked design/task card; current pointer remains SCRIPT-STATIC-IMPORT-TARGET-AUTHORITY-D0
Task: MIR-LOOP-COMPARE-TRANSACTION-HARDENING-D0
Date: 2026-08-22
Priority: harden the selected Dynamic I9 transaction boundary before live publication
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
CurrentCard: docs/development/current/main/investigations/mirbuilder-static-import-target-authority-d0-2026-08-22.md
NextCard: MIR-DYNAMIC-CURSOR-EOF-FAILFAST-P0 (after the current import D0 is accepted)
---

# Selected Dynamic Compare hardening D0

## Six-line brief

Decision: accept the feedback as four ordered hardening cells. Fix the cursor EOF panic first; then design one pre-effect claim boundary and one prepare -> reserve -> commit transaction for I9. Bridge equality and affine type cleanup remain separate small cells; the caller-zero generic leaf stays parked.
Source authority + canonical issuer: the verified Dynamic operation/cleanup rows own claim order; CanonicalSsaFunctionSessionV2 owns destination/type facts; CanonicalLoopCompareI64WriterV1 owns the single physical append; DynamicV2PhysicalValueLedgerV1 owns V13 publication. A private I9 commit aggregate may co-seal these existing products but must not issue new source meaning.
Non-authority: append-time census claims, raw ValueId equality, post-append type/ledger checks, assert-based pairing, the generic caller-zero Loop ledger, AST/name/ordinal lookup, and fallback/retry.
Fail-fast boundary: all claim validation, writer preparation, bridge relation checks, and result-slot availability must finish before the first I9 Compare/Branch/CallOut effect. After the final reservation, only move-only infallible commits may remain; any rejected preflight discards the unpublished session.
Smallest next slice: MIR-DYNAMIC-CURSOR-EOF-FAILFAST-P0, a behavior-narrow typed-reject fix plus an extra-claim negative test. It does not open live publication or the broader preclaim design.
Non-claims: no imported target authority, no DraftAdmission/ModuleDrain/ExternalCommit proof, no generic Loop activation/retirement, no cross-block dominance, no backend, and no performance work.

## Audit result

The current HEAD confirms the following rows.

| Finding | Classification | Evidence | Action |
| --- | --- | --- | --- |
| claim_operation indexes operation_order[next_operation] after get() returned None | correctness bug | selected_dynamic_physical_emitter/operation_cursor.rs::claim_operation | P0 now |
| I8/I9 and claim_if run after strict Compare, ledger commit, and Branch | transaction hardening | selected_dynamic_physical_emitter/i8_i9_control.rs | preclaim D0/I0 |
| Backedge cleanup and I13..I16 claims run after physical work | same hardening family | continuation_backedge.rs | include in preclaim scope, do not mix authorities |
| V13 reserve_result precedes CanonicalLoopCompareI64WriterV1::emit, whose preparation can reject | ordering defect | i8_i9_control.rs; canonical_compare_writer.rs | prepare/reserve split |
| PendingDynamicV2PhysicalValuePublishV1::commit uses three assert_eq! checks | panic-capable pairing gap | selected_dynamic_physical_emitter/value_ledger.rs::commit | private co-sealed commit aggregate |
| destination and Bool plan derive Clone/Copy or Clone | affinity hardening | canonical_ssa/session/destination.rs; emission/compare_type.rs | remove in a separate P2 cell |
| OuterReturn checks owner/binding/block but not physical value equality with Header-current | bridge completeness gap | selected_dynamic_physical_emitter/body_state_bridge.rs | add narrow relation check |
| loop_recipe_physicalizer/compare_i64_writer.rs is a test-only adapter but is compiled non-test to re-export the production writer | build-scope/naming mismatch, not a correctness blocker | loop_recipe_physicalizer/mod.rs, resolved_lowering/mod.rs | parked cleanup; never cfg-out the production writer |
| pure_operation_emitter.rs retains old post-append checks | caller-zero baseline debt | generic Loop module header and caller census | preserve until activation/retirement Decision |

Independent baseline evidence on this HEAD:

    bash tools/checks/rust_mirbuilder_loop_compare_connect0_guard.sh
      -> green: one selected I9 writer caller, Dynamic-only V13 commit, no legacy fallback

    CARGO_BUILD_JOBS=4 cargo test --lib selected_dynamic_physical_emitter -- --nocapture
      -> 9 passed; existing compiler warnings are baseline and unrelated to this card

No source or fixture change is authorized by this audit card while the current
SSOT remains design_stop.

Worker audit confirmation:

    Hilbert (read-only) agrees that the EOF path is the only immediate
    correctness bug in this feedback set. The worker also identified that the
    current I9 path has a fallible Branch emission after the Compare/result
    commit. Therefore reserve-last includes Branch preparation, not only the
    Compare writer preparation. P3 items and the generic caller-zero leaf stay
    parked and are not merged into the selected Dynamic authority.

## Authority boundary

    verified Dynamic operation/cleanup rows
      -> pre-effect physical claim preparation
      -> canonical same-block operand + destination + Bool preparation
      -> strict writer preparation
      -> last fallible V13 result reservation
      -> one private I9 commit aggregate
      -> one Compare append, Bool publication, V13 publication, Branch

The aggregate is only a co-seal of existing receipts. It must not:

    re-resolve a target
    re-read AST or Builder variable names
    allocate a second ValueId
    publish into the generic Loop ledger
    turn a failed claim into Absent/noncandidate
    call the legacy Compare emitter

## Ordered task sequence

### 1. MIR-DYNAMIC-CURSOR-EOF-FAILFAST-P0

Smallest executable cell, after the current import D0 is accepted.

Change only the cursor boundary:

    let Some(expected) = self.operation_order.get(self.next_operation).copied() else {
        return Err(DynamicV2RecipeOperationCursorRejectV1::DuplicateItem);
    };
    if expected != item {
        return Err(DynamicV2RecipeOperationCursorRejectV1::PhysicalOrder {
            expected,
            actual: item,
        });
    }

Acceptance:

    extra claim after complete operation order -> DuplicateItem, never panic
    wrong item before end -> PhysicalOrder with expected/actual
    claim_if/claim_exit exhausted -> existing typed DuplicateItem
    focused cursor tests pass
    CONNECT0 guard, pointer guard, and source-size check pass

Non-claims: no batch claims, no writer change, no ledger change, no live
publication.

### 2. MIR-DYNAMIC-PHYSICAL-PRECLAIM-D0

Design before implementation. The selected Dynamic physical leaves must not
discover operation meaning after an effect has already been appended.

The D0 must decide one of these bounded forms and record why:

    preflight-and-consume:
      validate the exact claim set, consume it before effects, and make every
      claim failure terminal to the unpublished outer session

    prepare-and-commit:
      reserve a private claim batch before effects and commit it after effects
      without another fallible check; dropping it poisons the session

The first named cohort is:

    I8 + I9 + If claim before I9 Compare/Branch
    I13..I16 + Backedge cleanup claim before backedge physical effects

The census must also record every selected Dynamic CallOut, InnerReturn, Fault,
and Exit claim site. They are not silently declared safe because they are
outside the first cohort; each remains either in the preclaim cohort or an
explicit later cell.

The D0 must census all remaining selected Dynamic leaves before widening the
cohort to InnerReturn, Fault, or Exit. Operation census and cleanup cursor stay
separate owners even when their preclaim timing is unified.

Acceptance:

    no claim_operation/claim_if/cleanup.claim occurs after the corresponding
      Compare, Branch, CallOut, Const, Add, assignment, or Return effect
    claim failure leaves MIR/type/ledger unchanged before outer discard
    no fallback/retry catches a claim rejection
    one claim owner and one terminal state are named

NoSafeSlice: preclaim requires a second order authority, allows continuation
after a failed reservation, or cannot prove the outer session is discarded.

### 3. MIR-LOOP-COMPARE-PREPARE-RESERVE-I0

Implement only after the preclaim D0 is accepted. Split the current writer
front door into:

    CanonicalLoopCompareI64WriterV1::prepare(...)
      -> PreparedCanonicalCompareAppendV1 / Bool plan

    preclaim batch
      -> prepare Branch and every other downstream physical effect
      -> last fallible Dynamic V13 reserve_result

    private PreparedSelectedDynamicI9CompareV1::commit(self)
      -> strict append
      -> Bool commit
      -> Dynamic V13 commit
      -> Branch

CanonicalLoopCompareI64WriterV1::emit must not remain the selected I9
production entry after this cell. prepare_canonical_compare_append remains
the writer preparation authority; it must run before reserve_result.

Branch emission is part of the same transaction. Any fallible target/current
block/operand preparation required by Branch must complete before
reserve_result. After reservation, Branch may only be a private infallible
commit consuming the prepared Compare definition/result relation. A sequence
of Compare append -> V13 commit -> fallible emit_branch is not accepted.

The private aggregate owns the prepared writer and the exact pending V13 slot
together. The pending ledger commit must no longer accept an arbitrary
CanonicalCompareDefinitionSourceV1 from the caller or use assert_eq! to
pair unrelated products. Its infallible commit is reachable only through the
aggregate that produced the definition and reserved the slot.

Acceptance:

    writer preparation rejection -> no V13 reservation and no MIR append
    V13 reservation is the last fallible operation
    after reservation, no Result-returning validation remains
    no post-append type or ledger lookup remains on the strict path
    no pending-result assert_eq! pairing remains
    one Compare append, one Bool fact, one V13 publication, one Branch

NoSafeSlice: the aggregate cannot make definition/slot pairing private and
move-only, or the writer still needs a repair-capable legacy front door.

### 4. MIR-LOOP-BODY-BRIDGE-RETURN-AFFINITY-P0

Keep this separate from the physical transaction rewrite.

Required narrow checks:

    OuterReturn owner/binding/block must match Header-current
    OuterReturn physical_value must equal header_current.physical_value()
    ReservedCanonicalCompareDestinationV1 is Debug-only and move-only
    PreparedCanonicalCompareBoolTypeV1 is Debug-only and move-only

The bridge check is a relation check, not a new semantic source. Add a
negative test that changes only the physical value and rejects before any
publication. Type affinity must be verified by compile/guard evidence rather
than runtime behavior.

### 5. Parked cleanup: MIR-LOOP-GENERIC-COMPARE-RETIRE-D0

Do not touch the generic caller-zero leaf in the selected Dynamic hardening
series. Its old append-then-publish behavior remains a known baseline debt
until a separate generic activation or retirement Decision names its owner,
caller, and replacement. Do not merge the Dynamic ledger with the generic
Loop ledger to make this debt look solved.

The compare_i64_writer.rs adapter cleanup may be a behavior-neutral later
BoxShape:

    keep canonical_compare_writer.rs in production
    move the test-only trait adapter/re-export behind cfg(test), or rename it to
      an explicitly migration-scoped adapter after caller census

This cleanup is not a prerequisite for the selected I9 transaction boundary.

## Finite state table

| State | Owner | Effect | Allowed next |
| --- | --- | ---: | --- |
| Unprepared | verified Dynamic row/cursor | none | claim/writer preparation |
| ClaimsPrepared | operation/cleanup cursor owner | none | Compare/Branch preparation or reject |
| EffectsPrepared | strict writer + Branch preparation | none | V13 reservation or reject |
| ResultReserved | Dynamic value ledger | none | private commit only |
| Committed | private I9 aggregate | one bounded physical sequence | terminal |
| RejectedBeforeEffect | current phase owner | none | outer unpublished discard |
| Poisoned | pending claim/result token | no continuation | outer unpublished discard |

ResultReserved is never returned as a free capability. It is consumed by the
private aggregate immediately, just like the strict writer plan.

## Required guards and evidence

    claim_operation EOF branch is typed and tested
    no selected I9 claim call follows a physical effect
    writer prepare caller count = 1; old emit caller count = 0 after I0
    V13 reserve is after every fallible writer/preclaim step
    pending V13 commit has no assert_eq! pairing
    destination and Bool plan have no Clone/Copy derives
    OuterReturn/Header-current physical equality is checked
    selected Dynamic has one writer/ledger route and zero fallback
    generic old leaf remains explicitly caller-zero if untouched
    all touched production files < 760 lines; 800 is a hard stop

## NoSafeSlice for live publication

Keep live publication closed if any of these remains:

    import target authority is not accepted by the current D0
    claim failure can be caught and route execution can continue
    writer preparation can fail after V13 reservation
    pending definition/ledger pairing still relies on assert or arbitrary input
    OuterReturn relation is only owner/name/block-based
    strict I9 can reach generic emit/fallback/retry
    DraftAdmission/ModuleDrain/ExternalCommit evidence is still missing

This card does not change the current pointer. The active task remains
SCRIPT-STATIC-IMPORT-TARGET-AUTHORITY-D0; this hardening card is the ordered
downstream prerequisite to revisit before live publication is claimed.
