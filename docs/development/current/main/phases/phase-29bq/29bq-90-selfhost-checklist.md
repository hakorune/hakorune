---
Status: SSOT
Scope: Phase 29bq の “作業運用チェックリスト” を 1枚に固定（selfhost canary / pre-selfhost async stabilization / cleanliness）。
Related:
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-116-emit-mir-entry-order-blocker.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
---

# Phase 29bq — Selfhost / Gates Checklist (SSOT)

目的: “やることが多い” 状態でも、**順序・記録・ゲート緑維持**をチェックリストで守り、迷走を避ける。

## 0.25) Current blocker (2026-04-14)

- active blocker:
  - `none`
- latest landed blocker family:
  - `program_json_contract_pin` / `joinir_port04_phi_exit_invariant_lock` / `joinir_port07_expr_parity_seed`
  - compat bridge, parser/helper simplification, and disabled legacy-lowerer removal from mainline owners are landed
- current lane mode:
  - failure-driven
  - while blocker=`none`, use this lane for narrow cleanup only
  - next cleanup cut is `loop owner seam cleanup`

## 0.3) Loop owner split (current design direction)

- target flow:
  - `facts -> route -> recipe -> cfg skeleton -> join sig -> phi materializer -> verifier -> cleanup`
- owner rules:
  - `facts` are descriptive-only
  - `route` chooses only
  - `recipe` is normative and owns structural obligations
  - `cfg skeleton` emits blocks/edges/terminators only
  - `phi materializer` owns PHI/block-param closure only
  - `verifier` fail-fast only
  - `cleanup` canonicalizes only
  - do not absorb all of `plan/` into `recipe`; shrink `plan/` into a temporary lowering namespace and remove the name only after owner split is complete
- migration rule:
  - move one exact loop family at a time
  - do not combine acceptance-row widening with broader PHI hardening in the same commit
  - first landed family seam is `LoopCondReturnInBody` join-sig extraction; next move is `phi materializer` for one family only
- landed one-family completion:
  - `LoopCondReturnInBody`
    - `facts`
    - `route`
    - `recipe`
    - `join sig`
    - `phi materializer`
    - `verifier`
    - `cleanup`
- landed one-family completion:
  - `LoopCondContinueOnly`
    - `facts`
    - `route`
    - `recipe`
    - `cleanup`
    - `phi materializer`
    - `verifier`
- landed one-family completion:
  - `LoopCondBreakContinue`
    - `facts`
    - `route`
    - `recipe`
    - `cfg skeleton`
    - `phi materializer`
    - `verifier`
    - `cleanup`
- landed one-family completion:
  - `LoopCondContinueWithReturn`
    - `facts`
    - `route`
    - `recipe`
    - `cfg skeleton`
    - `phi materializer`
    - `verifier`
    - `cleanup`
    - route-local body helpers
- next one-family inventory (`GenericLoopV1`):
  - already separate:
    - `facts`
    - `route`
    - `recipe`
    - `cfg skeleton`
    - body lowering lives under `generic_loop_body/`
    - body terminality / continue-edge detection
  - landed:
    - route-local carrier prepare/body/finalize orchestration
    - route-local condition/step handoff
    - body-local fallthrough continue suppression
    - `body_check::validation_v0`
    - `body_check::validation_v1`
    - `body_check::shape_detection`
  - still mixed:
    - `body_check::shape_resolution` closeout
  - next step:
    - close out `body_check::shape_resolution`, then close out `GenericLoopV1` and inventory `nested_loop_depth1`
- next one-family inventory (`nested_loop_depth1`):
  - already separate:
    - `facts`
    - `route-local acceptance / fallback dispatch`
    - `preheader freshness rewrite`
    - `stmt-only fastpath ownership`
  - still mixed:
    - none confirmed
  - next step:
    - close out `nested_loop_depth1` and inventory `nested_loop_plan`
- next one-family inventory (`nested_loop_plan`):
  - already separate:
    - shared recipe-first fallback bridge
    - `loop_cond_continue_with_return` bridge
    - `loop_cond_break_continue` bridge
    - shared recipe fallback orchestration
    - recipe fallback selection policy
  - still mixed:
    - none confirmed
  - next step:
    - close out `nested_loop_plan` and inventory `generic_loop_body::nested_loop_plan`
- next one-family inventory (`generic_loop_body::nested_loop_plan`):
  - already separate:
    - local recipe-fallback ordering
    - `strict_nested_loop_guard` / `freeze_no_plan`
    - depth1 fastpath handoff
  - still mixed:
    - none confirmed
  - next step:
    - close out `generic_loop_body::nested_loop_plan` and inventory `loop_scan_phi_vars_v0`
- next one-family inventory (`loop_scan_phi_vars_v0`):
  - already separate:
    - nested-loop depth1 fastpath handoff
    - nested-loop recipe stmt-only / fastpath handoff
    - found-if branch stmt partition / nested dispatch
    - nested-loop segment arm
    - linear segment verification / lowering
  - still mixed:
    - none confirmed
  - next step:
    - close out `loop_scan_phi_vars_v0` and inventory `loop_scan_methods_block_v0`
- next one-family inventory (`loop_scan_methods_block_v0`):
  - already separate:
    - nested-loop recipe-first fallback handoff (`lower_nested_loop_plan`)
    - linear block recipe arm split
    - nested-loop stmt-only fastpath ownership
    - segment-level nested dispatch
  - still mixed:
    - none confirmed
  - next step:
    - close out `loop_scan_methods_block_v0` and inventory `loop_scan_methods_v0`
- next one-family inventory (`loop_scan_methods_v0`):
  - already separate:
    - nested-loop recipe-first fallback handoff
    - linear segment verification / lowering
    - nested stmt-only recipe handoff
    - helper-family closeout check
    - nested loop fallback bridge wrapper
    - nested segment dispatch
    - nested fallback segment wrapper
  - status:
    - landed and closed
  - likely first seam:
    - inventory next owner-local family
  - likely follow-on seams:
    - pin next exact seam after inventory
  - next step:
    - record the loop_scan_methods_v0 closeout and inventory the next smallest family
- next one-family inventory (`loop_scan_v0`):
  - already separate:
    - `facts`
    - `recipe`
    - route finalize
    - helper-family closeout (`apply_loop_final_values_to_bindings`)
    - nested-loop recipe-first fallback handoff
    - linear segment verification / lowering
    - nested stmt-only recipe handoff
    - nested segment dispatch
  - status:
    - landed and closed
  - still mixed:
    - none confirmed
  - next step:
    - record the `loop_scan_v0` closeout and inventory `loop_break_steps`
- next one-family inventory (`loop_break_steps`):
  - already separate:
    - `gather_facts_step_box`
    - `apply_policy_step_box`
    - `normalize_body_step_box`
    - `body_local_derived_step_box`
    - `carrier_updates_step_box`
    - `post_loop_early_return_step_box`
    - `emit_joinir_step_box`
    - `merge_step_box`
  - status:
    - landed and closed
  - still mixed:
    - `none confirmed`
  - next step:
    - record the `loop_break_steps` closeout and inventory `loop_break::api`
- next one-family inventory (`loop_break::api`):
  - already separate:
    - `promote_decision`
    - `promote_prepare_helpers`
    - `promote_finalize_helpers`
    - `promote_runner`
  - status:
    - landed and closed
  - still mixed:
    - `none confirmed`
  - next step:
    - inventory `body_local_policy`
- next one-family inventory (`body_local_policy`):
  - already separate:
    - `body_local_policy_helpers`
    - `body_local_policy_inputs`
    - `body_local_policy_types`
    - `body_local_policy_runner`
  - status:
    - landed and closed
  - still mixed:
    - `none confirmed`
  - likely follow-on seams:
    - `none confirmed`
  - next step:
    - re-inventory the next owner-local family under `loop_break`

- next one-family inventory (`loop_break/facts/body_local_facts`):
  - already separate:
    - `body_local_facts_helpers`
    - `body_local_facts`
    - `body_local_trim_matcher`
    - `body_local_digit_matcher`
    - `body_local_common`
    - `body_local_facts_shape_matchers`
  - status:
    - landed and closed
  - still mixed:
    - `none confirmed`
  - next step:
    - inventory `GenericLoopV1`

## 0.5) Milestone Quick Check（blocker capture後の節目）

次の3コマンドは節目チェック（push前/週次終端/回帰疑い時）として実行する。

- `cargo check --bin hakorune`
- `bash ./tools/selfhost/run_lane_a_daily.sh`
- `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`

判定:
- 3つとも PASS: failure-driven の通常運用を継続（新規fixture追加禁止）。
- どれか FAIL: 本文の PROBE→FIX→PROMOTE へ進む（1ブロッカー=1コミット）。

## 0) 原則（運用SSOT）

- selfhost は目的化しない（compiler-first / cleanliness-first）。
- 1ブロッカー = 1受理形 = fixture+gate = 1コミット（BoxCount と BoxShape を混ぜない）。
- fast gate FAIL の状態で `cases.tsv` を増やさない（WIP は stash）。
- “ログで通す” を禁止。stdout はユーザー出力、診断は stderr / ring0（strict+planner_required の sentinel は stderr 固定）。
- 移植順序は `selfhost-parser-mirbuilder-migration-order-ssot.md` を必ず守る（mirbuilder先行、parser後行）。

## 1) Start-of-work（毎回）

- [ ] `git status -sb`（dirty の場合は「意図した差分だけ」か確認）
- [ ] `git log -1 --oneline`
- [ ] `cargo check --bin hakorune`

## 2) Daily gates（軽い健康診断）

### 2.1 JoinIR fast gate（最優先）

- [ ] `bash ./tools/selfhost/run_lane_a_daily.sh`（sync/promotion guard + `--only bq`）

### 2.2 Async stabilization（VM+LLVM）

Phase‑0 semantics / backend parity の維持（真の並列性は非ゴール）。

- [ ] `bash tools/smokes/v2/profiles/integration/async/async_min_vm.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/llvm/async_min_harness.sh`

### 2.3 Selfhost canary（opt-in）

stdout 比較の揺れを避けるため、原則 `HAKO_JOINIR_DEBUG=0`。

- [ ] `HAKO_JOINIR_DEBUG=0 ./tools/selfhost/run.sh --gate --planner-required 1 --timeout-secs 120`

### 2.3.1 Selfhost quick/probe（重い時の標準）

- [ ] quick(5件): `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5`
- [ ] probe(1件): `./tools/selfhost/run.sh --gate --planner-required 1 --filter <case_substring> --max-cases 1`
- [ ] CPU が空いている場合は gate 並列を使う: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 20 --jobs 4`（既定は `--jobs 4`、不安定時のみ `1`）

### 2.3.2 Route parity smoke（入口ドリフト検知）

- [ ] 日常入口は `tools/selfhost/run.sh` を使う（`--gate|--runtime|--direct`）
- [ ] `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh`
- [ ] `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
- [ ] `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [ ] runtime route smoke は `SH-RUNTIME-SELFHOST mode=pipeline-entry` と `mode=stage-a` の両方を確認する
- [ ] EXE route を確認する場合は `bash ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh <fixture> exe`（parser EXE がある時のみ）
- [ ] Stage-B の gate 入口は `tools/selfhost/run_stageb_compiler_vm.sh` を維持し、inline direct 呼び出しを増やさない

### 2.3.3 Selfhost canary 実行頻度ルール（重さ管理SSOT）

- [ ] 日常運用は `quick(5件)` + `probe(1件)` を標準にする（full canary を常時回さない）。
- [ ] 旧基準値（2026-02-08）: full canary は `187` cases / `total_secs=798` / `avg_case_secs=4.27`（比較用に保持）。
- [x] full canary 基準値（履歴固定, 2026-02-10）: `198` cases / `total_secs=871` / `avg_case_secs=4.40`（parallel `jobs=4`）。
- [x] 現行運用基準（2026-02-28〜2026-03-01）: quick + steady-state evidence を日常基準とする（本節 2.4 の実測値を正本とする）。
- [ ] full canary は節目だけ実行する:
  - `planner_required_selfhost_subset.tsv` を更新して PROMOTE するとき
  - Rust 実装（bridge/parser/mirbuilder）を変更して push 前の最終確認をするとき
  - 進捗台帳（`29bq-91` の snapshot）を更新するとき
- [ ] docs-only 変更は full canary を省略し、`phase29bq_fast_gate_vm.sh --only bq` だけを必須にする。
- [ ] 同じ作業ブロックで full canary を複数回回さない（再実行は「FIX直後の再検証」に限定）。

### 2.3.4 MirBuilder移植後の標準チェック（.hako parser/mirbuilder変更時）

この節は Rust 本体ではなく、`.hako` 側の parser/mirbuilder を移植した後に「Rust compiler 契約が壊れていないか」を確認するための既定手順。

- [ ] quick（毎コミット）: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh`
- [ ] milestone（PROMOTE前/日次終端）: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1`
- [ ] bq同時再確認が必要なときだけ: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1 --with-bq`
- [ ] `.hako` parser/mirbuilder 変更で quick が FAIL した場合は subset PROMOTEを止め、`CURRENT_TASK.md` に freeze/reject 先頭1行を記録する。
- [ ] docs-only / Rust-only 変更ではこの節を必須にしない（`--only bq` を優先）。
- [ ] Rust `--emit-mir-json` / MIR JSON externalization を触ったら: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_emit_mir_entry_order_ternary_basic_vm.sh`
- [ ] llvmlite harness `mir_call` collection initializer lowering を触ったら: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_harness_arraybox_birth_ternary_basic_vm.sh`

### 2.3.5 Identity check（stage1-first 既定）

- [ ] de-rust lane を触る日は先に boundary inventory を確認: `docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md` の `Current boundary inventory (2026-02-11 snapshot, stage1-first)`
- [ ] smoke default（`--cli-mode` 省略）: `bash ./tools/selfhost_identity_check.sh --mode smoke --skip-build`
- [ ] full default（`--cli-mode` 省略）: `bash ./tools/selfhost_identity_check.sh --mode full --skip-build`
- [ ] 既定パス（`--cli-mode` 省略時 / stage1-first）: `target/selfhost/hakorune.stage1_cli` / `target/selfhost/hakorune.stage1_cli.stage2`
- [ ] `--cli-mode auto` は互換診断専用（`[identity/compat-fallback]` の観測用途）で、full-mode の証拠として扱わない
- [ ] stage0 route を使う場合は明示: `bash ./tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode stage0 --bin-stage1 target/selfhost/hakorune --bin-stage2 target/selfhost/hakorune.stage2`

### 2.4 Green だったら（failure-driven運用; PROBE→FIX→PROMOTE）

“selfhost を回しているのに進まない” を防ぐため、Green のときは **無理に新規fixtureを増やさず**、失敗発生時だけ追加する。

- [ ] 日常ループ（最軽量）: `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`（実測 `~0:10`）
- [ ] 節目チェック（コミット前）: `./tools/selfhost/run.sh --steady-state --quiet`（実測 `~1:36`; 詳細は `/tmp/phase29bq_selfhost_steady_state_*`）
- [ ] ログ掃除が必要な時だけ: `./tools/selfhost/run.sh --steady-state --quiet --cleanup-old-logs`（2日超の steady-state ログを削除）
- [ ] 日次終端だけ runtime parity: `./tools/selfhost/run.sh --steady-state --quiet --with-runtime-parity`（実測 `~1:45`）
- [ ] bq fail時は blocker collector が自動起動される（必要なら `./tools/selfhost/run.sh --steady-state --no-collect-blocker` で無効化）
- [ ] `.hako` mirbuilder の移植本体は `29bq-113-hako-recipe-first-migration-lane.md` の順序（R0→R6）に従う。

#### (A) PROBE（ローカル; コミットしない）

- [ ] `CURRENT_TASK.md` が `Current blocker: none` の間は、新規fixtureを追加しない（quick + 既存probeのみ維持）。
- [ ] 新規追加は次の3条件のどれかを満たした時だけ実施する:
  - `first_freeze_or_reject` が新規形で発生した
  - 既存green fixture が回帰した
  - language/contract SSOT の Decision 変更が入った
- [ ] PASS → (C) PROMOTE へ / FAIL → (B) FIX へ

#### (B) FIX（1ブロッカー=1コミット）

- [ ] FAIL の `first_freeze_or_reject` を `/tmp/*summary` で確定し、`CURRENT_TASK.md` に “2行” だけ記録する
- [ ] ブロッカーを 1コミットで潰す（fixture + fast gate pin）

#### (C) PROMOTE（subset 追加だけの1コミット）

- [ ] PROBE で追加が必要になった “最小1ユニット” を `planner_required_selfhost_subset.tsv` に反映する（**コード変更禁止**）
- [ ] selfhost canary を通してから commit（PASS が受理条件）

## 3) Milestone gates（節目だけ）

時間が重いので “節目” のみに限定する（普段は回さない）。

- [ ] `bash ./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`

必要なら（回帰距離短縮のための追加）:
- [ ] `bash ./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`

## 4) FAIL したとき（最短で固定）

### 4.1 Selfhost / planner_required のブロッカー採取

- [ ] `./tools/smokes/v2/profiles/integration/joinir/phase29bq_collect_planner_required_blocker_vm.sh apps/tests/<fixture>.hako <label>`
- [ ] `/tmp/phase29bq_joinir_blocker_<label>_*.summary` を確認
- [ ] `CURRENT_TASK.md` に “2行” だけ記録（summary先頭 + first_freeze_or_reject）

### 4.2 fast gate が FAIL のとき

- [ ] `git stash push -u -m "wip/<topic> (fails fast gate)"`（WIP は退避）
- [ ] `docs/development/current/main/10-Now.md` または phase README に “結論だけ” 反映（ログ本文を抱えない）

## 5) Commit / Push（毎コミット）

- [ ] `cargo check --bin hakorune`（最低限）
- [ ] 必要な “Daily gates” が緑の状態で commit（BoxShape/BoxCount を混ぜない）
- [ ] `git push private main`

## 6) よくある混線の注意

- `nowait/await` の議論:
  - selfhost v1 サブセットに “必須” ではないが、構文削除で逃げない（意味論は pre-selfhost SSOT に pin）。
- “spawn” の語:
  - docs の一般名。Nyash の表面構文は `nowait`（用語混同を避ける）。
