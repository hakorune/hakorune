---
Status: design_stop__branch_continuation_ledger_scope_pending
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-D0
Parent: mir-call-parser-array-push-b2-cutover-d0-2026-09-23
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B3-BRANCH-CONTINUATION-D1
Implementation permission: false; design and source census only. Do not add an If fixture, widen a Recipe, switch a caller, or create a new semantic receipt in this card.
---

# StringHelpers Loop-If ArrayPush design D0

## Six-line brief

```text
Decision: isolate the real StringHelpers.split_lines ArrayPush inside the
existing Loop-If source boundary; keep its outside-loop substring tail as a
separate generic call until an owner is chosen.
Source authority + canonical issuer: the existing resolver loop/If source
forest and issue_source_core_method_calls_with_named_arrays_v1; the existing
LoopCond source Facts/Recipe consumer must remain the only semantic issuer.
Non-authority: Hako method names, push text, AST scans, MIR shape, compatibility
lowerers, and the B2 straight-line fixture.
Fail-fast boundary: exact loop parent, If branch/body site, ArrayPush row,
receiver BindingRef, and branch continuation must co-seal before Composer or
Builder effects; missing/foreign/duplicate rows stop by existing owner.
Smallest next slice: census the one split_lines Loop-If push and decide whether
the existing LoopCond source owner can carry it without a second route.
Non-claims: no generic If support, no tail-push switch, no runtime Text/Fault
proof, no raw push/set/insert retirement, no VM/AOT parity, and no serializer
acceptance.
```

## Boundary and finite inventory

The candidate source is `lang/src/shared/common/string_helpers.hako` around
`split_lines`: `arr.push(s.substring(last, i))` occurs inside an `If` nested in
the Loop, and the final `arr.push(s.substring(last))` occurs after the Loop.
The first site is one selected candidate; the second is explicitly outside the
Loop-owned named-array obligation and must not be folded into it.

The B2 issuer already owns named-array construction, receiver identity, text
source, and exact call-site rows for straight-line Loop bodies. This D0 must
show whether `ResolvedLoopPlacementV1::Body` and the existing LoopCond source
forest retain the nested If branch and continuation as one co-sealed input. If
the existing owner cannot expose that relation, the result is `NoSafeSlice`;
do not manufacture a branch receipt or reclassify the call by name.

## B3-D0 task queue

1. **Source census.** Record the exact function, Loop site, If site, ArrayPush
   site, substring argument site, receiver binding, and final tail site. The
   inventory has one in-Loop candidate and one excluded post-Loop sibling.

2. **Issuer coverage.** Trace the existing named-array issuer from the resolver
   method ledger through nearest-loop placement and text-source co-seal. Check
   whether the nested If push gets one row and whether the tail push correctly
   remains outside the named-array set.

3. **LoopCond owner audit.** Read the existing LoopCond Facts/Recipe/physical
   consumer and identify the exact branch/continuation fields it already owns.
   The existing `GeneralIf` path can locate and lower the nested If through
   `CallableLoopSourcePartsBlockV1` and `lower_if_join_state_core`; it does not
   yet prove that the callable ledger's `values`, active origins, and pending
   named-array rows follow the branch reset/join/backedge. Do not treat the
   Builder binding map alone as this proof.

4. **Failure inventory.** Name the existing terminal for foreign If site,
   missing branch continuation, duplicate ArrayPush take, value demand, and
   residual tail/source rows. A direct `Ok(None)` from a lookup is not a
   rejection; the required caller/finish terminal must be observed or the row
   remains open.

5. **Tail separation.** Keep the post-Loop substring push as an explicit
   non-claim. Select it later only if a separate source statement owner and
   physical emission contract exist; do not use it to widen this LoopCond row.

6. **D0 exit.** The source/issuer/GeneralIf path is finite and the existing
   LoopCond owner is reusable, but the branch-continuation ledger relation is
   unresolved. Keep `design_stop` and hand that one question to the D1 card
   below; do not call it `NoSafeSlice` and do not open a fallback.

## D0 readout and corrected boundary

The read-only owner audit found this existing path:

```text
named-array issuer
  -> normal_callable_loop_source_facts::loop_cond::issue
  -> CallableLoopSourceRouteTokenV1::issue_with_source_relations
  -> SourceLoopCondPhysicalInputV1::validate_for_source_port / preflight
  -> loop_cond_bc_source::lower_loop_cond_break_continue_source
  -> associated_source::callable_loop_source_items::GeneralIf
  -> CallableLoopSourcePartsBlockV1::singleton
  -> callable_loop_source_lowering::lower_join_if_source
  -> dispatch::if_join::lower_if_join_state_core
```

`GeneralIf` is therefore an existing owner, not a request for a new route or
semantic receipt. Its join core snapshots `MirBuilder` variable maps and
publishes join ValueIds. The source port separately consumes exact assignments
through `rebind` and exact named ArrayPush rows through
`take_source_array_push`. The audit found no existing terminal that proves
those ledger mutations are branch-scoped and restored/published together.
That missing proof is the only open D0 boundary.

The selected source inventory is one nested Loop→If ArrayPush with
`substring(last, i)` and one excluded post-Loop sibling with
`substring(last)`. The latter is a separate source statement row; it must not
be counted as LoopCond coverage.

## Required evidence and stops

This design may use source reads and the existing worker audit only. It must
not add code, fixtures, guards, `Verified*`/`Prepared*` receipts, production
caller switches, or deletions. B2's six positive rows and negative matrix are
dependency evidence; they do not prove LoopCond/If acceptance. The first
implementation slice, if accepted, must name the exact production caller and
delete/retain set before entering `fast` mode.
