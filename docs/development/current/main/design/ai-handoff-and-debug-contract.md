Status: SSOT
Scope: JoinIR planner-required debugging + AI handoff (Phase-agnostic)
Related:
- Entry: `docs/development/current/main/10-Now.md`
- Gates SSOT: `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- Phase example: `docs/development/current/main/phases/phase-29bq/README.md`
- Reject/handoff SSOT: `docs/development/current/main/design/plan-reject-handoff-gap-taxonomy-ssot.md`
- Freeze tag SSOT: `docs/development/current/main/design/planfrag-freeze-taxonomy.md`
- BoxCount checklist SSOT: `docs/development/current/main/design/boxcount-new-box-addition-checklist-ssot.md`

# AI Handoff & Debug Contract (SSOT)

This doc defines the minimal, repeatable workflow for continuing JoinIR `planner_required` debugging across AI sessions.
It is designed to prevent “works on my machine” drift, SSOT mismatches (Facts vs StepTree/extractor), and mixed-scope commits.

## Non-negotiables

- No `.hako` workarounds to “pass gates”. Expand Rust-side acceptance via the smallest box/plan rule.
- No algebraic AST rewrites. Use analysis-only views (`CondBlockView`, canonical views) and conservative observation.
- `planner_required` must fail-fast: no silent fallback to legacy behavior.
- 1 blocker = 1 accepted shape = 1 fixture + fast gate = 1 commit (no mixed fixes).
- If a change expands an accepted shape, update observation SSOT in the same commit (StepTree / extractor / parity).

## Runtime identity (avoid “wrong binary”)

Always pin the root + binary explicitly when running canary/selfhost:

- Build: `cargo build --release --bin hakorune`
- Run: `NYASH_ROOT=/home/tomoaki/git/hakorune-selfhost NYASH_BIN=target/release/hakorune ...`

Do not rely on implicit `$NYASH_ROOT` / `$NYASH_BIN`.

## Debug loop (SSOT order)

1. Read the current blocker from the phase SSOT (`docs/development/current/main/phases/<phase>/README.md`) and the log path it points to.
2. From the log, identify the *actual failing function* (often a generated program function; the entry fixture is not the failing function).
3. Extract the StepTree/shape for that function and classify the failure:
   - **BoxCount**: planner returned `None` / `freeze:contract` because no box accepts the shape.
   - **BoxShape**: the shape should be accepted, but contracts/logs/SSOT are inconsistent or responsibility boundaries are unclear.
4. Create a minimal fixture that reproduces the shape (not the whole selfhost program), and pin it in:
   - `apps/tests/<fixture>.hako`
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`
5. Implement the smallest box/feature change to accept exactly that shape.
6. Re-run the single fast gate case; then re-run canary to move to the next blocker.

## Selfhost canary rerun rule (scan_methods_loop_min)

`phase29bq_selfhost_planner_required_dev_gate_vm.sh` は canary/health-check であり、稀に `scan_methods_loop_min` が落ちることがある。
この場合の運用は SSOT として固定する（迷走防止）。

- `scan_methods_loop_min` が FAIL したら **1回だけ** 同じコマンドを再実行する
- 2回目も FAIL する（再現する）場合は、`/tmp/*selfhost*scan_methods_loop_min*.log` を記録して **停止**（その場で別修正に進まない）

## Logging contract (keep logs short)

Prefer stable, structured tags over ad-hoc `eprintln!`:

- Reject logs: `reject: <reason> idx=<n> kind=<ASTKind> handoff=<target>`
- WASM route trace (dev/diagnostic only, `NYASH_WASM_ROUTE_TRACE=1`): `[wasm/route-trace] policy=<default|legacy-wasm-rust> plan=<native-shape-table|bridge-rust-backend|legacy-rust> shape_id=<id|->`
- WASM route policy freeze (parse boundary, default-only): `[freeze:contract][wasm/route-policy] NYASH_WASM_ROUTE_POLICY='<value>' (allowed: default)`
- `reject: exit_allowed_recipe_build_failed` means ExitAllowed recipe construction failed; treat as out-of-scope and update the recipe SSOT or accept shape.
- Contract freeze (Pattern6/7): `[joinir/phase29ab/scan_with_init/contract]`, `[joinir/phase29ab/split_scan/contract]`
- Generic-loop candidate diagnostics (dev/debug only): `[plan/reject_detail] box=generic_loop_v* reason=no_valid_loop_var_candidates last_fail=<Kind>`
- Nested-loop guard (dev/debug only): `[plan/freeze:nested_loop_guard] func=<...> span=<...> recipe_contract=<Some|None> route_kind=<...> depth=<...>`
- Recipe-first verification (dev/debug only, from `matcher.rs`): `[recipe:verify] route=loop_break status=<ok|fail>`
- Recipe-first verification (dev/debug only, from `matcher.rs`): `[recipe:verify] route=if_phi_join status=<ok|fail>`
- Recipe-first match (dev/debug only): `[recipe:match] kind=<...> break=<...> continue=<...> return=<...>`
- Recipe-first entry (dev/debug only): `[recipe:entry] <route>: recipe_contract enforced`
- Recipe-first entry (dev/debug only, Phase C4): `[recipe:entry] <route>: recipe-only entry`
- Recipe-first entry (dev/debug only, Phase C8): `[recipe:entry] if_phi_join: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C9): `[recipe:verify] route=loop_continue_only status=ok`
- Recipe-first compose (dev/debug only, Phase C9): `[recipe:compose] route=loop_continue_only path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C9): `[recipe:entry] loop_continue_only: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C10): `[recipe:verify] route=loop_true_early_exit status=ok`
- Recipe-first compose (dev/debug only, Phase C10): `[recipe:compose] route=loop_true_early_exit path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C10): `[recipe:entry] loop_true_early_exit: recipe-only entry`
- Recipe-first compose (dev/debug only): `[recipe:compose] route=<route> path=<recipe_block|recipe_first|direct_pipeline>`
- Recipe-first compose (dev/debug only): `[recipe:compose] route=if_phi_join path=recipe_block`
- Recipe-first verification (dev/debug only, Phase C11): `[recipe:verify] route=loop_simple_while status=ok`
- Recipe-first compose (dev/debug only, Phase C11): `[recipe:compose] route=loop_simple_while path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C11): `[recipe:entry] loop_simple_while: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C12): `[recipe:verify] route=loop_char_map status=ok`
- Recipe-first compose (dev/debug only, Phase C12): `[recipe:compose] route=loop_char_map path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C12): `[recipe:entry] loop_char_map: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C13): `[recipe:verify] route=loop_array_join status=ok`
- Recipe-first compose (dev/debug only, Phase C13): `[recipe:compose] route=loop_array_join path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C13): `[recipe:entry] loop_array_join: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C14): `[recipe:verify] route=scan_with_init status=ok`
- Recipe-first compose (dev/debug only, Phase C14): `[recipe:compose] route=scan_with_init path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C14): `[recipe:entry] scan_with_init: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C14): `[recipe:verify] route=split_scan status=ok`
- Recipe-first compose (dev/debug only, Phase C14): `[recipe:compose] route=split_scan path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C14): `[recipe:entry] split_scan: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C14): `[recipe:verify] route=bool_predicate_scan status=ok`
- Recipe-first compose (dev/debug only, Phase C14): `[recipe:compose] route=bool_predicate_scan path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C14): `[recipe:entry] bool_predicate_scan: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C14): `[recipe:verify] route=accum_const_loop status=ok`
- Recipe-first compose (dev/debug only, Phase C14): `[recipe:compose] route=accum_const_loop path=recipe_block`
- Recipe-first entry (dev/debug only, Phase C14): `[recipe:entry] accum_const_loop: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C15): `[recipe:scan_methods] verified OK`
- Recipe-first compose (dev/debug only, Phase C15): `[recipe:compose] route=scan_methods_v0 path=recipe_first`
- Recipe-first entry (dev/debug only, Phase C15): `[recipe:entry] scan_methods_v0: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C15): `[recipe:scan_methods_block] verified OK`
- Recipe-first compose (dev/debug only, Phase C15): `[recipe:compose] route=scan_methods_block_v0 path=recipe_first`
- Recipe-first entry (dev/debug only, Phase C15): `[recipe:entry] scan_methods_block_v0: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C15): `[recipe:scan_phi_vars] verified OK`
- Recipe-first compose (dev/debug only, Phase C15): `[recipe:compose] route=scan_phi_vars_v0 path=recipe_first`
- Recipe-first entry (dev/debug only, Phase C15): `[recipe:entry] scan_phi_vars_v0: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C15): `[recipe:scan_v0] verified OK`
- Recipe-first compose (dev/debug only, Phase C15): `[recipe:compose] route=scan_v0 path=recipe_first`
- Recipe-first entry (dev/debug only, Phase C15): `[recipe:entry] scan_v0: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C16): `[recipe:collect_using_entries] verified OK`
- Recipe-first compose (dev/debug only, Phase C16): `[recipe:compose] route=collect_using_entries_v0 path=recipe_first`
- Recipe-first entry (dev/debug only, Phase C16): `[recipe:entry] collect_using_entries_v0: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C16): `[recipe:bundle_resolver] verified OK`
- Recipe-first compose (dev/debug only, Phase C16): `[recipe:compose] route=bundle_resolver_v0 path=recipe_first`
- Recipe-first entry (dev/debug only, Phase C16): `[recipe:entry] bundle_resolver_v0: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C16): `[recipe:loop_true] verified OK`
- Recipe-first compose (dev/debug only, Phase C16): `[recipe:compose] route=loop_true_break_continue path=direct_pipeline`
- Recipe-first entry (dev/debug only, Phase C16): `[recipe:entry] loop_true_break_continue: recipe-only entry`
- Recipe-first verification (dev/debug only, Phase C17): `[recipe:loop_cond_break_continue] verified OK`
- Recipe-first verification (dev/debug only, Phase C17): `[recipe:loop_cond_continue_only] verified OK`
- Recipe-first verification (dev/debug only, Phase C17): `[recipe:loop_cond_continue_with_return] verified OK`
- Recipe-first verification (dev/debug only, Phase C17): `[recipe:loop_cond_return_in_body] verified OK`
- CondProfile observation (dev/debug only): `[condprofile] skeleton=<...> params=<n>`
- CondProfile log source: `verifier/mod.rs::debug_observe_cond_profile` (1 line, stable tag, debug only)
- CondProfile step mismatch (dev/debug only): `[condprofile:step_mismatch] step_k=<k> profile_step=<...>`
- CondProfile idx_var match (dev/debug only): `[condprofile:idx_var] facts=<name> profile=<name|missing> match=<true|false>`
- CondProfile step mismatch source: `verifier/mod.rs::debug_observe_cond_profile_step_mismatch`
  (1 line, stable tag, debug only)
- CondProfile completeness (dev/debug only): `[condprofile:complete]`
- CondProfile incomplete (dev/debug only): `[condprofile:incomplete] missing=<...>`
- CondProfile completeness source: `verifier/mod.rs::debug_observe_cond_profile_completeness`
  (1 line, stable tag, debug only)
- CondProfile priority (dev/debug only): `[condprofile:priority] {condprofile|legacy}`
- CondProfile priority source: `verifier/mod.rs::debug_observe_cond_profile_priority`
  (1 line, stable tag, debug only)
- CondProfile legacy fallback (dev/debug only): `[condprofile:legacy_fallback] route=<name>`
- Normalization fallback (dev/debug only): `[normalization/fallback] func=<...> reason=<...> err=<...>` (router/suffix_router)
- MIR verifier fail-fast (dev/debug only): `[mir/verify:phi_undefined] fn=<...> bb=<...> inst_idx=<...> used=<...>`
- MIR verifier fail-fast (dev/debug only): `[mir/verify:multiple_definition] fn=<...> value=<...> first_block=<...> second_block=<...> (optional: first_inst_idx=<...> first_inst=<...> first_span=<...>) (optional: second_inst_idx=<...> second_inst=<...> second_span=<...>) (optional: value_caller=<file:line:col|none>)`
- MIR verifier fail-fast (dev/debug only): `[mir/verify:dominator_violation] fn=<...> use_block=<...> def_block=<...> value=<...> (optional: kind=phi_input phi_block=<...> phi_inst_idx=<...> phi_dst=<...> phi_dst_caller=<file:line:col|none> bad_in_caller=<file:line:col|none>) (optional: kind=inst inst_idx=<...> (optional: copy_used_by_idx=<...> copy_used_by=<...>) (optional: value_caller=<file:line:col|none> copy_dst_caller=<file:line:col|none>)) (optional: def_inst_idx=<...> def_inst=<...>) (optional: def_cmp_*_const=<...> def_bin_*_const=<...>)`
- MIR verifier fail-fast (dev/debug only): `[mir/verify:invalid_phi] fn=<...> bb=<...> phi=<...> reason=<...>`
- LoopForm exit-phi fail-fast (strict/dev+planner_required; debug-only log): `[loopform/exit_phi:undefined_input] fn=<...> exit=<...> var=<...> pred=<...> incoming=<...> observed=<...>`
- If-form fail-fast (strict/dev+planner_required): `[freeze:contract][if_form:cond_def_block_mismatch] fn=<...> pre_branch=<...> def_block=<...> cond=<...> (optional: rhs_const="<...>")`
- If-join PHI incoming dominance (strict/dev+planner_required): `[freeze:contract][if_join/phi_incoming_not_dominating] fn=<...> merge_bb=<...> join=<name> dst=%<...> then_pred=<...> then_in=%<...> then_def_bb=<...> else_pred=<...> else_in=%<...> else_def_bb=<...> caller=<file:line:col>`
- If-join carry reset to init (strict/dev+planner_required; debug-only): `[if_join/carry_reset_to_init] fn=<...> merge_bb=<...> join=<...> dst=%<...> pre_in=%<...> pre_const=<...> pre_root=%<...|none> then_pred=<...> then_in=%<...> then_const=<...> then_root=%<...|none> else_pred=<...> else_in=%<...> else_const=<...> else_root=%<...|none> caller=<file:line:col>`
- LocalSSA arg fail-fast (strict/dev+planner_required): `[freeze:contract][local_ssa/non_rematerializable_arg] fn=<...> bb=<...> kind=Arg v=<...> def_kind=<...> def_block=<...> (optional: use=<...> operand=<...> dst=%<...> op=<...> emit_caller=<file:line:col>) (optional: varmap_hits=[...] pin=<name|none>) (optional: has_type=<...> has_origin_newbox=<...> reserved=<...> next_value_id_hint=<...>) (optional: def_blocks_has=<...> def_blocks_bb=<...>)`
- LocalSSA non-dominating use (strict/dev+planner_required; Phi input contract only): `[freeze:contract][local_ssa/non_dominating_use] fn=<...> bb=<...> kind=<Cond|CompareOperand> v=<...> phi_def_bb=<...> reason=<phantom_pred|missing_input|incoming_not_dominating> bad_pred=<...> bad_in=<...|none> in_def_bb=<...|None>`
- Cond lowering freshen contract (strict/dev+planner_required): `[freeze:contract][cond_freshen/unremapped_valueid] fn=<...> old=%<...> new=%<...> site=<...> caller=<file:line:col>`
- Cond lowering freshen remap mismatch (strict/dev+planner_required; debug-only): `[freeze:contract][cond_freshen/remap_mismatch] fn=<...> old=%<...> new=%<...> use_by=CoreEffectPlan::BinOp operand=<lhs|rhs> dst=%<...> op=<...>`
- Cond lowering freshen merge map conflict (strict/dev+planner_required): `[freeze:contract][cond_freshen/merge_map_conflict] fn=<...> old=%<...> new1=%<...> new2=%<...>`
- Loop lowering effect forward-ref (strict/dev+planner_required): `[freeze:contract][loop_lowering/effect_forward_ref] fn=<...> bb=<...> use=%<...> use_idx=<...> def_idx=<...> def_kind=<...> use_by=CoreEffectPlan::BinOp dst=%<...> op=<...> operand=<lhs|rhs> (optional: path=<block_effects|body_effects|body_fallthrough|body_plan|if_effect_then|if_effect_else>)`
- Loop lowering effect cross-block forward-ref (strict/dev+planner_required): `[freeze:contract][loop_lowering/effect_cross_block_forward_ref] fn=<...> use_bb=<...> use=%<...> use_idx=<...> def_bb=<...> def_idx=<...> def_kind=<...> use_by=CoreEffectPlan::BinOp dst=%<...> op=<...> operand=<lhs|rhs> (optional: path=<block_effects|body_effects|body_fallthrough|body_plan>)`
- Loop lowering effect cross-plan forward-ref (strict/dev+planner_required): `[freeze:contract][loop_lowering/effect_cross_plan_forward_ref] fn=<...> use_idx=<...> use=%<...> def_idx=<...> def_kind=<...> use_by=CorePlan::Effect dst=%<...> op=<...> operand=<lhs|rhs> path=body_plan`
- Loop lowering PHI input availability (strict/dev+planner_required): `[freeze:contract][loop_lowering/phi_input_not_available_in_pred] fn=<...> phi_tag=<...> phi_dst=%<...> phi_block=<...> pred=<...> incoming=%<...> incoming_def_bb=<...|None>`
- Loop lowering step-PHI input registration (strict+planner_required; debug-only): `[loop_lowering/step_phi_input:add] fn=<...> origin=<effect_emission|exit_lowering|plan_build|loop_lowering_merge> pred_bb=<...> dst=%<...> incoming=%<...> incoming_def_bb=<...|None> (optional: name=<...> caller=<file:line:col> phi_tag=<...>)`
- If-join PHI emission (strict+planner_required; debug-only): `[if_join/emit_phi] fn=<...> merge_bb=<...> join=<...> dst=%<...> then_pred=<...> then_in=%<...> else_pred=<...> else_in=%<...> caller=<file:line:col>`
- If-join payload creation (strict+planner_required; debug-only): `[if_join/payload] fn=<...> name=<...> dst=%<...> pre=%<...> then=%<...> else=%<...> span=<...> file=<...> pre_span=<...|unknown> then_span=<...|unknown> else_span=<...|unknown> caller=<file:line:col>`
- Plan lowering seq forward-ref (strict/dev+planner_required): `[freeze:contract][plan_lowering/seq_forward_ref] fn=<...> bb=<...> ctx=<seq|if_then|if_else> use_idx=<...> use=%<...> operand=<lhs|rhs> use_by=CoreEffectPlan::BinOp dst=%<...> op=<...> def_idx=<...> def_kind=<...> use_origin_span=<...|unknown>`
- Plan lowering seq undefined operand (strict/dev+planner_required): `[freeze:contract][plan_lowering/seq_undefined_operand] fn=<...> bb=<...> ctx=<seq|if_then|if_else> use_idx=<...> use=%<...> operand=<lhs|rhs> use_by=CoreEffectPlan::BinOp dst=%<...> op=<...> use_origin_span=<...|unknown> (optional: list_const_int3_dsts=[%...] list_const_int3_origin_spans=[...|unknown] list_has_use_const=<yes|no> list_add_binops=[...])`
- Loop lowering effect undefined operand (strict/dev+planner_required): `[freeze:contract][loop_lowering/effect_undefined_operand] fn=<...> bb=<...> use=%<...> use_idx=<...> use_by=CoreEffectPlan::BinOp dst=%<...> op=<...> operand=<lhs|rhs> plan_def=none (optional: path=<block_effects|body_effects|body_fallthrough|body_plan|if_effect_then|if_effect_else|effect_in_loop>) (optional: span=<...> span_start=<...> span_end=<...> file=<...>) (optional: use_origin_span=<...|unknown>)`
- PHI input contract skip unwired (debug-only): `[phi_input_contract:skip_unwired] fn=<...> use_bb=<...> v=%<...> phi_def_bb=<...> inputs=<n> (optional: phi0_pred=<...> phi0_in=%<...>)`
- PHI input contract skip provisional (debug-only): `[phi_input_contract:skip_provisional] fn=<...> use_bb=<...> v=%<...> phi_def_bb=<...> preds=<n> inputs=0 sealed=false`
- LocalSSA arg context (strict/dev+planner_required; debug-only): `[local-ssa/arg-context] fn=<...> bb=<...> kind=Arg v=<...> args=[...] params=[...] entry=<...>`
- Call arg scope fail-fast (strict/dev+planner_required): `[freeze:contract][call/arg_out_of_function_scope] fn=<...> call=<...> bb=<...> role=<recv|callee|arg[n]> v=<...> args=[...] params=[...] entry=<...> span=<line X, column Y> span_start=<...> span_end=<...> file=<path or unknown> mir_dump=</tmp/...|disabled|write_failed> undef_in_func=[%<id>...] first_undef_use=bb<id> inst=<...>|term=<...> used=%<id> undef0_varmap_hits=[...] undef0_pin=<name|none>`
- BinOp Add operand definition fail-fast (strict/dev+planner_required): `[freeze:contract][ops/binop_add:operand_not_defined] fn=<...> bb=<...> operand=<lhs|rhs> v=%<...> span=<line X, column Y> span_start=<...> span_end=<...> file=<path or unknown>`
- Builder emit BinOp operand fail-fast (strict/dev+planner_required): `[freeze:contract][builder/binop_operand_out_of_function_scope] fn=<...> bb=<...> op=<...> operand=<lhs|rhs> v=%<...> span=<line X, column Y> span_start=<...> span_end=<...> file=<path or unknown> mir_dump=</tmp/...|disabled|write_failed> caller=<file:line:col>`
- MIR diagnostics helper contract (strict/dev+planner_required): `builder_emit.rs` / `control_flow.rs` / `phi_helpers.rs` の freeze は `crate::mir::diagnostics::FreezeContract` を使い、`caller=` は `caller_string(...)`、`mir_dump=` は `mir_dump_value(...)` を通す（手組み禁止）。
- JSON v0 bridge copy dominance (strict/dev+planner_required): `[freeze:contract][json_v0_bridge/non_dominating_copy] fn=<...> bb=<...> src=<...> def_block=<...> op=<emit_copy|merge_edge_copy>`
- Call arg scope provenance (strict/dev+planner_required; debug-only): `[call/arg_scope:provenance] fn=<...> bb=<...> role=<...> v=<...> varmap_hits=[...] pin=<...> next=<...>`
- Call arg build provenance (strict/dev+planner_required; debug-only): `[call/arg_build:undefined_value] fn=<...> bb=<...> arg_idx=<...> v=<...> ast=<node_type> span=<Span> next=<...>`
- Value lifecycle fail-fast (strict/dev+planner_required): `[freeze:contract][value_lifecycle/typed_without_def] fn=<...> tag=<...> missing_count=<n> missing0=%<id> missing0_ty=<...> missing=[%<id>...] typed_count=<n> def_count=<n> span=<...> span_start=<...> span_end=<...> file=<...> value_caller=<...|none> pin=<...|none> varmap_hits=[...]`
- Literal lowering alloc (strict/dev+planner_required; debug-only): `[lit/lower:alloc] fn=<...> bb=<...> v=%<...> lit=<...> span=<...> file=<...> next=<...> emit=<ok|skipped:reason> caller=<file:line:col>`
- Literal lowering plan (strict/dev+planner_required; debug-only): `[lit/lower:plan] fn=<...> bb=<...> v=%<...> lit=<...> span=<...> file=<...> next=<...> path=<block_effects|body_effects|body_fallthrough|body_plan> emit=plan_effect`
- Literal lowering emit (strict/dev+planner_required; debug-only): `[lit/lower:emit] fn=<...> bb=<...> v=%<...> lit=<...> span=<...> file=<...> next=<...> emit=ok`
- BinOp lowering (Add + literal 3; strict/dev+planner_required; debug-only): `[binop/lower:lit_int3] fn=<...> bb=<...> lhs=%<...> rhs=%<...> side=<lhs|rhs|both> consts_len=<n> rhs_const_def=<yes|no> lhs_const_def=<yes|no> caller=<file:line:col>`
- Normalizer binop operand missing def (strict/dev+planner_required): `[freeze:contract][normalizer/binop_operand_missing_def] fn=<...> bb=<...> dst=%<...> op=<...> operand=<lhs|rhs> v=%<...>`
- Stmt effects binop+lit3 (strict/dev+planner_required; debug-only): `[stmt/effects:binop_lit3] fn=<...> kind=<assignment|local|print|call> bb=<...> effects_len=<n> const_int3_dsts=[%...] add_binops=[dst=%.. lhs=%.. rhs=%..]`
- Loop block_effects binop+lit3 (strict/dev+planner_required; debug-only): `[loop/block_effects:binop_lit3] fn=<...> bb=<...> effects_len=<n> const_int3_dsts=[%...] add_binops=[dst=%.. lhs=%.. rhs=%..]`
- Loop header cond leaf effects (strict/dev+planner_required; debug-only): `[loop_header/effects:leaf] fn=<...> bb=<...> cond=%<...> effects_len=<n> const_int3_dsts=[%...] binop_add_rhs=[%...]`
- Call stmt effects binop+lit3 (strict/dev+planner_required; debug-only): `[callstmt/effects:binop_lit3] fn=<...> bb=<...> effects_len=<n> const_int3_dsts=[%...] add_binops=[dst=%.. lhs=%.. rhs=%..] kind=<method|function>`
- Cond value effects int3 (strict/dev+planner_required; debug-only): `[cond_value/effects:int3] fn=<...> bb=<...> value=%<...> effects_len=<n> const_int3_dsts=[%...]`
- Cond if effects lit3 origin (strict/dev+planner_required; debug-only): `[cond_if/effects:lit3_origin] fn=<...> bb=<...> effects_len=<n> const_int3_dsts=[%...] origin_spans=[line <n>, column <n>...]`
- Bool expr effects binop+lit3 (strict/dev+planner_required; debug-only): `[bool_expr/effects:binop_lit3] fn=<...> bb=<...> effects_len=<n> const_int3_dsts=[%...] add_binops=[dst=%.. lhs=%.. rhs=%..] kind=<simple|and_or|compare>`
- Loop parts body_plans lit3 origin (strict/dev+planner_required; debug-only): `[loop_parts/body_plans:lit3_origin] fn=<...> bb=<...> plans_len=<n> const_int3_dsts=[%...] origin_spans=[line <n>, column <n>...]`
- Entry block plans lit3 origin (strict/dev+planner_required; debug-only): `[entry/block_plans:lit3_origin] fn=<...> kind=<exit_only|stmt_only|exit_allowed|no_exit|no_exit_stmt_lowerer> bb=<...> plans_len=<n> const_int3_dsts=[%...] origin_spans=[line <n>, column <n>...]`
- If-join lowered plans lit3 origin (strict/dev+planner_required; debug-only): `[if_join/lowered_plans:lit3_origin] fn=<...> bb=<...> one_sided_exit=<0|1> plans_len=<n> const_int3_dsts=[%...] origin_spans=[line <n>, column <n>...] origin_missing=<n>`
- Cond-branch plans lit3 origin (strict/dev+planner_required; debug-only): `[cond_branch/plans:lit3_origin] fn=<...> bb=<...> plans_len=<n> const_int3_dsts=[%...] origin_spans=[line <n>, column <n>...] origin_missing=<n> caller=<file:line:col>`
- PHI lifecycle define provenance (debug-only): `[phi_lifecycle/define] fn=<...> bb=<...> dst=%<...> tag=<...>`
- PHI lifecycle patch provenance (debug-only): `[phi_lifecycle/patch] fn=<...> bb=<...> dst=%<...> inputs=<n> phi0_pred=<...> phi0_in=%<...> (optional: phi1_pred=<...> phi1_in=%<...>) tag=<...>`
- PHI lifecycle fail-fast (strict/dev+planner_required): `[freeze:contract][phi_lifecycle/provisional_left_unpatched] fn=<...> phi_dst=%<...> phi_bb=<...> tag=<...> orig_err=<...> reason=<...> rollback_count=<n>` (freeze-friendly: errors starting with `[freeze:` and single-line are preserved in full; other errors truncated to ~80 chars)
- CF common PHI fail-fast (strict/dev+planner_required): `[freeze:contract][cf_common/phi_block_missing] fn=<...> bb_id=<...> dst=<...> (optional: op=<...> tag=<...>)`
- Loop lowerer PHI patch fail-fast: `[freeze:contract][lowerer/phi_patch_missing] block=<...> dst=<...> tag=<...> error=<...>`
- BlockSchedule non-dominating Copy fail-fast (strict/dev+planner_required): `[freeze:contract][schedule/non_dominating_copy] fn=<...> bb=<...> src=<...> def_block=<...>`
- LocalSSA non-dominating copy fail-fast (strict/dev+planner_required): `[freeze:contract][local_ssa/non_dominating_copy] fn=<...> bb=<...> src=<...> def_block=<...> (optional: def_kind=<...>)`
- JoinIR bridge non-dominating copy fail-fast (strict/dev+planner_required): `[freeze:contract][joinir_bridge/non_dominating_copy] fn=<...> bb=<...> src=<...> def_block=<...> op=<...> (optional: branch=<then|else>)`
- If-join duplicate PHI dst fail-fast (strict/dev+planner_required): `[freeze:contract][if_join/duplicate_phi_dst] fn=<...> new_merge_bb=<...> existing_phi_bb=<...> dst=%<...> join=<...>`
- Copy emission contract (implemented by `src/mir/builder/emission/copy_emitter.rs`; prefer reason-based tags for future unification): `[freeze:contract][copy/non_dominating] fn=<...> bb=<...> src=<...> def_block=<...> reason=<...>`
- BlockSchedule non-dominating src (strict/dev+planner_required): `[freeze:contract][schedule/non_dominating_src] bb=<...> src=<...>`
- Builder emit non-dominating Copy fail-fast (strict/dev+planner_required): `[freeze:contract][builder/non_dominating_copy] fn=<...> bb=<...> src=<...> def_block=<...>`
- Builder emit fail-fast: `[freeze:contract][builder/emit_missing_block] fn=<...> bb=<...> inst=<...>`
- Control-flow utility fail-fast: `[freeze:contract][builder/capture_jump_without_function] target_bb=<...>`
- PHI helper fail-fast: `[freeze:contract][builder/phi_insert_without_function_context] dst=%<...>`
- Lexical scope fail-fast (strict/dev+planner_required): `[freeze:contract][lexical_scope/unbalanced_pop] fn=<...> depth=<...> action=<pop|clear>`
- Selfhost timing (summary log): `[diag/selfhost] stageb_secs=<n> timeout_secs=<n>` / `[diag/selfhost] run_secs=<n> timeout_secs=<n>`
- VM runtime error (stderr, single-line; quiet-exit path): `[vm/error] <message>` (newlines are escaped as `\\n`)
- Optional GC mode diagnostics (dev/diagnostic only; metrics ON): `[gc/optional:mode] mode=<...> collect_sp=<...> collect_alloc=<...>`
- Runtime route contract fail-fast (strict/dev+planner_required): `[contract][runtime-route][expected=mir-json] route=stage-a source=<...> got=program-json strict_planner_required=1`
- Runtime route acceptance sentinel: `[contract][runtime-route][accepted=mir-json] route=stage-a source=<...> lane=<direct|compat-program-to-mir|compat-rust-json-v0-bridge>`
- Runtime execution-path observability (dev/verbose plugin init): `[runtime/exec-path] plugin_loader_backend=<enabled|stub> plugin_exec_mode=<...> box_factory_policy=<...>`
- Runtime route direct-v0 guard fail-fast: `[freeze:contract][runtime-route/direct-v0-bridge-disabled] route=stage-a source=<...> lane=direct-v0-bridge status=retired`
- Runtime route parser-flag removal contract: `--parser ny` is removed at CLI boundary (clap reject), and `NYASH_USE_NY_PARSER=1` is legacy no-op.
- LLVM hot trace summary (perf/dev only): `[llvm/hot] fn=<...> binop_total=<n> binop_mod=<n> compare_total=<n> compare_keep_i1=<n> compare_to_i64=<n> call_total=<n> resolve_local_hit_binop=<n> resolve_global_hit_binop=<n> resolve_fallback_binop=<n> resolve_local_hit_compare=<n> resolve_global_hit_compare=<n> resolve_fallback_compare=<n> resolve_local_hit_call=<n> resolve_global_hit_call=<n> resolve_fallback_call=<n>`
- Emit route verifier fail-fast (strict/dev emit paths): `[freeze:contract][emit-mir/direct-verify] route=<vm|mir> errors=<n>` + `[emit-mir/direct-verify] route=<vm|mir> detail=<...>`
- Emit route verifier fail-fast (strict/dev emit paths): `[freeze:contract][emit-exe/direct-verify] route=<vm|mir> errors=<n>` + `[emit-exe/direct-verify] route=<vm|mir> detail=<...>`
- Callsite retire emit-side fail-fast (strict/dev): `[freeze:contract][callsite-retire:legacy-<boxcall|externcall>] fn=<...> bb=<...> inst_idx=<...> op=<...>`
- RC insertion PHI/edge fail-fast (`rc-insertion-minimal`): `[freeze:contract][rc_insertion/phi_edge_mismatch] fn=<...> cleanup=<break|continue> pred=<...> target=<...> reason=<...> (optional: value=%<...>)`
- VM step budget exceeded (debug-only dump): `vm step budget exceeded (max_steps=<...>, steps=<...>) at bb=<...> ... mir_dump=</tmp/...|write_failed> (optional: mir_dump_snip=</tmp/...|write_failed>) (optional: trace_tail=[...]) (optional: loop_signature=<bbA->bbB...>)`
- Gate sentinels (strict/dev + planner_required; prefix-free; stderr):
  - `[joinir/planner_first rule=<RuleId>] label=<Label>` (planner-first routing sentinel)
  - `[joinir/no_plan reason=no_loop] func=<...>` (planner_required + no loops in function body)
  - `[phase132/gate] StepTree root for '<func>'` (StepTree gate sentinel)
  - `[plan/loop_break/promotion_hint:<TrimSeg|DigitPos>]` (loop_break_recipe LoopBodyLocal promotion hint)
  - `[flowbox/adopt box_kind=Loop features=<...> via=<shadow|release>]` (Flowbox adopt routing sentinel)
    - pre_plan の shadow adopt が正規経路のケースは、fast gate の `planner_tag` としてこれを使う
- Fail-fast logs: one line with function + rule + reason (avoid stack traces).
- Parity mismatches must not panic; return a fail-fast error with (fn, root, idx, kind).

## Freeze tag taxonomy (SSOT)

Freeze tags must reflect the layer that produced the failure:

- Plan/Facts/Planner: `[plan/freeze:<reason>]` (or `[plan/reject:<reason>]` for non-freeze rejection)
- Normalizer/Recipe: `[freeze:contract][<area>]`
- JoinIR Lowering: `[joinir/freeze]` or `[joinir/<phase>/<pattern>/contract]`

Tests must assert the correct class of tag for the layer they exercise. See
`docs/development/current/main/design/planfrag-freeze-taxonomy.md`.

## Plan trace (SSOT): rule-take

When debugging “planner returned None” across AIs, the core failure is often “we can’t tell where the candidate disappeared”.
To fix this without adding a large logging framework, we standardize a single-line trace tag:

- Tag: `[plan/trace]`
- Output is **dev/debug only** (guarded by `joinir_dev::debug_enabled()`).
- The format is stable and grep-friendly:
  - `stage=<...> rule=<...> result=<...> extra=<...>`

Minimum required trace points (must use `[plan/trace]`):

1. Planner rule take (`single_planner::planner_hits_rule` → `trace_try_take_planner`)

Note:
- CandidateSet / pattern_shadow trace points were retired with single-plan boundary cleanup
  (commit `0df74eaa5`).

Additional plan trace tags (SSOT):

- `[plan/trace:continue_only]`
  - Purpose: Validate ContinueIfNestedLoop span calculation and then_body slicing.
  - Output fields (1 line): `ctx=<...> loop_idx=<...> prelude_len=<...> postlude_len=<...> then_len=<...> postlude_span=(s,e)`
- `[plan/trace:nested_loop_depth1]`
  - Purpose: Confirm which branch lower_nested_loop_depth1_any chose.
  - Output fields (1 line): `ctx=<...> path=<...> rule=<...>`
- `[plan/trace:loop_var_candidates]`
  - Purpose: Summarize generic_loop_v1 loop var candidate filtering when no_valid_loop_var_candidates.
  - Output fields (1 line): `ctx=<...> raw=<n> filtered=<n> reasons=...`
- `[plan/trace:if_phi_normalize]`
  - Purpose: Inspect Pattern3IfPhi normalizer input when a variable is missing.
  - Output fields (1 line): `ctx=<...> var=<name> locals_count=<n> scope=<...>`
- `[plan/trace:nested_loop_guard]`
  - Purpose: Observe strict_nested_loop_guard nested-loop detection inputs.
  - Output fields (1 line): `func=<...> nested_loop=<bool> facts_present=<bool>`
- `[plan/trace:nested_loop_guard_entry]`
  - Purpose: Identify strict_nested_loop_guard entrypoint.
  - Output fields (1 line): `ctx=<entry> features=<features> recipe_contract=<Some|None>`
- `[plan/trace:return_obligation]`
  - Purpose: Observe Return port obligations when Return is observation-only.
  - Output fields (1 line): `port=Return var=<name|none> state=<state|Empty> ctx=<context>`
- `[plan/trace:entry_candidates]`
  - Purpose: Observe loop recipe-first entry candidates before selection.
  - Output fields (1 line): `ctx=<...> candidates=<list|none>`
- `[plan/trace:entry_candidates_gate]`
  - Purpose: Confirm entry-candidate logging gate state (strict/dev + planner_required + debug).
  - Output fields (1 line): `strict_or_dev=<bool> planner_required=<bool> debug_enabled=<bool>`
- `[plan/trace:facts_summary]`
  - Purpose: Record which loop facts candidates are Some/None before routing.
  - Output fields (1 line): `ctx=<...> scan_methods=<0|1> scan_methods_block=<0|1> loop_scan=<0|1> loop_scan_phi_vars=<0|1> collect_using_entries=<0|1> bundle_resolver=<0|1>`
- `[plan/trace:entry_route]`
  - Purpose: Observe which loop entry route was taken (recipe_first / shadow_adopt / none).
  - Output fields (1 line): `ctx=<...> route=<...>`
- `[plan/trace:loopcond_flags]`
  - Purpose: Summarize LoopCondBreak facts flags before accept/reject.
  - Output fields (1 line): `break=<n> continue=<n> return=<n> exit_if=<n> continue_if=<n> cond_update=<n> nested=<n> no_break_or_continue=<bool> allow_cluster_without_exit=<bool>`
  - Example: `[plan/trace:loopcond_flags] break=1 continue=0 return=0 exit_if=1 continue_if=0 cond_update=0 nested=0 no_break_or_continue=false allow_cluster_without_exit=false`
- `[plan/trace:loopcond_break_kind]`
  - Purpose: Observe LoopCondBreak break_kind classification (observation-only).
  - Output fields (1 line): `kind=<Single|Multi> exit_sites=<n>`

Example grep:

- `rg -n \"\\[plan/trace\\]\" /tmp/phase29bq_selfhost_*.log | head -200`

Enable:

- `HAKO_JOINIR_DEBUG=1` (preferred) or `NYASH_JOINIR_DEBUG=1` (legacy)
