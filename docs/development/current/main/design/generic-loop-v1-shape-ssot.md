---
Status: SSOT
Scope: JoinIR / generic_loop_v1 shape detection (hint-only)
Related:
- Code SSOT: `src/mir/builder/control_flow/plan/generic_loop/facts/body_check.rs`
- Policy enum: `src/mir/policies/generic_loop_v1_shape.rs`
- Overlap policy: `src/mir/policies/generic_loop_overlap_policy.rs`
- Phase map: `docs/development/current/main/phases/phase-29bq/README.md`
- Acceptance SSOT: `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md`
---

# generic_loop_v1 ShapeId SSOT (hint-only)

This document defines the canonical shape IDs used by `generic_loop_v1` facts.
Shape IDs are a fixed vocabulary; overlap is a fail-fast error.

Important: ShapeId is **not** the acceptance truth. Acceptance is defined by
Recipe/Verifier (SSOT: `generic-loop-v1-acceptance-by-recipe-ssot.md`).

## Contract

- Shape detection is centralized in `detect_generic_loop_v1_shape`.
- If multiple shapes match, freeze with tag `ambiguous` (overlap is a BoxShape bug).
- Overlap decision is SSOT in `generic_loop_overlap_policy::classify_v1_shape_matches`.
- Facts store `shape_id` when a shape matched (coverage/diagnostic hint).
- In strict/dev + planner_required, `generic_loop_v0` extraction is disabled (v1-only).

## Operational rules (SSOT)

- ShapeId list is authoritative: add/remove shapes **only** by editing this list.
- Each new shape requires: **fixture + fast gate + Acceptance Map** update.
- `detect_generic_loop_v1_shape` must be exclusive:
  - 0 match → None
  - 1 match → ShapeId
  - ≥2 match → Freeze::ambiguous
- Freeze tag format: `[plan/freeze:ambiguous] generic_loop_v1: shape overlap: <shape,...>`
- Lower must not re-detect shapes; it may consume `shape_id` from Facts as a hint but must not
  re-derive acceptance/semantics from it.

## Acceptance rule (SSOT pointer)

planner_required の “受理” は ShapeId ではなく、Recipe/Verifier で決まる（SSOT参照）:
- `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md`

## .hako migration policy (v1-only)

- .hako selfhost/mirbuilder must implement **v1-only** loop acceptance.
- `generic_loop_v0` is **Rust-side compatibility only** and must not be auto-ported.
- If a v0 behavior is needed in .hako, prefer “Recipe composability” first.
  ShapeId is allowed only when it reflects a real skeleton-level distinction (see acceptance SSOT).

## Body-check mapping note (intentional)

- `generic_loop_v0` uses the v1 allowlist for release compatibility.
- `generic_loop_v1` keeps the v0 body check to stay conservative until shape SSOT stabilizes.

## ShapeId list (fixtures)

- `ParseBlockExpr`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_block_expr_min.hako`
  - Gate id: `selfhost_parse_block_expr_min`
- `ParseMap`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_map_min.hako`
  - Gate id: `selfhost_parse_map_min`
- `PeekParse`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_peek_parse_min.hako`
  - Gate id: `selfhost_peek_parse_min`
- `RewriteKnownItoaComplexStep`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_rewriteknown_itoa_complex_step_min.hako`
  - Gate id: `selfhost_rewriteknown_itoa_complex_step_min`
- `RewriteKnownTrimLoopCondAndMethodCall`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_rewriteknown_trim_loop_cond_and_methodcall_min.hako`
  - Gate id: `selfhost_rewriteknown_trim_loop_cond_and_methodcall_min`
  - Cond: AND (`<`/`>` compare with loop_var) + `_is_space(substring(...))` (shape checks condition)
  - Step: `loop_var = loop_var ± 1` (body_len=1)
- `ParseProgram2NestedLoopIfReturn`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_min.hako`
  - Gate id: `selfhost_parse_program2_nested_loop_if_return_min`
  - Cond: `loop_var < 1`
  - Body (outer): `Local(inner=0), Loop(inner: if inner==0 return 0; inner=inner+1), step`
  - Body (inner): `If(loop_var==0 return 0), step`
  - Step: `loop_var = loop_var + 1` (body_len=3 or 2)
- `ParseProgram2NestedLoopIfElseReturn`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_min.hako`
  - Gate id: `selfhost_parse_program2_nested_loop_if_else_return_min`
  - Cond: `loop_var < 1`
  - Body (outer): `Local(inner=1), Loop(inner: if inner==0 return 0 else return 0), step`
  - Body (inner): `If(loop_var==0 return 0 else return 0)` (body_len=1)
  - Step: `loop_var = loop_var + 1` (body_len=3)
- `ParseProgram2NestedLoopIfReturnVar`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_var_min.hako`
  - Gate id: `selfhost_parse_program2_nested_loop_if_return_var_min`
  - Cond: `loop_var < 1`
  - Body (outer): `Local(inner=0), Loop(inner: if inner==0 return inner; inner=inner+1), step`
  - Body (inner): `If(loop_var==0 return loop_var), step` (body_len=2)
  - Step: `loop_var = loop_var + 1` (body_len=3 or 2)
- `ParseProgram2NestedLoopIfReturnLocal`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_return_local_min.hako`
  - Gate id: `selfhost_parse_program2_nested_loop_if_return_local_min`
  - Cond: `loop_var < 1`
  - Body (outer): `Local(inner=0), Loop(inner: if inner==0 { local t=0; return t }; inner=inner+1), step`
  - Body (inner): `If(loop_var==0 { local t=0; return t }), step` (body_len=2)
  - Step: `loop_var = loop_var + 1` (body_len=3)
- `ParseProgram2NestedLoopIfElseReturnLocal`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_local_min.hako`
  - Gate id: `selfhost_parse_program2_nested_loop_if_else_return_local_min`
  - Cond: `loop_var < 1`
  - Body (outer): `Local(inner=1), Loop(inner: if inner==0 return 0 else { local t=0; return t }), step`
  - Body (inner): `If(loop_var==0 return 0 else { local t=0; return t })` (body_len=1)
  - Step: `loop_var = loop_var + 1` (body_len=3)
- `ParseProgram2NestedLoopIfElseIfReturn`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_if_return_min.hako`
  - Gate id: `selfhost_parse_program2_nested_loop_if_else_if_return_min`
  - Cond: `loop_var < 1`
  - Body (outer): `Local(inner=2), Loop(inner: if inner==0 return 0 else if inner==1 return 0 else return 0), step`
  - Body (inner): `If(loop_var==0 return 0 else if loop_var==1 return 0 else return 0)` (body_len=1, cond: `loop_var < 3`)
  - Step: `loop_var = loop_var + 1` (body_len=3)
- `ParseProgram2NestedLoopIfElseReturnVar`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_var_min.hako`
  - Gate id: `selfhost_parse_program2_nested_loop_if_else_return_var_min`
  - Cond: `loop_var < 1`
  - Body (outer): `Local(v=0), Local(j=1), Loop(inner: if inner==0 return 0 else return v), step`
  - Body (inner): `If(loop_var==0 return 0 else return v)` (body_len=1)
  - Step: `loop_var = loop_var + 1` (body_len=4)
- `WhileCapAccumSum`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_while_cap_min.hako`
  - Gate id: `while_cap`
  - Variant: optional leading `local` init + optional effect-only stmt before step
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_phi_injector_collect_phi_vars_len_loop_min.hako`
  - Gate id: `phi_injector_len_loop`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_phi_injector_collect_phi_vars_k_loop_no_exit_min.hako`
  - Gate id: `selfhost_phi_injector_k_loop_no_exit_min`
- `UsingCollectorLineScan`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_usingcollector_loop_full_min.hako`
  - Gate id: `selfhost_usingcollector_loop_full_min`
- `ScanAllBoxesNextI`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_scan_methods_loop_min.hako`
  - Gate id: `selfhost_blocker_scan_methods_loop_min`
- `DecodeEscapesLoop`
  - Fixture: `apps/tests/phase29bq_selfhost_blocker_decode_escapes_loop_min.hako`
  - Gate id: `selfhost_decode_escapes_loop_min`
  - Note: inner hex-scan loop (body_len=6; Local, Local, If, If, Assign, Assign).
- `DivCountdownBy10`
  - Fixture: `apps/tests/phase29bq_div_countdown_by10_min.hako`
  - Gate id: `div_countdown_by10`
  - Cond: canon side extracts loop_var from `v > 0` or `v >= 1` (shape does not check)
  - Step: `v = v / 10` (10 fixed)
  - Body[0]: `local d = v % 10` (10 fixed)
  - Body len: 4 (modulo local, effect local, effect assign, division step)
  - Note: int_to_str pattern for digit extraction.
- `ScanWhilePredicate`
  - Fixture: `apps/tests/phase29bq_funcscanner_extract_ident_min.hako`
  - Gate id: `funcscanner_extract_ident_min`
  - Cond: compound AND `(pos < n) && (method_call() == 1)` (canon side handles AND)
  - Step: `pos = pos + 1` at last position
  - Body len: 4 (local_decl, method_call, method_call, step)
  - Note: scan-while-predicate pattern for identifier extraction.
- `EffectStepOnly`
  - Fixture: `apps/tests/phase29bq_funcscanner_append_defs_min.hako`
  - Gate id: `funcscanner_append_defs_min`
  - Cond: loop_var candidates extracted by ConditionCanon (shape does not check condition)
  - Step: `loop_var = loop_var + 1` at last position
  - Body len: 2 (method_call, step)
  - Note: minimal effect-only loop for collection iteration.
