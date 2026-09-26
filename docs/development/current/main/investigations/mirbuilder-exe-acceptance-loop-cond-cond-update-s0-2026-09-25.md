# MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-COND-UPDATE-S0

Status: active__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D5
  (accepted — `SourceCallOutsideSelectedFamily` forks into two sealed
  coverage lineages; F2 `ConditionalUpdateIf` parts arm is the
  selected bounded slice)
Owner: workstream row H / unified resume gate 1
Authority: D5 Decision. Sole physical owner stays the located-source
parts driver `lower_loop_cond_source_item`; the lowering reuses the
existing conditional-update/Select owner via its port-aware facade.
No new recipe type, no new route, no Facts change.

## Slice

1. `src/mir/builder/control_flow/plan/parts/associated_source/`
   `callable_loop_source_items.rs` — add the
   `LoopCondBreakContinueItem::ConditionalUpdateIf` arm:
   - locate the `if` statement via `port.body_stmt(body,
     if_stmt.index())` (same pattern as `ExitIfTree`);
   - require `port.stmt_syntax(&source)` is `ASTNode::If` and its
     `else_body.is_some()` matches the recipe's `else_body.is_some()`
     (`RecipeBodyMismatch` otherwise);
   - locate the condition via `port.child_expr_from_stmt(&source,
     ExprChildRoleV1::IfCondition)` and verify
     `require_condition_view_match(cond_view, port.expr_syntax(&condition))`;
   - locate then/else carriers via
     `port.child_body_from_stmt(&source, BodyChildRoleV1::IfThen /
     IfElse)`;
   - drive `try_lower_conditional_update_if_input(port, builder,
     current_bindings, carrier_phis, carrier_step_phis,
     carrier_updates, Some(break_phi_dsts), condition, &then_carrier,
     else_carrier.as_ref(), error_prefix)`;
   - `Ok(None)` from the facade is a recipe-vs-source divergence ->
     named reject (`loop-cond-item-conditional-update-mismatch`),
     never a silent fallthrough;
   - the facade derives branch exits from the located statements;
     verify they equal the recipe's `then_exit`/`else_exit`
     (`ExitKind::Break{1}`/`Continue{1}` -> `CoreExitPlan::Break(1)`/
     `Continue(1)`, `None` -> `None`); mismatch -> named reject;
   - recipe `then_body`/`else_body` `None` must correspond to a
     located branch whose only content is the tail exit (or empty);
     a `None` recipe branch over a non-empty located branch is a
     named reject.
2. No changes to `conditional_update.rs` unless the facade lacks a
   needed hook — if so, extend the existing facade rather than
   bypassing it.

## Overlap analysis (required, source-read)

- `ConditionalUpdateIf` today hits the wildcard `_` arm ->
  `loop-cond-item-unsupported`. The arm is additive; no existing
  variant arm is touched.
- Exit-bearing conditional updates (`if c { x = 1; break }`) are
  produced by `loop_cond_break_continue` for exit-bearing bodies; the
  same arm covers them because the facade already handles
  `CoreExitPlan` tails via `attach_phi_args_if_continue_or_break`.
  This widens the located driver's vocabulary by exactly one item
  kind.
- `GeneralIf`/`ProgramBlock` arms keep their owners; the recipe
  builder decides which item kind an `if` statement becomes — the
  arm never re-classifies.
- The `NoExitBody` extractor rejects `ConditionalUpdateIf` bodies
  upstream; the arm serves the sibling exit-free
  `conditional_update_seen` facts path and exit-bearing bodies
  equally.

## Pins (required before close)

- Positive A (json shape): `loop(start < n) { if end < 0 { end = n }
  ; ... }` compiles past `loop-cond-item-unsupported`; the
  conditional update emits `Select`/carrier-update effects
  (MIR-JSON probe).
- Positive B (exit-bearing): `loop(i < n) { if c { x = 1; break }
  i = i + 1 }` lowers through the new arm with the break phi args
  attached.
- Negative A: a `ConditionalUpdateIf` whose recipe `else_body`
  presence disagrees with the located `if` rejects at the named
  boundary, not via the wildcard.
- Negative B: an unsupported branch statement inside the located
  then/else keeps the facade's reject; the driver surfaces it as the
  existing named error.
- Focused tests pin the arm at the `lower_loop_cond_source_item`
  level (recipe-first, located source).

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`
  (or sibling) to pin: the `ConditionalUpdateIf` arm in
  `callable_loop_source_items.rs`, its `require_condition_view_match`
  gate, the facade reuse (`try_lower_conditional_update_if_input`),
  and the new test names.

## Fail-fast boundary

- `Ok(None)` from the facade, recipe/source mismatches, unsupported
  branch statements, or unsupported exit kinds all map to named
  rejects under `{SOURCE_PARTS_ERR}` — never silent skipping, never
  partial plans.

## Evidence (landed — 2026-09-25)

Implementation:
- `callable_loop_source_items.rs` gained the
  `LoopCondBreakContinueItem::ConditionalUpdateIf` arm. It locates the
  `if` via `port.body_stmt`, requires `ASTNode::If`, checks else parity
  with `else_body.is_some() || else_exit.is_some()` (an AST else holding
  only a tail exit records `else_body = None` + `else_exit = Some(..)`),
  verifies `CondBlockView` via `require_condition_view_match`, seals
  branch body/exit shape via the new `verify_cond_update_branch_recipe`
  helper, then reuses the existing port-aware facade
  `try_lower_conditional_update_if_input`. `Ok(None)` maps to the named
  terminal `loop-cond-item-conditional-update-unsupported`; seal
  mismatches map to `RecipeBodyMismatch`. No facade changes were needed.

Focused tests (`callable_loop_source_items_tests.rs`, 18/18 pass):
- `source_item_lowers_conditional_update_if_through_located_branches`
  (no-else `if i == 0 { j = 1 }` -> `Select` plan, binding updated)
- `source_item_lowers_conditional_update_if_with_tail_break`
  (`{ j = 1; break }` -> `ExitIf{BreakWithPhiArgs{depth:1}}`)
- `source_item_lowers_conditional_update_if_with_tail_continue`
  (`{ j = 1; continue }` -> continue exit plan)
- `source_item_lowers_conditional_update_if_with_exit_only_else`
  (`else { break }` recipe = `else_body:None` + `else_exit:Break{1}` —
  the key parity case)
- `source_item_rejects_conditional_update_if_else_parity_drift`
- `source_item_rejects_conditional_update_if_exit_parity_drift`
- `source_item_rejects_conditional_update_if_unsupported_shape`
  (duplicate-target branch -> facade `None` -> named terminal)

Facade note: a branch holding *only* a tail exit with no update anywhere
(`if c { continue }` with no else) is declined by the facade
(`has_any_assignment_input` requires one update); the recipe issuer does
not classify that shape as `ConditionalUpdateIf`, so the arm's `None ->
named reject` path is the correct surface.

Real-app measurement (debug binary `target/quick/hakorune`,
`NYASH_BIN` override):
- json-stream-aggregator advanced past
  `callable-loop/parts loop-cond-item-unsupported` to
  `[freeze:contract][callable-semantic-lowering/incomplete-consumption]
  variables=11/12 missing_variables=[Body(2),LoopBody(2),Receiver]` —
  the `me` receiver site of `me.ingestLine(stream.substring(..))` in the
  `ingest` loop body. That is the receiver-consumption / instance-call
  coverage family (D5 fork B, parked DeclaredInstance lane), outside
  this slice.
- Suite: 4 pass / 7 fail — unchanged count, no green regression.
  - binary-trees / boxtorrent-mini / mimalloc-lite: unchanged
    `SourceCallOutsideSelectedFamily` first-stops (coverage forks).
  - allocator-stress: unchanged `NamedArray(TextSourceMissing)`.
  - typed_object_untyped_field: unchanged MIR-emit failure.
  - typed_object_newbox_min: unchanged LLVM `opt` type error.
- `loop_cond` focused regression: 78/79 — the single failure
  `program_block_with_exit_signals_prefers_recipe_only` reproduces on
  the parent baseline (known debt, documented in LOOP-COND-NO-EXIT-S0).
- Guards: `current_state_pointer_guard.sh` ok,
  `mirbuilder_qualified_route_scope_guard.sh` ok, `git diff --check`
  clean.

## Non-claims

- Does not claim json_stream_aggregator green — after this arm the
  same loop still needs `me.ingestLine`/`substring` emission and the
  `find` publication consumption (`UnconsumedSelected`, F3b).
- Does not claim binary_trees/boxtorrent/mimalloc progress — their
  first-stop loops sit behind the sealed coverage forks.
- No coverage-contract change; `SourceItemsMissing`,
  `SourceCallOutsideSelectedFamily`, `UnconsumedSelected` remain
  named terminals.
- No new recipe variant, no Facts extractor change, no fallback.
