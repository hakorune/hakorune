---
Status: design_stop__stringhelpers_loop_if_arraypush
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-D0
Parent: mir-call-parser-array-push-b2-cutover-d0-2026-09-23
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-I0
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
the Loop, and the final `arr.push(s.substring(last, n))` occurs after the Loop.
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
   Decide whether it can consume the source If subtree once, with no AST
   rescanning, name matching, or second physical carrier map.

4. **Failure inventory.** Name the existing terminal for foreign If site,
   missing branch continuation, duplicate ArrayPush take, value demand, and
   residual tail/source rows. A direct `Ok(None)` from a lookup is not a
   rejection; the required caller/finish terminal must be observed or the row
   remains open.

5. **Tail separation.** Keep the post-Loop substring push as an explicit
   non-claim. Select it later only if a separate source statement owner and
   physical emission contract exist; do not use it to widen this LoopCond row.

6. **D0 exit.** Accept only with a finite source→Facts→Recipe→consumer map,
   one existing owner, named negative terminals, and a clear implementation
   permission tuple. Otherwise seal this family `NoSafeSlice` and return to the
   already-inventoried task scheduler without opening a fallback.

## Required evidence and stops

This design may use source reads and the existing worker audit only. It must
not add code, fixtures, guards, `Verified*`/`Prepared*` receipts, production
caller switches, or deletions. B2's six positive rows and negative matrix are
dependency evidence; they do not prove LoopCond/If acceptance. The first
implementation slice, if accepted, must name the exact production caller and
delete/retain set before entering `fast` mode.
