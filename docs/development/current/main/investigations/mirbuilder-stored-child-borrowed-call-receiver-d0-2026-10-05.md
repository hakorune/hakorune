# Stored-child borrowed call receiver D0

Status: prerequisite source-flow split T0 verified / semantic receiver S0 selected
Execution row: MIRBUILDER-STORED-CHILD-BORROWED-CALL-RECEIVER-S0
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

Lexical source preparation currently accepts only Lexical(Local binding).
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

## Selected prerequisite / CALL-ARGUMENT-SPLIT-T0

Decision: BoxShape only. home_local_call_flow.rs is796 lines; the required
receiver admission cannot grow this parent. Move its three existing argument
sealing functions together into private home_local_call_arguments.rs. Only
seal_lexical_i64_arguments_at becomes pub(super), reimported privately by parent;
its three callers, recursive child calls and private relation issuance remain
unchanged. Worker confirmed the boundary, borrowed-callback-first ordering,
strict fallback only for positively selected sites, exact lexical receiver gate
and ordered arguments. No source or physical meaning changes in T0.

Source authority/canonical issuer and production consumers are unchanged:
parent local/nullable/discard/borrowed-terminal call lenders all use the same
helper, now owned by the argument-sealing child. Replaced responsibility is
only its location inside the796-line parent; no alternate implementation stays.

Scope: parent, new private child, existing owner README paragraph, selected
scope guard pins, this card and current pointer. Snapshot mixed README before
editing; preserve protected WIP. Acceptance: verbatim moved-body comparison,
focused call_result, borrowed source/entry and existing lexical/terminal call
families, resolved semantics tests, rustfmt and scope/pointer guards. Existing
package known3 and root1351 guard debt remain classified baseline. No new
mirror test is needed for mechanical code motion.

T0 does not admit stored receivers or solve the app. After verified landing,
return to this card's receiver consumer mapping and select semantic S0 when
that mapping closes. Goal remains active.

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

Previous goal turn: progress, S1 closeout and stored receiver authority/card
landed `b77747e5c0`. Current-first/pointer PASS; no Cargo at entry.
T0 selected and implemented before any receiver semantics. Worker reviewed the
actual diff: three original argument functions moved together, parent659/child144,
borrowed-first callback, None/strict predicate, errors, recursion, order and
prior Homes unchanged. Post-rustfmt comparison confirms identical bodies except
new pub(super) visibility and terminal blank-line formatting. Snapshots protect
mixed owner README at `/tmp/hako-stored-child-call-argument-split-T0-before/`.

Cargo98982 terminal0 call_result34/34. Sequential Cargo8197 terminal0:
borrowed_formal_source14/14, entry21/21, lexical_i6410/10,
terminal_result19/19 and resolved_semantics349/349. Cargo15441 terminal0:
borrowed_source_publication29/29 and lexical_instance_call15/15.
Logs `/tmp/hako-stored-child-call-argument-split-T0-{call-result,source,entry,
lexical,terminal,resolved,publication,instance}.log`. All filters executed tests.
New private-owner guard pins run before guard17362 terminal1 reaches unchanged
root1351 debt (sha2565fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467).
No whole guard/package green claim, no new package run or app EXE claim.
Rustfmt, whitespace and current-pointer checks PASS. No live Cargo/rustc.

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
