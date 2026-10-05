# Stored-child borrowed call receiver D0

Status: active design mapping; implementation not selected
Execution row: MIRBUILDER-STORED-CHILD-BORROWED-CALL-RECEIVER-D0
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
