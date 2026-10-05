# Stored-child borrowed call receiver D0

Status: Birth cleanup prerequisite verified / C receiver integration dependency design
Execution row: MIRBUILDER-STORED-CHILD-BORROWED-C-RECEIVER-INTEGRATION-D0
Scope: required original direct `me.<Provider field>.m(borrowed actual)` receiver
in the existing ordinary-new lexical call/result pipeline.
Related:
- mirbuilder-borrowed-i64-call-result-source-d0-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Current evidence

Direct lexical borrowed I64 return S1 landed/pushed `61d94d5cf2`; remote hash
confirmed equal. Original local and direct chain witnesses reach issued JSON
and C Normal/Fault; this does not prove the unchanged app or full finite goal.
Production HakoAllocHeap.isLiveHandle in page_heap_box.hako returns
`me.small_page.isLiveHandle(handle)` at250 and medium_page at254. The facade
also directly returns `me.heap.isLiveHandle(handle)`. Page.isLiveHandle at142
returns `me.block_used.get(handle.block_id)`, a separate Array-get result need.
No .hako rewriting is authorized to manufacture a supported local receiver.

At D0 entry, lexical source preparation accepted only Lexical(Local binding).
SourceTarget.receiver_binding and terminal/projection consumers independently
assume that spelling. Existing local field provenance joins exact FieldAccess,
lexical Me/Receiver, selected owner box and field-write class. Class agreement
alone is insufficient for an initialized owned child or its borrowed lifetime.

Current prepare_source_claims seals field_residences and provider-owned children
before prepare_lexical_source_targets_v1. Provider source residence is the sole
birth-side write; reassignments/non-birth/unattributed writes veto residence.
Thus the source residence product is available at preflight; late completion
is not needed to infer field class. Canonical child object inventory and entry
Home lifetime still require exact lending at final issuance/packet creation.
Read-only worker audit_cost_oct5 consulted for this closed receiver mapping;
primary will integrate one Decision here before selecting implementation.

## Decision brief

Decision: extend the existing source-target/disposition/packet with one closed
stored-child receiver arm; retain the lexical-local arm and exact site checks.
Do not fabricate a Local binding or establish a parallel target/result registry.
Source authority + canonical issuer: original resolved MethodCall.receiver_site,
exact FieldAccess and parent Receiver binding, selected instance owner, existing
field-write/residence issuer and canonical owned_field_children inventory;
existing lexical call issuer remains the only affine target authority.
Non-authority: selector names, annotation alone, MIR receiver/result tags,
empty loan, layout-only class inference, copied ownership or arbitrary fields.
Fail-fast boundary: wrong owner/site/field/class, uninitialized/weak/reassigned/
unattributed/Provided fields, nested receivers, absent entry lifetime, foreign
callee, nonborrowed/non-I64 result and ungrounded cycles.
Smallest next slice: original direct Provider field receiver target, borrowed
actual and terminal packet transport using the existing source-I64 dependency
proof. Exact enum/data joins and entry lending must be closed first.
Non-claims: receiver admission does not prove Page Array.get I64 result, C
ordinary borrowed-parent field projection, Provided ownership or full app EXE.

## Required bounded series and ordering

1. Source receiver mapping: immutable exact receiver arm in the existing target;
   source-result composition and borrowed terminal actuals consume it. Reuse
   original parent entry Home and retain the canonical field read in existing
   bindings/physical validation. Borrow the child; create no child lease or
   teardown. Source-positive witness uses an independently proven I64 leaf,
   so Array.get is not silently treated as solved.
2. Physical projection: existing C index currently proves child reads using
   same-function object_field_set/release siblings; ordinary borrowed parent
   methods lack these rows. Use the existing published owned_object_residences
   tuple (field ordinal, child object id) in the sole index owner, preserve
   borrowed origin -2, exact parent/slot/class identity and release prohibition.
   Keep Birth sibling authority until its callers are intentionally migrated.
3. Page Array.get source-I64 result: independent source/use/result authority;
   an i64 declaration or physical tag alone cannot prove the returned body.
4. Unchanged production probe and subsequent mandatory frontiers, preserving
   full finite pipeline/cutover/retirement acceptance and protected WIP landing.

Each implementation row is selected only after its own authority, consumer,
replacement and acceptance are fixed. Independent audit improvements and parked
lanes are not made mandatory by this card.

## Acceptance to close source mapping

Positive: exact original direct returned calls through two Provider fields of
one child class; declared and inferred source-proven I64 results; null/object
borrowed incoming actuals, declaration orders and both optimization settings.
Verify original source -> retained terminal -> final MIR -> issued JSON; execute
Normal/Fault after physical projection supports that exact shape. Retain local
and direct lexical regressions, entry and independent physical validation.

Negative: wrong receiver site/owner/field/class, absent/foreign residence,
reassigned/Provided/weak fields, absent parent entry Home, nested receivers,
foreign callee, wrong actual/result, unseeded cycles. Physical followup requires
wrong/nonowned slot, foreign target, forged root identity and borrowed-child
release rejection from a valid Normal/Fault base. No generic error or zero-test
filter counts as isolated evidence.

## Restart and preserved state

No Cargo/rustc live at current entry. Source code is not changed by D0 selection.
Protected C/Array/S2 WIP remains unstaged; no stash/reset or broad staging.
S1 package630/exact known3 and HEAD-identical root1351 scope-guard debt are
classified baseline, not whole-green evidence. Last unchanged app observation
is artifact-unowned-lifecycle-site; no fresh post-S1 probe or EXE claim.
Goal active; internal mapping work continues and is not a user blocker.

## Integrated phase-availability finding

Read-only worker confirmed an important distinction: prepare_source_claims
finishes source residences, but its provider seal may populate only a child's
own teardown inventory. The parent's field-child tuple is normally inserted
later by append_source_claims. Requiring map.get(parent) at early lexical target
preparation would introduce declaration/claim-position dependence.

Use the existing owned_field_children_of issuer on demand with the original
selected parent source and constructor definition, inserting only a consistent
result into the same owned_field_children map. Retain the exact admitted tuple
in the existing closed receiver arm. Do not permit a class-only intermediate
arm. Existing constructor semantic definition/destruction lookup supplies
canonical object identity without relying on a caller's later local claim.
Final affine issuance must still join the original parent entry Home; class
identity is not a runtime lease. Exact payload and packet consumer mapping are
pending worker review; implementation remains unselected.

## Canonical receiver payload Decision

Worker verified parent identity is available before claim walk through
instance_constructor_semantic::with_source_object_definition(original box row),
which uses object_for/same_source_as. Entry Home does not create this identity:
entry_receiver_box_proof joins selected owner/slot, original box and exact loan
receiver, supplying the parent binding/lifetime independently.

Use the existing source target's receiver field as a closed enum:
`Lexical { binding }` or
`StoredOwnedChild { parent_binding, parent_site, field, child }`.
Here field is the existing CanonicalFieldRefV1 (already carries parent object),
child is CanonicalObjectIdV1, parent_site is the original Me site, and the outer
receiver_site remains the original FieldAccess site. No redundant parent ID.
Issue the stored arm only after original Provider/declared class agreement,
unique nonweak field and exact admitted Object-child tuple are corroborated.
On-demand canonical tuple insertion uses the same map; later append_source_claims
retains its existing occupied-entry equality check. Source preparation ordering
therefore cannot become a proof source.

Remaining design task before fast selection: enumerate terminal/source/physical
packet consumers of receiver_binding and define their closed-arm handling,
including the original parent read plus recorded ObjectFieldGet correspondence.
No fabricated receiver read, alias local, child-owned Home or teardown is allowed.

## Closed prerequisite / CALL-ARGUMENT-SPLIT-T0

BoxShape landed at366632a676: original argument sealers moved to private
home_local_call_arguments, sole caller/issuer semantics unchanged. Parent659 /
child144. Detailed Decision and evidence are retained in that commit.
This closes only the source-size prerequisite, not stored receiver admission.

## Closed consumer mapping for subsequent semantic S0

Worker census closes the bounded semantic unit: original direct ReturnValue
whose value is a MethodCall on `me.<Provider field>`, with source-proven I64
callee and at least one original borrowed actual. Preparation restricts this
new arm to those original return sites. Local/nested/nullable/discard/loop
receiver admission remains separate; no annotation-only result authorization.

- result_composition consumes the existing target receiver enum, checking the
  original exact FieldAccess/Me sites for StoredOwnedChild instead of comparing
  against Lexical(Local). Incoming/use classifiers already clone the same target
  and classify exact argument sites; they need no second receiver inference.
- The extracted terminal argument sealer admits the closed structural spelling
  only when the existing selected borrowed callback supplies its argument rows.
  Existing strict and nested lexical gates stay lexical-only.
- lexical_return prepares a closed receiver projection: Lexical ExactLexicalRead
  or StoredChild { parent: ExactLexicalRead, read: recorded ObjectFieldGet }.
  Parent read is checked at original Me site/binding. The field read uses the
  source target's canonical field and its dst alone becomes the call receiver;
  record it in the same original source group's ordered bindings.
- PreparedLexicalCallProjection materialization checks this closed arm;
  validate_recorded includes its field producer and existing final binding/
  PhysicalBoundary checks validate exact emitted correspondence. No read registry.
  has_producer_at includes the field read where an emitted group owns it.
- Nonselected consumers (local/nested/nullable terminal, raw terminal and loop
  call port) explicitly require the Lexical arm. Never return parent_binding as
  receiver_binding for compatibility: parent lifetime is not a child-owned Home.
  Existing call/exit frame and caller cleanup remain; no child teardown or Birth
  ConstructionChildCall discharge is borrowed for an ordinary parent field.

S0 implementation scope after T0: source target/provenance and private stored
receiver source join; existing source-owned issuer access/on-demand map seal;
borrowed result composition; extracted arguments child; lexical receiver
projection and direct terminal emitter; explicit lexical-only access in current
consumers and fixtures; private/source acceptance tests, owner explanation and
selected guard. Any further required contract is recorded here before widening.
Source-target and child-field packet have one authority and one consumption path.

Worker also found a completion-selection prerequisite: minimal
`bridge(p) { return me.child.read(p) }` has no local construction or formal field
read. Current plain seed Completion at issue365 skips the verified Home walk;
the later actuals-only borrowed probe cannot retain its terminal relation.
Exclude only owners with an original selected StoredChild direct terminal target
from that seed path, using the same prepared target arm. Keep unrelated borrowed
calls and ordinary seed owners unchanged. The minimal no-local-new source must
be a positive; the production Heap's guarded reads must not hide this boundary.
The799-line co-seal facade cannot grow: expose the existing predicate through a
private owner/helper with a bounded call-site replacement or a separate required
BoxShape split if mapping requires more code. Never compress lines to fit.

## T0 verified closeout

366632a676: moved-body comparison and focused call_result34/source14/entry21/
lexical10/terminal19/resolved349/publication29/instance15 PASS. Logs remain
/tmp/hako-stored-child-call-argument-split-T0-*.log. Guard only reaches known
HEAD-identical root1351 debt; no package/C/app/full-goal green claim.
Pointer/whitespace PASS. See commit for full command evidence.

## Selected semantic S0

Authority and consumer mapping above are closed. Select bounded original direct
stored Provider receiver Return(MethodCall), source-proven borrowed I64 callee,
exact parent entry Home and canonical field/child tuple. Existing target and
result/terminal/physical packet own the new arm; no parallel implementation.
Replaced production rejection is the direct field receiver target/terminal
spelling in Heap/facade source. Local/nested/nullable/discard/loop arm admission,
C ordinary-parent projection and Array.get source result remain separate required
work. This split does not shrink production or full-goal acceptance.

Construction may begin under this same card's enum/source/packet/seed Decision.
Before edits snapshot the selected source/projection/builders and mixed README.
Keep issuer799 within the cap; a bounded source-claim request bundle/private
helper may replace existing arguments, but no compressed code or new semantic
receipt. Preserve exact returned-site, original entry binding, canonical field,
child class, argument order and all final independent correspondence checks.
Add minimum no-local-new source witness and two fields of the same child class;
pin explicit lexical-only handling for all nonselected consumers. Focused
positive/negative and final source->JSON acceptance close S0; C execution is
required after the separate physical projection owner closes its boundary.
No .hako rewrite, silent retry, child lease or copied Birth teardown.

## Required preflight split T1

T0 landed/pushed `366632a676`; remote hash confirmed equal. Previous goal turn
made concrete progress by moving the argument owner and validating its existing
production callers. Current-first/pointer PASS, no Cargo live at entry.

S0's source/entry/residence joins require extra context at lexical preparation
and completion selection. The co-seal facade is799 lines, so its preflight
block must move before semantic changes. Select BoxShape T1: move ordinary box
names, local candidate/class inventory, lexical targets, borrowed ingress and
source-I64 preparation together into the existing private lexical co-seal child. Preserve
all original calls, argument order, retained per-slot errors, declaration order
and all five output values; no new result/permission/registry. Source claims and
dynamic-slot selection stay before this block; the same declaration walk and
final ledger installation consume its outputs. Worker consultation requested.

Scope: co-seal facade, existing lexical child, owner README paragraph, selected
scope guard, this card and pointer. Snapshot the mixed facade/README before
changes. Acceptance: original moved-body comparison, source publication,
call_result, borrowed source/entry, lexical and resolved semantics families;
pointer/rustfmt/whitespace plus same classified root1351 scope debt. No new
mirror tests for code motion. Return to semantic S0 after T1 verification;
source740 is not grown, and source-owned issuer273 remains the required source
join owner for S0. No app or full finite completion is claimed by this split.

Worker review confirms all outputs are owned; preserve the five-tuple without
outer Result or lifetime wrapper. Reuse existing issue_lexical444 rather than
adding a new child solely for this block. names -> candidates -> class map ->
targets -> ingress -> result order and nested errors remain unchanged.

T1 construction: snapshots at
`/tmp/hako-stored-child-call-preflight-split-T1-before/` preserve mixed facade
and README. Source call preflight now lives in existing lexical child; no new
file/product. Statement-token comparison with the old block PASS after accounting
for one extra module level, helper reference parameters and local mutability
remaining on the parent's candidate map. All preparation calls/ordering and
retained nested errors are identical. Parent771/lexical522 remain below800.
Initial guard run stopped at a stale parent-location pin; moved that pin to the
actual helper while retaining the facade's sole-call pin. Corrected guard72588
terminal1 is the same root1351 baseline debt; new selected pins pass first.
Cargo83277 runs call_result, log
`/tmp/hako-stored-child-call-preflight-split-T1-call-result.log`; pending until
terminal evidence. No semantic receiver admission or slice completion yet.

## T1 verified closeout / semantic S0 resumed

353bf1d2fd: preflight-only BoxShape extraction, same five outputs, source claim
order, retained errors and sole ledger consumer; parent771. call_result34,
publication29/source14/entry21/instance15/resolved349 PASS. Known root1351 guard
only, pointer/whitespace PASS; no C/app/full-goal claim. Detailed original
Decision and evidence in commit and /tmp/hako-stored-child-call-preflight-split-T1-*.log.
T0/T1 are closed; required stored receiver/profile semantics and their final
restoration evidence are retained below. No further refactor selected.

## Closed receiver construction history / 7fdec38b6e

Original S0 construction, emitted packet/projection mapping, class-view failures
and isolated build corrections are preserved in7fdec38b6e. Current sealed source
contract remains the receiver payload and integrated PROFILE-D0 Decision below;
final source closeout evidence is retained at the end. Historical failed builds
and unselected candidate states do not select work or count as acceptance.

## Regression correction / PROFILE-D0

Previous goal turn progress: original packet/census mapping and identity/final
mutation evidence landed in WIP. Current-first/pointer and live Cargo84185 handle
revalidated, then84185 terminal0. Sequential regression process98481 terminal0:
call_result35 PASS, publication30 PASS, borrowed_formal_entry17 PASS,
resolved_semantics349 PASS, physical_boundary11 PASS. lexical_instance_call133/1
and package627/8 are not green. Exact prior known3 persist; NEW5 are the old
field_receiver_callee_keeps_dominated_view_sites_outside_transport test and four
page_heap fixture census/forward/composition/observation tests. Do not label them
baseline or close S0. Logs /tmp/hako-stored-child-receiver-S0-regression-*.log.

Worker traced the correction boundary: stored source targets change the calls
map before borrowed-formal graph closure and strict incoming census. This can
induce Page.resizeInPlace transport and then require its unsupported local field
receiver incoming. A result filter only at has_stored_terminal is too late.
Outer.run's check(9) has no caller formal; selected forwarded borrowed formal
identity must not be replaced by the home sealer's broad literal BorrowedActual.
Source-result guarded field proof itself requires object views from the incoming
actual fixpoint, so annotation-only or candidate->ingress->filter->retry is invalid.

Decision: source-profile classification must close before transport admission,
using existing source/result/actual owners. Do not broaden local/nested/nullable
receivers to repair these regressions. Non-authority: result annotation alone,
known-nullable skip alone, a second profile/registry, failed-ingress retry, or
changes to old fixtures that conceal new source admission. Exact mapping is
under read-only worker audit; select MIRBUILDER-STORED-CHILD-BORROWED-CALL-PROFILE-D0
on this same card, work_mode design_stop, next_execution none. This pauses Rust
construction while resolving the internal dependency; goal remains active and
no user decision/blocker is claimed. The healthy stored32 + negatives remain
required, together with restoring the old package acceptance and new unsupported
source controls. Separate C/Array/app/full pipeline obligations remain intact.

No Cargo/rustc live after98481. Selected rustfmt hunks applied only where they
intersect S0 edits (lexical callback/predicate, getter fixtures, field census,
new final negative); pre-existing import/closure/other WIP formatting retained.
New test owners remain below800; no source semantic change made during this audit.

## Integrated PROFILE-D0 Decision / pending source obligations

Worker audit closes the phase ordering. Use the existing result owner for one
closed Pending -> SourceSealed transition. Pending classifies original return
facts but lends no executable result, borrowed ABI, class view or child lifetime.
The existing source-target vector and owner-indexed result map remain the only
staged storage; no additional retained registry or ingress retry is introduced.

Source authority + canonical issuer: original use drafts and origins, exact
caller formal -> callee borrowed ordinal/argument site, source contract
DeclaredObject constraint, original verified return/use sites, canonical field
lender and the existing result dependency fold. classify_actual_seed already
lends the original caller draft origin + DeclaredObject class; the declaration
lender nullable_result_integer_field verifies canonical nonweak declared I64
storage without issuing an incoming object view. A callee's opaque formal may
therefore carry a conditional requirement from a declared caller formal; do not
permanently require callee DeclaredObject annotations. Opaque literal/exact-I64
results need no field-view obligation. Result annotations never choose the lane.

Pending field payload is private to the same result owner: exact formal,
original return/read site, original FieldReadOperand/non-null-use identity,
required class and canonical I64 field, plus the class-lending forwarded
formal/ordinal/site identity. Original call dependencies retain their canonical
selected key/callee owner and exact sites; prospective stored needs must not
pretend to have an owned receiver lease. The sole target issuer still joins its
original Provider residence/inventory/entry proof after eligibility, before
publishing the complete StoredOwnedChild source target.

Finite ordering:
1. Collect original borrowed use drafts/origins and return facts once, before
   transport profile closure. Preserve dominated-view rows outside transport.
2. Ground conditional I64 caller/callee return facts with the existing result
   dependency fold. Exact forwarded original borrowed formal is required for
   each stored candidate. No unseeded cycle or annotation-only seed.
3. Promote only those bounded stored rows, then lend the canonical receiver
   tuple through the existing source-owned issuer and same map. Eligibility
   precedes destruction/inventory mutation for unrelated/non-I64 candidates.
   Existing lexical source rows keep their original admission behavior.
4. Perform the existing profile closure, strict incoming census and all-incoming
   object-view fixpoint once. Selected errors stay errors: never erase the row
   and retry ingress or the old ABI.
5. Seal pending obligations against exact formal view.class/object, canonical
   field and original admitted use identity. Move only SourceSealed I64 results
   into the existing final result map. Check each dependency against the ready
   source target; no return AST reclassification after ingress.
6. Force terminal verification only for promoted stored target + SourceSealed
   I64 caller. Existing completion/result/actual/entry/physical owners remain
   mandatory and may not consume Pending evidence.

Consumer/delete-set: existing borrowed source owner separates draft collection
from its one profile/incoming finish; result owner owns conditional field and
call obligations/grounding/seal; lexical preflight orders these same products;
source-owned issuer checks eligibility before tuple seal; completion predicate
uses promoted/SourceSealed membership. Remove unconditional stored promotion,
ingress-after-the-fact result classification and candidate-exists force-walk
responsibilities in the same bounded correction. Keep independent final MIR and
external ABI validators, old lexical callers, and all canonical actual checks.

Acceptance: restore the NEW5 regressions without changing their production
fixtures or claims; keep stored8/32 JSON cases and identity/final negatives;
add opaque-callee + declared forwarded caller guarded-field positive and opaque
scalar-leaf positive. Reject missing/foreign class or canonical field, wrong
formal/ordinal/use site, conflicting incoming class, unseeded cycle, and Pending
consumption. Bool/Unit/Nullable/unproven Array.get and non-forwarding literal
stored calls stay outside this S0 profile. Required Page Array.get/C projection
followups are preserved, not replaced by permanent annotation restrictions.

Smallest next construction is this same source/result/profile correction after
exact pending payload and function loans are written against current types.
Source665/result621/lexical549 are measured: plan responsibility split if a
selected owner approaches760; never grow facade781 past800. Any necessary
BoxShape prerequisite stays separate from semantic correction. D0 keeps Rust
construction stopped until that exact payload/loan mapping is closed; no user
approval is required for these internal owner decisions. No live Cargo and no
code commit; existing WIP and snapshots remain protected. Scope guard after
selected formatting reaches the same root1351 baseline; pointer/whitespace PASS.

## Closed MIRBUILDER-BORROWED-SOURCE-DRAFT-SPLIT-T2

50f44a92ec: original owned draft collection moved verbatim to private child;
finish uses the same drafts, call-map duplicate check and profile ordering.
Source parent614 / child82. uses23/publication30/stored8 PASS; source13/1,
page_heap0/4 and package627/8 exactly preserve then-open S0 NEW5 + known3.
Those historical failures are not relabelled baseline; PROFILE-S0 restoration
is recorded below. Guard known root1351 only; pointer/whitespace PASS.
Logs /tmp/hako-source-draft-T2-*.log; full closed evidence in commit.

## PROFILE-D0 concrete payload / source-only receiver probe

T2 landed and pushed50f44a92ec; local/remote full SHA
50f44a92ec089e7060408f51a8a8b03287fa0ec5 matched. Existing S0 code and
appendices remain uncommitted; T2 staged only its own extraction/document hunks.
No Cargo/rustc live after focused sequence.

Worker and current-type inspection confirm the minimal result payload: keep
BorrowedI64ResultSourceV1 as the sole owner, with a private closed phase
Pending(field obligations) -> SourceSealed. A field obligation retains exact
BindingRefV1 formal, OwnedExprSiteV1 use site and read site, required class,
CanonicalFieldRefV1 and exact class-lending identity. Existing FieldReadOperand
row.site and kind.site differ and both must be retained/rechecked against the
same original draft. No cloned guard permission or fabricated class view.
BorrowedForwardUseDraftRowV1 is Debug-only, not Clone: retain/move its original
source identity or an exact private identity tuple; do not make an executable
forward receipt just to classify a pending field result. Literal/exact-I64
opaque results have an empty field-obligation list.

Prospective receiver belongs in the existing SourceReceiverNeedV1 Stored arm,
not a prematurely complete LexicalInstanceCallSourceTargetV1. Retain original
call/receiver/argument sites, parent binding/site, Provider field/class and
unique_instance_target's canonical key/slot/callee owner as source-only need.
The pure probe borrows the current original direct-return shape, exact entry
receiver and Provider residence checks; it stops before destruction_for,
owned_field_children_of and owned.entry mutation. Only grounded eligibility
invokes the existing stored_child_receiver_v1 canonical inventory issuer and
reconciles the need with its complete source target. Pending dependencies name
the same original need identity; no second lookup registry, fake Local binding
or owned child lease. Existing lexical target admission stays unchanged.

Remaining exact function-loan mapping: lend the T2 collected drafts to the
original-return pending classifier and source-only needs, ground conditional
field classes through the existing actual seed lender and result dependency
fold, promote in original source order, then consume the same drafts in the
one profile/strict incoming/object-view finish. Seal pending obligations after
that finish without a second return-AST scan. All actual/entry/result consumers
must require SourceSealed; missing class seeds remain named unavailable, never
remove/re-ingress/retry. The semantic correction is not yet selected fast until
these function boundaries and visibility are closed against current owners.

## PROFILE-D0 integrated function-loan Decision / T3 predecessor

Read-only worker completed the concrete mapping. Collect original use drafts
once; classify original returns once into the existing result owner's Pending
phase; original source-needs retain prospective stored key/slot/owner/sites
without receiver authority; conditional class and existing dependency fold
ground eligibility; sole stored issuer seals inventory only after eligibility;
finish ingress from the same drafts once, then seal pending fields and call
references without return AST reclassification. Pending field requirement may
remain None for old lexical opaque fields until incoming view is available;
stored promotion requires its conditional requirement grounded beforehand.
Unseeded/missing requirements never promote stored rows. Forward identity is
the exact existing row tuple, not a cloned guard. SourceSealed is mandatory
for actual projection, entry result selection and completion corroboration.
All pending/preparation visibility stays within the lexical owner.

MIRBUILDER-BORROWED-SOURCE-FINISH-SPLIT-T3 is the necessary final BoxShape
predecessor: expose collected drafts only within the lexical owner and make
the existing ingress finish consume that exact owned triple and the original
validated calls map. Old preparation façade remains calls-map -> collect ->
finish, preserving error order, graph closure, forwards, strict incoming and
object views. No new product, scan, source registry or borrowed ABI permission.
Scope: borrowed_formal_source.rs, its collection visibility, owner README, this
card and pointer. Acceptance: exact original finish-body comparison; source/use,
publication/stored and package regressions, with original S0 NEW5 kept distinct.
After T3, construct the already-decided profile correction; do not select further
BoxShape work absent measured hard-cap pressure. Production target remains
Heap.isLiveHandle -> Page.isLiveHandle and facade -> Heap original stored returns.

T3 worker verification independently confirms the original 82-line finish tail
byte-identical and lexical-owner-only visibility. Critical semantic boundary:
replace candidate-exists has_stored_terminal with a retained demand that
distinguishes unpromoted source needs from promoted target rows. Unpromoted
needs alone may return false. For a promoted stored row, owner SourceSealed I64
permits the verified terminal walk; owner Err/Pending/missing must return a
named selected error (Result<bool, issue> or the same retained demand), never
false followed by seed/plain-path selection. The current bool caller at
coseal_issue.rs338 feeds seed_completion; this is an explicit replacement edge.

Consumer census for SourceSealed: actuals::project_pending_borrowed_i64_arguments_v1
currently permits source results before final Completion, so it must require
source seal but still not demand contract_corroborated early. Entry's final
result-kind selector requires both source seal and Completion corroboration.
Result corroboration and terminal unannotated-I64 permission also require source
seal; dependency corroboration propagates failures without profile retry.
Do not change the current necessary pre-Completion ordering into a new cycle.

T3 closeout: original finish tail independently byte-identical (worker and
primary comparison). Cargo64464 terminal101: source13/1 same S0 source-not-i64.
Sequential83958 terminal: uses23/publication30/stored8 PASS, package627/8 with
exact T2 failed-test set (script PASS). No newly introduced failure; S0 NEW5
and known3 remain distinct and unclosed. Scope guard unchanged root1351 debt;
pointer/whitespace PASS. Logs /tmp/hako-source-finish-T3-*.log, no Cargo live.

Select MIRBUILDER-STORED-CHILD-BORROWED-CALL-PROFILE-S0 fast. Internal mapping
is closed: Direct or Local(original ResolvedInitializerRelationV1 source fact)
retains stable call origin once; Pending retains original return/call/receiver/
argument/selector/arity identities, canonical need key/slot/owner, field-use and
conditional declaration requirement. Resolve eligibility before owned inventory,
consume original drafts once through finish, then seal against strict incoming
and complete views. Sealed full-target dependencies enter the same final map.
Source-seal demands cover prefix projection, caller/callee dependency prepass,
completion success, final entry result-kind, and terminal result corroboration
(including annotated rows); old nonborrowed strict lexical callers remain intact.
Promoted stored failure is a named error, not seed fallback. Acceptance and
replacement scope remain the integrated PROFILE-D0 Decision above. No further
BoxShape predecessor is selected; source-only probe/Pending/seal construction
starts under this row, then focused tests must restore NEW5 and preserve stored32.

## PROFILE-S0 construction / first compile

Pending source returns now retain exact guarded-field use/read identity and
stable Direct/Local initializer call identity. Prospective Stored source needs
carry only entry/Provider source facts and canonical selected target reference.
Conditional class loans borrow original DeclaredObject/forwarded origins using
classify_actual_seed; they issue no incoming view. Same result dependency fold
schedules pre-promotion requirements, then validates sealed dependencies.
Inventory issuer runs only after grounded-I64 + exact forwarded formal demand;
strict profile/incoming/views consume the single original draft collection.
Final result map is sealed without a second return/initializer classification.
Existing source/actual/entry/result/terminal consumption guards now demand
SourceSealed. Promoted stored result errors propagate named, not seed fallback.

Implementation scope adds private result Pending/class-loan and profile-stage
children under the same lexical/result owner, no new retained registry. Sizes
463/137/137; result facade431, source652, source-owned child396, issue facade781.
Existing receiver WIP and protected C/Array/S2 changes remain uncommitted.
Cargo41199 first compile in progress: initial visibility/re-export and missing
constructor import errors are PROFILE-S0 implementation failures, not baseline.
No tests accepted or semantic closeout yet. Wait terminal before Rust edits.
Read-only worker auditing new code; no additional design stop/user decision.


## PROFILE-S0 regression restoration and audit correction

Cargo37604 terminal0 compiled the Pending/profile construction; stored8 PASS,
including the extended original JSON witness (declared, opaque guarded-field and
opaque scalar leaves; 96 combinations, original32 retained). Sequential45137
terminal0: Pending-consumption1, source14, page_heap4 and call_result35 PASS.
Package633/3 has only the exact known3; the original S0 NEW5 are restored, not
relabelled baseline. These results predate the following audit corrections and
therefore do not close the final slice.

Worker found two selected-error erasure edges: receiver issuance Err/None buried
in a target row, and strict-ingress Err replacing the entire result map with an
empty map. Promotion now propagates receiver failure immediately with its exact
original call site; result sealing retains the original strict-ingress error in
its existing owner rows. No selected seed/plain retry or result-source-missing
replacement is allowed.

Decision: conditional class grounding accumulates every declaration/forwarded
class proposal through the same temporary finite propagation. Only a singleton
consistent class lends a field requirement; conflicting proposals do not promote
stored candidates. Never select the first class by declaration order. This is a
private scheduling constraint, not a new incoming-view authority or eager error
for old lexical callers outside the selected profile. Strict all-incoming views
remain the sole executable class authority.

Added real source negatives: weak child with independent birth store must retain
receiver-unavailable after grounded eligibility; A/B wrappers with conflicting
field classes in both declaration orders must stay at one failure boundary;
foreign main actual must retain the original declared-object class error.
Cargo53632 is verifying these plus Pending-consumption (final result pending
child test filter); its result is not yet accepted. No concurrent Cargo or code
commit. Final scope guard reaches the same HEAD-identical root1351 debt;
whitespace PASS. Protected C/Array/S2 and previous receiver WIP are preserved.


Final-negative first run Cargo53632 terminal101 (5m30s build): weak issuer,
foreign actual and Pending-consumption3 PASS. Conflict reversal test failed
because its expect_err assumed package issuance itself must reject an
unselected candidate. Current finish closure removes both wrappers and then the
opaque leaf with zero admitted incoming, returning an empty borrowed profile;
this is the preserved outside-profile boundary, not an incoming-coverage error.
The test expectation is a PROFILE-S0 test failure, not baseline debt. Corrected
it to check both declaration orders produce zero definitions/incoming/views and
zero result permission, no promoted stored disposition, while preserving both
original main lexical call dispositions. Cargo57627 is live for this correction;
no implementation relaxation or additional executable authority was introduced.


Cargo57627 terminal0 (4m50s rebuild): all Pending/profile negatives4 PASS.
Sequential84509 terminal0: source14/page_heap4/call_result35/stored8,
resolved_semantics349 and physical_boundary11 PASS. Package636/3 has exactly
known ReceiverNonEscape Capture / IncompleteOrdinaryNewCoverage /
MapLifecycleUndertaking BorrowedEntryEscape, same failure names and boundaries;
no remaining PROFILE-S0 or original NEW5 failures. Original JSON96 witness and
stored identity/final-validation negatives remain green. Scope guard reaches
HEAD-identical1351 SHA2565fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467.
Pointer and whitespace PASS; this is not a whole-green guard claim.

Retirement audit after this acceptance: prepare_borrowed_formal_ingress_v1 has
zero production callers, only the two direct ingress corruption tests. Preserve
that adapter and its reexport under cfg(test); production now has only the
unified Pending/profile preparation. Former late return-classifier/guarded-field
classifier functions are absent. Independent ingress and physical validators
are preserved. Selected result and lexical issue formatting adds no new meaning
and stays447/577 lines (source654). Cargo46559 rechecks the package after this
production adapter retirement/formatting; no final result accepted yet.
No S0 commit or C projection/Array.get/full-goal completion is claimed.


## Owned-object layout publication prerequisite / isolated closeout audit

Cargo46559 terminal101: post-adapter retirement package636/known3, unchanged
failure names/boundaries. Result/lexical/source sizes447/577/654, composition106
formatted without meaning changes. Main worktree scope guard still only the
HEAD-identical root1351 debt; pointer/whitespace PASS.

Explicit source35-path commit candidate separates the four mixed files by
selected HEAD-based patches; protected Array/C/S2 hunks remain unstaged.
Read-only snapshot audit confirms the other26 tracked paths had no pre-existing
protected delta. /tmp/hako-stored-child-S0-stage/selected-source.patch preserves
that candidate; detached checkout /tmp/hako-stored-child-S0-checkout contains
only HEAD93b1c4d478 plus this patch. Cargo57160 is live for isolated stored-child
acceptance, using the sole shared Cargo target directory; no restart/parallel
Cargo. This tests the actual candidate, not the unrelated WIP.

Static audit exposes a required publication dependency: source JSON96 asserts
owned_object_residences, currently supplied only by verified but uncommitted S2
metadata. Source closeout cannot claim that metadata exists in HEAD. Do not drop
this acceptance or land all C/Array/S2 incidentally.

Decision: isolate the existing canonical inventory publication responsibility
as MIRBUILDER-OWNED-OBJECT-LAYOUT-PUBLICATION-S0, before receiver/profile landing.
Source authority + canonical issuer: original ledger owned_field_children_for;
read-only finalized handoff -> existing physical layout ABI -> JSON tuple list;
optional exact-key C layout decoder validates that same external schema.
Non-authority: matching emitted stores/releases, declared class alone, ordinal
names, new source scan, copied ownership, empty tuple permission.
Fail-fast boundary: explicit unproven inventory, foreign object/slot/child,
duplicate or overlapping tuple, and unknown child layout. A passive class/null
view alone grants no owned residence; preserve existing null-only declared
object acceptance rather than requiring construction authority for an ignored
view. Worker identified this counterexample for the current publication getter;
verify it before closeout and distinguish absent from explicitly unproven data.
Selected consumers/replacement: physical_abi layout projection and JSON metadata;
C layout schema parser. No index-kind borrowing or Birth store/discharge markers
in this prerequisite. Preserve S2 marker and Array/Unit-call WIP outside stage.
Acceptance: constructed plain owned-child source projects exact tuple; null-only
ignored declared parent remains accepted without owned tuple permission;
external tuple missing/duplicate/wrong class/slot rejects as appropriate while
legacy layouts remain admissible. Source S0 original JSON96 stays mandatory.
No code commit or source/C/Array/full-goal closeout yet.


Isolated source Cargo57160 terminal101: source receiver identity1 PASS; JSON and
final-MIR stored receiver negatives2 stop at object-field-read-definition-invalid,
which is a missing selected final-validator hunk in the initial candidate, not
baseline debt. Add only the existing exact canonical user-field membership arm
of invoke.rs to the saved36-path source candidate; keep its Unit call arm outside
scope. This preserves independent final validation and is required for the same
stored read. No source acceptance is weakened.

Worker confirms publication separation and existing inventory tri-state semantics.
Getter now retains outerNone as unissued and Some(None) as named unproven error;
only Some(Some(children)) supplies tuple evidence. Constructed claim/provider
inventory issuance and Normal/Fault teardown obligations remain unchanged.
Implementation is selected as the stated publication prerequisite; scoped8-file
index contains metadata, external decoder and focused source/schema tests only.
Protected Array/Unit/Birth-markers/partial-cleanup changes remain unstaged.
No stash/reset/discard. Source candidate staging was reversed with its saved
exact patch; source code and tests remain intact for later reapplication.

Detached checkout was restored by reversing only its exact source candidate,
then received only the metadata8-file candidate (source S0 not present).
Cargo72160 is live for owned_object_layout focused source capture2 tests,
covering both optimization settings. No second Cargo. Exact detached C shim
build9823 terminal0; source tests must complete before external schema tests.
Added external decoder checks:4 unchanged inputs,4 legacy optional-metadata
shapes,14 malformed tuple inputs (duplicate/range/self/unknown/Array-overlap/
extra-key/ordinaltype), exact abi-layout failure. These prove schema only, not
V4 borrowed child handling, S2 store/discharge or runtime ownership.


Publication first isolated Cargo72160 terminal101 (6m11s): constructed canonical
object tuple test1 PASS. Null-only test stopped at an overly strict is_ok assertion
on the existing pretransform Document report, before metadata issuance. The report
is exactly caller main -> Transport.probe argument0 Void/formalInteger with the
sealed BorrowedTaggedValue carrier; existing S1 source publication classification
already retains this reference observation. It is not a new metadata failure or
whole-goal green claim. Correct the test to classify only that exact singleton
report and still require final physical ABI/JSON null/class view corroboration.
All other errors remain test failures. Cargo73568 is live for this correction;
no parallel Cargo. The constructed capture2 variants already exist unchanged.
Detached C metadata shim build and existing physical schema regression PASS.
Metadata9-file staged candidate excludes source S0, Array/Unit and Birth markers;
new compiler owner explanation records optional unissued vs explicit unproven
inventory and external schema admission without executable ownership.


## Closed provider Birth publication prerequisite / e0355851e6

2026-10-06: landed/pushed e0355851e6; remote branch updated from93b1c4d478.
Source authority/canonical issuer: existing finalized_root_handoff ledger;
completed local-commit Birth relations seed the source-owner provider closure.
All provider relation owner/object/construction and same-target full handoff
checks remain before selection. Original caller site owner selects provider
receipts; child handoff owner extends the finite closure. Original record order
and per-site actuals survive; only definitions deduplicate. Physical matcher
continues all-actual consumption/all-target reference, without retry/fallback.
No class/symbol/physical-caller lookup becomes executable source authority.
Original all-constructor source/emission validation remains unchanged.

Counterexample cause: an unused Parent.birth exported provider -> Leaf actual
without exporting its caller. Old unconditional provider transport is replaced;
no matcher relaxation. Source-owned closure repair is independent of metadata.
Deep owned-object provider nesting is outside current source acceptance; selected
witness uses supported plain sibling providers and shared child target instead.

Standalone /tmp/hako-provider-birth-S0-checkout contained HEAD93b1 plus only
prerequisite code/test paths.89930 terminal0 compiled-entry4 includes provider16
source combinations and selected missing/duplicate/foreign-owner negatives.
Existing nonscalar negative1 PASS; package609/exact known3. Scope guard known
HEAD-identical root1351 only (hash recorded below); pointer/whitespace PASS.
Code368/test366 lines. Narrow owner README and card/pointer committed together;
protected Array match arm, metadata/source/C/Array/S2 WIP excluded.

One stale per-new rejection assertion, introduced d5823ab49ba, was found during
33374 terminal101 and corrected to the549a54c811 landed scalar contract:
source Local stays Local; tag1 requires exact caller Integer evidence and
missing type evidence still rejects actual-kind-unavailable. Not absorbed into
known3, no new production admission. Earlier50500/32162 positives and accidental
main rerun52077 interrupted130 are historical, not final closeout evidence.
Full prior Decision/commands are preserved by e0355851e6. Logs remain in
/tmp/hako-owned-object-layout-S0-stage/provider-*.log.

## Closed owned-object layout publication / e00407c579

Exact tri-state inventory -> ABI -> optional JSON tuples and external decoder
landed/pushed e00407c579. Constructed2/thin+scratch null4 final ABI, compiled-entry4,
external4 original/4 legacy/14 exact malformed tuple negatives PASS; no V4/runtime
claim. Full Decision/evidence in commit and metadata-*.log under
/tmp/hako-owned-object-layout-S0-stage/. Protected source/C/Array/S2 excluded.

## Source receiver/profile S0 verified closeout

Provider e0355851e6 and metadata e00407c579 landed/pushed. Sole source candidate
/tmp/hako-stored-child-S0-final-checkout = e00407c579 + saved36-path source patch,
without C/Array/S2 WIP. Cargo28548 terminal0: source JSON96 witness and exact
receiver identity/final-recorded read corruption3 PASS. Sequential1426: stored22,
source14/page_heap4/call_result35/resolved349/physical_boundary11/compiled-entry4/
metadata2 PASS; package615/exact known3 only, same boundaries confirmed. Initial
result_pending_tests filter matched0 and is NOT evidence; corrected exact
borrowed_formal_result::pending::tests executes4 PASS, all included in package.
Logs /tmp/hako-stored-child-S0-stage/final-isolated-*.log. Scope guard reaches
only unchanged root1351; pointer/whitespace PASS. All36 staged source files
byte-match the tested isolated candidate. No changed-row red remains.

Production result Pending/profile uses one original draft collection, conditional
class proposals are monotone/singleton only, promotion borrows sealed inventory
and SourceSealed gates projection/completion/terminal consumers. Selected weak
receiver and foreign ingress errors remain exact; conflicts do not promote.
The late AST classifiers are deleted; prepare_borrowed_formal_ingress_v1 and
reexport are cfg(test) only for2 corruption tests, production caller-zero.
Selected field-read final validator arm retained, Unit arm and all other WIP
excluded. Source meaning closes, not C runtime/Array.get/unchanged-app/goal.

Next selected C receiver D0: integrate original tuple -> ordinary_i64 receiver
Copy-root -> borrowed child class/origin mapping against actual protected shim
code. No ordinary_nullable_handle ownership extension or new source registry.
Read-only worker first verifies current helper signatures, role/root gates and
Normal/Fault + malformed tuple/class/borrowed release acceptance; resolve one
Decision internally, then select bounded implementation. Source fixture JSON96
remains mandatory. C design stop is temporary selection, not goal pause/blocked.

## C receiver integrated Decision / 2026-10-06

Decision: lend source-issued owned_object_residences into the existing index
for ordinary_i64 stored-child reads, preserving sibling Birth corroboration.
Authority/issuer: sealed inventory -> published canonical tuple; existing
receiver_object and exact original receiver/Copy producer root. Known child0
is valid. Metadata does not issue ownership, argument handoff or a result lane.
Consumer/replacement: lv4_child_field_object and lv4_index_seed; replace scalar
classification of these exact ordinary borrowed-parent reads. Same-function
set/release siblings still agree; no Birth marker/discharge changes. Reuse the
existing bounded Copy-root helper through a static declaration, not a second
root walker. Terminal root must be original receiver, HANDLE/origin-2 and exact
parent object. Existing ordinary-call receiver check retains child class.
Non-authority: intermediate Copy kind, PHI union, target spelling, layout alone.
No flow widening: BorrowedTypedLive actually owns ordinary nullable/tag3 argument
handoffs, not this receiver; both callers remain unchanged. Read-only worker
audit_cost_oct5 independently verified these gates against protected C WIP.
Acceptance: compile all96 unchanged source-issued inputs; opt-pair real stored
callee Normal/Fault with Scratch Birth store injection and cleanup once; valid
base negatives for absent/nonowned tuple, foreign class/root and borrowed-child
release; schema duplicates/range/unknown remain exact external rejection.
Same-class sibling swap is source/final-MIR identity evidence, not C reproof.
Smallest slice: index mapping + focused C test, original test-only Scratch
source capture if needed, narrow owner contract. Protected C/Array/S2 unchanged
outside selected hunks; Page Array.get and full goal still owed.

## C construction evidence / Birth cleanup dependency

Main protected C build terminal0; tuple/root mapping constructed in index only.
Initial original96 probe terminal1 at Parent.birth merge, not the receiver.
Temporary copied shim debug confirms marker258 at first stored child survives
second new Fault straight to shared ReturnFault; original source-only producer
lacks prior-field discharge. Preserve these96 byte-original captures in
/tmp/hako-stored-child-C-receiver-S0-before/original96 as S2 bypass negatives.
No C gate weakened and no new production baseline classification.
Read-only worker confirms protected source issuer/emitter already carries
prior-only newest-first discharge and storage-only child Birth reclaim.
This Rust ownership correction is an independent prerequisite before mapping
closeout; C child-call BoxCount/Array lifetime must not be swept into its commit.
Current sole Cargo30017 regenerates the unchanged source96 with that producer
and adds test-only Scratch-in-Leaf.read opt-pair for real callee Fault/child0.
No new capture/Normal/Fault acceptance asserted yet. New C focused script is
syntax-checked only until this run finishes. External schema regression on main
C accepts4 original/4 legacy and rejects14 malformed tuples. Earlier C ingress
script invocation lacked mandatory argument (harness error, not production);
corrected run99176 uses existing local-zero source capture.

## Composed C evidence / prerequisite selected

Cargo30017 terminal0:2 focused tests regenerate original96 unchanged-source
inputs and real stored Leaf.read -> Scratch.birth opt-pair, canonical Leaf0.
C49543 terminal0: all96 unchanged fresh inputs compile; exact receiver Copy
wire positive;9 negatives (foreign base/class, missing/foreign tuple, child/
Copy/parent-field release, duplicate/range tuple) reject at named gates.
Both opts execute Normal rc5 and real callee Fault rc70: Normal stores4/Home4;
Fault stores4/Home3/reclaim1, original Scratch diagnostic site, frame once.
This proves composed protected producer+consumer, not isolated commit scope.
C ingress99176 terminal0; external schema regression PASS. Cleanup1 and issuer2
focused tests PASS. Scope guard only unchanged root1351; first guard invocation
had a wrong script filename (harness error, corrected actual guard run).
All logs under /tmp/hako-stored-child-C-receiver-S0-before/. No Cargo live.

Decision: integrate independent prior-field cleanup prerequisite before landing
C receiver. Source issuer seals prior-only newest-first store inventory; Birth
callee owns partial-field Fault teardown, caller reclaims unpublished storage
only; post-Normal store Fault tears down completed child then prior fields.
Selected production replacements: constructor Fault ingress/disposition,
builder store state/emission/final validation and caller unpublished reclaim.
Nine production paths identified by read-only worker: instance_construction;
construction state/emission/validation + new fault_cleanup; selected admission;
local-commit reclaim/emission_prepare/emission_validation. Stage semantic hunks,
exclude child-call grammar/census and independent Array lifetime changes.
Acceptance: isolated HEAD + prerequisite only compile/issuer2/cleanup1 and
source capture/C partial-state negative/regressions, guards; required test
fixtures/state fields and owner contract join prerequisite. Later C receiver
landing retains source/C Fault evidence after the necessary consumer dependency.
No S2 bypass/fixture repair, scope reduction or whole-goal completion.

Prerequisite test wiring: existing construction-state scalar fixtures receive
empty inventory/discharge coordinates; new fault_cleanup_tests and construction
fault tests get their own hooks, excluding child-call hooks. Add a coordinated
healthy-root source/final-MIR counterexample restoring old caller field cleanup
and require reclaim-duplicate-field-cleanup; old96 Birth-side bypass is not proof
of this independent caller fence. Focused construction-state2, source issuer2,
cleanup1, ordinary-new completion/owned-child/physical-boundary and fresh96 remain
required before the isolated prerequisite closes.

## Prior-field cleanup isolated construction / 2026-10-06

Previous goal turn was progress: composed C96/NormalFault evidence and selected
prerequisite were recorded/pushed at1a3c4abc17. This turn isolates HEAD1a3c4abc17
+ nine production-path semantic hunks, focused test/wiring paths and narrow package
README/scope guard in /tmp/hako-birth-prior-field-cleanup-S0-checkout.
No child-call grammar/Unit-role or Array-local lifetime/root-step hunk enters.
Shared main differences remain protected; scoped candidate is saved under
/tmp/hako-birth-prior-field-cleanup-S0-stage/. No main source replacement/stage.
Read-only worker reviewed extraction/order/Provided/Normal teardown and new
coordinated caller cleanup test; no authority or retirement widening.

First isolated Cargo43822 interrupted130: checkout regenerated an ignored lock,
which differed from main. Original Cargo/rustc terminal confirmed; retain generated
lock as evidence, copy original main lock, then use --locked. Cargo44219 terminal101
had two missing test imports; imports, temporary Vec borrow and exact freeze token
fixed. These were harness/candidate failures, not baseline. Cargo60480 terminal0:
new healthy source-ledger/MIR control followed by coordinated old caller field
release rejects exactly ordinary-new/local-commit/reclaim-duplicate-field-cleanup.
It proves New validator ownership, not whole MIR/Home/pipeline completion.

Final candidate adds existing check_binding over every mutant retained binding
before the ownership error, and Object cleanup healthy/foreign child/base/frame/
field/Fault-continuation controls (Array cleanup test retained). Main new Object
test is preserved; isolated package Cargo71164 is live, sole shared-target Cargo,
quick/jobs4/--locked. No final package/Object/stronger-correspondence PASS yet.
Initial scope guard stopped at old jump_landing pin after helper relocation;
updated sole-owner pin + cleanup/caller controls now reaches only unchanged
root1351 hash5fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467.
Whitespace/lock equality checked. Guard is not whole PASS.
Next after terminal: exact red classification, cleanup/state/physical/completion/
owned-child controls, fresh unchanged source96 + composed C check, then verified
semantic-hunk integration/commit/push; C dependency/source capture/app/full-goal
obligations remain intact. Goal active, not blocked or paused.

## Prior-field cleanup verified closeout / 2026-10-06

Selected16 source/test/contract/guard paths byte-match the isolated candidate
based on1a3c4abc17; child-call grammar/Unit and Array-local changes are excluded.
Cargo71164 terminal101: package618 PASS and exact known3, matching source615
baseline names and causes: birth receiver Capture; main-static-child
IncompleteOrdinaryNewCoverage; qualified Map argument BorrowedEntryEscape.
No introduced or unclassified red remains. Earlier interrupted lock/missing
imports are corrected candidate failures, not final acceptance.

Sequential36353 terminal0: cleanup2 (Array/Object plus child/base/frame/field/
Fault continuation negatives), construction state2, source issuer2,
prior-installation1, owned-array children2, physical boundary11,
source original-receiver1 (regenerates96), compiled-entry21 PASS. Package includes
stronger coordinated old caller cleanup negative: every retained physical
binding independently checks before reclaim-duplicate-field-cleanup rejection.
C23240 terminal0: all96 freshly regenerated unchanged-source inputs compile
with composed protected C consumer. Original pre-correction null witnesses,
both opts, reject partial-Birth bypass at unsupported-cohort with no artifact.
These are composed C dependency evidence, not an isolated C landing claim.
Logs: /tmp/hako-birth-prior-field-cleanup-S0-stage/*-final.log,
fresh-source-C96.log and old-producer-C-negative.log. No Cargo/rustc live.

Production issuer seals prior-only newest-first inventory once; Birth owns
partial-field Fault discharge. Caller unpublished reclaim is storage-only;
Normal completed-child teardown remains retained. Selected origin.children,
reclaim.children and reclaim-children-source-missing/drift edges are removed
from admission/local-commit/reclaim; no shared Normal inventory deletion.
Scope guard reaches only unchanged root1351 with recorded HEAD-identical hash;
not whole guard PASS. Whitespace/pointer PASS. Narrow README repairs the stale
universal user-provider exclusion to existing source-proven provider authority;
this wording correction adds no admission. Protected mixed main WIP retained.

Next Decision: read-only worker audit_cost_oct5 audits C mapping prerequisites
against HEAD and actual shim. Select exact existing sibling-Birth helper / row
role / S2 dependencies before any C integration, then integrate one Decision.
No implementation authority is inferred from composed green alone. Next row
MIRBUILDER-STORED-CHILD-BORROWED-C-RECEIVER-INTEGRATION-D0 owns only this dependency
question. Required C Normal/Fault controls, Page Array.get, unchanged-app probe,
production cutover/selected legacy retirement and whole-goal acceptance remain.
