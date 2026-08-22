Status: Backedge cleanup preclaim I0 landed; InnerReturn preclaim D0 is the next design stop
Task: MIR-LOOP-COMPARE-TRANSACTION-HARDENING-D0
Current execution row: MIR-DYNAMIC-INNER-RETURN-PRECLAIM-D0
Date: 2026-08-22
Priority: harden the selected Dynamic I9 transaction boundary before live publication
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
CurrentCard: docs/development/current/main/investigations/mirbuilder-loop-compare-hardening-d0-2026-08-22.md
NextCard: MIR-DYNAMIC-INNER-RETURN-PRECLAIM-D0 (after Backedge preclaim I0)
---

# Selected Dynamic Compare hardening D0

## Six-line brief

Decision: accept the feedback as ordered hardening cells. The cursor EOF P0, I8/I9/If preclaim I0, CallOut I0..I7 preclaim I0, and Fault I6/I7 cleanup preclaim I0 are closed. Use preflight-and-consume for the next Backedge cleanup/I13..I16 claims at their pre-effect boundary, without a new batch receipt or second cleanup authority; later design one prepare -> reserve -> commit transaction for I9. Bridge equality and affine type cleanup remain separate small cells; the caller-zero generic leaf stays parked.
Source authority + canonical issuer: the verified Dynamic operation/cleanup rows own claim order; CanonicalSsaFunctionSessionV2 owns destination/type facts; CanonicalLoopCompareI64WriterV1 owns the single physical append; DynamicV2PhysicalValueLedgerV1 owns V13 publication. A private I9 commit aggregate may co-seal these existing products but must not issue new source meaning.
Non-authority: append-time census claims, raw ValueId equality, post-append type/ledger checks, assert-based pairing, the generic caller-zero Loop ledger, AST/name/ordinal lookup, and fallback/retry.
Fail-fast boundary: after the existing pure InnerReturn row/site, ThenTerminal brand/predecessor, and required relation validation, consume the existing InnerReturn cleanup and I11/Exit claims before `select_block()` or any MIR/SSA/ledger effect. A claim failure is terminal and the unpublished outer session is discarded; no rollback, retry, or fallback exists. Compare writer and publication remain separate cells.
Smallest next slice: MIR-DYNAMIC-INNER-RETURN-PRECLAIM-D0, a design-only census of the existing InnerReturn cleanup and I11/Exit claims before their first physical effect. It does not open the Compare transaction, live publication, or generic Loop authority.
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

The Outside terminal closeout is now complete. This card is the next accepted
bounded lane; only the cursor EOF P0 below is active, and the broader preclaim
and live-publication cells remain closed.

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

### Cursor EOF P0 closeout evidence

`claim_operation()` now returns typed `DuplicateItem` when the operation
order is exhausted; it no longer indexes past the end of `operation_order`.
The focused cursor suite is green (2 passed), including an explicit
`catch_unwind` negative proof that an extra claim does not panic. The selected
Dynamic CONNECT0 guard and strict-writer guard remain green, the cursor source
is 538 lines, the pointer guard is green, and `git diff --check` is clean.

This cell changed no writer, ledger, claim timing, publication, fallback, or
ordinary/generic Loop route. The next accepted work is
`MIR-DYNAMIC-PHYSICAL-PRECLAIM-I0`.

### 2. MIR-DYNAMIC-PHYSICAL-PRECLAIM-I0

Decision accepted: use `preflight-and-consume` for the first named cohort.
After the existing pure validation in `i8_i9_control.rs` and before the first
`issue_physical_value_id`/`i64_const::emit_with_dst`, consume exactly:

    claim_operation(I8)
    claim_operation(I9)
    claim_if()

The existing `DynamicV2PhysicalOperationCensusV1` remains the sole claim
owner. Do not add a claim batch, a second order authority, rollback, or a new
semantic receipt. If any claim fails, return the existing typed emitter reject;
the caller's `reject_begin()` discards the unpublished outer session.

The selected Dynamic physical leaves must not discover I8/I9/If operation
meaning after an effect has already been appended. The exact boundary is after
the current row/corridor/brand validation and before destination ValueId
issuance. Claims are consumed once and are never reused after a later physical
failure because the whole unpublished session is terminal.

The first named cohort is:

    I8 + I9 + If claim before I9 Compare/Branch

The remaining claim sites are explicitly outside this I0 and remain separate
follow-on cells:

    I0..I7 CallOut operation claims
    I6/I7 Fault cleanup claims
    Backedge cleanup plus I13..I16 operation claims
    InnerReturn cleanup plus I11/Exit/Return claims

They are not silently declared safe. Operation census and cleanup cursor remain
separate owners even when a later cell unifies their pre-effect timing.

Acceptance:

    I8/I9/If claims occur before I8 Const and before any I9 ValueId/Compare/Branch effect
    I8/I9/If claim failure produces typed reject with no cohort effect
    normal selected Dynamic fixture reaches each of the three claims once
    no duplicate I8/I9/If claim edge remains after the physical effects
    no fallback/retry catches a claim rejection
    one existing claim owner and one terminal discard path are named

I0 closeout evidence: the three claims now occur once, before the first I8
`issue_physical_value_id`/Const effect; the old post-Compare/Branch claim edge
is gone. The selected Dynamic focused suite is 10/10, CONNECT0 and strict-
writer guards are green, the pointer guard and diff check are green, and
`i8_i9_control.rs` is 298 lines. The line-order guard is the structural
effect-zero proof: a claim rejection is encountered before any cohort ValueId
or instruction mutation, and the existing `reject_begin()` remains the only
terminal discard path.

NoSafeSlice: I8/I9/If claims cannot be placed before the first cohort effect,
preflight requires a second order authority, claim failure can continue into a
fallback/retry, or the unpublished outer session discard cannot be proven.

Worker audit evidence: Huygens independently confirmed the exhaustive claim /
effect census, selected `preflight-and-consume` for I8/I9/If, and kept
Backedge/Fault/InnerReturn outside this I0. The audit found no need for a new
receipt or second authority.

The next design stop after this completed I0 is
`MIR-DYNAMIC-FAULT-CLEANUP-PRECLAIM-D0`, covering only the I6/I7 cleanup
claims; Backedge, InnerReturn, and operation claims remain separate.

### 3. MIR-DYNAMIC-CALLOUT-PRECLAIM-D0

Decision accepted: the same existing
`DynamicV2PhysicalOperationCensusV1` can preflight-and-consume the exact
ordered I0..I7 claims after row/site/brand validation and identity observations
but before the first `loop_operation::publish_i64_value` MIR/ledger effect.
This is a pre-MIR/ledger physical-effect boundary, not a claim that the whole
session is mutation-free: identity observations already occur before it and
remain covered by the existing unpublished outer discard.

Acceptance:

    every I0..I7 claim site and preceding physical effect is recorded
    one bounded pre-effect boundary or an explicit NoSafeSlice is named
    operation census and cleanup cursor remain separate owners
    outer unpublished discard and no-fallback behavior are proven
    a focused positive/negative test and reusable line-order/effect-zero guard
      can be specified without touching Backedge/Fault/InnerReturn

NoSafeSlice: identity observations cannot be safely discarded with the
unpublished outer session, CallOut claims need a second source/order authority,
the pre-MIR/ledger boundary cannot be placed before the first physical effect,
or preclaiming them would require changing the selected Dynamic route or
generic Loop authority.

Worker audit evidence: Dirac independently confirmed that the first physical
effect is `loop_operation::publish_i64_value` and that moving the existing
I0..I7 loop immediately before it is safe under the current `reject_begin()`
discard contract. The audit explicitly kept identity observations, Fault,
Backedge, and InnerReturn outside the I0 boundary.

### 4. MIR-DYNAMIC-CALLOUT-PRECLAIM-I0

Implement only the accepted boundary above:

    existing row/site/brand validation
    existing identity observations
    claim_operation(I0..I7) exactly once in verified order
    loop_operation::publish_i64_value and the existing CallOut physical lane

Remove the old end-of-corridor claim loop. Do not add a claim batch, reorder
authority, rollback, fallback, or any cleanup claim. The focused selected
Dynamic fixture must still close its existing operation census exactly once.

Acceptance:

    I0..I7 claim loop occurs once before the first publish_i64_value effect
    positive selected Dynamic fixture reaches all eight claims and closes
    claim/effect line-order guard is green and proves no post-effect claim edge
    typed cursor wrong-order/duplicate/exhausted behavior remains unchanged
    no Builder publication, generic Loop route, or fallback changes
    touched production files remain below 760 lines

I0 closeout evidence: the I0..I7 loop now occurs once immediately before
`loop_operation::publish_i64_value`; the old end-of-corridor loop is gone. The
selected Dynamic focused suite is 10/10, the dedicated CallOut preclaim guard,
CONNECT0 guard, strict-writer guard, and pointer guard are green, the CallOut
owner remains below 760 lines, and `git diff --check` is clean.

NoSafeSlice: a claim failure can bypass `reject_begin()`, the old claim loop
cannot be removed completely, or the boundary requires a second authority.

The next design stop is `MIR-DYNAMIC-FAULT-CLEANUP-PRECLAIM-D0`. It covers only
the I6/I7 cleanup claims and must not mix operation claims, Backedge, or
InnerReturn cleanup.

### 5. MIR-DYNAMIC-FAULT-CLEANUP-PRECLAIM-D0

Decision accepted after the Laplace read-only audit: the existing
`DynamicV2PhysicalCleanupCursorV1` can preflight-and-consume both I6 and I7
cleanup rows after the current corridor/brand/site-pair validation and before
the first physical Fault/CallOut-End effect. The cursor remains the sole
cleanup authority; no cleanup batch, rollback, or operation-census authority
is introduced. The caller's existing `reject_begin()` still discards the
unpublished outer session on every later failure.

The exact bounded state is:

    Issued
      -> CorridorValidated
      -> I6Claimed
      -> I7Claimed
      -> FaultEffects
      -> terminal unpublished session

The accepted implementation task is
`MIR-DYNAMIC-FAULT-CLEANUP-PRECLAIM-I0`:

    corridor.matches(brand)
    site_pair_matches(i6, i7)
      -> claim(I6Fault)
      -> claim(I7Fault)
      -> existing Fault / CallOut-End emission

Remove the old post-effect claims. Do not touch Backedge, InnerReturn,
operation claims, publication, or the Compare writer transaction.

Acceptance:

    I6/I7 cleanup claims occur exactly once before the first
      `emit_fault_terminal` and `emit_checked_callout_end` effect
    duplicate/missing cleanup rows remain typed rejects
    foreign brand/site mismatch rejects before either cleanup claim
    later physical failure still reaches `reject_begin()`/discard
    the selected Dynamic fixture reaches `cleanup.close()` exactly once
    a reusable line-order/effect-zero guard is green
    `fault_terminals.rs` remains below 760 lines

NoSafeSlice: a cleanup claim can bypass `reject_begin()`, needs rollback or a
second authority, or the physical Fault/End effect must precede the claim.

Worker evidence: Laplace confirmed the current I6 claim follows
`emit_checked_callout_fault` and the current I7 claim follows
`emit_checked_callout_end` plus Fault. The current corridor checks provide the
preclaim identity boundary, and `reject_begin()` is the only terminal discard
path. NoSafeSlice was not found.

### 6. MIR-DYNAMIC-FAULT-CLEANUP-PRECLAIM-I0

Implementation complete. The two existing cleanup claims now occur exactly
once after corridor/brand/site-pair validation and before the first
`emit_fault_terminal` or `emit_checked_callout_end` effect. The old post-effect
claims were removed; later physical failure still returns through
`reject_begin()` and discards the unpublished session. No cleanup batch,
rollback, operation claim, Backedge, InnerReturn, publication, or writer
authority was added.

The reusable guard
`rust_mirbuilder_dynamic_fault_cleanup_preclaim_i0_guard.sh` proves the two
claim sites, line order, source-size boundary, and card/index registration.
The selected Dynamic focused suite is 10/10, the Fault, CallOut, CONNECT0,
strict-writer, pointer, and diff guards are green, and `fault_terminals.rs` is
109 lines.

NoSafeSlice was not triggered: both claims use the existing cursor, and every
later rejection reaches the existing unpublished-session discard path.

The next design stop is `MIR-DYNAMIC-BACKEDGE-PRECLAIM-D0`. It must audit only
the Backedge cleanup row and I13..I16 operation claims; InnerReturn and the
Compare writer transaction remain separate.

### 7. MIR-DYNAMIC-BACKEDGE-PRECLAIM-D0

Decision accepted after the Sagan read-only audit: the existing cleanup cursor
and operation census can be preflight-and-consumed before the hidden
`select_block()` mutation and before the first V15 ledger/MIR effect. The
existing lifecycle plan owns the Backedge cleanup row and the existing ordered
operation census owns I13..I16; they remain separate sole authorities. No batch,
receipt, rollback, or second authority is introduced.

The exact bounded state is:

    row/cutpoint validation
      + Continuation brand/predecessor validation
      + Header brand validation
      -> claim(Backedge cleanup)
      -> claim_operation(I13..I16)
      -> select_block()
      -> identity claim
      -> V15/ValueId/Const/Add/assignment/End
      -> Jump and CFG/PHI seal

The accepted implementation task is
`MIR-DYNAMIC-BACKEDGE-PRECLAIM-I0`:

    move Header brand validation before all claims
    consume cleanup.claim(Backedge) exactly once
    consume claim_operation(I13), I14, I15, I16 exactly once
    remove the old post-effect claim sites

Acceptance:

    every Backedge/operation claim occurs before `select_block()`, the first
      `values.publish()` effect, ValueId issuance, or MIR append
    duplicate/missing cleanup and operation-order drift remain typed rejects
    foreign Continuation/Header rejects before any claim
    later physical failure still reaches `reject_begin()`/discard
    the normal fixture closes both the cleanup cursor and operation census
    a reusable line-order/effect-zero guard is green
    `continuation_backedge.rs` remains below 760 lines

NoSafeSlice: identity/CFG validation cannot be completed before claims,
claims cannot be safely discarded through the unpublished session, a second
cleanup/census authority is needed, or a claim failure can continue to another
route.

Worker evidence: Sagan found `select_block()` mutates current-block and SSA
cache state, identity claim marks source coverage, and `values.publish()` is
the first ledger effect. The existing caller routes all later errors through
`reject_begin()`/`discard_unpublished()`. NoSafeSlice was not found.

### 8. MIR-DYNAMIC-BACKEDGE-PRECLAIM-I0

Implementation complete. Header brand validation now occurs before the existing
Backedge cleanup and I13..I16 operation claims. Those claims occur exactly once
before `select_block()`, which mutates the Builder session, and before the first
V15 ledger/ValueId/MIR effect. The old post-effect claims were removed; later
physical failure still returns through `reject_begin()` and discards the
unpublished session. Cleanup cursor and operation census remain separate
authorities.

The reusable guard
`rust_mirbuilder_dynamic_backedge_preclaim_i0_guard.sh` proves one Backedge
cleanup claim, one I13..I16 batch, Header validation before claims, line order,
source-size boundary, and card/index registration. The selected Dynamic
focused suite is 10/10, Fault/CallOut/Backedge/CONNECT0/strict-writer/pointer
guards and diff checks are green, and `continuation_backedge.rs` is 351 lines.

NoSafeSlice was not triggered: claims use the existing cursor/census and every
later rejection reaches the existing unpublished-session discard path.

The next design stop is `MIR-DYNAMIC-INNER-RETURN-PRECLAIM-D0`. It must audit
only the InnerReturn cleanup and I11/Exit claims; Compare prepare/reserve/commit
and publication remain separate.

### 9. MIR-DYNAMIC-INNER-RETURN-PRECLAIM-D0

Design-only next cell. Audit `inner_return_then.rs`, recording the exact order
of InnerReturn site/row validation, ThenTerminal brand/predecessor checks,
`select_block()`/identity reads, first End/SSA/ledger effect, the InnerReturn
cleanup claim, I11 operation claim, and Exit claim. Decide whether the existing
cleanup cursor and operation census can be preflight-and-consumed before the
first physical effect while keeping both authorities separate. Do not
implement, add a batch, or open Compare/publication.

Acceptance:

    every InnerReturn cleanup/operation claim and preceding effect is recorded
    one pre-effect boundary or explicit NoSafeSlice is named
    `reject_begin()` discard and no-fallback behavior are proven
    a focused positive/negative test and reusable guard can be specified
      without changing Fault, Backedge, Compare, or publication authority

NoSafeSlice: site/CFG/SSA validation cannot be completed before claims, claim
failure cannot be discarded through the unpublished session, a second
cleanup/census authority is needed, or a required physical effect must precede
the claim.

### 10. MIR-LOOP-COMPARE-PREPARE-RESERVE-I0

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

### 11. MIR-LOOP-BODY-BRIDGE-RETURN-AFFINITY-P0

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

### 12. Parked cleanup: MIR-LOOP-GENERIC-COMPARE-RETIRE-D0

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

Cursor EOF P0 and the I8/I9/If preclaim I0 are closed. Enter the CallOut
preclaim D0 below before opening prepare/reserve/commit or live publication.
The generic caller-zero leaf and all unrelated import/publication work remain
parked.
