Status: Compare prepare/reserve/commit I0 complete; Bridge affinity P0 is next
Task: MIR-LOOP-COMPARE-TRANSACTION-HARDENING-D0
Current execution row: MIR-LOOP-COMPARE-PREPARE-RESERVE-I0
Date: 2026-08-22
Priority: harden the selected Dynamic I9 transaction boundary before live publication
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-BOUNDARY-D0
CurrentCard: docs/development/current/main/investigations/mirbuilder-loop-compare-hardening-d0-2026-08-22.md
NextCard: MIR-LOOP-BODY-BRIDGE-RETURN-AFFINITY-P0
---

# Selected Dynamic Compare hardening D0

## Six-line brief

Decision: the conditionally accepted Compare transaction I0 is implemented. The cursor EOF P0, I8/I9/If preclaim I0, CallOut I0..I7 preclaim I0, Fault I6/I7 cleanup preclaim I0, Backedge cleanup/I13..I16 preclaim I0, and InnerReturn cleanup/facts I0 are closed. I0 adds only a same-owner Branch prepare/commit seam, borrow-free Compare preparation, last-fallible V13 reservation, and one private move-only co-seal; bridge equality, publication, and the caller-zero generic leaf remain separate.
Source authority + canonical issuer: the verified Dynamic operation/cleanup rows own claim order; CanonicalSsaFunctionSessionV2 owns destination/type facts; CanonicalLoopCompareI64WriterV1 owns the single physical append; DynamicV2PhysicalValueLedgerV1 owns V13 publication. A private I9 commit aggregate may co-seal these existing products but must not issue new source meaning.
Non-authority: append-time census claims, raw ValueId equality, post-append type/ledger checks, assert-based pairing, the generic caller-zero Loop ledger, AST/name/ordinal lookup, and fallback/retry.
Fail-fast boundary: InnerReturn return facts now occur before End/V14. The selected-I9 transaction prepares all fallible claims, Compare, and Branch work before the last-fallible V13 reservation; only the private aggregate commits the prepared Compare, Bool, V13, and Branch products. No rollback, retry, or fallback exists.
Smallest next slice: MIR-LOOP-BODY-BRIDGE-RETURN-AFFINITY-P0, add only the narrow OuterReturn/Header-current relation check and compile/guard evidence for move-only destination/Bool plans. It does not open publication, generic Loop retirement, parser witness, or performance work.
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

Independent evidence on the I0 implementation:

    bash tools/checks/rust_mirbuilder_loop_compare_connect0_guard.sh
      -> green: one selected I9 writer caller, Dynamic-only V13 commit, no legacy fallback

    CARGO_BUILD_JOBS=4 cargo test --profile quick --lib selected_dynamic_physical_emitter
      -> 10 passed; existing compiler warnings are baseline and unrelated to this card

    CARGO_BUILD_JOBS=4 cargo test --profile quick --lib prepared_branch_has_no_effect_until_commit
      -> 1 passed; preparation leaves MIR/predecessor caches unchanged until commit

    CARGO_BUILD_JOBS=4 cargo check --profile quick
      -> passed; existing compiler warnings are baseline

The Outside terminal closeout and the selected Dynamic preclaim series are
complete through InnerReturn return-facts I0. The bounded Compare
prepare/reserve/commit implementation is now closed; publication, generic
Loop retirement, and performance remain separately closed.

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

Decision accepted after the Helmholtz read-only audit: the existing cleanup
cursor and operation census can be preflight-and-consumed after InnerReturn
site/relation and ThenTerminal brand/predecessor validation, before the hidden
`select_block()` mutation and before any End/SSA/ledger effect. The two
existing owners remain separate; no batch, receipt, rollback, or second
authority is introduced.

The exact bounded state is:

    I11 row/site/relation validation
      + ThenTerminal brand/predecessor validation
      -> claim(InnerReturn cleanup)
      -> claim_operation(I11)
      -> claim_exit()
      -> select_block()
      -> identity/SSA read
      -> End/Completion/ledger/seal

The accepted implementation task is
`MIR-DYNAMIC-INNER-RETURN-PRECLAIM-I0`:

    consume the existing InnerReturn cleanup row exactly once
    consume I11 and Exit claims exactly once
    remove the old post-End claim sites

Acceptance:

    all three claims occur before `select_block()`, SSA read, End append,
      and V14 ledger publication
    duplicate/missing cleanup, operation-order drift, and Exit duplication
      remain typed rejects
    foreign ThenTerminal/predecessor mismatch rejects before claims
    later physical failure still reaches `reject_begin()`/discard
    the normal fixture closes both the cleanup cursor and operation census
    a reusable line-order/effect-zero guard is green
    `inner_return_then.rs` remains below 760 lines

NoSafeSlice: site/CFG/SSA validation cannot be completed before claims, claim
failure cannot be discarded through the unpublished session, a second
cleanup/census authority is needed, or a required physical effect must precede
the claim.

Worker evidence: Helmholtz confirmed `select_block()` and the subsequent SSA
read can mutate session/SSA state, while End is the first explicit MIR effect.
The existing caller routes later failures through `reject_begin()` and
`discard_unpublished()`. `completion.claim_explicit_return()` and
`mark_return()` remain a separate post-effect Completion/identity hardening
cell and are intentionally not pulled into this I0. NoSafeSlice was not found.

### 10. MIR-DYNAMIC-INNER-RETURN-PRECLAIM-I0

Implementation complete. The existing InnerReturn cleanup, I11, and Exit
claims now occur exactly once after row/site/relation and ThenTerminal
validation but before `select_block()`, SSA binding read, End append, or V14
ledger publication. The old post-End claim sites were removed; later physical
failure still returns through `reject_begin()` and discards the unpublished
session. Completion/mark_return remain untouched as explicitly separate
authority.

The reusable guard
`rust_mirbuilder_dynamic_inner_return_preclaim_i0_guard.sh` proves one claim
edge for each row, line order before select/SSA/End/ledger effects,
source-size boundary, and card/index registration. The selected Dynamic
focused suite is 10/10, InnerReturn/Backedge/Fault/CallOut/CONNECT0/
strict-writer/pointer guards and diff checks are green, and
`inner_return_then.rs` is 246 lines.

NoSafeSlice was not triggered: the three claims use the existing cursor/census
and every later rejection reaches the existing unpublished-session discard
path.

The next design stop is `MIR-DYNAMIC-INNER-RETURN-FACTS-D0`. It must audit only
Completion `claim_explicit_return` and identity `mark_return`; Compare
prepare/reserve/commit, publication, and generic Loop remain separate.

### 11. MIR-DYNAMIC-INNER-RETURN-FACTS-D0

Decision accepted after the Pasteur read-only audit: once the existing SSA
binding receipt has validated owner, binding, and ThenTerminal block, the two
existing return-fact claims can be consumed before End. Completion remains the
sole owner of `claim_explicit_return`; identity remains the sole owner of
`mark_return`. No combined `ReturnFacts` semantic product, prepare/commit
aggregate, rollback, or second issuer is introduced.

The exact bounded state is:

    source/site/target/predecessor validation
      -> existing SSA binding receipt validation
      -> Completion claim_explicit_return
      -> identity mark_return
      -> physical End
      -> V14 ledger publication
      -> existing CFG/SSA finish validation

The accepted implementation task is `MIR-DYNAMIC-INNER-RETURN-FACTS-I0`:

    move the two existing claims before emit_checked_callout_end
    keep completion_consumption.rs and identity/ledger.rs unchanged
    retain separate owners and the existing reject_begin discard path

Acceptance:

    each return-fact claim occurs exactly once before End and V14 publication
    site/target/block/duplicate mismatch remains a typed reject
    later physical failure still reaches `reject_begin()`/discard
    positive selected Dynamic test and reusable line-order guard are green
    `inner_return_then.rs` remains below 760 lines

NoSafeSlice: the validated SSA receipt is unavailable before End, an existing
owner cannot consume its claim independently, or moving the claims requires a
combined semantic/Completion/identity authority.

Worker evidence: Pasteur confirmed that Completion stores the physical operand
witness in its own site slot while identity records source coverage in its own
ledger. Both are session-local mutations and later failure is discarded by the
existing unpublished-session terminal. The existing `finish()` remains the
final completeness check, not a replacement for these pre-End claims.

### 12. MIR-DYNAMIC-INNER-RETURN-FACTS-I0

Implementation complete. After the existing SSA receipt validates owner,
binding, and ThenTerminal block, Completion `claim_explicit_return` and
identity `mark_return` now occur exactly once before `emit_checked_callout_end`
and V14 publication. The owners and their modules are unchanged; the old
post-End claim edges are gone and later failure still reaches the existing
unpublished-session discard path.

The reusable guard
`rust_mirbuilder_dynamic_inner_return_preclaim_i0_guard.sh` now proves the
cleanup/I11/Exit claims precede select/SSA effects and the two return-fact
claims precede End/ledger effects. The selected Dynamic focused suite is
10/10, the Backedge/Fault/CallOut/CONNECT0/strict-writer/pointer guards and
diff check are green, and `inner_return_then.rs` is 250 lines.

NoSafeSlice was not triggered: both existing owners can consume their facts
after the validated SSA receipt, and the outer unpublished session remains
the discard boundary.

### 13. MIR-LOOP-COMPARE-PREPARE-RESERVE-D0

Decision accepted conditionally after the Dirac read-only audit: no current
NoSafeSlice, but implementation must stop if a private Branch seam or
definition/V13 co-seal cannot be made valid. The current exact order is:

    I8/I9/If preclaims
      -> I9 ValueId/Const effect
      -> operand/destination/Bool preparation
      -> V13 reserve_result
      -> fallible Compare writer preparation
      -> Compare append + Bool commit
      -> V13 commit with assert pairing
      -> fallible Branch emit

The required target order is:

    preclaims
      -> Compare preparation
      -> Branch preparation
      -> V13 reserve_result       # last fallible operation
      -> private commit:
           Compare append, Bool commit, V13 commit, Branch commit

The existing `CanonicalCfgSessionV1::emit_branch` has no prepare/commit seam;
it performs `preflight_edge` and MIR mutation together. The bounded I0 must add
a borrow-free, same-owner `PreparedCanonicalBranchV1` whose commit is the only
new Branch mutation path. The Compare prepared product must likewise be
borrow-free so Branch preparation can run before V13 reservation.

Decision candidates are intentionally narrow:

    accepted:
      all fallible preclaims and writer/Branch preparation
        -> last-fallible V13 reserve_result
        -> private co-sealed commit
        -> one Compare append, Bool commit, V13 commit, Branch commit

    NoSafeSlice:
      a prepared writer/Branch product cannot be committed without a
      Result-returning check, or definition/slot pairing cannot be made
      private and move-only without creating a second authority.

The worker audit named:

    Source authority + canonical issuer
    non-authorities and forbidden fallback edges
    exact first physical effect and last fallible step
    whether Branch has a prepare/commit seam
    how the pending V13 slot is co-sealed to the Compare definition
    acceptance/negative/guard evidence
    non-claims, especially publication and generic Loop retirement

Do not add a new semantic meaning, general dominance witness, generic Loop
ledger adapter, publication proof, or optimization work in this I0.

Worker evidence: Dirac confirmed the current last fallible operation is
`emit_branch` (the current code prepares V13 before Compare preparation), and
that Branch has no real seam. The worker found no present NoSafeSlice, but
requires the Branch prepare/commit seam, Compare borrow-free preparation,
last-fallible V13 reservation, a private move-only I9 aggregate, and removal of
arbitrary definition/slot `assert_eq!` pairing before implementation can be
accepted.

### 14. MIR-LOOP-COMPARE-PREPARE-RESERVE-I0

Implementation complete under the conditional D0 acceptance. The current writer
front door is split into:

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

`CanonicalLoopCompareI64WriterV1::emit` is test-only compatibility coverage and
is not a selected-I9 production entry. `prepare_canonical_compare_append`
remains the writer preparation authority; it runs before `reserve_result`.

Branch emission is part of the same transaction. Any fallible target/current
block/operand preparation required by Branch must complete before
reserve_result. After reservation, Branch may only be a private infallible
commit consuming the prepared Compare definition/result relation. A sequence
of Compare append -> V13 commit -> fallible emit_branch is not accepted.

The private aggregate owns the prepared writer, the session-bound Branch plan,
and the exact pending V13 slot together. The pending ledger commit no longer
accepts an arbitrary `CanonicalCompareDefinitionSourceV1` or uses `assert_eq!`
to pair unrelated products. Its infallible commit is reached only through the
aggregate that prepared the Branch and reserved the slot.

I0 evidence:

    `CanonicalCfgSessionV1::prepare_branch` is non-mutating and its prepared
    product carries the exact session reference into commit
    writer preparation rejection -> no V13 reservation and no MIR append
    V13 reservation is after Compare and Branch preparation
    after reservation, the private aggregate has no Result path
    no post-append type or ledger lookup remains on the strict path
    no pending-result assert_eq! pairing remains
    one Compare append, one Bool fact, one V13 publication, one Branch
    destination and Bool preparation products are move-only
    selected Dynamic focused suite: 10 passed
    canonical CFG prepared-Branch test: 1 passed
    CONNECT0, strict-writer, preclaim, pointer, and diff checks: green
    touched production sources remain below 760 lines (maximum: 711)

NoSafeSlice was not triggered: the aggregate makes the definition/slot pairing
private, the Branch plan is session-bound, and the strict writer does not use a
repair-capable legacy front door. Publication and generic retirement remain
closed by explicit non-claims.

### 15. MIR-LOOP-BODY-BRIDGE-RETURN-AFFINITY-P0

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

### 16. Parked cleanup: MIR-LOOP-GENERIC-COMPARE-RETIRE-D0

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
