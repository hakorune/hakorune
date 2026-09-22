---
Status: closeout__selected_arraypush_ingress_delete_set_empty
Task: MIR-CALL-PARSER-ARRAY-PUSH-B2-CUTOVER-D0
Parent: mir-call-parser-array-push-b2-negative-i0-2026-09-23
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-D0
Implementation permission: false; design and finite census only. Do not switch a caller, add a semantic receipt, or delete shared MethodCall code in this card.
---

# ArrayPush selected ingress and old-edge cutover design D0

## Six-line brief

```text
Decision: treat the normal ArrayPush source path as already selected at the
existing NamedArray issuance/effect boundary, then design one exact census for
the remaining source-to-generic MethodCall edge before any deletion claim.
Source authority + canonical issuer: issue_source_core_method_calls_with_named_arrays_v1
and its exact take_source_array_push rows, consumed by
loop_body_lowering_associated_input::lower_method_call_statement_input.
Non-authority: Scan.run fixture names, push strings, receiver MIR types, MIR
scans, backend queries, compatibility probes, and the existence of old helpers.
Fail-fast boundary: selected obligation missing/foreign/duplicate/shape drift
must stop at the named source/package owner; unrelated source calls must not be
rejected merely because their selected ArrayPush row is absent.
Smallest next slice: census six finite ArrayPush rows, trace missing-row flow,
and freeze the exact retain/delete inventory for the shared MethodCall edge.
Non-claims: no caller switch, no new receipt, no StringHelpers/If expansion,
no raw push/set/insert retirement, no runtime Text/Fault proof, and no VM/AOT
or serializer acceptance.
```

## Corrected current boundary

The normal source path is not waiting for a fresh production switch. When the
brand catalog is present, `normal_callable_semantic_package/issuer.rs` already
selects the named-array issuance, and
`control_flow/plan/normalizer/loop_body_lowering_associated_input.rs` already
returns `CoreEffectPlan::NamedArrayPush` for the exact source statement. The
B2 negative card proves the relation/session/row boundaries around that path.
The next design must therefore distinguish a completed selected ingress from a
remaining generic sibling; it must not report the existing branch as an
unimplemented caller switch.

## Authority and selected caller

The finite production boundary is the source statement lowerer
`lower_method_call_statement_input()` and the package-owned
`take_source_array_push()` obligation. The caller is the existing installed
`RawInvocationChildPortV1` loop path, reached from
`PreparedLocatedRawLoopChildEntryV1::lower_v1_with_root_scope_and_callable_ledger`.
The Rust test fixture is evidence for the owner and relation only; it is not a
production caller. Compatibility/VM/AOT ports remain outside this design.

The candidate old edge is the generic MethodCall continuation after a selected
ArrayPush is not consumed:

```text
selected source site
  -> lower_method_call_statement_input
  -> CoreEffectPlan::MethodCall
  -> effect_emission.rs MethodCall
  -> receiver_is_array_like / try_emit_known_array_method_write
```

This is a candidate inventory, not a delete authorization. The final
`push|set|insert` branch is shared by raw and other callers, so it remains
retained unless the census proves a caller-zero exclusive symbol. If the
selected normal cohort never enters this continuation, the physical delete set
is empty and must be recorded as empty rather than manufactured by adding a
wrapper.

## B2-CUTOVER-D0 task queue

1. **Record the already-landed selection.** Cross-check the issuer branch and
   the `NamedArrayPush` early return against the B2 positive tests. Record that
   this is a source-to-physical selection checkpoint, not a new caller switch.

2. **Build the finite selected census.** Map literal, substring, and two-push
   source rows with optimization off/on from exact source site to
   `take_source_array_push`, `NamedArrayPush`, `ArrayElementWrite`, retained
   obligation, and package finish. The six rows must identify one owner and one
   site each; names and MIR shape cannot substitute for a row.

3. **Trace missing-row behavior.** Starting from a selected source site, show
   whether a missing row can reach generic MethodCall, which existing owner
   stops it, and whether Builder state remains untouched. The direct
   `take_source_array_push` `Ok(None)` result is not a rejection by itself;
   only the required caller/finish terminal closes this row. Do not turn every
   unrelated source call into an ArrayPush error.

4. **Freeze the retain/delete inventory.** Enumerate production, test-only, and
   public-contract callers for `lower_method_call_statement_input`, the
   `CoreEffectPlan::MethodCall` ArrayPush-like branch, and
   `effect_emission.rs`'s known-array writer. Mark each shared caller and set
   the delete set to the exact exclusive symbols, or explicitly
   `DeleteSetEmpty__SharedMethodCallStillLive`.

5. **Choose the next real application owner.** `StringHelpers.split_lines` has
   a push inside `If` and a substring tail push outside the loop. It is outside
   the current straight-line B2 scope. Keep it parked until If/LoopCond source
   admission and its continuation owner are designed; do not use it as proof
   for this card.

6. **D0 exit and successor.** Accept only when the six-row census, missing-row
   terminal, caller inventory, exact retain/delete set, and reusable guard are
   recorded. Any unresolved source-to-generic relation is `NoSafeSlice`, not a
   fallback or a permissive `Ok(None)`. The successor I0 may implement one
   already-mapped edge only after the authority chain and delete set are closed.

## Read-only D0 census and decision

The six positive rows are the three source bodies in
`published_backend_view/named_array_source_tests.rs`—literal text, a
`substring(0, 1)` result, and two literal pushes—under optimization off and on.
Each row reaches the existing package issuer, the exact
`CallableLoopSourceExpressionPortWithRelationV1::exact_source_statement_call`
take, `CoreEffectPlan::NamedArrayPush`, one retained `ArrayElementWrite`, the
named-array validation, and the existing C-frame query. No row enters the
generic MethodCall emitter. The value-demand negative remains terminal before
the published consumer.

The missing-row branch is also bounded. `take_source_array_push` returns
`Ok(None)` only when the exact site has no named-array row (or the source port
is the raw/default port); that is the legitimate generic MethodCall family for
unselected calls. A present named row with wrong method or arity is a named
`named-array-call-shape` error, and package finish rejects an unconsumed row or
unemitted write. The installed brand-catalog package issuer is the only
production source of the selected row, so this census found no production path
that silently loses a selected ArrayPush row.

The generic MethodCall helper has three shared normalizer callers: the raw
facade, LoopCond utility, and GenericLoop direct-associated body. Its physical
known-array writer is shared by `effect_emission.rs`, `unified_emitter.rs`,
and `boxcall_emit.rs`. Because those callers serve raw and unrelated methods,
there is no exclusive physical delete candidate for the selected B2 cohort.
The D0 delete decision is therefore
`DeleteSetEmpty__SharedMethodCallStillLive`; no wrapper or synthetic caller is
created to make deletion appear non-empty.

The next real source candidate is `StringHelpers.split_lines`: its push inside
the Loop's `If` is outside the straight-line B2 body, while its substring tail
push is outside the Loop entirely. Both remain excluded here and are handed to
`MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-D0` for a separate source/LoopCond
authority decision.

## Required evidence and explicit stops

The D0 design must name the exact focused tests and guards from B2, plus a
source-level guard that pins the issuer, the `NamedArrayPush` early return, and
the absence of a name-based generic fallback for selected rows. It must classify
all reds as current-change, known baseline debt, or informational census.

Do not add `Verified*`/`Prepared*` semantics, a second physical carrier map,
new receiver/name classifiers, a runtime serializer claim, a production
caller switch, or a deletion merely to improve the deletion ratio. This D0
closes with the explicit empty delete set above; the successor is design-only
until the LoopCond/If source owner is named.
