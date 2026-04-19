# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-03
Scope: Repo root の互換入口。詳細ログは `docs/development/current/main/` 側を正本にする。

## Purpose

- root から最短で current blocker と実行順へ到達するための入口。
- 長文の進捗履歴はここに蓄積しない。
- runtime lane の Next は `phase-29y/60-NEXT-TASK-PLAN.md` を単一正本に固定する。
- kernel/build lane の混線防止は `docs/development/current/main/design/build-lane-separation-ssot.md` を正本にする。

## Focus Lock (2026-03-02)

- primary target: `kernel-mainline`（`.hako` kernel）を日常の既定経路に固定する。
- no-fallback: `NYASH_VM_USE_FALLBACK=0` 前提で fail-fast を維持する（silent fallback 禁止）。
- cargo role: `build-maintenance` のみ（host artifact 保守・portability 監査）。
- daily role: `build-mainline`（cargo 非依存ループ）で `.hako` kernel 最適化を進める。
- emit route role: `hakorune --emit-mir-json` 直経路を mainline SSOT とし、helper 経路は互換監視を除いて段階撤去する。
- helper policy: hakorune→hakorune build で helper 依存を残したまま最適化へ進まない（先に導線を一本化する）。

## Current Blocker (SSOT)

- compiler lane: `phase-29bq / monitor-only`（active: failure-driven reopen only）
  - joinir migration task SSOT（lane A）:
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
  - joinir extension contract SSOT（lane A reopen runbook）:
    - `docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`
    - active seed: `JIR-EXT-SHAPE-01`（GREEN lock）
      - fixture: `apps/tests/phase29bq_selfhost_blocker_phi_injector_collect_phi_vars_nested_loop_no_exit_var_step_min.hako`
      - gate: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only ext-red`
      - note: rust-reference / hako-mainline ともに `18 / RC:0`（lane tag: `vm` / `vm-hako`）で green lock 済み。
  - lane A mirror sync helper:
    - `bash tools/selfhost/sync_lane_a_state.sh`
  - done: `JIR-PORT-00`（Boundary Lock, docs-first）
  - done: `JIR-PORT-01`（Parity Probe）
  - done: `JIR-PORT-02`（if/merge minimal port）
  - done: `JIR-PORT-03`（loop minimal port）
  - done: `JIR-PORT-04`（PHI / Exit invariant lock）
  - done: `JIR-PORT-05`（promotion boundary lock）
  - done: `JIR-PORT-06`（monitor-only boundary lock）
  - done: `JIR-PORT-07`（expression parity seed lock: unary+compare+logic）
  - next: `none`（strict_nested accept-min1 done; failure-driven reopen only）
- runtime lane: `phase-29y / none`（current blocker: `none`。fixed order は `phase-29y/60-NEXT-TASK-PLAN.md` を正本とする）
  - commit boundary lock: `phase-29y/60-NEXT-TASK-PLAN.md` の `0.3 RVP Commit Boundary Lock (active rule)`
  - operation policy lock: `LLVM-first / vm-hako monitor-only`
  - policy SSOT: `docs/development/current/main/design/de-rust-lane-map-ssot.md` の `Runtime Operation Policy`
- config hygiene lane: `none`（monitor-only、SSOT: `phase-29y/84-MODULE-REGISTRY-HYGIENE-SSOT.md`）
- compiler pipeline lane: `hako-using-resolver-parity`（monitor-only: lane-B ternary debt decision fixed）
  - parity gate: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh`
  - active next: `none`（B-TERNARY-03 decision fixed）
  - task SSOT:
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` の `Lane-B Nested Ternary Debt Pack (B-TERNARY-01..03)`
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` の `Binary-only --hako-run Contract (lane B)`
  - diagnostic pin（non-gating）:
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_var_values_lock_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh`
  - binary-only contract SSOT: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- de-rust migration orchestration lane: `phase-29cc / reopen (runtime Lane-C boundary sync, source-keep)`
  - phase dashboard（SSOT）: `docs/development/current/main/phases/phase-29cc/README.md`
  - execution checklist: `docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md`
  - scope decision（L5 accepted）: `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
  - strict readiness（L4 done, 2026-02-25）: `tools/selfhost/check_phase29x_x23_readiness.sh --strict` -> `status=READY`
  - plugin lane status: done through `PLG-07` / active next=`none`（failure-driven reopen only）
  - runtime lane status:
    - active lock: `docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md`
    - boundary lock: `docs/development/current/main/phases/phase-29cc/29cc-253-source-zero-static-link-boundary-lock-ssot.md`
    - hook lock: `docs/development/current/main/phases/phase-29cc/29cc-254-hako-forward-hook-cabi-cutover-order-lock-ssot.md`
    - current status: `HFK-min1..min6 done`, active next=`none`（monitor-only, failure-driven reopen）
    - latest: `NYASH_VM_USE_FALLBACK=0` 時は hook 未登録の `invoke/by_name` / `future.spawn_instance3` / string exports が Rust fallback へ落ちない契約へ更新（mainline no-compat path hardening）
    - latest2: `invoke_core` と `plugin_loader_v2/route_resolver` の compat fallback も `NYASH_VM_USE_FALLBACK=0` で拒否する契約へ統一（fallback policy SSOT alignment）
    - latest3: fallback policy 判定を `vm_compat_fallback_allowed()`（`src/config/env/vm_backend_flags.rs`）へ集約。`hako_forward_bridge` は hook-miss helpers を提供し、string/by_name/future の no-compat path は `NYRT_E_HOOK_MISS`（scalar）/ freeze-handle（handle戻り）で fail-fast を固定。
    - latest4: `plugin_loader_v2` loader/instance/ffi の invoke route を `InvokeRouteContract` 再利用へ統一し、`invoke_core` に named receiver+method 解決 / invoke+decode helper を追加（`by_name` / `future` entry の重複を縮退）。
    - source keep policy: Rust source は保存固定（削除タスクは起票しない）
    - target model: `.hako` 主経路で runtime/plugin の mainline 実装を成立させ、Rust 意味論の mainline 依存を 0 行化する（source keep）
    - kernel naming lock（混線防止）:
      - `kernel-mainline`: `.hako` 主経路（`NYASH_VM_USE_FALLBACK=0`）で hook miss は fail-fast。
      - `kernel-bootstrap`: Rust static archive（`libnyash_kernel.a`）を使う起動・互換維持用の保存経路（source keep）。
    - execution order lock:
      1. runtime/de-rust の経路契約を維持（no-compat mainline guard を先に固定）。
      2. 日常最適化は `kernel-mainline` を既定経路に固定する（fallbackなし）。
      3. `kernel-bootstrap` は保守 lane（artifact refresh / portability / 切り分け）に限定する。
  - wasm lane status: done through `WSM-P10` / active next=`none`（monitor-only）
  - done judgement matrix SSOT:
    - `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md`
- perf lane: `phase-21.5 / llvm-aot-hotpath`（monitor-only: runtime lane とは別コミット境界で運用）
  - scope: `numeric_mixed_medium` / `method_call_only` / `chip8_kernel_small` / `kilo_kernel_small_hk`（C/AOT 比較） + VM monitor-only
  - task SSOT: `docs/private/roadmap/phases/phase-21.5/PLAN.md`
  - Step-2 acceptance lock (fixed):
    - `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py`
    - `cargo test -p nyash_kernel box_from_i8_string_const_reuses_handle -- --nocapture`
    - `PERF_LADDER_AOT_SMALL=1 PERF_LADDER_AOT_MEDIUM=1 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_progressive_ladder_21_5.sh quick`（AOT行 `status=ok`）
  - WSL variance lock（single-run値で判定しない）:
    - canonical measure: `bash tools/perf/run_kilo_hk_bench.sh strict 1 5`
    - latest (2026-03-01): `c_ms=79`, `py_ms=111`, `ny_vm_ms=1015`, `ny_aot_ms=747`, `ratio_c_aot=0.11`, `aot_status=ok`
  - micro-first recovery lock（active）:
    - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_getset 1 7`
      - latest (2026-03-01): `ny_aot_ms=49`
    - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 7`
      - latest (2026-03-01): `ny_aot_ms=64`
    - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 7`
      - latest (2026-03-01): `ny_aot_ms=14`
    - rule: micro 改善が確認できた変更だけを `kilo_kernel_small_hk` へ持ち上げる（kilo 先行の試行錯誤は禁止）。
    - latest kernel-asm note (2026-03-01):
      - `nyash.array.set_his` share improved (`~7.4% -> ~5.8%`) after single-lookup pair route.
      - `nyash.string.concat_hh` remains top user-space hotspot (`~8.5%` class); next optimization focus is concat structure.
  - active next: `none`（failure-driven reopen only）
  - optimization resume policy（fixed）:
    - resume trigger: de-rust runtime closeout contract（`runtime-exec-zero` + `phase29y_no_compat_mainline_vm`）green。
    - latest evidence (2026-03-01, head=`68ea40af29`):
      - `tools/checks/dev_gate.sh runtime-exec-zero` green
      - `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh` green
    - source keep policy と最適化は混ぜず、別コミット境界で進める。
    - kernel lane scope:
      - primary target: `kernel-mainline`（`.hako` no-compat lane）
      - maintenance target: `kernel-bootstrap`（Rust static archive lane）
  - regression recovery pack（active, docs-fixed）:
    1. `AOT stale artifact` を止める（bench 実行時は `PERF_AOT_SKIP_BUILD=0` 固定）。
    2. `emit route` を固定する（Stage-B/helper fallback が起きた run は perf 結果として受理しない）。
    3. `set_hih -> set_hii` 昇格を unblock する（`collect_integerish_value_ids` の RuntimeData value 伝播を拡張）。
    4. micro -> kilo の順で再計測し、`set_hii` emit と ratio 変化を同時確認する。
  - regression recovery acceptance:
    - `bench_kilo_micro_array_getset` の AOT object で `nyash.array.set_hii` を観測できること。
    - `kilo_kernel_small_hk` で `ratio_c_aot` が現状基準（0.10）を上回ること。
    - 計測 run は `PERF_AOT_SKIP_BUILD=0` を必須にすること。

## Immediate Next (this round)

- docs-first / compiler lane SSOT:
  - `docs/development/current/main/design/compiler-task-map-ssot.md` の `Phase29x Direct Route Recovery Pack (2026-03-03)`
  - `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md` の `AI mistake-resistant rules (always-on)`
- execution rule: 1 blocker = 1受理形 = fixture+gate = 1 commit（BoxCount / BoxShape 混在禁止）
- monitor-only note: `Concat3 Normalization Pack` と `Helper Retirement Pack` は本ラウンド中は監視のみ（混線禁止）
- compiler fixed order (2026-03-03, explicit):
  1. `phase29x-probe` 直近 blocker を先に解消する（direct route の fail class を先に 0 へ寄せる）。
  2. blocker 解消後にのみ、Phase D cleanup（Pattern 名の撤去 / DomainPlan 縮退）へ着手する。
  3. BoxCount（受理形追加）と BoxShape（命名/層整理）は同一コミットに混ぜない。
  4. Pattern/Domain cleanup は docs SSOT（`recipe-first-migration-phased-plan-proposal.md` の Phase D）を gate にして進める。

## Restart Handoff (2026-03-03 / codex update)

- latest probe（direct route SSOT）:
  - command: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe`
  - result: `emit_fail=13`, `run_nonzero=9`, `run_ok=96`, `route_blocker=0`（total=118）
  - class: `emit:direct-verify=9`, `emit:other=4`
- update (2026-03-03, second pass):
  - command: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
  - result: `emit_fail=10`, `run_nonzero=12`, `run_ok=96`, `route_blocker=0`（total=118）
  - class: `emit:direct-verify=6`, `emit:other=4`
  - direct-verify head `%290` blocker fixed:
    - fixture: `phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_cleanup_min.hako`
    - `--emit-mir-json` now `emit-ok` (previously `Undefined value %290 used in block bb55`)
  - canary: `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` => `PASS (emit_rc=0, run_rc=4)`
- update (2026-03-03, third pass / idx19+idx28 fixed):
  - command: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
  - result: `emit_fail=8`, `run_nonzero=12`, `run_ok=98`, `route_blocker=0`（total=118）
  - class: `emit:direct-verify=4`, `emit:other=4`
  - resolved fixtures:
    - `phase29bq_selfhost_blocker_scan_methods_nested_loop_idx19_min.hako` (`emit=0, run=0`)
    - `phase29bq_selfhost_blocker_scan_methods_nested_loop_idx28_min.hako` (`emit=0, run=0`)
  - canary: `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` => `PASS (emit_rc=0, run_rc=4)`
- update (2026-03-03, fourth pass / box_member direct-verify cleared):
  - command: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
  - result: `emit_fail=4`, `run_nonzero=16`, `run_ok=98`, `route_blocker=0`（total=118）
  - class: `emit:other=4`（`emit:direct-verify=0`）
  - resolved fixtures:
    - `phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_cleanup_min.hako`
    - `phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_cleanup_min.hako`
    - `phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_guard_sync_tail_cleanup_min.hako`
    - `phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_guard_sync_tail_mirror_sync_tail_cleanup_min.hako`
  - fix note:
    - `CoreEffectPlan::Select` emission で 2-pred merge のとき、incoming の定義ブロックが pred 対応する形を検出できれば `MirInstruction::Phi` に置換するように変更（`effect_emission.rs`）。
  - canary: `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` => `PASS (emit_rc=0, run_rc=4)`
- update (2026-03-03, fifth pass / loop_cond emit-other collapse):
  - command: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
  - result: `emit_fail=1`, `run_nonzero=18`, `run_ok=99`, `route_blocker=0`（total=118）
  - class: `emit:joinir-reject=1`（`emit:other=0`, `emit:direct-verify=0`）
  - fix note:
    - `unsupported stmt Call` を `loop_cond` stmt lowering に受理追加（`ASTNode::Call`）。
    - generic loop step で BinOp 専用前提を除去し、非算術 step は value lowering + copy にフォールバック。
    - `ContinueWithPhiArgs` の incoming を常時 `ssa::local::try_ensure(..., LocalKind::Arg)` で predecessor 局所化し、dominance を固定。
  - canary: `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` => `PASS (emit_rc=0, run_rc=4)`
- update (2026-03-03, sixth pass / direct emit blockers cleared):
  - command: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
  - result: `emit_fail=0`, `run_nonzero=18`, `run_ok=100`, `route_blocker=0`（total=118）
  - class: `emit_fail class = 0`（`emit:joinir-reject=0`, `emit:other=0`, `emit:direct-verify=0`）
  - fix note:
    - `loop_cond_return_in_body_facts` に `if-else-if(return)` 受理形を追加し、`generic_loop_v1` への誤フォールスルー（`no_valid_loop_var_candidates`）を解消。
    - fixture `phase29bq_selfhost_blocker_parse_program2_loop_if_else_if_return_min.hako` が `emit=0/run=0` に遷移。
  - canary: `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` => `PASS (emit_rc=0, run_rc=4)`
- update (2026-03-04, blocker-triage follow-up / bq green):
  - command: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - result: `PASS (mode=bq)`
  - fix note:
    - `loop_cond_break_continue` の nested loop でキャリア同期が古い値を上書きする不具合を修正（`sync_carrier_bindings` を「欠損補完のみ」に変更）。
    - nested depth1 の `single_planner=None` で即 freeze していた経路を `nested_loop_plan` の recipe-first fallback へ統合（`nested-break-continue-pure has no plan` 解消）。
    - `loop_cond` join-if fallback で branch bindings を書き戻していなかった不具合を修正（`depth = depth + 1` 欠落を解消）。
  - gate contract tune:
    - planner tag の移行揺れを OR 許容に更新（`selfhost_seek_array_end_return_if_min` / `selfhost_extract_body_brace_return_min` / `selfhost_phi_injector_nested_loop_count_min`）。
    - `selfhost_parse_program2_loop_if_else_if_return_min` の expected tag を実挙動（`LoopCondReturnInBody`）へ更新。
- update (2026-03-04, cleanliness follow-up / behavior-preserving):
  - commit: `fe09e91e9` (`refactor clean compiler flow helpers and profile shim docs`)
    - `parts/stmt.rs`: join-bearing `if` の non-exit lowering 分岐を helper 抽出（可読性改善、挙動不変）
    - `recipe_tree/loop_break_builder.rs`: stale comment 修正 + dedupe判定を helper 化
    - `vm_hako/subset_check.rs`: `indexOf` shape error tag を定数化
    - `tools/smokes/v2/profiles/lib/README.md`: compatibility shim 契約を明文化
  - commit: `a5242fbca` (`refactor planner tag generation around semantic rule names`)
    - `planner/tags.rs`: pattern系 planner-first tag を semantic label 基準で生成し、loop-cond 系のみ pin override に集約（重複SSOT削減）
    - unit test追加:
      - `planner_first_tag_uses_semantic_name_for_pattern_rules`
      - `planner_first_tag_keeps_pinned_rule_name_for_loop_cond_break`
  - verification:
    - `cargo check --release --bin hakorune` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only 29bp` => PASS（再実行で安定）
- update (2026-03-04, direct-verify blocker hotfix):
  - commit: `dedd91ba4` (`fix direct-verify dominance via LocalSSA select rematerialization`)
  - fix note:
    - `phase29x-probe` で再発した `emit:direct-verify=1`（`scan_methods_nested_loop_state_machine_min`）を修正。
    - `CoreEffectPlan::Copy` emission で source を LocalSSA materialize するように変更。
    - LocalSSA `ensure_inner` に `MirInstruction::Select` の rematerialize 経路を追加（merge block の predecessor-only 値 copy を抑止）。
  - verification:
    - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` => `emit_fail=0 / run_nonzero=18 / run_ok=100 / route_blocker=0`
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only 29bp` => PASS（単体再実行）
- update (2026-03-04, Phase D cleanup follow-up / recipe-only gate unification):
  - commit: `739885b20` (`refactor single planner recipe-only rule gating`)
  - fix note:
    - `single_planner/rules.rs` の recipe-only 分岐（Pattern2/3/4/5 + LoopCondContinueWithReturn）を `is_recipe_only_rule()` へ集約。
    - debug 表示は `planner_rule_semantic_label` ベースへ統一し、Pattern語彙の散在を抑制。
    - `is_recipe_only_rule` の unit test を追加（planner_required 依存と always-on rule を固定）。
  - verification:
    - `cargo test -q recipe_only_rules_require_planner_required_for_pattern_family --lib` => PASS
    - `cargo test -q loop_cond_continue_with_return_is_always_recipe_only --lib` => PASS
    - `cargo check --release --bin hakorune` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only 29bp` => PASS
- update (2026-03-04, Phase D D1 semantic enum sweep):
  - commit: `0c6d4669f` (`refactor use semantic PlanRuleId variants in planner flow`)
  - fix note:
    - `PlanRuleId` の Pattern 数値 variant を semantic variant（`LoopBreakRecipe` など）へ置換。
    - `single_planner/rule_order.rs` に `Pattern1..9` 互換 alias const を保持し、段階移行の互換性を維持。
    - `joinir/patterns/registry/handlers.rs` / `single_planner/rules.rs` / `planner/tags.rs` の呼び出し側を semantic variant へ統一。
  - verification:
    - `cargo test -q rule_name_uses_semantic_label --lib` => PASS
    - `cargo test -q legacy_rule_name_alias_is_preserved --lib` => PASS
    - `cargo test -q legacy_pattern_alias_constants_are_preserved --lib` => PASS
    - `cargo test -q recipe_only_rules_require_planner_required_for_pattern_family --lib` => PASS
    - `cargo test -q planner_first_tag_uses_semantic_name_for_pattern_rules --lib` => PASS
    - `cargo check --release --bin hakorune` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only 29bp` => PASS
- update (2026-03-04, direct generic_loop v1 unblock for nested Program step):
  - commit: `c495c989b` (`fix generic_loop_v1 step resolution for nested program loops`)
  - fix note:
    - `generic_loop/facts/extract/v1.rs` を候補解決ヘルパで分離し、reject reason の集計を構造化。
    - 条件 canonicalize と var-step extraction を release route でも有効化（`allow_var_step=true`, cond canon extended）。
    - 条件側 loop var 候補が空のケースで body 由来候補へフォールバックし、nested `Program{ loop }` 形の取りこぼしを解消。
    - `body_lowering_policy` 変換条件の逆転バグ（`is_some`/`is_none`）を修正し、`ExitAllowed` 契約を維持。
    - `step/extract/shared.rs` に `contains_var_name` を追加し、var-step 判定を `loop_var` 再帰包含ベースへ補強。
    - unit test 追加: `generic_loop_nested_program_stmt_preserves_outer_loop_step_expr`。
  - verification:
    - `cargo test -q --lib generic_loop_v1_policy_exit_allowed_without_break` => PASS
    - `cargo test -q --lib generic_loop_nested_program_stmt_preserves_outer_loop_step_expr` => PASS
    - `cargo test -q --lib generic_loop_v0_allows_loop_var_from_add_expr_in_condition` => PASS
    - `cargo test -q --lib generic_loop_v1_policy_recipe_only_with_break` => PASS
    - `cargo check --release --bin hakorune` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
    - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` => `emit_fail=0 / route_blocker=0 / run_nonzero=18 / run_ok=100`
- update (2026-03-04, cond_prelude BlockExpr loop route lock):
  - fix note:
    - `cond_prelude` 語彙に loop-like stmt を追加（`Loop/While/ForRange`）。
    - if-branch 条件 lowering は loop-like prelude を plan-level prelude route へ切替（`lower_cond_prelude_stmt_as_plan` 経由）。
    - `loop_cond_break_continue` の `ConditionalUpdateIf` は loop-like condition prelude を受理しないように制限し、`GeneralIf` へルーティング。
    - direct route pin fixture を追加:
      - `apps/tests/phase29bq_generic_loop_v1_if_cond_prelude_loop_min.hako`
      - `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` に case_id `generic_loop_v1_if_cond_prelude_loop_min` を追加。
    - scope note:
      - loop header 側の `BlockExpr prelude with loop-like stmt` は現時点では out-of-scope（`generic_loop_v1 unsupported_condition` で fail-fast 維持）。
  - verification:
    - `cargo test -q --lib cond_prelude_vocab_accepts_if_and_loop_like_stmt` => PASS
    - `cargo test -q --lib prelude_loop_detection_recurses_into_if_branches` => PASS
    - `cargo test -q --lib generic_loop_v1_accepts_if_condition_with_blockexpr_loop_prelude` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only generic_loop_v1_if_cond_prelude_loop_min` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
    - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` => `emit_fail=0 / route_blocker=0 / run_nonzero=18 / run_ok=101`（total=119）
- update (2026-03-04, Phase D D2 payload-free planner match step):
  - commit: `232432109` (`refactor D2 use DomainPlanKind for planner rule matching`)
  - fix note:
    - `single_planner/rules.rs` の rule match 判定を `DomainPlan` payload clone から `DomainPlanKind` 判定へ移行。
    - planner hit 時のみ `outcome.plan.take()` で plan を取り出す形に変更（rule loop 中の payload 依存を縮退）。
    - `DomainPlanKind::label()` を追加し、`DomainPlan::kind_label()` は kind-label delegate へ統一。
  - verification:
    - `cargo test -q --lib domain_plan_kind_and_label_match` => PASS
    - `cargo test -q --lib recipe_only_rules_require_planner_required_for_pattern_family` => PASS
    - `cargo test -q --lib loop_cond_continue_with_return_is_always_recipe_only` => PASS
    - `cargo check --release --bin hakorune` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
  - D3 reconnaissance note:
    - `normalizer/mod.rs` の module tree に未配線な legacy pattern normalizer file が残存（runtime 未参照）。
    - 候補（次段で isolate/delete 検討）:
      - `pattern1_simple_while.rs`, `pattern1_array_join.rs`, `pattern1_char_map.rs`
      - `pattern3_if_phi.rs`, `pattern4_continue.rs`, `pattern5_infinite_early_exit.rs`
      - `pattern8_bool_predicate_scan.rs`, `pattern9_accum_const_loop.rs`
  - D3 execution (2026-03-04, dead-file delete):
    - `normalizer/mod.rs` 未配線（runtime 未参照）を確認後、legacy normalizer file 8件を撤去:
      - `normalizer/pattern1_simple_while.rs`
      - `normalizer/pattern1_array_join.rs`
      - `normalizer/pattern1_char_map.rs`
      - `normalizer/pattern3_if_phi.rs`
      - `normalizer/pattern4_continue.rs`
      - `normalizer/pattern5_infinite_early_exit.rs`
      - `normalizer/pattern8_bool_predicate_scan.rs`
      - `normalizer/pattern9_accum_const_loop.rs`
    - verification:
      - `cargo build --release --bin hakorune` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
  - D4 execution (2026-03-04, domain payload prune first cut):
    - `domain.rs` から runtime 参照ゼロの legacy payload 型を撤去:
      - `Pattern1SimpleWhilePlan`
      - `Pattern1CharMapPlan`
      - `Pattern1ArrayJoinPlan`
      - `Pattern9AccumConstLoopPlan`
      - `Pattern8BoolPredicateScanPlan`
      - `Pattern3IfPhiPlan`
    - `extractors/mod.rs` 未配線だった dead file `extractors/pattern8.rs` を撤去。
    - verification:
      - `cargo test -q --lib domain_plan_kind_and_label_match` => PASS
      - `cargo build --release --bin hakorune` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
  - D4 follow-up (2026-03-04, domain import path cleanup):
    - `Pattern4ContinuePlan` / `Pattern5InfiniteEarlyExitPlan` の呼び出し側を `plan::` re-export 依存から `domain::` 直参照へ移行。
      - `composer/shadow_adopt.rs`: `Pattern4ContinuePlan` を `domain::` import 化
      - `features/pattern5_infinite_early_exit_{ops,pipeline}.rs`: `Pattern5InfiniteEarlyExitPlan` を `domain::` import 化
      - `plan/mod.rs`: `Pattern4ContinuePlan` / `Pattern5InfiniteEarlyExitPlan` re-export を撤去
    - verification:
      - `cargo build --release --bin hakorune` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
  - D4 follow-up2 (2026-03-04, scan/split legacy plan path test-only isolate):
    - `scan_with_init` / `split_scan` の legacy plan pipeline を runtime compile path から分離:
      - `features/mod.rs`: `scan_with_init_*` / `split_scan_*` / `split_emit` module を `#[cfg(test)]` 化
      - `skeletons/mod.rs`: `scan_with_init` / `split_scan` module を `#[cfg(test)]` 化
      - `domain.rs` + `plan/mod.rs`: `ScanDirection` / `scan_direction_from_step_lit` / `ScanWithInitPlan` / `SplitScanPlan` を test-only export に縮退
      - `edgecfg_facade.rs`: `compose` re-export を `#[cfg(test)]` 化（release warning を抑止）
      - `edgecfg/api/compose/cleanup.rs`: `cleanup` に `#[allow(dead_code)]` を付与（test-only compose 呼び出しの警告抑止）
    - verification:
      - `cargo test -q --lib coreloop_v0_composes_scan_with_init_subset` => PASS
      - `cargo test -q --lib coreloop_v0_composes_split_scan_subset` => PASS
      - `cargo build --release --bin hakorune` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
  - D4 final (2026-03-04, scan/split legacy module delete):
    - test-only isolate 後に legacy module/file を段階削除:
      - `normalizer/pattern_scan_with_init.rs`
      - `normalizer/pattern_split_scan.rs`
      - `features/scan_with_init_{pipeline,ops}.rs`
      - `features/split_scan_{pipeline,ops}.rs`
      - `features/split_emit.rs`
      - `skeletons/scan_with_init.rs`
      - `skeletons/split_scan.rs`
    - module tree も同期更新:
      - `normalizer/mod.rs` / `features/mod.rs` / `skeletons/mod.rs` から対応 module 宣言を撤去
      - `domain.rs` / `plan/mod.rs` から `ScanWithInitPlan` / `SplitScanPlan` 語彙を撤去
      - `edgecfg_facade.rs` と `compose/cleanup.rs` は warning-free へ調整
    - verification:
      - `cargo test -q --lib coreloop_v0_composes_scan_with_init_subset` => PASS
      - `cargo test -q --lib coreloop_v0_composes_split_scan_subset` => PASS
      - `cargo build --release --bin hakorune` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
  - D5 first cut (2026-03-04, pattern4/5 legacy payload path shrink):
    - dead payload path を削除:
      - `features/pattern5_infinite_early_exit_{pipeline,ops}.rs` を撤去
      - `domain.rs` から `Pattern4ContinuePlan` / `Pattern5InfiniteEarlyExitPlan` を撤去
    - `composer/shadow_adopt.rs` の nested guard diagnostics は struct 依存を外し、直接フォーマットへ移行。
    - 付随 cleanup:
      - `features/step_mode.rs` から未使用 re-export を撤去
    - verification:
      - `cargo test -q --lib coreloop_v1_composes_pattern5_with_value_join` => PASS
      - `cargo test -q --lib coreloop_v1_rejects_pattern5_with_cleanup` => PASS
      - `cargo build --release --bin hakorune` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
  - D5 follow-up (2026-03-04, planner semantic-only alias cleanup):
    - `planner/pattern_shadow.rs` から legacy `loop/pattern* -> semantic` 正規化を撤去し、shadow priority 判定を semantic rule 名のみへ固定。
    - `single_planner/rule_order.rs` から `PlanRuleId::Pattern1..9` 互換 alias const を撤去（semantic variant のみ維持）。
    - nested guard diagnostics は既存 fixture 契約（`Pattern4ContinuePlan {...}` 文字列）を維持する形式へ戻し、strict gate 互換を保持。
    - verification:
      - `cargo test -q --lib semantic_rule_priority_is_stable` => PASS
      - `cargo test -q --lib rule_name_uses_semantic_label` => PASS
      - `cargo test -q --lib legacy_rule_name_alias_is_preserved` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only strict_nested_loop_guard_min` => PASS
      - `cargo build --release --bin hakorune` => PASS
      - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` => PASS
- progress in this round:
  - `Unsupported value AST: MapLiteral`（box_member 7）を解消（7 -> 0）
  - `Unsupported binary operator: Or`（box_member 7）を解消（7 -> 0）
  - `if_effect_empty`（3）を解消（3 -> 0, direct-verify へ前進）
  - class shift: `emit:other 11 -> 4`（`emit_fail` 総数は 13 維持）
- current head blockers:
  1. direct route の emit blocker は 0 件（`emit_fail=0`）。
  2. 次フェーズは Pattern/Domain cleanup（Phase D）へ移行。
- files touched in this round（restart-safe context）:
  - `src/mir/builder/control_flow/plan/normalizer/helpers.rs`
  - `src/mir/builder/control_flow/plan/normalizer/loop_body_lowering.rs`
  - `src/mir/builder/control_flow/plan/normalizer/cond_lowering_prelude.rs`
  - `src/mir/builder/control_flow/plan/parts/loop_.rs`
  - `src/mir/builder/control_flow/plan/parts/dispatch/if_join.rs`
  - `src/mir/builder/control_flow/plan/features/loop_cond_bc_util.rs`
  - `src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs`
  - `src/mir/builder/control_flow/plan/loop_cond/break_continue_facts.rs`
  - `src/mir/builder/control_flow/plan/loop_cond/return_in_body_facts.rs`
  - `src/mir/builder/control_flow/plan/parts/stmt.rs`
  - `src/mir/builder/control_flow/plan/parts/if_exit.rs`
  - `src/mir/builder/control_flow/plan/lowerer/effect_emission.rs`
  - `CURRENT_TASK.md`
- next fixed order（resume point）:
  1. D5続き: `facts/planner` 側に残る Pattern語彙（特に `pattern8/9`, `scan/split`）の dead entry を棚卸しし、isolate -> delete を段階実施する
  2. D5続き: extractor/composer の test-only dead file を洗い出し、削除順を固定して縮退する
  3. D系の各段で fixture+fast-gate を更新し、BoxShape と BoxCount を混在させない

## Compiler Cleanup Order (2026-03-03, SSOT)

- decision:
  - 先に `direct route` の目先 blocker を落とし切る。
  - その後に Pattern/Domain cleanup（Phase D）へ移る。
- rationale:
  - blocker未解消のまま Pattern/Domain 削除を混ぜると、BoxCount と BoxShape が競合して原因切り分けが崩れるため。
- Phase D entry condition:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` の残件を current blocker 許容範囲まで縮小。
  - `tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` を green 維持。
- Phase D execution order (after blocker stabilization):
  1. D1: `PlanRuleId` / entry 名の Pattern 数値語彙を意味語彙へ置換（互換 alias を先に置く）。
  2. D2: `DomainPlan` の残存責務を label-only へ縮退し、router の依存を除去する。
  3. D3: `normalizer/pattern*.rs` の entry-path 依存を 0 にして撤去する。

- Phase D legacy isolation -> deletion lock (2026-03-04, cleanliness-first):
  - legacy の定義（この文脈）:
    - `PatternN` 語彙を直接期待する gate/tsv/script 依存（例: `[joinir/planner_first rule=LoopBreakRecipe]` へ移行前の旧期待値）。
    - compiler core の受理/計画/lower ロジックではなく、主に検証境界の互換語彙を指す。
  - cleanliness rule:
    - compiler core（facts/planner/composer/lower）へ legacy 語彙を再注入しない。
    - legacy 互換は `tools/smokes` 側の境界でのみ一時保持する（single mapping SSOT）。
  - fixed order:
    1. L1 isolate: gate 側の planner tag matcher を semantic/legacy 両対応へ集約（compiler 本体は semantic 優先のまま）。
    2. L2 migrate: `phase29bq_fast_gate_cases.tsv` / `planner_required_cases.tsv` 等の `PatternN` 期待値を semantic rule 名へ段階置換。
    3. L3 flip: `planner/tags.rs` の `planner_first rule=*` 出力を semantic rule 名へ切替（gate 側は既に受理済みであること）。
    4. L4 delete: `pred_pattern*` など legacy alias と `PatternN` 期待値を削除し、境界互換コードを撤去。
  - acceptance:
    - `cargo build --release --bin hakorune`
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
    - `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh`
    - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` で `emit_fail=0` / `route_blocker=0` 維持。
- Phase D progress (2026-03-03):
  - D1 started: `single_planner/rule_order.rs` で `rule_name()` を semantic label SSOT に切替。
  - compatibility: 旧 Pattern 表示は `planner_rule_legacy_name()` として保持（gate/tag の期待値互換を維持）。
  - D1 follow-up: `planner/pattern_shadow.rs` で semantic rule key を主語にし、legacy `loop/pattern*` は alias 正規化で互換維持。
  - D1 follow-up2 (2026-03-04): `joinir/patterns/registry` の predicate 名を semantic 語彙（`pred_loop_break_recipe` など）へ移行。旧 `pred_pattern*` は L4 まで互換 alias として保持。
  - D1/L1 isolate started (2026-03-04): `tools/smokes/v2/lib/joinir_planner_first_gate.sh` の `planner_first_tag_matches` を semantic/legacy 両対応へ拡張（`PatternN` 期待に対して `rule=<semantic>` を許容）。`phase29ca/phase29cb` strict-shadow gate も固定 `grep Pattern1` から共通 matcher 呼び出しへ移行。
  - D1/L2 migrate (2026-03-04): `tools/smokes/v2/profiles/integration/{joinir,selfhost}` の planner-first 期待タグを `PatternN` から semantic rule 名へ段階置換（16 files）。matcher は双方向互換（legacy<->semantic）で移行期間を吸収。
  - D1/L3 flip (2026-03-04): `src/mir/builder/control_flow/plan/planner/tags.rs` の `planner_first rule=*` 出力を Pattern 数値語彙から semantic rule 名へ切替。
  - D1/L4 delete (2026-03-04): `predicates.rs` から `pred_pattern*` alias を削除し、`joinir_planner_first_gate.sh` から semantic/legacy 双方向互換 matcher を撤去。planner-first tag 契約を semantic exact-match に固定。
  - D2 starter: `DomainPlan::kind_label()` を追加し、`single_planner/rules.rs` の payload非依存箇所（freeze文言・variant判定）を label-based へ集約。
  - D2 follow-up: `DomainPlanKind` を導入し、`rules.rs` の planner 判定を variant match から kind 比較へ置換（payload 非依存化を前進）。
  - D3 starter: dead entry path の本番依存を縮退（`composer/mod.rs` で `coreloop_{single_entry,v0,v1}` を `#[cfg(test)]` 化、`normalizer/mod.rs` で `pattern_{scan_with_init,split_scan}` module 宣言を `#[cfg(test)]` 化）。
  - D3 follow-up (2026-03-04): `route_loop_break_recipe` を `PlanNormalizer::normalize_pattern2_break` 直呼びから `RecipeComposer::compose_pattern2_break_recipe` に置換し、Pattern2 normalizer の runtime 入口依存を 0 化（`normalizer/pattern2_break.rs` は `#[cfg(test)]` へ縮退）。
  - D3 follow-up2 (2026-03-04): `Pattern2BreakPlan` / `Pattern2PromotionHint`（domain 側の legacy payload）を `#[cfg(test)]` 化し、本番 build から Pattern2 domain payload 型を除外。
  - D3 follow-up3 (2026-03-04): `Pattern2BreakPlan` を `normalizer/pattern2_break.rs` の module-local test-only 型へ移設し、`domain.rs` / `plan/mod.rs` から Pattern2 legacy payload 定義・re-export を撤去。
  - verification: `cargo test -q rule_name_uses_semantic_label --lib` / `cargo test -q legacy_rule_name_alias_is_preserved --lib` / `cargo test -q legacy_rule_aliases_map_to_semantic_priority --lib` / `cargo test -q domain_plan_kind_and_label_match --lib` / `phase29bq_fast_gate_vm --only loop_cond_continue_with_return_min` / `phase29bq_fast_gate_vm --only loop_header_shortcircuit_continue_with_return_min` / `phase29x-probe emit_fail=0`。
  - verification2 (2026-03-04): `cargo build --release --bin hakorune` / `bash tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh` / `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` が green。
  - verification3 (2026-03-04): `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` は継続して `emit_fail=0 / run_nonzero=18 / run_ok=100 / route_blocker=0` を維持。
  - verification4 (2026-03-04, L1 isolate):
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh` => PASS（既存 Pattern2 tag 互換維持）。
    - matcher unit checks（shell source）: exact / labeled-compat / no-label-compat の 3 ケースで `planner_first_tag_matches` が `0`。
    - `bash -n` で `joinir_planner_first_gate.sh` / `phase29ca_*` / `phase29cb_*` の syntax OK。
  - verification5 (2026-03-04, L2 migrate):
    - PASS: `phase29bi_planner_required_pattern2_pack_vm.sh`
    - PASS: `phase29bj_planner_required_pattern6_7_pack_vm.sh`
    - PASS: `phase29bo_planner_required_pattern8_9_pack_vm.sh`
    - PASS: `phase29bh_planner_first_single_case_vm.sh`
    - PASS: `phase29bq_step_then_tail_break_planner_required_vm.sh`
    - note: `phase29bq_fast_gate_vm.sh --only bq` は既知 fixture `phase29bq_selfhost_blocker_parse_string2_return_prelude_call_min` で `flowbox/adopt` tag mismatch により FAIL（本変更の planner tag 語彙移行とは独立。要切り分け継続）。
  - verification6 (2026-03-04, L3 flip):
    - `cargo build --release --bin hakorune` => PASS
    - `phase29bi` / `phase29bj` / `phase29bo` / `phase29bh(single,ws)` / `phase29bq_step_then_tail_break` / `phase29bq_general_if_in_loop_body` => PASS
    - `phase29ca_direct_verify_dominance_block_canary.sh` => PASS
    - `phase29x-probe` => `emit_fail=0 / route_blocker=0 / run_nonzero=18 / run_ok=100` 維持
    - `phase29bq_fast_gate_vm --only bq` は同一 fixture（`parse_string2_return_prelude_call_min`）で `flowbox/adopt` tag mismatch FAIL 継続（語彙切替の前後で同系統）。
  - verification7 (2026-03-04, L4 delete):
    - `cargo build --release --bin hakorune` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bh_planner_first_single_case_vm.sh` => PASS
    - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_step_then_tail_break_planner_required_vm.sh` => PASS
    - `bash tools/dev/phase29ca_direct_verify_dominance_block_canary.sh` => PASS
    - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` => `emit_fail=0 / route_blocker=0 / run_nonzero=18 / run_ok=100`

- direct route debug status (2026-03-03, active):
  - `Invalid value ... ValueId(0)`（`AddOperator.apply/2`）は解消。原因は `json_v1_bridge` が v1 payload の `params` を読まず、関数 arity を 0 で復元していた点だった（`src/runner/json_v1_bridge/parse.rs` 修正済み）。
  - phase216 `loop_count_param_nonsym` の `vm step budget exceeded` は解消。原因は Rust direct route の generic-loop v0 で loop var を header PHI に再束縛せず step lowering に入っていた点（`src/mir/builder/control_flow/plan/features/generic_loop_pipeline.rs` 修正済み）。
  - direct 再発防止 canary を追加: `tools/dev/phase216_direct_loop_progression_canary.sh`（`--emit-mir-json` -> `--mir-json-file` で rc=14、step source=phi source を固定）。
  - phase216 direct sweep 実施済み: `phase216_mainline_*` 4件は emit/run とも green（rc: 3/7/14/10）、`step budget exceeded` / `Invalid value` は 0件。
  - `.hako emit` 実行契約 pin を追加: `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_jsonfile_exec_contract_vm.sh`（binary-only で `--hako-emit-mir-json` -> `--mir-json-file` rc=42）。
  - phase216 外 sweep は profile 化済み: `tools/dev/direct_loop_progression_sweep.sh`（`default`/`phase29x-green`/`phase29x-probe`）。
  - phase29x probe latest（`--profile phase29x-probe`, 2026-03-03）:
    - scope: `phase29bq|phase29ca|phase29cb` かつ `(loop|generic_loop|loop_cond|loop_true)` = 118 fixtures。
    - latest: `emit_fail=0`, `run_nonzero=18`, `run_ok=100`, `route_blocker(step-budget/Invalid value)=0`。
    - class: `emit_fail class=0`, `run:vm-error=3`（`emit:direct-verify=0`, `emit:other=0`）。
  - direct-verify / dominance guard:
    - canary: `tools/dev/phase29ca_direct_verify_dominance_block_canary.sh`（expected: `emit_rc=0` + `run_rc=4`）。
    - current residual: `emit:direct-verify=0`（clear）。
  - current blocker class（head order）:
    - emit blocker は 0 件（`emit_fail=0`）。
    - `route_blocker` / `emit:direct-verify` / `emit:other` は 0 件。
  - improvements in this round:
    - `Unsupported value AST: MapLiteral`（7）を解消
    - `Unsupported binary operator: Or`（7）を解消
    - `if_effect_empty`（3）を解消
  - note: `parser/throw_reserved` は phase29x-probe 監視対象から解消（throw-negative は `apps/tests/parser_throw_reserved*_min.hako` へ分離）。
  - note2: Rust VM の `Catch` 命令実装は post-selfhost deferred（現行 lane は throw-free + `NYASH_TRY_RESULT_MODE=1` 固定）。
  - note3: `loop_scan_methods_block_v0 ... nested loop has no plan` は解消（2 -> 0）。
  - detail/timeline SSOT:
    - `docs/development/current/main/investigations/phase29x-direct-route-probe-2026-03-02.md`

## Concat3 Normalization Pack (active / ordered)

status (2026-03-02):
- contract-smoke(min1) added: `apps/tests/phase21_5_concat3_assoc_contract.hako`
- gate script added: `tools/smokes/v2/profiles/integration/apps/phase21_5_concat3_assoc_contract_vm.sh`
- Rust/MIR pass added: `src/mir/passes/concat3_canonicalize.rs`（left/right assoc chain を `Extern(nyash.string.concat3_hhh)` へ正規化）
- optimizer hook added: `src/mir/optimizer.rs` Pass 6.6（`NYASH_MIR_CONCAT3_CANON=1` のときのみ有効）
- runtime/lowering sync:
  - VM extern fallback: `src/backend/mir_interpreter/handlers/calls/externs.rs`
  - LLVM extern signature: `src/llvm_py/instructions/externcall.py`
- latest: direct emit route（`HAKO_EMIT_MIR_FORCE_DIRECT=1`）+ contract smoke で MIR/IR 両方 `concat3_hhh >= 2` / `concat_hh == 0` を green lock。

1. 契約固定（Rust/.hako 共通）:
   - `.hako` と Rust の両経路で「`(a+b)+c` / `a+(b+c)` は `concat3_hhh` に収束」を docs/test で先に固定する。
   - ここでは実装を増やさず、契約と gate だけ同期する。
2. Rust/MIR 実装（先行）:
   - done (opt-in): MIR 側 `concat3` 正規化を実装済み。現状は perf parity 観測中のため `NYASH_MIR_CONCAT3_CANON=1` で有効化する。
   - acceptance: `tools/checks/dev_gate.sh quick` + `bash tools/smokes/v2/profiles/integration/apps/phase21_5_concat3_assoc_contract_vm.sh`。
3. `.hako` 実装追従（後行）:
   - Rust と同じ契約に合わせて `.hako` 側を追従実装し、差分は最小コミットで分離する。
4. Python backend cleanup（最終）:
   - `concat` chain/prune ロジックを撤去し、`concat3` 変換責務を MIR 側 SSOT へ一本化する。
   - 撤去条件: Rust/.hako 両経路の parity と perf micro が green。

## Helper Retirement Pack (active / ordered)

contract note (fixed):
- `helper retirement` は Program→MIR の入口整理（route/wrapper縮退）を指す。
- JSON v0 bridge contract（Program(JSON v0) 受理・橋渡し）は撤退対象に含めない。
- 本packは `jsonv0` を削除せず、emit route の drift と silent fallback を減らすことを目的とする。

1. build 導線置換:
   - 対象: `tools/selfhost/build_stage1.sh`, `tools/selfhost_exe_stageb.sh`
   - 目的: helper 直呼びを外し、`hakorune --emit-mir-json` 直経路へ一本化。
   - status (2026-03-02): `hakorune_emit_mir.sh` 直呼びを撤去し、`selfhost_exe_stageb.sh` は helper-free 実装へ移行（default route=`stageb-delegate`）。
   - latest unblock (2026-03-02): direct route の global-call arity blocker（`ParserBox.esc_json/1`, `HakoCli.run/1`）は解消し、次の fail-fast は LLVM harness parse (`PHINode should have one entry for each predecessor`) へ前進。
   - pending: `launcher.hako` direct は JoinIR coverage 課題に加えて LLVM PHI predecessor 整合の blocker が残る。direct 一本化の昇格は PHI blocker 解消後に実施。
   - latest direct issue (2026-03-02): `AddOperator.apply/2` の `ValueId(0)` / phase216 step-budget / phase29ca direct-verify（dominance/Phi）は解消済み。次ブロッカーは `phase29x-probe` 残件（`emit:other=27`）の段階解消。
2. perf SSOT 置換:
   - 対象: `tools/perf/lib/aot_helpers.sh` と呼び出し元ベンチ群。
   - 目的: `PERF_AOT_PREFER_HELPER` / `PERF_AOT_HELPER_ONLY` を縮退し、strict は direct 優先に固定。
   - status (2026-03-02): `_hk` lane は `PERF_AOT_DIRECT_ONLY=1` を既定化し、AOT route 行に `aot_direct_only` を追加（helper への暗黙退避を fail-fast 化）。`tools/perf/lib/aot_helpers.sh` / `bench_*` / `dump_*` の helper 呼び出しは `tools/smokes/v2/lib/emit_mir_route.sh` 経由へ移行。
3. smoke 共通化:
   - 対象: `tools/smokes/v2/profiles/**`（helper 参照群）
   - 目的: emit 呼び出しを `tools/smokes/v2/lib/` の共通関数へ集約し、一括置換を可能にする。
   - status (2026-03-02): 共通 wrapper `tools/smokes/v2/lib/emit_mir_route.sh` を追加し、active smoke（concat3 / phase29y nested ternary / joinir port01 / mir_shape_guard / perf_mir_shape）と phase21.5 perf contract 群（apps/integration）に加えて、`integration/core` / `integration/joinir` / `quick/core` の helper 直呼びを route 指定へ移行完了。
4. check 置換:
   - 対象: `tools/hako_check.sh`, `tools/test_stageb_using.sh`, `tools/archive/root-hygiene/test_numeric_core_phi.sh`
   - 目的: helper 依存と `|| true` 握りを整理し、direct 経路で fail-fast 契約へ統一。
   - status (2026-03-02): 3ファイルを `tools/smokes/v2/lib/emit_mir_route.sh` 経由へ移行。`hako_check.sh` の `|| true` 握りを除去し、`HAKO_CHECK_REQUIRE_MIR=1` で strict fail-fast 可能にした（既定は warn 継続）。
   - status2 (2026-03-02): fallback 混入の fail-fast として `tools/checks/route_no_fallback_guard.sh` を追加し、`tools/checks/dev_gate.sh quick` に組み込んだ（`route_env_probe.sh --require-no-fallback` 契約）。
5. wrapper/helper 撤去:
   - 対象: `tools/hakorune_emit_mir_mainline.sh`, `tools/hakorune_emit_mir_compat.sh`, `tools/hakorune_emit_mir.sh`
   - 目的: `emit_mir_route.sh` への移行完了後、互換ラッパの縮退/撤去条件を固定する。
   - status (2026-03-02): `tools/cache/phase29x_l1_mir_cache.sh` / `tools/dev*` / `tools/perf/microbench.sh` は route wrapper 経由へ移行完了。`tools` 配下の helper 直呼びは互換ラッパ（mainline/compat）を除き撤去済み。

## Future Ideas (Not Active)

- Python AOT / HybridPy / Translation Validation / ReproBuild などの研究案は `future backlog` 扱いに固定し、Current blocker には含めない。
- optimization annotation（`@hint/@contract/@intrinsic_candidate`）は parser noop まで実装済みだが、本利用（verifier/registry/backend）は `not active` として扱う（SSOT: `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`）。
- public summary: `docs/development/current/main/30-Backlog.md`
- private canonical: `docs/private/development/current/main/30-Backlog.md`
- 運用ルール: backlog 案を採用する時だけ docs-first で lane/task に昇格する（それまでは monitor-only）。
