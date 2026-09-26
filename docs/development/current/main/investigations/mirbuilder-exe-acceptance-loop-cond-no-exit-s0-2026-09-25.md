# MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-NO-EXIT-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D4
  (accepted — no-exit conditional loop is the shared `facts-absent`
  family; `ConditionalUpdateIf` parts arm is the next inventoried
  candidate)
Owner: workstream row H / unified resume gate 1
Authority: D4 Decision. Sole issuer stays
  `CallableGenericLoopSourceFactsIssuerV1::issue_once`; sole physical
  lane stays `loop_cond::issue` ->
  `lower_loop_cond_break_continue_source`. Recipe physicalization
  (`LoopWithExit{false,false,false}`) already models no-exit loops.

## Slice

1. `src/mir/builder/control_flow/plan/facts/`
   `loop_cond_no_exit_facts.rs` (new, mirroring
   `loop_simple_while_facts.rs` structure):
   `try_extract_loop_cond_no_exit_facts_with_projection(condition,
   body, source_projection)` returning
   `Result<Option<LoopCondBreakContinueFacts>, Freeze>`:
   - `is_true_literal(condition)` → `Ok(None)` (LoopTrue family owns);
   - `!is_supported_bool_expr_with_canon(condition, allow_extended)`
     → `Ok(None)`;
   - `count_control_flow_with_returns(body)` reporting ANY
     break/continue/return or nested loop → `Ok(None)`
     (LoopCondBreakContinue owns exit-bearing bodies; nested stays a
     named downstream boundary for S0);
   - reuse `build_loop_cond_break_continue_recipe(body, allow_nested=
     false, allow_extended, ...)` to mint the item recipe, then keep
     only items in {`Stmt`, `ProgramBlock{stmt_only: None}`,
     `GeneralIf`} — any other item kind → `Ok(None)`
     (`ConditionalUpdateIf`-bearing bodies already produce facts via
     `conditional_update_seen` upstream; `ExitLeaf`/`TailBreak`/
     `NestedLoopDepth1` and exit-ifs stay `facts-absent` or move to
     their existing named terminals);
   - emit `log_accept("loop_cond_no_exit", ...)` and
     `log_reject("loop_cond_no_exit", ...)` entries matching the
     existing reject-reason vocabulary;
   - construct `LoopCondBreakContinueFacts` directly with
     `accept_kind = LoopCondBreakAcceptKind::NoExitBody`,
     `has_handled_guard_break = false`, `handled_var_name = None`,
     `continue_branches = vec![]`, `body_exit_allowed = None`,
     `body_lowering_policy` matching the plain `Stmt`-block policy
     used by the break/continue extractor for stmt bodies.
2. `src/mir/builder/control_flow/facts/loop_cond_break_continue.rs`:
   add `LoopCondBreakAcceptKind::NoExitBody`; extend
   `release_allowed()` to keep `NoExitBody` non-release (conservative,
   same as `ProgramBlockNoExit`).
3. `src/mir/builder/control_flow/plan/features/loop_cond_bc.rs`:
   add the `NoExitBody` arm to `pin_accept_kind_contract`.
4. `src/mir/builder/control_flow/plan/facts/loop_builder.rs`
   (`try_build_loop_facts_inner`): invoke the new extractor AFTER the
   `loop_cond_break_continue` cluster/base evaluation and assign into
   the existing `loop_cond_break_continue` facts field — disjoint by
   construction (this extractor requires zero exit-signal items).
   Ordering precedent: cluster profiles already feed the same field.
5. `skeleton`/`features` still run through the existing `has_any` ->
   `try_extract_loop_skeleton_facts` path; a skeleton-less loop that
   produced facts stays a `Freeze::bug` as today.

## Overlap analysis (required, source-read)

- `loop(i < literal) { i = i + 1 }` keeps its current route: today
  `loop_simple_while` alone matches → `matched=[LoopSimpleWhile]` →
  `sole_family()=None` → `NonGenericOrOverlapping` → node route. With
  this slice the matched set becomes `[LoopSimpleWhile,
  LoopCondBreakContinue]` → still `sole_family()=None` → same node
  route. Zero behavior change.
- `loop(var < bound) { stmt* }` no-exit: today `facts=None` →
  `facts-absent`. With this slice `matched=[LoopCondBreakContinue]` →
  `sole_family()=LoopCondBreakContinue` → `loop_cond::issue` →
  `lower_loop_cond_break_continue_source`. This is the intended
  promotion.
- Exit-bearing or `ConditionalUpdateIf`-bearing bodies are claimed by
  `loop_cond_break_continue` upstream before this extractor runs; the
  extractor's item filter additionally rejects them defensively.
- `source_projection` availability: `source_loop_bridge::from_input`
  arms a forest projection for EVERY root loop site (shape-agnostic,
  exits may be empty), so `Armed` covers these sites; `Unarmed`/
  `BridgeAbsent` keep existing `LoopCondRouteRejected`-class
  terminals.

## Pins (required before close)

- Positive A (step-only variable bound): `loop(i < n) { i = i + 1 }`
  inside a selected static method compiles past
  `callable-loop/facts-absent`; emitted MIR shows header cond +
  backedge (MIR-JSON probe).
- Positive B (carrier + method-call body): `loop(i < capacity) {
  free.push(i); i = i + 1 }` on `ArrayBox`-shaped receivers compiles
  past the same terminal.
- Negative A: `loop(i < n) { if c { break } ; i = i + 1 }` still
  takes the existing `loop_cond_break_continue` extractor (accept tag
  / route unchanged).
- Negative B: a body whose recipe emits a non-covered item
  (`ConditionalUpdateIf`, `NestedLoopDepth1`, `TailBreak`,
  `ExitIf*`) keeps its existing terminal (`facts-absent` for pure
  no-exit shapes, or the established downstream terminal for
  exit-bearing shapes) — the extractor returns `Ok(None)`, never a
  partial product.
- Pin: `NoExitBody` is non-release under `release_allowed()`.

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`
  (or sibling) to pin: extractor name, its position after the
  `loop_cond_break_continue` evaluation in `loop_builder.rs`,
  `NoExitBody` accept-kind arms, the item-kind allowlist, and the new
  test names.

## Fail-fast boundary

- Extractor never fabricates facts: any unsupported condition, exit
  signal, nested loop, or non-covered item kind → `Ok(None)` → the
  loop keeps `facts-absent` (or its existing terminal).
- `pin_accept_kind_contract` must name `NoExitBody`; an unregistered
  accept kind remains a compile-time coverage failure, not a silent
  pass.

## Landed evidence (2026-09-25)

Implementation:
- `plan/facts/loop_cond_no_exit_facts.rs` —
  `try_extract_loop_cond_no_exit_facts` (condition/body only; the
  `source_projection` parameter from the draft was dropped because the
  extractor is acceptance-pure and the bridge arms every root loop).
- `loop_builder.rs`: runs only when the `loop_cond_break_continue`
  cluster/base result is `None` (disjoint by construction).
- `NoExitBody` added to `LoopCondBreakAcceptKind`, kept non-release in
  `release_allowed()`, pinned in `pin_accept_kind_contract`.
- `RejectReason::ExitSignalPresent` + `for_loop_cond_no_exit` handoff
  (ConditionIsTrue→LoopTrueBreakContinue, ExitSignalPresent→
  LoopCondBreakContinue, everything else OutOfScope).
- `build_loop_cond_break_continue_recipe` /
  `matches_parse_string2_shape` widened `pub(super)` ->
  `pub(in crate::mir::builder)` (recipe builder reuse, no behavior
  change).

Pins: 9/9 tests in `loop_cond_no_exit_facts::tests` —
`accepts_variable_bound_step_only_body`,
`accepts_carrier_method_call_body`, `rejects_break_body`,
`rejects_conditional_update_if`, `rejects_true_condition`,
`rejects_nested_loop`, `rejects_return_in_body`,
`pipeline_pins_no_exit_body_kind`,
`pipeline_keeps_exit_bearing_body_on_sibling`.
Guard section added to `mirbuilder_qualified_route_scope_guard.sh`
(green). Focused regression: 58/59 `loop_cond` tests pass; the single
red (`program_block_with_exit_signals_prefers_recipe_only`) reproduces
identically on the stashed parent — known baseline debt, not a
regression.

Fresh real-app receipt (debug `hakorune`, `selfhost_build.sh`):
- mimalloc-lite / boxtorrent-mini / binary-trees:
  `callable-loop/facts-absent` ->
  `callable-loop/route-not-front-selected
  LoopCondRouteRejected(SourceCallOutsideSelectedFamily)`
  at `HakoAllocPage.seedBlocks/0` / `BinaryTreesBench.iterationCheck/3`.
- json-stream-aggregator: unchanged at `loop-cond-item-unsupported`
  (`ConditionalUpdateIf`, F2's boundary).
- Suite count unchanged: 4 pass (3 typed-object mins + marker probe),
  7 fail. No green regressed.

Newly inventoried boundary (NOT this slice's family): the LoopCond
route token requires call-coverage evidence for body method calls —
exact-target "selected" static publications or resolver CoreMethod
rows (`raw_loop_child_port.rs::source_target_for_loop` ->
`issue_with_source_relations`). Observed sub-boundaries, all honest:
- call-free body (`loop(i<n){i=i+1}`) -> `SourceItemsMissing`
  (non-empty items check, `normal_callable_loop_source_route.rs:257`);
- instance/builtin calls only (`free_stack.push`, `builder.make`,
  `me.bump`, `.itemCheck()`) -> `SourceCallOutsideSelectedFamily`
  (`normal_callable_loop_source_route_items.rs:329`);
- singleton-covered static call -> `StaticResultPublicationResidual
  (UnconsumedSelected)` because `consume_publication` is keyed on
  `has_loop_break_composite_source_candidate`
  (`raw_loop_child_port.rs:88-95`).
These belong to the call-coverage authority, not the loop-facts
vocabulary; they are the D5 candidate family.

## Non-claims

- Does not claim LoopCondReady/MIR emission for no-exit loops — the
  call-coverage contract gates route issuance before that stage; only
  bodies with an issued covered call reach the lowerer.
- Does not claim json_stream_aggregator — its `ConditionalUpdateIf`
  item needs the separate parts-driver slice (D4 F2).
- Does not claim boxtorrent_mini/binary_trees full green.
- No node-route / winner-spine changes; `FactsAbsent` remains the
  named terminal for shapes outside every family.
- No new Facts type, no `Option` fallback, no `.hako` workaround, no
  change to `OrdinaryNew`/birth lanes.
- `SourceItemsMissing`/`SourceCallOutsideSelectedFamily`/
  `UnconsumedSelected` are kept as named terminals — no contract was
  relaxed to sneak past them.
