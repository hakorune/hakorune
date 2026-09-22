---
Status: fast__named_array_state_negative_matrix
Task: MIR-CALL-PARSER-ARRAY-PUSH-B2-NEGATIVE-I0
Parent: mir-call-parser-array-push-b2-i0-2026-09-23
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B2-CUTOVER-D0
Implementation permission: true for test-only negative coverage through the existing package install/loan bridge; do not switch a production caller or add a new semantic receipt
---

# ArrayPush BindingRef negative matrix B2-NEGATIVE-I0

## Six-line brief

```text
Decision: finish the negative evidence for the already landed source-to-physical
carrier projection and retained ArrayPush seam before selecting a caller switch.
Source authority + canonical issuer: the existing
CallableGenericLoopSourceRelationViewV1 and CallableSemanticLoweringState rows,
consumed by CallableLoopSourceExpressionPortWithRelationV1.
Non-authority: labels, method names, MIR scans, route precedence, fallback,
synthetic BindingRef->ValueId tables, and runtime/serializer observations.
Fail-fast boundary: foreign owner/session/site, missing physical carrier,
duplicate or missing source item, value-demanded or malformed receiver/arity,
and every residual ledger row must stop before a downstream consumer.
Smallest next slice: add one named test for each remaining state-owned negative,
reuse the existing owner/session and named-array completion checks, then run the
carrier guard and focused GenericLoop/package suites.
Non-claims: no new Verified*/Prepared* receipt, no production caller switch,
no runtime Text lifetime, no VM/AOT/platform parity, and no old-edge deletion.
```

## Accepted fixture decision

The missing callable-state fixture authority is resolved without a production
hook. Tests must issue the real source-backed package with
`issue_normal_callable_semantic_package_with_brand_catalog_v1`, call
`prepare_install(...).commit()`, open `begin_lowering(...)`, and consume the
selected row through
`with_selected_lowering_input_and_core_methods(...)`. The callback's selected
input and borrowed source rows are the only inputs to
`CallableSemanticLoweringState::from_exact_source_with_dynamic_source_and_core_methods`.
This keeps source, owner, session, and row identity co-sealed and permits no
private-field mutation or synthetic row construction.

The fixture helper belongs in the existing Builder state test family (split at
760 lines if needed). It must install the existing entry values before reading
the ArrayBox receiver, and it must return the exact `SourceExprSiteV1` for the
named ArrayPush row plus the state under test. A second package instance is the
foreign-session witness; it must never lend rows to the first state's source.

## Current boundary and evidence

The preceding B2-I0 slice landed the relation-aware source port, the exact
source `ArrayPush` handoff, final induction publication through the existing
callable ledger, and the reusable structural guard. Positive evidence covers
literal, substring, and two-push loop rows with optimization both off and on;
the retained typed write and existing C-frame query are observed. The value-
demanded source form and a missing physical carrier label are terminal tests;
the non-carrier BindingRef projection is unmapped, and the existing
physical-adapter foreign-owner test remains in the boundary.

This card closes the remaining negative inventory. A green positive compile is
not evidence that a duplicate source row, receiver/arity drift, or a residual
ledger row is rejected, so each row gets a named test or an explicit existing
owner test reference before this card can close.

## B2 negative task queue

1. **Foreign session and site** — keep the existing owner mismatch test and add
   the smallest source-session/site drift fixture at the adapter boundary. The
   rejection must occur before Composer, PlanLowerer, or Builder effects.
2. **Physical carrier** — retain the missing-label test and cover a foreign
   BindingRef that is not a carrier without turning it into a name lookup. The
   missing-label and non-carrier tests now close this row.
3. **Source-item consumption** — add the package-loan-backed named-array state
   fixture and test duplicate exact-site consumption plus a required ArrayPush
   with no matching source row. The existing
   `source_core_method_take_rejects_duplicate_exact_site` remains shared-owner
   evidence; the new state test must prove the ArrayPush-specific row through
   the real selected input and borrowed row set.
4. **Call shape** — pass only a wrong method or arity at the state consumer and
   require `named-array-call-shape`; do not mutate private contracts or broaden
   the production matcher. The upstream
   `selected_array_contract_rejects_reassignment_value_demand_and_non_text`
   test covers the source target boundary, but not the state consumer's final
   shape branch.
5. **Finish** — leave one source read, rebind, or named write unconsumed in the
   package-loan-backed state and assert the existing ledger finish rejects the
   residual. Reuse `finish_with_named_arrays`; the existing
   `package_inventory_rejects_dropped_draft_and_allocation_without_write`
   covers the emission collector, while the callable-state residual fixture
   remains open. Do not add a second residual scanner.
6. **Closeout** — run the B2 carrier guard, source-route guard, current-state
   pointer guard, and focused suites. Record the exact test names and classify
   any red as current-change or baseline debt before choosing cutover design.

## Explicit stop rules

Do not make missing negatives permissive with `Ok(None)`, default values, or a
fallback route. Do not create a new semantic receipt, physical carrier map,
runtime ownership proof, or production caller switch in this card. If a named
negative cannot be constructed through the existing owner, record the missing
fixture authority and return to design stop rather than manufacturing one.

The former fixture gap is closed by the accepted package install/loan bridge
above. Missing-row behavior must be observed through the required caller or
collector: the raw `take_source_array_push` API may return `Ok(None)` for an
unmatched site, so that result alone is not a named rejection and cannot close
the row.

The next card after this matrix is a design decision for the one selected
production caller and its old-edge retirement. It is not implied by local green
tests in this card.
