# Borrowed I64 call-result source D0

Status: S0 landed / S1 verified; scoped landing pending
Execution row: MIRBUILDER-BORROWED-I64-DIRECT-CALL-RESULT-SOURCE-S1
Scope: required I64 result composition inside the existing borrowed-result owner;
no implementation until source-site/target/result/completion mapping is closed.
Related:
- mirbuilder-terminal-field-return-finished-projection-s0-2026-10-05.md
- mirbuilder-declared-object-tagged-borrow-physical-s1-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision brief

Source authority: original resolved function's verified return sites, binding and
sole initializer sites; already prepared LexicalInstanceCallSourceTargetV1 and
existing VerifiedCallableResultContractCohortV1 completion. Exact owner/site/key/
batch-slot/result correspondence is mandatory. Source/result facts precede MIR.
Canonical issuer: existing ordinary_new_borrowed_formal_result::source_result,
prepare_borrowed_i64_results_v1 and corroborate_borrowed_i64_result_v1; existing
lexical instance call issuer remains the only target/affine call authority.
No separate result registry, annotation-manufactured proof or MIR inference.

Production consumer: ordinary-new borrowed scalar call/result issuance and its
existing local/direct return continuation. Selected old responsibility is the
result classifier's blanket rejection of a proven call-result source as
borrowed-result/source-not-i64. Other unproven result forms remain fail-closed.
Initial bounded shape: exact lexical call directly returned, or a never-rebound
local whose sole initializer is that exact call. Join the original prepared
call target to the callee's independently proven I64 result and completion;
never treat a declared i64 annotation alone as a body proof.

Non-authority: receiver/selector names, caller constants, layout observations,
physical Call/Invoke result tags, a missing/empty loan, fallback, .hako rewriting.
Fail-fast boundary: missing/foreign/ambiguous call target or initializer, wrong
owner/site, rebind, unproven callee result, mixed nullable/scalar exit set,
incomplete completion or circular dependency without independent seed.

Initial construction question (closed by Integrated Decision below): the existing prepare phase precedes prefix
walk and completion corroboration. Map exact source dependencies without
requiring the caller's completed prefix as its own prerequisite. Close issuer
ordering, dependency/cycle handling and completion lending before implementing.
Read-only worker audit_cost_oct5 confirmed this owner/site/completion mapping;
primary adopts one Decision here. The first prospective S0 is the sole-local
call-result return; direct-call return and stored-child receiver remain subsequent
required obligations, not reduced goal scope. Dependency/issuer ordering is closed below for S0; direct-return/stored-child
requirements remain subsequent work.

## Evidence and prerequisite separation

S1 original source witness with local out=recv.read(p); return out stopped at
borrowed-result/source-not-i64; S1 card149 preserves that required obligation.
Current classifier accepts integer literal/exact I64 formal/guarded I64 field,
then null/new nullable; all other return sites reject. Prepared lexical targets
are available before borrowed result preparation (coseal_issue126-159).
Final corroboration checks the exact complete return-site set and declaration.

Real Heap.isLiveHandle returns me.small_page.isLiveHandle(handle) or medium_page;
these are stored-child direct receivers. Worker confirmed lexical source target
preparation admits only Lexical(Local binding), while source field provenance
currently proves local recv=me.field, not a direct structural field receiver.
That receiver target/borrowed actual mapping is an independent required
prerequisite; scalar result composition alone cannot complete this production
shape. Do not change .hako to manufacture the supported local receiver spelling.
No claim that this D0 or its first slice resolves the app's first terminal.

## Required next acceptance

- Existing source-issued lexical leaf I64 call, direct and sole-local return,
  annotated/unannotated caller where source completion proves I64; object/null
  incoming borrowed parameter and both optimization settings.
- Forwarding chains and reversed declaration order; dependency handling must
  preserve exact source rows. Self/mutual unsupported cycles remain rejected.
- Reject rebound/duplicate initializer, foreign owner/site/target, wrong result,
  nullable/scalar mixed exits, missing completion and annotation-only permission.
- Existing literal/formal/field/nullable results and final borrowed-use guards
  retain their acceptance. Execute issued JSON Normal/Fault cleanup when the
  resulting physical shape is already supported; preserve exact rejection otherwise.
- Separate stored-child target mapping, Provided ownership and comparison/use
  vocabulary from scalar composition. Keep independent final MIR/wire validation.
- Map focused tests, selected guard and owner README before selecting fast S0.

## Restart

Previous slice a47039d3fe landed/pushed: three production field-return validation
callers share the existing FinishedBindings checker; original-block-only read
lookup retired. Source27/entry21/boundary11/JSON29/borrowed-use17 and six C
Normal/Fault programs PASS. Package623/exact known3 and HEAD-identical root1351
scope-guard debt remain classified baseline, never whole-green evidence.
No Cargo live. Older C/Array/S2 WIP remains protected and scoped landing is owed.
Unchanged app last observed artifact-unowned-lifecycle-site, no EXE. Full finite
pipeline, production/retirement and acceptance are still open; parked lanes unchanged.

## Integrated Decision / S0

Worker review closed dependency/issuer ordering. Reuse incoming.source already
retained by PreparedBorrowedFormalIngress, so the 799-line co-seal issuer stays
untouched and needs no additional argument or preparation wrapper this slice.
S0 admits only a sole local initialized by an exact lexical call into a callee
already selected in the existing borrowed-result map. Direct returned call and
stored-child receivers remain later required obligations.

The existing result owner keeps private source candidates and original call
rows as dependencies. Ground only leaf proofs, then monotonically ground I64
callers whose every dependency is grounded I64; unresolved/Nullable/Err/cyclic
callees leave the original explicit pending error. A literal exit alongside a
recursive call is not a whole-function seed. No candidate permission escapes
preparation. Original return set remains mandatory.

Local proof requires matching owner/Local binding/declaration, one original
initializer, function-wide non-rebind, exact MethodCall receiver and ordered
argument sites, exactly one original incoming target. No alias chain in S0.
After all completion products exist, existing lexical issuance runs its original
callee corroboration for every source target before issuing any affine row.
Then check each retained dependency equals exactly one canonical prepared row
and its callee has source-I64, nonempty return set and corroborated completion;
propagate invalidation into the existing pending result errors until stable.
No unrelated pending result failure becomes a global freeze before its demand.

Production switch: prepare_borrowed_i64_results_v1/source_result lend composed
proofs to the existing prefix and call issuance; issue_lexical_instance_call_
dispositions consumes the all-owner prepass rather than per-edge order-sensitive
corroboration. Existing PrefixLocalFlow i64-call result / I64Scalar return and
cleanup emitters are reused. Blanket source-not-i64 remains for unproven forms.

Implementation paths: borrowed_formal_result.rs and private composition child,
lexical_instance_call.rs, private package positive/negative tests, published
source test child, selected scope guard and owner README row. Preserve snapshots
of mixed README/guard and parent modules before edits. All Rust owners stay <800.
Required focused positives: literal/exact field leaf, one local call return,
annotated/unannotated, object/null both opts, chain and reverse declaration order.
Negatives: rebind, wrong source target/site/ordinal/owner, no independent seed,
Bool/Nullable/Unit/missing completion and dependent invalidation. Retain prior
source27/entry21/boundary11/JSON29 and exact known3 package baseline classification.
Execute Normal/Fault issued programs where the existing physical lane admits.
Selected guard retains HEAD-identical root1351 debt and is not whole green.

## Construction / verification in progress

Previous goal turn: progress, validated production field-return switch and
retirement landed/pushed a47039d3fe; pointer selection5ac73e56c2. Entry this turn
current-first/pointer PASS, no Cargo/rustc live, protected WIP unchanged.
Worker confirmed original incoming.source is sufficient for borrowed-only S0;
no parent799 API growth, new wrapper or BoxShape slice is needed.
Private composition child now retains exact local/initializer/target dependencies,
grounds the existing result map and checks all completion dependencies before
affine lexical issuance. Existing local I64Scalar return emitter is reused.
Source witness covers two-hop chain, both declaration orders, annotated/unannotated,
object/null and both opts (16 inputs); package negatives cover unsupported return
sources, target/site/owner drift and callee invalidation propagation.
Cargo72275 terminal101: one compile setup error (receiver enum compared to ref),
fixed without changing authority. Cargo48667 terminal0: call_result 30/30 PASS, including source-issued two-hop
chain16 cases and initial source negatives. Log
`/tmp/hako-borrowed-call-result-S0-focused-corrected.log`.
Worker soundness review found no concrete defect; meaningful coverage additions
are mutual/unseeded literal self-cycle and final two-hop invalidation.
Private negative fixture now contains bridge+wrap and verifies both errors for
five target/site/owner/leaf-completion changes; source cases also cover Nullable,
mutual cycle and literal-exit recursive cycle. Cargo44325 terminal0: composition::tests 2/2 PASS, log `/tmp/hako-borrowed-call-result-S0-composition-negative.log`.
No Cargo remains live.
C33198 terminal0: all16 issued programs execute object5/null7, plus all four
object-case/three null-case birth-store Fault positions and exact cleanup counters.
`/tmp/hako-borrowed-call-result-S0-c-execution.log`; no C implementation change.
Owner README and guard pins updated; selected guard is799 lines, new source
owners below800. Before snapshots at `/tmp/hako-borrowed-call-result-S0-before/`
protect mixed README/guard differences. No code commit or acceptance claim yet.

## S0 closeout

Source28/28, entry21/21, boundary11/11, full JSON29/29 PASS on the final test build.
Private composition2/2 PASS with mutual/self/literal recursive sources, Bool/
Null/Unit/Nullable callee rejection, rebind, target-slot/argument/owner drift,
missing leaf result and wrong leaf return-site propagation through both callers.
The first call_result run30/30 and final full source run cover actual chain16
cases in both orders/annotations/domains/opts; source output fixtures remain .hako
unchanged. C16 Normal/Fault PASS on the issued chain programs. No new emitter,
physical representation, C implementation or source/result registry.
Package625 PASS/exact known3 unchanged boundaries: ReceiverNonEscape Capture,
IncompleteOrdinaryNewCoverage, MapLifecycleUndertaking BorrowedEntryEscape.
Guard57139 terminal1 is HEAD-identical root1351 debt, sha256
5fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467; selected pins
pass before that debt. Whole package/guard remain red, not full-green evidence.

Original source candidates are private until grounded in the same prepared map.
Existing lexical issuer now corroborates all result completions/dependencies
before its affine loop; its production per-edge/order-sensitive corroboration
call is removed. Unrelated pending errors remain pending until selected demand.
Owner README documents this meaning; selected owners/result621/composition167/
tests139/source49/source-parent631/lexical388/guard799 all below800.
Before snapshots isolate five tracked selected paths from protected WIP; new
private/source/C tests are selected additions. No app EXE or post-S0 CLI probe
claimed. Direct lexical returned call, direct stored-child receiver source target,
Provided ownership and pending C/Array/S2 landing remain mandatory later work.
Full finite pipeline goal is active. Scoped S0 landed/pushed ee60c70fb7; remote
branch hash verified equal. Existing protected C/Array/S2 WIP remains unstaged.

## Next Decision / direct lexical call S1

Read-only worker confirmed no missing authority for this bounded shape.
Original returned MethodCall value site supplies the call site directly; local
initializer path keeps its own declaration/non-rebind obligations. Both paths
share the same exact-call join to retained incoming.source. Callee must already
be a grounded I64 in the S0 result map; all-owner completion/dependency prepass
and affine lexical issuance remain unchanged. Rename the private helper to
reflect both source placements, without a parallel direct-call proof product.

Production consumers: source_result and its same prepared map -> original
issue_borrowed_i64_terminal_call/TerminalI64CallReturn, final ready closure,
borrowed_terminal_arguments_v1 and existing terminal-call emitter. Original
terminal owner requires Return(MethodCall), lexical receiver, source-sealed
arguments and BorrowedActual; no empty loan or literal-only nonborrowed widening.
Replaced edge: blanket source-not-i64 for this exact direct-return placement.
Stored-child direct receiver targets, ownership/Provided and other vocabulary
remain separate required obligations. No facade799/emitter/C/ABI change.

S1 acceptance: leaf/caller annotated and unannotated, null/object, direct
2-hop chain, both declaration orders and optimization settings; source->final
MIR->issued JSON->C Normal/Fault exact results/cleanup. Preserve local chain16,
existing result/entry/physical source families and rejection of foreign owner/
site/target, nonborrowed/non-I64 callee, missing completion and ungrounded cycles.
Guard pins reuse the same result owner/dependency tests and remain below800;
root1351 baseline debt stays classified, never whole-green evidence.
Before S1 edits, snapshot selected source/helper/tests/README/guard for scoped
staging. No Cargo live. Build/CI/app/goal completion is not inferred from S0.

## S1 construction / current evidence

Previous conversational prompt-writing turn made no authoritative goal progress.
Current-first and pointer PASS; no preexisting Cargo/rustc process at entry.
S1 before snapshots at `/tmp/hako-borrowed-direct-call-result-S1-before/` protect
selected files and mixed owner README. Original MethodCall return sites and
sole-local initializer sites now select one shared exact incoming call join;
existing grounded result map, completion prepass, terminal issuer and C ABI
remain unchanged. No .hako rewrite or stored-child receiver admission.
Source direct two-hop chain16 covers leaf+caller annotated/unannotated, null/
object, both orders and opts; original local chain16 retained. Private direct
source negatives cover Bool/Null/Unit/Nullable and self/mutual/literal-exit cycles.
C harness accepts an explicit input stem to run both original issued families
through the same Normal/Fault counters. Worker read-only soundness review
requested; Cargo41174 focused `call_result` is live, log
`/tmp/hako-borrowed-direct-call-result-S1-focused.log`; results pending.
No acceptance or commit claimed; protected C/Array/S2 WIP remains intact.

Cargo41174 terminal101: focused32 has31 PASS/one new direct source failure at
object/reversefalse/annotatedfalse/optfalse,
`ordinary-new/local-commit/physical-boundary/finished-sequence`. This is a
selected new failure, not baseline; S1 remains incomplete. Source local chain16
and private direct non-I64/cycle tests PASS. Worker found no source mapping
soundness defect; nonborrowed callee fence now added as an explicit negative.
Next build Cargo40902 retains focused call_result and adds exact boundary
sequence diagnostic context (function/block/expected/actual) to locate the
existing terminal finishing mismatch, log
`/tmp/hako-borrowed-direct-call-result-S1-diagnostic.log`. Physical boundary
owner added to scope solely to diagnose/resolve this selected finished
correspondence; independent final validation is retained. No Rust edits while
Cargo live. Guard54029 terminal1 matches unchanged HEAD root1351; pointer,
whitespace and C-harness syntax PASS. No slice commit or acceptance claim.

Cargo40902 terminal101: focused33 has32 PASS/new direct source failure.
Nonborrowed callee annotation negative PASS with exact source-not-i64.
Cargo94919 terminal101: focused34 has32 PASS/two new direct failures.
New typed-borrow source unit shows annotated bridge owner slot3 has no retained
sole terminal relation; final block13 contains ordinary raw Method Call, not
evidence of duplicate lifecycle call emission. Post-RC receiver canonicalization
changes receiver4 to dominating root3, causing finished-sequence. Do not admit
that rewrite in the lifecycle physical checker to conceal a missing source
terminal selection. Worker confirmed source scan stages exact actuals before
terminal demand; a Homecoverage Err at terminal observer582–616 can erase a Call
relation. Cargo15825 now runs the selected source-unit diagnostic with original
completion/root-flow context, log
`/tmp/hako-borrowed-direct-call-result-S1-flow-diagnostic.log`. S1 incomplete,
no code commit or C/direct acceptance. No Rust changes while Cargo live.

Read-only next-prerequisite mapping (not execution selection): direct stored-child
receiver must reuse exact MethodCall receiver FieldAccess, lexical Receiver and
selected owner, source field-write class plus initialized Provider residence and
existing entry Home lifetime. Existing target/packet needs a closed receiver arm,
not a fabricated Local binding or second registry. Original owned residence tuple
is required before emitting borrowed child ObjectFieldGet; no child ownership or
teardown. C indexed child seeding currently uses same-function store/release;
ordinary published-parent reads need existing owned_object_residences tuple and
borrowed origin, with wrong slot/class/ownership and child-release negatives.
Array-get result in Page.isLiveHandle remains a distinct required prerequisite.
Current S1 remains selected; no parked lane, app source or ABI changed.

Integrated S1 correction Decision: worker located sole child transport filter
`coseal_helpers::retain_child_terminal_relation`87–109, applied at issuer686.
It drops every map-free Call even after the source terminal lender correctly
issued it. Preserve only Call rows with existing immutable Lexical argument arm,
whose sole issuer checks BorrowedActual before issuance (home_local_call_flow532).
Existing final consumers use the same discriminator. No new permission/product,
ordinary strict-call widening or physical correspondence change. Add negative
using original source-issued strict root Call vs borrowed Call at this filter;
existing typed child relation/all-exits-ready and direct chain remain positives.
PhysicalBoundary diagnostics restored exactly to the before snapshot: no checker
weakening or diagnostic change will land. Cargo15825 intentionally interrupted
(rc130) after this concrete finding superseded its diagnostic purpose; rustc2991754
confirmed absent before edits. This interruption is not a test PASS.

Cargo88718 terminal101: focused34 has33 PASS/direct publication now stops at
call-frame-drift. Typed borrowed child retains its Call and all exits ready;
old raw Call path is no longer the observed boundary. Helper filter test
separate invocation failed at strict-control row unwrap (strict argument7 is
not an admitted original terminal); corrected control to existing zero-argument
strict terminal. No test success claimed for that setup failure.
Worker integrated frame mapping: root_call_entry validation hardcodes RootOwned
for all source owners; canonical callable entry defaults Borrowed and only exact
AppMain identity selects RootOwned. Existing root_completion.owner is sufficient
role authority. Select expected frame mode from that same owner identity, keep
Invoke.dst correspondence and all original binding checks. No new frame receipt,
ABI or emitter. Original-source root packet test has healthy control, then
changes both recorded frame and physical frame to Borrowed and requires exact
call-frame-drift; source direct child chain provides Borrowed positive.
Selected root_call_entry validation/tests added to scope with before snapshots.

Frame-check test fixture uses a full function binding inventory; exit cleanup
must exclude its entry frame/jump. Corrected that test inventory before requiring
the healthy control. Cargo70281 intentionally interrupted130 at setup review,
rustc3001712 confirmed absent before this edit; not PASS. Final corrected build
records source direct flow, filter transport and original owner-bound frame
role together. No PhysicalBoundary or emitter changes.

## S1 verified closeout

Final corrected Cargo85097 terminal0: call_result34/34 PASS, including direct
two-hop chain16 and retained original local chain16. Both leaf+caller annotation
on/off states, two declaration orders, null/object and both opts execute through
original source, final MIR and issued JSON. No claim of every independent
annotation permutation. Child filter source-issued borrowed/strict-zeroarg
control1/1 and original root frame-role coherent-row/MIR mutation1/1 PASS.
Source29/29, entry21/21, boundary11/11, JSON29/29 and composition5/5 PASS; logs
`/tmp/hako-borrowed-direct-call-result-S1-{source,entry,boundary,json,composition}.log`.
Final focused `/tmp/hako-borrowed-direct-call-result-S1-final-focused.log`;
filter/frame logs `S1-child-filter-final.log` and `S1-frame-role.log`.
C58301 terminal0: direct16 programs return object5/null7 with exact Normal cleanup
and every Birth-store Fault position; C47617 terminal0: original local16 same
Normal/Fault acceptance. Logs `S1-c-direct.log` / `S1-c-local.log` under same prefix.
No C implementation, ABI, emitter or PhysicalBoundary change.

Package630 PASS/exact known3 unchanged names/boundaries: ReceiverNonEscape
Capture, IncompleteOrdinaryNewCoverage, MapLifecycleUndertaking BorrowedEntryEscape.
Final guard5686 terminal1 remains HEAD-identical root1351, sha256
5fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467; never whole green.
Selected source/result621/composition175/private tests217/source98/helpers139/
frame-validation301/frame-tests479/guard799 are below800. rustfmt, whitespace
and pointer PASS. No active Cargo/rustc.

Production replacement: same exact incoming dependency join handles original
direct Return(MethodCall) and sole local initializer; map-free child completion
now transports the existing borrowed Lexical Call instead of dropping it; final
call checker uses original root owner for RootOwned/Borrowed rather than root-only
mode. All original site/result/argument/frame/cleanup checks remain. Scalar/
nonborrowed/foreign/cyclic sources retain named failure boundaries.
Scoped staged patch will use the S1 before snapshots; protected C/Array/S2 WIP
and all app .hako stay unchanged. No fresh app EXE or full-goal claim.
Direct stored-child receiver, Page Array-get I64 result, Provided/vocabulary
and protected C/Array/S2 landing remain required future work.
