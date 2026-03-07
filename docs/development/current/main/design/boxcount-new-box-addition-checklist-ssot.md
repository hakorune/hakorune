---
Status: SSOT
Scope: JoinIR planner-required BoxCount (Phase 29bq family)
Related:
- Debug workflow SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- Reject/handoff SSOT: `docs/development/current/main/design/plan-reject-handoff-gap-taxonomy-ssot.md`
- Box registry (code-side): `src/mir/builder/control_flow/plan/REGISTRY.md`
- Phase example: `docs/development/current/main/phases/phase-29bq/README.md`
---

# BoxCount: “新しい箱を一発で追加する”チェックリスト（SSOT）

目的: 1 blocker を **1つの受理形**として、迷わず `Facts→Recipe→Verifier→Lower` を通し、fixture+fast gate で契約固定する。

## 0. 前提（必須）

- `.hako` workaround で通さない（Rust側の小箱で受理形を増やす）。
- no AST rewrite（analysis-only view）。
- current runtime boundary は `Facts → Recipe → Verifier → Lower`。historical planner-payload wording は新規追加点の主語にしない。
- **fast gate FAIL の状態で `phase29bq_fast_gate_cases.tsv` に追加/更新してコミットしない**（WIPはstash）。
- “完成品キット”（骨格/PHI/walker の複製）を増やさない。まず BoxShape（共通部品SSOT/入口統一/テーブル化）を検討する（SSOT: `docs/development/current/main/design/lego-composability-policy.md`）。

## 1. 入口の確定（SSOT順）

1. `docs/development/current/main/phases/<phase>/README.md` の最新ブロッカー関数名とログパスを読む
2. ログから *実際に落ちている関数* を確定（entry fixtureではないことが多い）
3. `StepTree root for '.../...'` で形（bodyのstmt列）を最小化してメモする
4. `plan/trace` で候補の消失点を分類（Gap taxonomy SSOT参照）

## 2. 追加する“箱”のスコープを決める（1形だけ）

この2つをREADMEに明文化できるなら着手してよい:

- **In-scope (v1 minimal)**: 受理する形（fixture由来の1形）
- **Out-of-scope**: 受理しない形（reject→handoff先も書く）

Reject→handoffの表現は `docs/development/current/main/design/plan-reject-handoff-gap-taxonomy-ssot.md` に従う。

## 3. fixture（最小再現）を先に作る

- 置き場所: `apps/tests/phase29bq_selfhost_blocker_<short>.hako`
- 原則:
  - 1つの loop/if 断片だけに絞る（selfhost全体は持ち込まない）
  - 出力は `print(<int>)` などで安定化
  - 期待stdoutは **数字 or 空**（スモークはstdout完全一致）

## 4. fast gate へ pin（PASSできる状態になってから）

- 置き場所: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`
- 必須列:
  - `fixture` / `expected(stdout)` / `allowed_rc` / `planner_tag` / `case_id` / `note`
- 注意:
  - `expected` は `0` / `1` / `__EMPTY__` のいずれか（MISS等は不可）
  - `planner_tag` は実際に出るタグのみ（例: `[joinir/planner_first rule=LoopCondBreak]`）

## 5. 変更点（コード側の“追加点”一覧）

新箱（Loop(cond)系）を増やす時の最小タッチポイント（典型）:

1. Facts
   - `src/mir/builder/control_flow/plan/<new_box>/facts.rs`
   - 抽出呼び出し: `src/mir/builder/control_flow/plan/facts/loop_facts.rs`
2. Planner candidate push（planner_requiredのときだけ）
   - `src/mir/builder/control_flow/plan/planner/build.rs`
3. Recipe payload / route module export
   - `src/mir/builder/control_flow/plan/mod.rs`
   - route module の README / entry wrapper（必要な場合）
4. Verifier / compatibility-lane lower wrapper
   - `src/mir/builder/control_flow/plan/recipe_tree/verified.rs`
   - `src/mir/builder/control_flow/plan/normalizer/mod.rs`（legacy wrapper が必要な場合のみ）
5. Pipeline（lower本体）
   - `src/mir/builder/control_flow/plan/features/<new_box>_pipeline.rs`
   - `src/mir/builder/control_flow/plan/features/mod.rs` に `mod` 追加
6. Single-planner rule mapping（planner_tag / accept route を取れるように）
   - `src/mir/builder/control_flow/plan/single_planner/rule_order.rs`
   - `src/mir/builder/control_flow/plan/single_planner/rules.rs`
7. Registry / README（人間の導線）
   - `src/mir/builder/control_flow/plan/REGISTRY.md`
   - `src/mir/builder/control_flow/plan/<new_box>/README.md`

### 5.x loop_cond_continue_only 固有（レシピ追加）

新しい `ContinueOnlyStmtRecipe` variant を追加する際は、以下のチェックリストを参照:
→ `src/mir/builder/control_flow/plan/loop_cond_continue_only/README.md#new-recipe-addition-checklist`

## 6. 最小の検証コマンド（順）

1. `cargo check --bin hakorune`
2. `cargo build --release --bin hakorune`
3. `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only <case_id>`
4. canary（確認として1回）:
   - semantic example: `NYASH_ROOT=/home/tomoaki/git/hakorune-selfhost NYASH_BIN=target/release/hakorune HAKO_JOINIR_DEBUG=1 SMOKES_ENABLE_SELFHOST=1 SMOKES_SELFHOST_FILTER=if_phi_join ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
   - compat example: `NYASH_ROOT=/home/tomoaki/git/hakorune-selfhost NYASH_BIN=target/release/hakorune HAKO_JOINIR_DEBUG=1 SMOKES_ENABLE_SELFHOST=1 SMOKES_SELFHOST_FILTER=phase118_pattern3_if_sum_min ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
     - 現役の route semantics は `if_phi_join` として読み、legacy selfhost test stem / fixture key の詳細は pin inventory SSOT を参照する。
     - pin taxonomy SSOT: `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`
     - selfhost gate の絞り込みは `--only` ではなく `SMOKES_SELFHOST_FILTER=<substring>` が正本。

## 7. よくある失敗（このチェックで防ぐ）

- Factsが `Some` を返すのに Lower が下ろせない（Facts→Lower契約破れ）
  - 対策: “受理条件”をREADMEに固定し、LowerはFacts/Recipeのみに従う（再判定しない）
- “受理条件”が意味論寄りになり、別の場所の lower/feature とズレる（acceptance drift）
  - 対策: Facts は **Recipe（レシピ）** を返す（`Ok(Some(recipe))` のとき lower 可能であることを契約にする）。
  - 対策: Lower は **recipe だけを見る**（同じ判定ロジックを再実装しない）。
- rejectログを出したのに手元で処理を続けてしまい、候補が残る
  - 対策: Reject→handoffを“入口”で一括判断する（必要なら Unsupported recipe を使う）
- 「別関数のログ」を見て誤認する
  - 対策: 常に `StepTree root for '.../...'` をSSOTとする
