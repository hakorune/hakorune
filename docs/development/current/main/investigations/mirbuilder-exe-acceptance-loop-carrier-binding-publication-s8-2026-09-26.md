# MIRBUILDER-EXE-ACCEPTANCE-LOOP-CARRIER-BINDING-PUBLICATION-S8

Status: landed
Date: 2026-09-27
Parent: workstream row H (unified resume, gate 1);
  MERGE-PHI-DOMINANCE-D20 decision accepted
Mode: fast — one responsibility, one production edge.

## Responsibility

Publish loop-carrier phis through the **same binding authority the
lowering reads actually resolve**. Carrier header/step/after phis
are already emitted and patched by
`LoopCondBreakContinuePhiMaterializer` + the `loop_lowering` phi
lifecycle, but loop-header/body/exit reads resolve through the
binding-id SSA store (`binding_ctx` / `MirBindingSsaAdapterV1`)
rather than the `variable_map` entries `finalize_loop_variables`
publishes — so the phi dsts are dead on arrival and
`prune_unused_phi_instructions`/DCE strips them.

## Boundary

- One bounded question first: which resolver do carrier reads use
  in the failing lane — `build_variable_access`/`variable_map` or
  `binding_ctx` BindingId→value — and where does carrier
  publication need to land for header (body reads), after (exit
  reads), and step/backedge transport.
- Both lanes converge on `lower_loop_generalized`; the fix lives in
  the shared binding publication, not in either entry port.
- No verifier change, no dialect change, no optimizer touch, no
  route addition or removal.
- The `CarriersOnly` edge-args transport stays the declared edge
  vocabulary; populated values are the missing wiring, not a new
  layout.

## Acceptance

1. Focused negative pin: minimal single-carrier `loop(cond)` with a
   post-loop read — emitted exit read binds the after-phi dst, not
   the body's last-writer.
2. Focused negative pin: header phi inputs are
   {preheader: init, backedge: step/carried}, and body reads bind
   the header phi dst — not the pre-loop value.
3. `find/3`-shape: the in-body carrier update is transported (no
   dead update statement, non-empty carrier transport on the
   backedge/step path).
4. `NYASH_BIN=target/debug/hakorune`
   `hakorune --backend mir --emit-mir-json
   apps/json-stream-aggregator/main.hako` — the 13-violation SSA
   family closes or the residual narrows to a named next family.
5. Pointer + workstream guards green; receipts/docs synced.

## Non-claims

- No claim this closes overall MirBuilder; Gate 1 evidence only.
- No verifier weakening, no LoopBuilder revival, no env-gated
  no-phi dialect.
- No new Recipe/authority minted: the binding publication wires
  into the existing read resolver, whichever the census selects.

## Evidence

- Read authority (D20 census): armed lanes resolve every source
  `Variable` through `CallableSemanticLoweringState.values[binding]`
  (`exact_source_variable_value` / `read_variable`); `variable_map`
  is a second store the ledger reads never consult.
- `normal_callable_semantic_lowering_state.rs`: `binding_names`
  index built from resolver-owned declaration records;
  `publish_source_loop_final_value_named` resolves a carrier name to
  exactly one materialized binding — unknown or ambiguous names
  freeze. `publish_source_loop_final_value` now invalidates the
  tracked dynamic origin before replacing `values[binding]`.
  `source_values_snapshot`/`restore_source_values` give the `values`
  projection the same transaction discipline `variable_map` already
  had; consumption receipts and `active_origins` stay monotone.
- `normal_callable_loop_source_port.rs`: the port forwards
  `publish_loop_carrier_value` / `source_values_snapshot` /
  `restore_source_values` to the ledger — one authority, no second
  store.
- `parts/loop_/loop_v0.rs`: `lower_loop_v0_core` takes
  `Option<CallableLoopSourceExpressionPortV1>`; header carrier phis
  and `final_values` after-phi dsts publish into the ledger when
  `Some`. The raw `lower_loop_v0` facade passes `None`; the armed
  callable hooks pass `Some(port)` — the fix lives in the shared
  frame owner, not in either entry port.
- `features/loop_cond_bc_source.rs` + `loop_true_break_continue_source.rs`:
  publish `phi_bindings()` header dsts and `phi_closure.final_values()`
  into the ledger at the same points they fill `current_bindings` /
  `variable_map`.
- `callable_loop_source_lowering.rs`: branch-first state cores
  (`lower_exit_if_state_core`, `lower_if_join_state_core` via the
  source ports) snapshot `values` before branch lowering, restore
  before the alternate branch and before condition lowering, keep
  the continuing side's values for the continuation, and publish
  `CoreIfJoin` dsts back into the ledger for names that remain in
  `current_bindings`.
- `normal_default_pipeline.rs` (D19 wiring residual): the document
  consume now reports `verify_document_module`'s result instead of
  `result.verification_result`. The sealed-corridor domain arm is
  corridor admission the document lane never enforces
  (CATALOGED-CALL-EDGE-DOMAIN-D19); `verify_module` callers and the
  sealed result itself are unchanged.
- Focused pins (`normal_default_pipeline_loop_tests.rs`):
  `callable_source_return_in_body_loop_reads_bind_header_phi_dst`
  (find/3 shape — in-body compare and `return i` resolve a phi dst)
  and `callable_source_loop_exit_read_binds_after_phi_dst`
  (post-loop `return s` binds the after-phi dst).
- Production smoke:
  `hakorune --emit-mir-json /tmp/s8_final.mir.json
  apps/json-stream-aggregator/main.hako` → `MIR JSON written`;
  the 13-violation dominance/merge family is closed, 65 phis
  retained across 18 functions, `JsonLine.find/3` reads phi
  destinations (header `%18 = phi [46,bb312],[43,bb320]`; body and
  found-return bind `%18`; backedge `%43 = %18 + 1`).
- Surfaced baseline debt, classified: `verification_result`'s
  sealed invoke stage produced the `call-argument-type-drift`
  family before this slice — it was masked because
  `verify_document_module` aborted on dominance first. Classified
  as `known baseline debt` per D19; the document consume fix above
  implements the already-accepted decision.
- Known baseline debt, unrelated to this slice: the VM lane
  (`--backend vm`) declines `JsonStreamAggregator.ingest/1`'s loop
  at family selection (`zero selected family candidates`) — a
  recipe/winner-selection question upstream of carrier
  publication.
- Known baseline debt, unrelated to this slice (focused run:
  42 pass / 2 fail):
  - `source_loop_bridge::tests::nested_scope_loop_does_not_abort_callable_source_bridge`
    — asserts `CallableLoopSourceBridgeV1::from_input` stays
    unarmed on a nested-scope loop; the arm predicate and its
    whole call graph (`callable_source_ledger`,
    `issue_loop_cond_break_continue_source_forest_projection_v1`,
    `exact_stmt`) sit outside this diff.
  - `normal_default_root_catalog_lifecycle_tests::parser_scan_package_passes_callable_source_handoff_without_fallback`
    — `[freeze:contract][static-result-ingress/no-exact-static-target]`
    in the root-catalog lifecycle ingress contract, a module this
    slice does not touch.

## Follow-ups

- The sealed `verification_result` still carries the corridor
  domain arm for the artifact lanes by design; corridor admission
  for String-domain edges stays a later family.
- `ingest/1` family-selection decline on the VM lane.
