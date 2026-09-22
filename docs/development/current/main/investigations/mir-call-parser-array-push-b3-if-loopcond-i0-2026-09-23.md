---
Status: design_pending__loopcond_carrier_relation
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-I0
Parent: mir-call-parser-array-push-b3-branch-continuation-d1-2026-09-23
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-CLOSEOUT
Implementation permission: suspended until MIR-CALL-PARSER-ARRAY-PUSH-B3-LOOPCOND-CARRIER-RELATION-D2 accepts the exact Recipe carrier-to-BindingRef and join-output mapping. The value/origin scope rule is accepted, but no code/fixture work may start while CURRENT_STATE is design_stop.
---

# StringHelpers Loop-If ArrayPush value/origin scope I0

## Six-line brief

```text
Decision: make the selected StringHelpers.split_lines Loop→If ArrayPush use
the existing LoopCond/GeneralIf path with a branch scope for current values and
active origins, while all static source consumption and write obligations stay
monotonic across compile-time branch lowering.
Source authority + canonical issuer: resolver SourceExprSiteV1/BindingRefV1,
issue_source_core_method_calls_with_named_arrays_v1, and the existing
LoopCond Facts/Recipe/GeneralIf physical consumer.
Non-authority: names, AST rescans, MIR phi shape, runtime branch frequency,
post-Loop substring(last), compatibility lowerers, and shared raw MethodCall.
Fail-fast boundary: exact branch/continuation sites and BindingRefs must match;
value/origin scope close, join publication, named-array one-shot consumption,
and residual finish checks must all succeed before the function completes.
Smallest next slice: implement the existing-owner value/origin scope and prove
the one nested push with zero/one/multiple iteration and true/false branches.
Non-claims: no tail push, runtime Text/Fault ABI, serializer, VM/AOT parity, or
retirement of the shared push/set/insert writer.
```

## Implementation tuple

1. **Exact source relation.** Obtain the `arr`, `i`, and `last` BindingRefs from
   the same callable source ledger declaration/variable/assignment relations.
   Do not derive numeric bindings from declaration order. Keep the nested push
   at `Body(5)/LoopBody(1)/IfThen(0)` and the tail at `Body(6)/IfThen(0)` out of
   this row.

2. **Physical value scope.** Add an internal scope operation to
   `CallableSemanticLoweringState` and its existing dynamic-origin owner. On
   branch entry capture the exact current values for the source bindings; on
   branch reset restore those values and active origins; after the join publish
   the existing join ValueIds back to the same outer BindingRefs. Historical
   origin records remain monotonic.

3. **Join integration.** Thread the scope around the existing
   `lower_if_join_state_core` reset/publication order in
   `callable_loop_source_lowering.rs`. Do not create a second branch solver or
   reselect `GeneralIf`. The implicit else branch keeps the entry value of
   `last`; the then branch rebinds `last = i + 1`.

4. **Loop continuation.** Connect the joined `i`/`last` values to the existing
   LoopCond carrier/phi owner for header, backedge, and exit publication. Do
   not use `publish_source_loop_final_value` as a generic origin publication;
   preserve exact BindingRef ownership and add the smallest owner-local method
   needed for active-origin join publication.

5. **Named write accounting.** Consume the nested ArrayPush row exactly once
   through `take_source_array_push`. Keep consumed rows, consumed-site sets,
   `named_array_writes`, and finish collection monotonic; runtime false/zero
   iterations do not roll back compile-time source obligations.

## Focused acceptance

Positive evidence must cover the selected source owner with zero, one, and
multiple possible loop iterations, both newline and non-newline paths, and the
joined `last` value reaching the next iteration and loop exit. The physical
write must still use the existing `NamedArrayWriteEmissionPortV1`.

Negative evidence must reject foreign branch sites, stale/foreign BindingRefs,
duplicate branch-scope close, duplicate assignment or ArrayPush consumption,
missing join publication, active-origin mismatch, and residual source/write
rows. Reuse existing terminals where applicable:
`source-site-parent-mismatch`, `recipe-source-mismatch`,
`duplicate-assignment-consumption`, `duplicate-core-method-call-consumption`,
`named-array-call-shape`, and `incomplete-consumption`. Add only owner-local
value/origin scope errors required to make branch close fail-fast.

## Retain/delete and closeout

The selected production route is already the source LoopCond/GeneralIf route;
this slice does not switch a new caller. The generic `MethodCall` writer remains
retained because raw and unrelated callers share it, so the delete set is
explicitly empty. Closeout requires focused positive/negative tests, a stable
structural guard, module README update, classified reds, and pointer sync.

The post-Loop `substring(last)` push, runtime Text/Fault ownership, native
backend acceptance, and serializer remain separate non-claims.
