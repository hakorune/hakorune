---
Status: closeout__value_origin_scope_subdecision
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-BRANCH-CONTINUATION-D1
Parent: mir-call-parser-array-push-b3-if-loopcond-d0-2026-09-23
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B3-LOOPCOND-CARRIER-RELATION-D2
Implementation permission: this card accepts only the value/origin scope rule. It does not authorize implementation until the Recipe carrier-to-BindingRef relation and exact join output mapping are co-sealed by D2.
---

# StringHelpers Loop-If branch continuation and ledger scope D1

## Six-line brief

```text
Decision: prove whether the existing LoopCond/GeneralIf owner can co-seal the
nested split_lines ArrayPush with branch entry/reset/join/backedge/exit state
for the exact `i` and `last` BindingRefs and their source origins.
Source authority + canonical issuer: the existing resolver source forest and
issue_source_core_method_calls_with_named_arrays_v1; the existing LoopCond
Facts/Recipe consumer remains the sole semantic issuer and physical owner.
Non-authority: Hako names, AST rescans, Builder variable-map snapshots alone,
MIR phi shape, push text, compatibility lowerers, and the post-Loop tail push.
Fail-fast boundary: branch site, continuation site, BindingRef, active-origin
state, and named ArrayPush row must be one exact transaction; missing/foreign/
duplicate/rollback/residual state stops at an existing named terminal.
Smallest next slice: map the existing ledger APIs to one branch transaction and
decide whether no new receipt or carrier map is required.
Non-claims: no implementation, generic If support, tail-push support, runtime
Text/Fault proof, raw push/set/insert retirement, VM/AOT parity, or serializer.
```

## Finite source inventory

The selected source witness is `lang/src/shared/common/string_helpers.hako`
`split_lines`:

| row | source shape | D1 treatment |
| --- | --- | --- |
| loop row | `arr.push(s.substring(last, i))` inside `loop` → `if ch == "\n"`; the then branch also assigns `last = i + 1` | selected nested Loop-If row |
| tail row | `arr.push(s.substring(last))` after the loop | explicit non-claim; separate future statement owner |

The row identity must remain the resolver `SourceExprSiteV1` and the existing
`BindingRefV1` relation for `arr`, `i`, and `last`. Method names, source text,
or a MIR array shape cannot select either row.

## Existing owner map

```text
source forest / named-array issuer
  -> normal_callable_loop_source_facts::loop_cond::issue
  -> SourceLoopCondPhysicalInputV1
  -> loop_cond_bc_source::lower_loop_cond_break_continue_source
  -> GeneralIf in associated_source::callable_loop_source_items
  -> callable_loop_source_lowering::lower_join_if_source
  -> dispatch::if_join::lower_if_join_state_core
```

The existing join core snapshots `MirBuilder` variable maps, resets each branch,
normalizes branch maps, emits join payloads, and publishes continuation ValueIds.
The source port, however, borrows the callable ledger and performs exact
operations independently:

* `read_variable` consumes an exact source read and reads the current
  `BindingRefV1` value;
* `rebind` consumes the assignment row, invalidates its dynamic origin, and
  writes the new ValueId;
* `take_source_array_push` consumes the exact named-array source row and stores
  an emission port until `finish_with_named_arrays`;
* `publish_source_loop_final_value` publishes a loop final binding only after
  the physical loop owner closes.

D1 subdecision: extend the existing physical ledger owner with a value/origin
scope used by the same branch reset/join order as
`lower_if_join_state_core`. A Builder map snapshot without the ledger state is
insufficient, but a whole-ledger rollback is also incorrect. Both branches are
compiled once, so source obligations remain monotonic even when runtime takes
only one branch. This rule is accepted; the physical carrier mapping needed to
connect join outputs to BindingRefs remains open and belongs to D2.

The scope resets and publishes only current `values` and active origins for the
exact source BindingRefs. It must not roll back consumed reads, assignments, or
calls; materialized locals; removed source rows; `named_array_writes`; or
historical `value_origins`. `take_source_array_push` is therefore consumed once
by the statically lowered then body and its emission port remains owned by the
normal finish collector.

## Ordered D1 tasks

1. **Exact row census.** Record the Loop site, If site, push site,
   `substring(last, i)` argument site, `arr` receiver BindingRef, `i` and
   `last` assignment/read sites, and the post-Loop tail site. Verify that the
   nested row is under the Loop source root and the tail is not.

2. **Issuer and recipe coverage.** Show that the named-array issuer emits one
   row for the nested push, that `GeneralIf` is the selected `IfContractKind::Join`
   item, and that no second classifier or source item map is needed. The tail
   must remain outside the selected named-array set.

3. **Branch transaction inventory.** Map each existing ledger mutation used by
   the row (`read_variable`, `rebind`, `take_source_array_push`, pending
   `named_array_writes`, dynamic origin invalidation, and final publication) to
   branch entry, branch reset, join continuation, backedge, and loop exit.
   Snapshot/restore only current values and active origins; leave all static
   consumption and emission state monotonic. The smallest owner is
   `CallableSemanticLoweringState` plus its
   `CallableDynamicOriginLoweringStateV1`, not a new semantic receipt.

4. **Origin and BindingRef proof.** Specify how the exact `i` and `last`
   BindingRefs and their active origins are restored after the discarded branch
   and published after the join. A ValueId-only or name-only argument is a
   rejection, not a partial acceptance.

5. **Named-array one-shot proof.** Decide how the nested ArrayPush row is taken
   exactly once across zero iterations, one iteration, multiple iterations,
   true/false branches, and loop backedge. The post-Loop tail row must not be
   consumed by the LoopCond owner.

6. **Failure terminals.** Bind missing/foreign branch continuation,
   source-site parent mismatch, duplicate row take, duplicate rebind,
   rollback failure, unconsumed row, and emitted-write residual to existing
   terminals. `Ok(None)` from a lookup is not a rejection until the caller or
   finish owner proves the required row was absent by contract.

7. **D1 exit decision.** Accept the scope semantics, but do not open I0 yet.
   `SourceLoopCondPhysicalInputV1` and its Recipe still lack an exact
   carrier/key/BindingRef/join-output relation. Resolve that existing-product
   gap in D2 before implementation; do not recover identity from join names.

## Design audit correction

The read-only owner audit established that the existing `GeneralIf` owner is
reusable and that source consumption must stay monotonic. It also found that
LoopCond currently collects carrier names from AST and its phi materializer
consumes `&[String]`; `SourceLoopCondPhysicalInputV1` has no co-sealed
BindingRef-to-Recipe-carrier relation. Adding the branch scope before this
relation is fixed could attach a name-keyed join result to the wrong source
binding. Therefore this D1 scope rule is a subdecision, not implementation
readiness. D2 owns the remaining mapping question.

## Required evidence and non-claims

This card records the accepted design only. The successor I0 owns source edits,
focused tests, and a reusable guard for the value/origin scope. B2's
straight-line ArrayPush matrix is dependency evidence only and does not prove
branch ledger scope. The post-Loop `substring(last)` row and runtime Text/Fault
ownership remain separate cards.
