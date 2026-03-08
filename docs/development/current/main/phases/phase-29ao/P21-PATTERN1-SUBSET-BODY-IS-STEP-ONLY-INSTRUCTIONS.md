---
Status: Ready
Scope: code+tests+docs（LoopSimpleWhile subset / historical label 1 の誤マッチ防止、仕様の“正しさ”を固定）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs
  - src/mir/builder/control_flow/plan/extractors/loop_simple_while.rs
  - src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs
---

# Phase 29ao P21: LoopSimpleWhile subset の「body は step のみ」契約固定（CorePlan shadow adopt の安全化; historical label 1）

Date: 2025-12-30  
Status: Ready for execution  
Goal: strict/dev の LoopSimpleWhile shadow adopt（Facts→CorePlan(skeleton)）が意味論を壊さない条件を、抽出段（Facts/Extractor）で SSOT として固定する。

## 背景 / 問題

- 現在の LoopSimpleWhile（historical label 1, SimpleWhile）は “PoC subset” として `condition + step(i=i+1) + それ以外は無し` を前提に CorePlan/Frag を組み立てる。
- しかし、抽出が **「step が見つかればOK」** になっていると、`sum = sum + 1; i = i + 1` のような “追加の body 効果” を含むループを LoopSimpleWhile と誤認しうる。
- strict/dev の shadow adopt でこの誤マッチが起きると、CorePlan skeleton が body 効果を落として意味論が変わる（Fail-Fast でも検知しづらい）。

このP21では、**LoopSimpleWhile subset の契約を「body は step のみ」へ引き締め、誤マッチを構造条件で遮断**する。

## 非目的

- LoopSimpleWhile の対応範囲拡張（“body に任意の副作用を許す” など）
- ループ一般を CorePlan に移行する大改造（P22+）
- 新しい env var / by-name 分岐 / 恒常ログ追加

## 実装方針（SSOT）

### 1) 「body は step のみ」を SSOT として 1 箇所に置く

要求:
- LoopSimpleWhile の subset 条件を **Facts と Plan extractor の両方が同じ判定**で使う（分岐の二重定義を避ける）。

推奨:
- 新規: `src/mir/builder/control_flow/plan/policies/loop_simple_while_subset_policy.rs`
  - `pub(crate) fn is_loop_simple_while_step_only_body(body: &[ASTNode], loop_var: &str) -> bool`
  - 許可する body（最小）:
    - `i = i + 1`（PoC subset）
    - `ScopeBox` は今回は不許可でよい（必要になったらP22で拡張）
  - 不許可:
    - それ以外のステートメントが 1 つでも存在
    - `i = i + 1` が複数回
    - `i` 以外の代入、call、if、など

### 2) Facts 側（LoopSimpleWhileFacts）をこの SSOT で gate する

対象:
- `src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs`

やること:
- `extract_loop_increment_plan` で increment を見つけた後に、
  `is_loop_simple_while_step_only_body(body, &loop_var)` を必須条件として追加。
- NG の場合は `Ok(None)`（fallback 維持、Freeze は使わない）。

狙い:
- planner 側が LoopSimpleWhile facts を持たないようにし、shadow adopt の危険域を遮断。

### 3) Plan extractor 側（DomainPlan LoopSimpleWhile / historical label 1）も同じ SSOT で gate する

対象:
- `src/mir/builder/control_flow/plan/extractors/loop_simple_while.rs`

やること:
- `extract_loop_simple_while_parts` の最終判定（parts/condition/inc を満たした後）で
  `is_loop_simple_while_step_only_body(body, &parts.loop_var)` を必須条件として追加。
- NG の場合は `Ok(None)`（legacy fallback 維持）。

## 回帰テスト（必須）

### A) 既存の “strict shadow adopt を踏む” テストは維持

- 既存: `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh`
  - fixture: `apps/tests/loop_simple_while_strict_shadow_min.hako`
  - historical fixture token は inventory lane にのみ残す
  - 期待: exit=3

### B) 誤マッチ防止の新規 fixture + smoke を追加（必須）

新規 fixture（例）:
- `apps/tests/loop_simple_while_subset_reject_extra_stmt_min.hako`
  - historical fixture token は inventory lane にのみ残す
  - `sum = sum + 1; i = i + 1` を含む（sum を return）
  - 正しい意味論: exit=3
  - LoopSimpleWhile subset で誤マッチすると body が落ちて exit=0 になるため、確実に検出できる

新規 smoke（joinir strict で実行）:
- `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_subset_reject_extra_stmt_vm.sh`
  - `NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1` を付けて VM 実行
  - 期待: exit=3

回帰ゲート（SSOT）へ追加:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に 1 行追加して、この新規 smoke を含める。
- `docs/development/current/main/phases/phase-29ae/README.md` にも追記。

## ドキュメント更新

- `docs/development/current/main/phases/phase-29ao/README.md`: P21 完了追記、Next を P22 に更新
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`: Next を P21 にし、完了後は P22 へ進める

## 検証コマンド（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p21): tighten loop simple while subset to step-only body"`
