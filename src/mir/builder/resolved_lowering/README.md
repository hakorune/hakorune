# Resolved lowering boundary

This directory owns the first production consumer of a sealed semantic owner.

Allowed inputs are only `CanonicalFirstFamilyPlanV1` values produced by the
whole-unit compiler preflight. Recursive lowering accepts sealed located-node
carriers and resolves lexical identity through exact source sites.

Invariants:

- `BindingRefV1 -> ValueId` is the canonical value environment.
- names are diagnostic cross-checks, never lookup keys.
- legacy `allocate_binding_id()` is structurally vetoed while an owner is installed.
- declarations, variable uses, assignment targets, and exits must all finish
  source coverage before the function draft can be published.
- canonical lowering seeds separate RegionId and ScopeId stacks from the sealed
  function/function-body roots; BlockExpr consumes one exact pair and retires
  only pair-owned BindingRefs at scope leave.
- I1b consumes one pre-Builder verified statement-If flow in source preorder.
  Both branches start from the same post-condition BindingRef baseline; the
  sealed join-source matrix selects final PHI inputs, and all PHIs define
  before one effect-authorized batch publication.
- RegionId and ScopeId stacks remain separate. Statement If consumes exact
  control/branch identities and coverage only; no durable RegionId-to-block
  map is published before SA4.
- legacy statement/expression dispatch, Planner/CorePlan, Lambda, production
  Loop activation, Main, REPL, and ProgramV0 are outside this boundary.

## Common V2 canonical session-open canary

`with_common_v2_canonical_session` is the caller-zero consumer of one
callback-scoped common V2 admission. It consumes the admission once, projects
the typed BlockExpr expectation inside `new_common_v2`, and creates one owned
`ResolvedFunctionCompletionConsumptionV1` from the installed cohort's borrowed
semantic Completion. The wrapper retains the same pre-session envelope beside
the sole `CanonicalSsaFunctionSessionV2`, preventing a second sibling loan.

This canary opens an unpublished session owner only. It does not mutate a
`MirBuilder`, emit CFG/SSA/PHI/operations, claim Completion, prepare DraftSeal,
or lower lifecycle/Text/route code. `new_selected_dynamic` and raw BlockExpr
counts are not valid substitutes; the first session-effects boundary remains
an explicit design-stop task.

## Common V2 physical-entry stamp retention I0

The consuming physical-entry session takes the existing prepared
`PhysicalFunctionEntryCohortStampV1` and attaches it to the same canonical
session that consumed the prepared skeleton. Attachment is move-only and
one-shot, and happens before the outer Builder transaction opens. The common
session wrapper exposes only a callback-scoped borrow for later consumers;
there is no copied stamp, second Port loan, reconstructed cohort, or detached
result receipt. The stamp is mechanical same-cohort evidence only. Missing or
already-consumed stamps, owner/key/signature/lane drift, foreign borrows, and
late escape reject before physical result/edge/CFG effects. Focused positive,
missing-stamp, and session-admission canary tests cover the seam.

## Common V2 Length-result materialization canary I0

The first consumer after the condition operand inventory is deliberately a
Builder-neutral `LengthCallMaterializationCanaryV1`. It checks the retained
source Length relation, the fixed two-row operand inventory, and the same
physical-entry cohort stamp exactly once inside the canonical session. The
receipt carries only source keys and cohort provenance; it creates no
`ValueId`, type, `CallSlot`, CFG edge, or terminator. A missing stamp, owner or
producer drift, malformed Length shape, and a second issue attempt reject
before effect. This is a canary for the future Length result issuer, not a
physical call lowering or a permission to emit the parent Bool comparison.

## Common V2 StringLen target-plan I0 (2026-08-17)

The caller-zero target-plan seam now fixes the source-backed realization facts
needed before a physical Length call: the canonical StringBox target, receiver
relation, zero arguments, I64 result, PureRead effect, non-suspending policy,
and same-cohort target/manifest brands. `PreparedLoopV2StringLenCallTargetPlanV1`
is non-Clone and physical-ID-free; it is issued only by the common session from
the retained S6C logical call, CallSlot row, operand inventory, and physical
entry stamp. The plan is one-shot and cannot be rebuilt from `CoreMethodOp`,
`/N`, MIR/JSON, or a legacy emitter.

Focused tests cover same-cohort target facts, canonical `StringBox.length`,
plan/canary item-block-result parity, duplicate plan rejection, missing stamp,
and late callback discard. This I0 emits no `Call`, `ValueId`, type, Compare,
edge, CFG/PHI, Completion/DraftSeal, lifecycle, Text route, fallback, retry,
or production caller. The next design stop is the canonical Length Call/result
materializer.

## Common V2 source-segment block allocation I0

The first common-V2 Builder effect is deliberately limited to source-backed
segment blocks. A callback-scoped allocation plan is issued from the existing
physical-ID-free layout, and the canonical session allocates one unpublished
block per ordered source segment. The receipt keeps the source loop/block/
split relation with its physical block id; it does not allocate synthetic
After, edges, terminators, operations, or effects.

Owner/function/cursor/collision checks run before mutation. The surrounding
`CanonicalFunctionLoweringSessionV1` is the sole discard owner for late
callback failure; CoreContext ids remain monotonic, so discarded unpublished
ids are gaps and never reused. Positive and late-discard focused tests are
green. Synthetic After requires a separate source-backed design stop; CFG,
PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, and
production callers remain closed.

## Common V2 condition-block physical target I0

`CommonV2CanonicalSessionRefV1::with_condition_block_target` now lends one
callback-scoped `ConditionBlockPhysicalTargetRefV1` from the already allocated
source-segment receipt. The view carries the same owner, logical condition
block, physical `BasicBlockId`, and retained physical-entry stamp; it cannot be
repaired from the Builder cursor or retained for a second session. The outer
unpublished-function transaction remains the sole late-discard owner.

This I0 emits no `Call`, `ValueId`, `Compare`, edge, terminator, CFG/PHI,
Completion/DraftSeal claim, lifecycle, Text route, fallback, retry, or
production caller. Focused positive and late-callback-discard tests are green;
the next bounded slice is the callback-scoped Length receiver operand.

## Common V2 If continuation target placement I0 (2026-08-18)

`CommonV2CanonicalSessionRefV1::with_if_continuation_target` consumes the
existing JoinSig `Exit + Fallthrough(NextItem)` relation and the already
allocated source-segment receipt. It validates the one-branch S6C shape,
same-block strict target ordering, loop/split/stamp parity, and target-item
non-control status before allocating exactly one unpublished continuation
block through `CanonicalSsaFunctionSessionV2::create_unpublished_block`.

The returned `IfContinuationPhysicalTargetRefV1` is non-Clone and
callback-scoped. It carries only the owner, If/NextItem relation, source
segment provenance, physical target block, and retained entry stamp. The
session-local one-shot ledger rejects re-entry, and the outer unpublished
function transaction remains the sole late-discard owner. This I0 emits no
edge, terminator, operation, Return, BlockEnd, CFG/SSA/PHI, publication,
fallback, retry, or production caller; branch emission and item-to-split
authority remain the next design stop.

## Common V2 Return-read co-seal view I0 (2026-08-18)

The common pre-session envelope now retains one callback-scoped
`CommonV2ReturnReadCoSealRefV1` from the existing source-to-Recipe Return
binding. The view co-seals the Return-read operation, If/Exit placement,
physical-ID-free segment coverage, and the Join `Return -> FunctionExit` plus
`Fallthrough(NextItem)` relation in one cohort. It lends logical keys and split
ordinals only; it does not allocate or expose a physical block or `ValueId`.

The operation/control/layout drift negatives fail before the canonical session
can consider a later physical materializer. No Return instruction, terminal,
edge, CFG/SSA/PHI, Completion/DraftSeal, publication, fallback, retry, or
production caller is opened. The physical Return-read receipt and terminal
issuer remain the next design-stop decision.

## Common V2 physical Return-read receipt I0 (2026-08-18)

`CommonV2CanonicalSessionRefV1::with_return_read_physical_receipt` is the
session-local consumer of the existing Return-read co-seal. It consumes the
same-session segment allocation and one-shot continuation target, validates
owner/stamp/row/split parity, then asks canonical identity/SSA for the item-9
`BindingRef` read. The existing Loop operation type authority may finalize an
`Unknown` provisional PHI through the canonical session's existing i64 type
publication seam; this is type publication reuse, not a second semantic type
issuer. The receipt then claims the existing
Completion `FunctionExit` witness and marks the source return once.

The returned receipt is non-Clone and callback-scoped, and the session ledger
poisons duplicate issuance before effects. The outer unpublished physical
entry transaction remains the sole rollback owner, including late callback
failure. Focused positive and late-discard tests are green (2/2). This I0
does not emit `Return` or branch instructions, edges, PHI/CFG publication,
DraftSeal publication, fallback, retry, or a production caller; split/terminal
authority and branch/Return CFG writing remain the next design stop.

## Common V2 shared-segment scope I0 (2026-08-18)

`CommonV2CanonicalSessionRefV1::with_shared_segment_scope` owns one private,
one-shot segment allocation brand and lends the exact receipt through the
explicit Length-from-scope API. The Bool receipt retains that brand, so a
second allocation or foreign receipt cannot be paired later. Positive,
second-allocation, and late-callback rollback tests are green.

Return-read consumption also checks the logical condition identity before any
effect. The current S6C fixture intentionally rejects the outer Length/Bool
condition against the inner TextEq If as `ConditionLogicalMismatch`; this is
negative evidence, not a branch-ready claim. No branch, Return, edge,
PHI/CFG publication, fallback, retry, or production caller is opened.

## Common V2 Length receiver operand I0

`CommonV2CanonicalSessionRefV1::with_length_receiver_operand` consumes the
same source-segment receipt and projects the resolver-proven local
`BindingRef` from the Length call relation. It lends one
`LengthReceiverPhysicalOperandRefV1`, joining the source binding, condition
block, retained entry stamp, and the canonical identity/SSA
`CanonicalBindingReadReceiptV1`. The resolver relation and logical inventory
remain the semantic authorities; the canonical session is the only physical
read issuer.

The view is one-shot and callback-scoped. Missing/non-local receivers,
owner/type/target/stamp drift, duplicate entry, callback escape, and late
failure reject before any Length `Call` or result effect; the outer unpublished
function transaction remains the sole discard owner. Focused positive,
one-shot, and late-discard tests are green. Direct `StringBox.length` Call,
I64 result, Compare, edge/CFG/PHI, Completion/DraftSeal, lifecycle, Text,
route, fallback, retry, and production caller remain closed.

The direct Length Call/result D0 is now accepted as the next BoxShape: the
same canonical session must consume the target plan, receiver view, condition
target, and entry stamp, then issue exactly one generic `Call` plus one I64
result receipt. Its first I0 is caller-zero only; the existing outer
unpublished transaction discards the canary, so module publication and the
parent Bool/Compare path remain unopened.

## Common V2 direct Length Call/result I0 (2026-08-17)

The caller-zero canary now consumes that same-session target/receiver/
condition/stamp cohort and emits exactly one generic `StringBox.length` Call
with one canonical I64 result receipt. The generic unified Call emitter remains
the sole Call constructor; the session verifies the emitted callee, receiver,
destination, and `READ` effect before publishing the result type. A late
callback failure discards the unpublished function transaction, so the receipt
and Call never reach module publication. Focused direct-emitter and
late-discard tests are green. Parent Bool/Compare, edges/terminators, CFG/PHI,
Completion/DraftSeal, lifecycle, Text, route, fallback, retry, and production
remain closed. The follow-up receipt-lifetime BoxShape is accepted: the
receipt now owns an exclusive borrow of the exact `CommonV2CanonicalSessionRefV1`
that issued the Call/result, so it cannot be re-paired with another session or
escape the callback. The caller-zero lifetime I0 changes only that return
signature and its focused borrow/duplicate/late-discard gates; Bool/Compare
materialization remains a separate later row.

## Common V2 Bool-result materializer I0

`CanonicalLengthCallResultReceiptV1::consume_for_condition_bool` is the sole
bridge from the source Left ReadBinding and Length result to the physical
condition value. It consumes the same-session Length receipt, reads the seeded
Left binding at canonical entry (without provisional PHI creation), issues one
Bool ValueId/type, and emits one mechanical `Less` Compare in the physical
condition block. The returned non-Clone receipt retains the exclusive session
borrow. Missing seed, wrong role/type, duplicate/re-entry, and late callback
failure reject before publication; the outer unpublished transaction is the
only discard owner. No branch/edge/terminator, CFG/PHI, Completion/DraftSeal,
lifecycle, Text, route, publication, fallback, retry, or production caller is
opened. The next bounded gate is `LOOP-PRECUTOVER-AUTHORITY-G0`.

## Common V2 initial index seed source transport I0

The seed BoxShape is accepted, and the caller-zero source transport is now
landed. The seed authority comes from
`VerifiedS6CTypedInputRelationV1::initializer()`, the resolver-owned
`ResolvedInitializerRelationV1`, and the source ledger's
`ResolvedLiteralSourceV1::Integer(0)`. The transport I0 adds one private,
non-Clone `PreparedLoopV2InitialIndexSeedRelationV1` view to the same S6C
ingress/common envelope, carrying binding, I64 type, literal witness, index
carrier, and entry relation. Package/Port and the Bool materializer only lend
or consume it; they do not infer the seed. Missing/foreign site, binding,
carrier, owner, stamp, type, or literal evidence rejects before physical
effect. This I0 emits no Const, declaration, ValueId, read receipt, Bool,
Compare, CFG/PHI, lifecycle, Text route, fallback, retry, or production
caller. The seed materializer I0 below issues the physical `ConstI64(0)` from
this transport.

## Common V2 initial index seed materializer I0

`CommonV2CanonicalSessionRefV1::emit_initial_index_seed` is the sole physical
issuer for the source-backed pre-loop index seed. It reserves one ValueId,
writes one entry-block `ConstI64(0)`, and calls `publish_declaration_exact` for
the resolver BindingRef. The returned non-Clone receipt keeps an exclusive
session borrow and carries only the binding/carrier/value witness. Duplicate
entry, missing function, entry drift, and late callback failure reject before
publication or leave the outer unpublished function transaction as the sole
discard owner. No Bool/Compare, branch/edge, CFG/PHI, lifecycle, Text, route,
fallback, retry, or production caller is opened.

## Common V2 synthetic After allocation I0

The accepted After-boundary relation now has one caller-zero placement effect.
`CommonV2CanonicalSessionRefV1::allocate_v2_after_block` validates the
same-session RootAfter relation and the complete source-segment receipt,
issues one internal one-shot allocation plan, and asks
`CanonicalSsaFunctionSessionV2::create_unpublished_block` for exactly one
unpublished `BasicBlockId`. The public result is only the callback-scoped
`PreparedAfterBlockViewV1`; it carries source/frame evidence and a physical
block but no successor, edge, operation, or publication power.

The session-local allocation state rejects a second allocation. Existing
entry/segment collision and checked cursor-range guards run before mutation;
the outer `CanonicalFunctionLoweringSessionV1` remains the sole late-discard
owner. Monotonic cursor gaps after discard are non-semantic and are never
reused. Focused positive, one-shot, and late-discard tests are green. Parent
Resume, edges/terminators, operations, CFG/PHI, Completion/DraftSeal,
lifecycle, Text, route, fallback/retry, and production callers remain closed.

## Current selected Dynamic V2 handoff

The selected A-prime demand is a source/parameter/identity wrapper around the
single co-sealed `PreparedDynamicLoopOperationProgramV2`. It is not a second
physical-demand authority. The selected physical handoff consumes that demand
once and issues a Builder-free, move-only V2-native plan. The plan validates
the complete operation order, keeps the source-role projection only as a
diagnostic cross-check, derives segment boundaries from exact placement/control,
validates the CallSlot relation, and checks the bounded I10 disposition before
opening a session or allocating `ValueId`/`BasicBlockId`.

The selected package adapter is the one named production caller of this
handoff. It remains a candidate/unpublished route: the session can close its
physical profile, exact-two DraftSeal, and cataloged collector receipt, while
live module publication, Boundary execution, and old-edge retirement remain
later W6 work. The admission guard fixes the definition/focused-test/
production-caller census and rejects any second production caller. V1
conversion, raw Recipe/JoinIR re-reading, name/ordinal repair, fallback, and
retry are not valid alternatives.

The selected lifecycle R0 co-seals the retained four cleanup rows with the
admitted I6/I7 site plans before opening Builder state. `CheckedCallOutEnd` is
the sole physical lease-consumption vocabulary and `CheckedCallOutFault` is
the non-rejoining canonical fault terminal. The E1-E5 session now consumes the
bounded I8-I16 cursor and closes the unpublished profile/DraftSeal/collector
candidate; it does not open the LLVM/VM execution lane or publish a live module.

The current implementation row is the family-native V2 emitter, not another
semantic product. E1 now provides the first bounded continuation
`I7.Normal -> I8 ConstI64(0) -> V12 -> I9 CompareI64 -> V13 -> Branch`; it
uses the session-issued I7 Normal landing rather than the already-terminated
logical `BodyPrelude` target. The selected-fixture canary receives all targets
through canonical unpublished function/SSA/CFG sessions and may not accept a
raw `MirBuilder` or `BasicBlockId`. The invocation cleanup owner retains only
the I6/V10 Dynamic discharge receipt; I7/V11 is exact I64 and has no lease or
End row. If any capability is unavailable, the session rejects before its
first Builder effect. E2 now materializes I6 Fault as a successorless terminal
and I7 Fault as one canonical `CheckedCallOutEnd` for V10 followed by a
successorless `CheckedCallOutFault`; it never rejoins `After`. E3 now reads
I11/V14 in ThenTerminal, consumes the I6 End cutpoint, claims the inner
Completion return, and seals Then without emitting Return; DraftSeal remains
the sole Return writer. E4 now consumes I13/V15, I14/V16, I15/V17, and I16's
induction assignment in Continuation, emits the Backedge I6 End, jumps back to
Header, and seals the canonical Header PHI with Enter and Continuation inputs.
E5 closes the profile and hands the exact-two DraftSeal and cataloged collector
receipt to the unpublished candidate path; live publication remains closed.

The closed-function CheckedCallOut census is then lent once through a
non-Clone HRTB view. AOT JSON carries the canonical source/operand,
Normal/Fault landing, Normal-result, effect, and End-cut facts from that view;
the Boundary C1 consumer cross-checks the emitted JSON topology and rejects
drift before object emission. No downstream consumer rebuilds a site plan or
re-scans MIR to locate a call site.

The selected CallOut corridor keeps Normal-result typing at the canonical SSA
issuer: the existing `string_handle -> MirType::Box("StringBox")` route
projection is used for I6's `EndAuthorizedHandle`, while I7 uses the existing
`ScalarI64 -> Integer` projection. ValueId issuance, MIR type publication, and
the session-private lease/ImmediateI64 ledger row are one checked sequence; the
corridor does not mutate `type_ctx` or repair a missing type after emit.

The physical issuers are separate children of this selected V2 boundary. The
private E1 continuation consumes exact I9 (`V11:I64`, `V12:I64` -> `V13:Bool`)
plus the I7 CallSlot and I8 ConstI64 producer receipts, then routes its result
through the canonical CFG issuer and rejects any I9 Fault row. The cleanup issuer consumes the four scoped rows from
`invocation_cleanup.rs` in their fixed order (`I6 fault=[]`, `I7 fault=End(V10)`,
inner Return/Backedge=`End(V10)`) and excludes `V9`, `V17`, `V11`, and the I64
induction from cleanup ownership. The private E4 continuation/backedge leaf
borrows the retained I13-I16 rows, uses the canonical Header current value,
and emits only through Canonical SSA/CFG/identity owners; it does not invent a
PHI or Return. A move-only admission gate co-seals only those two physical
receipts; it is not a semantic, Fault, Completion, Recipe, or JoinSig owner.
Missing/foreign/ambiguous producer receipts or an unavailable End primitive
are `RejectBeforeEffect`. The selected canary consumes that negative-only
disposition into a private unpublished-session fence immediately before
Builder open; the session does not retain a disposition and no executable
readiness is issued. Generic compare, scope cleanup, name/last-use
inference, `MirType` repair, `nyash.integer.get_h`, fallback, and retry are
forbidden.

The selected handoff issues these two Builder-free requirements and a move-only
`SelectedDynamicV2PhysicalCapabilityAdmissionV1`. Its disposition remains
explicitly `RejectBeforeEffect` until `prepare_aot_activation` consumes it into
the private unpublished-session fence; the package adapter is its one named
production caller, while live module and Boundary execution callers remain
closed. The I8/I9 continuation may be
exercised by a real selected-fixture, unpublished-session canary so its receipt
boundary is tested, but that canary is not a capability-gate bypass or a live
publication. The handoff must consume the plan, move the
private ledger, and own a session-issued opaque target set. The consuming
handoff co-seals I8/V12/literal-0/placement/Prelude once; the emitter does not
rescan the whole operation program outside its private V2 cursor. The cursor
consumes the already verified 15-row Recipe array exactly once before Builder
state opens, checks dependency/use-before-produce and retained I6/I7
CoreMethod rows, and issues no ValueId or CallOut. The all-or-nothing capability gate remains
the sole production handoff. No `cfg(test)` semantic constructor, raw
block/value getter, or gate bypass is allowed. The preflight ledger borrow is
test-only once the consuming handoff exists. The numeric I6/I7/I8/I9 and
V10/V11/V12/V13 checks are private guards for this selected bounded cohort,
not a generic physical planner or a reusable V2 authority. Inner-Return source
identity and the Backedge loop key remain in the cleanup demand; the demand
may not be split through copy accessors before a future consuming emitter is
connected.

## Canonical V2 function finish

The three canonical V2 profile lowerers (`trivial_ssa`, `direct_accum`, and
`nested_predicate`) share one consuming finish terminal:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal
  -> ReadyFunctionDraftSealV1
```

Each profile closes its private effect/After/final-carrier ledger into one
move-only `ReadyCanonicalProfileCloseV1`. The common terminal consumes that
receipt and closes CFG, semantic/If control, identity/Binding SSA, PHI, the
resolved binding ledger, and Completion exactly once. It is the sole V2 issuer
of `ReadyFunctionDraftSealV1`.

The terminal accepts no raw body/site/end/target/current-block facts for
re-inference. Those identities are sealed when the exact resolved function
session opens; the profile receipt carries the already-claimed terminal
witness. A failed or duplicate close rejects before publication, and any late
failure discards the whole unpublished function. `PhiTxn` rollback is only
best-effort local cleanup; the outer function session owns atomic discard.

The existing non-V2 direct constructor caller is an explicit compatibility
allowlist entry. It may not gain callers and is retired by a later bounded row;
it is not part of the V2 finish migration. Focused guards must keep V2 direct
`ReadyFunctionDraftSealV1::new` callers at zero and keep all source files below
the repository's 800-line boundary. Every implementation slice updates this
README, the owning reference, and current-entry mirrors in the same commit.

Implementation receipt (`6bf3dd6b35`, 2026-08-07): the three V2 lowerers now
use the consuming terminal, including the previously missing DirectAccum CFG
finish. The one non-V2 constructor remains an explicit, non-growing
compatibility debt. The focused session, resolved-lowering, and draft tests,
the canonical finish guard, and the current-state pointer guard are green.
The caller-zero Loop physicalization/DraftSeal canary is closed; production
selection remains closed at the next docs-only
`CALLABLE-LOOP-PRODUCTION-EDGE-D0` stop.

The caller-zero topology slice is landed as historical test-only evidence
behind the `loop_recipe_physicalizer/` directory facade. It consumed one
move-only common boundary and one session-local `ReadyLoopEntryV1`, borrowed
the existing canonical CFG service, and allocated the old recursive
header/body/step/After blocks. That P0 topology shape is not the current R3
Callable physical boundary.

This is not a production physicalizer or selector. It emits no operation MIR,
Return, DraftSeal, publication, retry, fallback, or legacy deletion. The
DirectAccum binding port remains profile-specific and must not be reused as
the common port; no second CFG/SSA/PHI owner is allowed. The historical
passive `LOOP-RECIPE-OPERATION-EFFECT-S0` product issues an item-keyed
exact source/effect ledger before operation emission is opened. The product is
test-only, moves the verified Core once, and emits no operation MIR, Return,
DraftSeal, selector, retry, fallback, or production route. The callable
adapter is now closed as a separate source-evidence receipt. The Generic G0
anchor ledger is also closed: its 15 item keys are issued before source facts
are dropped, with item 3 matching the child-entry carrier relation.
Cross-profile parity is also closed as a diagnostic-only receipt; it compares
neither profile counts nor source order. Reviewed Decision B separates full
demand/preflight from leaf emission. The Builder-free complete operation
demand, topology module split, and the private logical-to-physical block
receipt are now closed. The private
`LOOP-RECIPE-OPERATION-EMITTER-CONST-S0` and the bounded
`LOOP-RECIPE-OPERATION-EMITTER-READ-I0` leaf canaries are closed. ReadBinding
projects the complete prepared program, claims the exact source through the
canonical BindingSSA/PHI owner, validates explicit entry requirements, and
returns distinct logical/physical receipts. Pre-claim rejects are typed;
claim/read/type/receipt failures terminate the unpublished function session as
one discard transaction. No single-operation demand extraction was added.
Full operation physicalization, production activation, carrier seeds,
selector, retry/fallback retirement, and legacy deletion remain closed until a
named production caller switch is authorized. Continuation, Tail, and the
bounded caller-zero DraftSeal seam are landed but remain test-only.

The current bounded preparation slice adds a private Prepared-product
move-only handoff, complete WriteBinding projection, typed pure leaf bridges
for Const/Binary/Compare, exact logical-to-physical operation target receipts
(all validated before the first leaf effect),
and phase-separated physical dispatch errors. The focused fixture proves a
Const -> Binary -> Compare chain without introducing a second CFG/SSA/PHI
owner. The callable Prelude adapter is now landed as caller-zero evidence:
exact resolver-backed argument bindings are read through canonical identity,
the external Prelude result is emitted through the shared direct-call emitter,
and the Loop initializer is materialized from its exact source site into a
separate `ReadyLoopEntryV1` binding/value. The Prelude result local is not the
Loop input and is never used as an implicit entry value. The
`CALLABLE-LOOP-AFTER-CLOSURE-P0` and `CALLABLE-LOOP-TAIL-COMPLETION-P0` rows
are now bounded caller-zero seams. The
latter reads the exact Tail binding through canonical identity, validates the
declared trivial ABI, claims `tail.value_site()` before the non-claiming
`read_entry_receipt`, and claims Completion/return coverage once. The sealed
After receipt moves a non-Clone profile-close receipt proving the exact
callable `7 = Pure4 + Read2 + Write1` coverage, Bool condition, owner,
terminal block, and After predecessor. The later finish step must consume it
through a non-no-op `finish_profile_close` closure. The bounded DraftSeal
canary consumes that evidence through the typed finish terminal and existing
DraftSeal prepare/commit, producing one `CompletedFunctionDraftV1` without
collector/module publication. The common physicalizer deliberately stops
before Tail, ABI, Completion, Return, and DraftSeal; the caller-zero profile
adapter owns that handoff. This separation is now exercised end-to-end by the
closeout receipt below. Production selection and legacy retirement remain
later bounded work.

The bounded `CALLABLE-LOOP-AFTER-CLOSURE-P0` slice is now landed as a
caller-zero continuation proof. It uses the real Prelude receipt, emits the
complete seven-operation Callable schedule (`Pure=4`, `Read=2`, `Write=1`),
then emits the fixed preheader/header/body/step/After edges and seals CFG and
BindingSSA in backedge-safe order. An unsealed `Unknown` PHI is typed only by
the verified Recipe value class; concrete or missing type facts reject as
`ResultTypeMismatch`. The success canary commits the draft and restores the
caller; rejection paths continue to discard the unpublished session.
Production selection, retry, and legacy deletion remain closed.

### Callable full physical canary closeout (2026-08-08)

`CALLABLE-LOOP-PHYSICAL-CANARY-P0` is now closed as caller-zero evidence. The
test-only bridge starts from the exact resolved-module input and its existing
resolver ledger, so S2 full-demand preparation, Prelude, topology, all five
operation families, sealed After, Tail/Completion, the sole
`finish_for_draft_seal` terminal, and DraftSeal prepare/commit all use one
owner-branded request. It does not re-resolve, clone source, or create a
second CFG/SSA/PHI owner.

The late-failure canary seeds a Recipe-derived Compare result key, observes a
typed duplicate after earlier emission, discards the complete unpublished
function, and reruns the same semantic fixture in a fresh session. The
focused P0 tests are green and the canary file remains below 800 lines. The
G0 parity design is accepted: a compiler-side composite pairs exact resolver
input with neutral S4, while `L0.After/b1` is split into neutral continuation
and a distinct tail capability. The Builder-free
`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` ingress and common
`LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` layout are now closed. R1 derives
recursive Recipe preorder and parent-resume segments without Builder effect;
R2 is closed as an adapter receipt only. R3-I0 now allocates one block per R1
segment plus one root After (no Step), retains the complete segment program,
preflights and emits every entry/transfer once through canonical
CFG/identity/PhiTxn, and returns the neutral After continuation used by the
existing Tail/Completion/DraftSeal path. Physical G0, selector, collector,
retry/fallback, and legacy retirement remain closed.

## Generic G0 exact-ingress I0 receipt (2026-08-08)

`src/mir/compiler/generic_g0_physical_prepare.rs` is the bounded `cfg(test)`
compiler-side ingress for `LOOP-CALLER-ZERO-PARITY-G0-I0-R0`. It pairs the
exact resolver input with neutral S4, validates source/owner/frame/forest/
entry/tail provenance, preserves G0's post-loop read and I64 ABI in
`VerifiedGenericG0TailCapabilityV1`, and proves all fifteen Recipe members
through common `prepare_all` with no Builder effect. It does not own AST,
resolver, CFG/SSA/PHI, physical IDs, Completion, DraftSeal, selector,
fallback/retry, or publication. Physical G0 and production cutover remain
closed.

## Full operation demand P0

The Builder-free `VerifiedLoopOperationPhysicalDemandV1` now consumes the
resolver semantic context, complete operation/effect ledger, and one logical
After continuation as a move-only product. `prepare_all` derives Recipe
Loop/Block/Item order and proves complete seven-row Callable and fifteen-row
Generic G0 coverage before any Builder effect. The resolved-lowering layer is
not imported by this product; physical block allocation, leaf operation
emission, session opening, and DraftSeal remain later boundaries.

## Caller-zero Loop physical prepare

`src/mir/compiler/loop_physical_prepare.rs` is a test-only pre-effect contract
boundary for `LOOP-PHYSICAL-PREPARE-P0`. It brands the exact resolved callable
input, derives a prelude target/result capability from the existing callable
index/header, and seals one Tail/ABI/Completion compatibility relation before
any Builder session opens. The moved topology-only compatibility
`VerifiedLoopPhysicalDemandV1` owns the co-sealed logical product for the
closed P0 topology probe; the retained resolved input remains a borrowed view.
It is not the current full-operation demand and cannot feed operation emission.

The current `helper.to_i64(n)` MethodCall fixture intentionally has no
resolver-issued direct callable target and therefore rejects with typed
`NoSafeSlice::MissingPreludeTarget`. It remains a typed `Method` negative. The
bounded `CALLABLE-STATIC-PREFIX-S0` fixture is separate: top-level
`int_to_str(n: i64)` calls catalog-backed `to_i64(n: i64)` as a real
`FunctionCall`, and the observer records only the resolver-issued target and
explicit `FreeStatic` shape. No target injection, name lookup, AST rematch,
physical ID, Builder effect, selector, retry, fallback, or production caller
is opened by this row.

The neutral shape vocabulary remains in
`callable_single_loop_source_shapes.rs`; embedded syntax/source-map/static-
fixture tests remain sibling test-only modules and all touched files stay
below the 800-line limit. `CALLABLE-STATIC-PREFIX-MAP-S1` is now closed as a
source-only relation: same-brand different-owner resolver targets are kept,
while foreign compilation brands reject as `ForeignOwner`. The next bounded
cell is `CALLABLE-STATIC-PREFIX-P0` for declaration-derived ABI/Prepared
evidence; no physicalizer or production route is opened.

`CALLABLE-STATIC-PREFIX-P0` is now closed: the static fixture yields one
positive Prepared relation whose caller ABI comes from the completion/header
declarations and whose callee ABI comes from the resolver target header. ABI
is no longer accepted as an external argument at this boundary. The next
step is a design-only audit of the common physicalizer/session finish seam;
physical Builder effects remain closed.

## Disconnected canonical CFG prerequisite

`canonical_cfg/` owns the SSA-C1 edge/seal substrate. It emits a terminator and
its cached predecessor witness as one fallible operation, derives predecessor
truth directly from terminators, and rejects late edges or cache drift without
calling CFG repair. During SSA-C1 it has zero production If, Loop, and Binding
SSA callers; the existing A+ If path remains unchanged.

## Callable segment block cutover R2

`LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` is closed as a bounded adapter cutover.
The private `LoopPhysicalSegmentBlockReceiptV1` adapts the closed R1 layout to
the existing canonical topology and brands each exact segment with its
physical block. It is not the R1 segment allocator. The selected Callable
dispatcher builds its complete item-to-segment index from the layout and
issues targets by segment; the selected canary no longer uses
logical-block-only execution lookup.

The receipt rejects missing, foreign, duplicate, and aliased placements. The
Callable canary preserves seven-row parity (`Pure=4`, `Read=2`, `Write=1`),
and its late failure still discards the unpublished function as one transaction
before a fresh-session rerun. No new CFG/SSA/PHI owner, G0 physical route,
recursive After writer, selector, fallback/retry, publication, or legacy path
was introduced. The old Step block remains outside the R1 transfer graph, so
this adapter is historical and no longer feeds the selected Callable canary.

## Callable recursive After R3-I0

`LOOP-COMMON-RECURSIVE-AFTER-R3-I0` is closed for the selected caller-zero
Callable canary. `segment_allocator` is the only R3 allocation entry: it
allocates exactly one physical block per R1 segment and one root After block.
`PreparedLoopPhysicalLayoutV1` carries an explicit sealed `entry_segment`, and
`CompletedLoopSegmentProgramV1` retains layout, entry, segment receipt,
completed operation receipts, and the value ledger. The recursive After module
preflights the entry edge and each R1 Jump/Predicate/OpenNestedLoop transfer,
emits them exactly once through canonical CFG/identity/PhiTxn, seals every
segment and root After block, and returns one neutral
`ReadyLoopAfterContinuationV1`.

The Callable `#[cfg(test)]` Tail adapter alone owns the profile-close receipt
and checks `7 = Pure4 + Read2 + Write1`; Tail, Completion, and DraftSeal remain
their existing owners. The focused canary
asserts exact segment coverage, a distinct root After, late-failure whole
session discard, and fresh-session reuse. No G0 physical allocation,
production selection, retry/fallback retirement, publication change, or broad
legacy deletion is claimed.

The source-backed Dynamic carrier P2 close does not yet produce this After
receipt. P2 seals only Header/Backedge and is deliberately discarded in its
tests. Reuse of recursive After is permitted only after a complete Dynamic
Recipe/full-program physical input covers the unchanged callable body. The
single-Tail completion canary is not a substitute for the method's inner and
final returns; those must merge into one function exit before the sole
Completion claim and DraftSeal Return.

## Common Predicate/carrier I0 (2026-08-08)

`LOOP-COMMON-PREDICATE-CARRIER-I0-R0` is closed. The neutral After receipt
contains only common owner/root-After/predecessor facts; each Predicate edge
uses its own verified Bool operation receipt and source segment. Callable's
coverage and condition-key checks stay in the profile wrapper.

`DerivedCarrierEntry` is represented by the full-program
`PreparedLoopDerivedCarrierSeedRowV1` and the private `CarrierSeed` emitter.
It delegates to canonical identity `read_entry_receipt` and never fabricates
an expression site, re-resolves a name, or creates a G0-specific SSA owner.
The focused Callable suite is 25/25 and the source files touched by this row
remain below 800 lines. The next row is the test-only G0 I1 canary; production
selection, retry/fallback retirement, and legacy deletion remain closed.

## Generic G0 I1 caller-zero canary (2026-08-08)

`LOOP-CALLER-ZERO-PARITY-G0-I1-R0` is closed as a `cfg(test)` profile
harness. The exact compiler-side G0 ingress is split once into the complete
common operation program and the profile-specific Tail. The harness honors
the resolver's instance receiver contract, publishes receiver/parameters
through canonical identity, allocates the five R1 segments plus root After,
and dispatches all fifteen rows exactly once. Item 3 uses the common
`CarrierSeed` emitter and canonical `read_entry_receipt`; item 4 is structural
nested control and emits no operation.

Each Predicate transfer consumes its own completed Bool receipt, so root and
child conditions cannot silently share one value. The G0 post-loop `b1` Tail
read is canonical, exact I64 Completion is claimed once, and the existing
`finish_for_draft_seal`/DraftSeal path is reached. A late duplicate after
earlier emission discards the whole unpublished session, and a fresh session
replays the same semantic shape. The carrier leaf uses the shared provisional
type publication contract for unsealed PHI values; it owns no type, SSA, CFG,
Tail, selector, retry, fallback, collector, or publication authority.

This is caller-zero evidence only. M8/M9, production selection, M10b/M11/M12,
and broad legacy retirement remain closed. The next row is the design-only
top-down audit `LOOP-CALLER-ZERO-PARITY-G0-POST-I1-AUDIT-D0`.

## Common V2 S6C TextEq operand issuer I0 (2026-08-18)

`CommonV2CanonicalSessionRefV1::with_s6c_text_eq_operands` is the first
physical consumer of the S6C inner TextEq prefix. It validates the retained
source/Recipe rows for `ReadBinding(index) -> V6`, `ConstI64(1) -> V7`,
`Add(V6,V7) -> V8`, and the following `Substring(V0,V6,V8) -> V9` relation,
then resolves exactly one Body segment from the same session brand. The
canonical identity/SSA owner reads V6, and the canonical session issues and
publishes V7/V8 as integers. The non-Clone receipt is callback-scoped and
retains the mutable session borrow, so the three physical values cannot be
detached and re-paired.

The one-shot ledger poisons duplicate entry before the first body instruction;
owner, body-loop/layout, source-shape, segment-brand, and canonical-read
drift reject without opening the later Substring effect. Focused tests cover
positive V6/V7/V8 emission, duplicate rejection, missing-seed rejection before
Const/Add, and whole-session late discard (4/4). This I0 does not issue V9,
runtime text residence/lease, TextEq/Bool, branch/Return/CFG/PHI,
Completion/DraftSeal, fallback, retry, publication, or a production selector.
The `nyash.string.eq_hh` export remains a generic raw-i64 transport and is not
an authority for this S6C lane.

## Common V2 S6C Substring callout admission I0 (2026-08-18)

The common physical-entry seam now transports the real collector
`ModuleInvocationBrandV1` through `InvocationBranded<PreparedPhysicalEntrySessionInputV1>`
into one canonical session. That session can issue one effect-free,
physical-ID-free Substring admission containing the landed source-backed target plan,
checked provider facts, one neutral single-site plan, and an opaque
`CommonV2SubstringEndObligationV1`. The session owns duplicate rejection and the
unpublished-function discard boundary; the invocation brand is the only plan stamp.

This I0 emits no `CheckedCallOut`, V9 `ValueId`, lease/token, text residence,
TextEq/Bool, CFG/PHI, Completion/DraftSeal, publication, fallback, retry, or
production call. `nyash.string.eq_hh`, selected-Dynamic I6/I7, raw handles, and
owner/provider IDs remain non-authority. Focused positive/foreign-brand and
callback-scoped lifecycle tests cover the transport seam; the next design stop is
the checked V9 materializer/residence owner.

## Common V2 S6C Substring V9 issuer I0 (2026-08-18)

`with_s6c_substring_v9_issuer` consumes that admission together with the
same-cohort V6/V7/V8 operand receipt and Body segment. It validates the
`Normal + HostHandle + EndAuthorized` wire, adopts the paired handle/token only
when the host generation is live text, and lends the text only through a
callback-scoped view. The materialization owns the one-shot finish; explicit
finish and `Drop` both release the End lease, so a callback error cannot leak a
runtime result or leave a partial session claim.

This is a caller-zero lifecycle seam only. It emits no `CheckedCallOut`, V9
`ValueId`, TextEq/Bool, CFG/PHI, Return, publication, fallback, retry, or
production call. Foreign/stale/mismatched leases, non-normal or immediate
wire results, duplicate body rows, and owner/segment/plan drift reject before
the next physical effect. The next design stop is the V9+ExactText V1
residence co-seal.

## Common V2 structure R0 (2026-08-18)

`CommonV2CanonicalSessionRefV1` remains the single session owner. Its
length-specific projections are implemented in the private
`common_v2_session_length.rs` child, while segment/condition/continuation
projections are in `common_v2_session_segments.rs`; both extend the same
parent type and cannot create a sibling session or semantic receipt. The
facade is now well below the 800-line source ceiling, and this refactor emits
no new MIR, runtime wire, TextEq, CFG, publication, fallback, or retry.

## Common V2 S6C canonical Substring callout materializer I0 (2026-08-18)

`S6CTextEqOperandReceiptV1::with_s6c_substring_callout_exact_text_co_seal` is
the compiler-side V9 lifecycle consumer. It co-seals the source-backed
Substring target, the checked single-site plan, the V6/V8 operand receipt, the
Body segment, and the existing source TextEq occurrence/ExactText sidecar in
the same canonical session before any callout mutation. The private
`common_v2_s6c_substring_callout_materializer.rs` child then installs that plan
and delegates all MIR writes to the existing canonical CFG/SSA writers:

```text
CheckedCallOut
  -> separate terminal Fault
  -> NormalResult(V9)
  -> callback-scoped V9 + ExactText occurrence consumer
  -> EndAuthorized End
```

The materializer accepts no runtime wire, handle, lease token, raw slot,
generation pair, or `eq_hh` transport. Its callback receives only the opaque
co-seal product; physical sidecar values and the runtime pair remain hidden. A
late callback/terminal failure is discarded by the surrounding unpublished-
function transaction; the same session cannot retry. Focused evidence covers
one Normal/Fault/End lifecycle, the exact source-left/V9 and If-condition
relation, and End absence until the co-sealed consumer callback returns.
TextEq V10, ExactText residence, inner CFG/Return, Completion/DraftSeal,
publication, fallback, retry, production selection, and `eq_hh` retirement
remain closed.

## Common V2 portable TextEq V10 design stop (2026-08-18)

The source `LoopOperationV2::TextEq(V9,V1 -> V10)` is verified and
non-faulting. The only accepted physical choice is
`Direct-or-RejectBeforeEffect` through the private
`CommonV2S6CPortableTextEqBoolCapabilityV1`. Its source chain is
`Equal(Text,Text) -> Bool` plus the existing S6C Facts/Recipe/co-seal; the
canonical SSA session alone may issue the Bool `ValueId`. Raw handle identity,
integer `MirInstruction::Compare`, `StringBox::equals`, C status/wire rows,
and `nyash.string.eq_hh` remain non-authority.

The content-view audit found a missing lifetime domain: the existing pinned C
residence frame contains ExactText rows, but S6C TextEq's lhs is the V9
Substring result. Treating V9 as an ExactText root would compare the wrong live
operand or go out of range. The worker-selected route keeps the ExactText
frame/ABI and V9 End owner separate, and adds one private V9 backend view
co-sealed with the existing `PinnedTextBackendFrameBorrowV1` and plan/frame
stamps. The current row is therefore
`COMMON-V2-S6C-PORTABLE-TEXTEQ-V9-VIEW-D0` in `design_stop`.
`with_text(&str)` and raw `ptr/len` callbacks remain canaries only; no pointer
may escape to MIR/compiler/JSON. Bool V10, If/Return CFG, publication,
production selection, performance promotion, fallback/retry, and `eq_hh`
retirement remain closed until the canonical V9 projection issuer and
rejected cases are named.

## TextRef residence scope D0 (2026-08-18; accepted)

The next bounded owner is the private
`CommonV2S6CTextEqResidenceScopeV1`. It mechanically consumes the existing
V9/ExactText occurrence co-seal and exposes one callback-scoped opaque view:

```text
co-sealed V9 End obligation + ExactText entry residence
  -> one scope consumer ([V9Ref, ExactTextRootsRef])
  -> ExactText finish
  -> canonical V9 End consume
```

The scope does not reissue source meaning, Recipe facts, runtime wires,
handles, slot/generation lanes, or `ValueId`s. ExactText roots are
invocation-scoped; V9 is occurrence-scoped and is never placed in the formal
root array. One consumer and one finish owner are enforced by move-only
callback boundaries. Primary/suppressed error precedence and late outer
transaction discard are part of the contract; implicit Drop cleanup,
fallback, retry, V10, CFG/Return, publication, and production remain closed.

The source ExactText occurrence cohort is now bound to published runtime
lanes through the explicit entry bridge. The runtime-private scope I0 is a
caller-zero lifecycle canary; it does not import Residence into MIR or permit
post-hoc pairing of two runtime owners.

## Common V2 source-bound V9 runtime producer D0 (2026-08-18; accepted)

The runtime boundary is one private Rust bridge immediately after the fixed
`hako.text.scan.substring.v1` provider call. MIR keeps the canonical
`CheckedCallOut -> NormalResult(V9) -> End` lifecycle and never imports
`TextFormalCallResidenceV1`, raw handles, lease tokens, or a runtime side
table.

The bridge consumes one backend-private
`SourceBoundV9RuntimeProducerPlanV1` and returns one move-only
`SourceBoundV9RuntimeResultV1`. The result is the sole `EndAuthorizedTextV1`
adopter and lends only a callback-scoped
`SourceBoundV9RuntimeInputRefV1<'_>` to the future TextRef scope. It has no
`into_parts`, handle/token accessors, lookup, clone, retry, or fallback.

Before the provider call, the plan proves the source item/block/result,
cohort/owner/session/segment, fixed provider ABI/arity, `READ` effect,
EndAuthorized shape, lease slot, and canonical End census. Before publishing
the result owner, the bridge validates the complete normal wire, matching live
generation and exact Text. Fault/Suspended/ImmediateI64/Forwarded/stale/
foreign/non-Text/malformed output rejects before V9 exposure. The fixed Rust
provider writes a complete normal/fault wire atomically; the bridge never
guesses or cleans a foreign token.

Normal cleanup is `scope consumer -> ExactText residence.finish -> canonical
End -> result.finish_at_canonical_end`. Fault has no owner; late failures are
terminal and the unpublished-function transaction remains the compiler
rollback boundary. The existing `issue_s6c_substring_v9_from_wire_v1` is a
caller-zero canary only and is not the final issuer. TextRef scope, TextEq V10,
CFG/Return, publication, production, direct leaf, C-speed, fallback/retry,
and `eq_hh` retirement remain closed until their own rows.

## Common V2 source-bound V9 runtime producer I0 closeout (2026-08-18)

The caller-zero canary is implemented in `runtime::source_bound_v9_runtime`.
One private bridge validates the fixed provider-return wire and is the only
`EndAuthorizedTextV1::adopt` caller on this path. Its move-only result exposes
only callback-scoped text input plus explicit finish/abort; no raw handle,
lease token, MIR value, residence, side table, fallback, or retry escapes.

Evidence: `cargo fmt --all`; `CARGO_BUILD_JOBS=4 cargo test --profile quick
--lib source_bound_v9_runtime` (7 passed / 0 failed); the existing Dynamic
lease suite (7 passed / 0 failed); the existing S6C issuer suite (3 passed /
0 failed); `CARGO_BUILD_JOBS=4 cargo check --profile quick`; the current-state,
S6C structure, and StringBox mutable-reachability guards; and `git diff
--check`. The first filtered command that reported zero tests was discarded;
the issuer tests were rerun by their exact discovered names. Warnings remain
baseline-only.

## Common V2 TextRef entry bridge D0/I0 (2026-08-18)

The accepted bridge is an index-only private physical plan. Its occurrence
entry point consumes the existing S6C occurrence/co-seal and
`PhysicalTextEntryLaneSidecarV1`, checks owner/entry/needle-row identity before
delegating to the sidecar issuer, records the exact slot/generation lane
indices plus an explicit root-index bijection, and never stores concrete
`{slot,generation}` values, handles, tokens, or `ValueId`s. The move-only plan
is consumed once; its callback lends the root-index-ordered already-published
pair vector to the future runtime Residence adapter without allowing ordinal
rematching.

`common_v2_s6c_textref_entry_bridge.rs` is caller-zero physical evidence only.
Its focused suite covers positive one- and multi-root ordering, non-monotonic
source ordinals, foreign bindings, non-adjacent lanes, short/zero runtime
lanes, and one-shot callback construction. The existing runtime Residence
owner remains the sole pin/root/finish authority; TextRef scope, V10, CFG,
Return, publication, production, fallback/retry, direct C-speed lowering, and
`eq_hh` retirement remain closed.

## Common V2 TextRef scope I0 (2026-08-18)

The runtime-private `TextEqResidenceScopeV1` is a one-shot move-only owner for
the existing source-bound V9 result and one invocation Text residence. Its
callback lends only an opaque V9 text view plus occurrence-ordered root view;
it emits no MIR and performs no TextEq comparison. Callback completion or
failure is followed by ExactText `finish`, then canonical V9 End finish. A
callback error is primary and cleanup failures are retained as suppressed
evidence; there is no implicit Drop cleanup, retry, fallback, or second
consumer.

Evidence: `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
text_eq_residence_scope` (2 passed / 0 failed), `cargo fmt --all`, and
`git diff --check`. The focused suite covers successful dual-ref lending,
root-order visibility, callback-primary failure, and exactly-once cleanup.
This is caller-zero runtime evidence only; V10, CFG/Return, publication,
production, direct C-speed lowering, and `eq_hh` retirement remain closed.

## Common V2 S6C base-root admission I0 (2026-08-18)

`issue_common_v2_s6c_text_content_root_admission_v1` is the sole compiler-side
co-seal between the callback-scoped S6C source corridor and the existing
ExactText entry bridge. It consumes the bridge plan and proves exactly two
role-labelled rows: root 0 is the source Subject and root 1 is the source
Needle, using `BindingRefV1`, logical ordinal, and the explicit published-pair
root-index bijection. Numeric lane order is not semantic authority.

The move-only admission lends the source view and rows together once. It stores
no `ValueId`, pointer, runtime frame, lease, generation value, V9 root, or MIR
effect. Missing/duplicate/foreign/reordered/non-U64/duplicate-root inputs
reject before effect. Focused evidence is green (`3/3`); this is caller-zero
compiler evidence only. The cursor/preheader plan below consumes this
admission; V10, CFG/SSA, publication, production, fallback/retry, direct
C-speed, and `eq_hh` retirement remain closed.

The fixed provider remains a canary and has no production caller.

## Common V2 S6C cursor/preheader plan I0 (2026-08-18)

`issue_common_v2_s6c_text_cursor_preheader_v1` is the sole consumer of the
Subject/Needle base-root admission for this effect-free row. It records two
preheader root-pair load obligations, CP index `0`, byte offset `0`, and the
existing source relation needed by the next scalar-equality leaf. The plan is
move-only and lends the source cohort, roots, cursor state, and relation
together through one callback.

This row emits no `ValueId`, `MirInstruction`, `PinnedTextOp`, access-plan row,
pointer, runtime frame, lease, generation value, V9 root, CFG block, or
production behavior. Focused evidence is green (`2/2`), `cargo check
--profile quick` is green, and the S6C structure guard enforces the physical
free boundary. The next selected row is the strict
`Utf8WidthAt`/`Utf8ScalarSliceEqWholeText` leaf; CFG/PHI/Return, runtime,
fallback/retry, performance, and `eq_hh` retirement remain closed.

## Common V2 S6C scalar-equality leaf capability I0 (2026-08-18)

`CommonV2S6CTextScalarEqualityLeafCapabilityV1` is the next narrow,
effect-free consumer of the one-shot cursor/preheader plan. The canonical
session consumes that plan exactly once and co-seals the existing Subject and
Needle root roles with the two backend-neutral leaf shapes:
`Utf8WidthAt` and `Utf8ScalarSliceEqWholeText`. V9 remains a derived tuple
(`SubjectRoot + byte_offset + scalar_width`); it is never registered as a root
or materialized as a runtime value.

The capability is callback-scoped and physical-ID-free. It stores no
`ValueId`, `MirInstruction`, `PinnedTextOp`, access-plan row, pointer, lease,
generation, frame, Bool result, CFG block, PHI, Return, or fallback route.
The current canonical session is the only consumer; late failure remains
owned by the outer unpublished-function transaction. Duplicate issue,
foreign session/entry, missing physical stamp, swapped roots, zero plan
stamp, and cursor drift reject before any effect.

Focused evidence is green: `cargo test --profile quick --lib
scalar_equality_leaf -- --nocapture` (2 passed / 0 failed),
`cargo check --profile quick`, formatter, diff, and the S6C structure guard.
The typed caller-zero cursor CFG/SSA integration now consumes this capability;
residence exit/finish, Return/Completion ownership, publication, production,
performance, fallback/retry, and `eq_hh` retirement remain closed.

## Common V2 S6C cursor CFG/SSA integration I0 (2026-08-18; caller-zero landed)

`materialize_common_v2_s6c_cursor_cfg_v1` is the only caller-zero shape that
consumes the scalar-equality leaf receipt. It stays inside the canonical
session and checks the same owner, physical-entry stamp, source segment,
TextEq/Return-read co-seal, and exact `Utf8WidthAt`/
`Utf8ScalarSliceEqWholeText` shapes before emitting byte-offset and CP-index
PHIs, the V10 inner branch, width/CP updates, and the existing backedge.
The outer V5 predicate remains distinct and may not be reused as V10.

`CanonicalConditionBoolResultReceiptV1::consume_s6c_cursor_cfg` is now the
sole typed handoff. It lends the installed source corridor, issues the
canonical-session entry bridge, consumes the base-root/cursor/leaf products
exactly once, and passes V5 internally without exposing a re-pairable outer
`ValueId` to the fixture. Positive and late-failure fixtures are green; the
outer unpublished function transaction remains the only late-discard owner.

This closes the integrated-consumer design stop as caller-zero compiler
evidence only. Residence exit, normal-return finish, runtime frame,
publication, production edge, fallback/retry, performance promotion, and
`eq_hh` retirement remain closed. The next selected row is the bounded
exact-two Completion residence-exit obligation.
