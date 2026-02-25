---
Status: Ready
Scope: Stage-2（release既定）を Pattern2(Break) の “planner subset” へ拡張する（仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29ao P39: Stage-2 — release adopt Pattern2(Break) subset (planner-derived only)

## 目的

- P36/P37/P38 で Pattern1/6/7 の “planner subset” を release 既定で composer 経由に寄せられた。
- P39 は同じ安全方針で、Pattern2(Break) のうち **planner subset（outcome.plan が Pattern2Break のケース）** を release 既定で `facts → composer → CorePlan` に寄せる。
- strict/dev の shadow adopt は引き続き「タグ必須」で回帰固定する（観測/Fail-Fast 維持）。

## 非目的

- Pattern2 の NotApplicable / Freeze 系を release adopt する（subset外、従来経路維持）
- 新しい env var 追加
- release でタグ出力（恒常ログ増加）
- エラー文字列の変更

## 実装（P37/P38と同型）

### Step 1: composer に Pattern2 release adopt 入口を追加

対象:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`
- `src/mir/builder/control_flow/plan/composer/mod.rs`（re-export）

追加する関数（例）:
- `try_release_adopt_core_plan_for_pattern2_break_subset(...) -> Result<Option<CorePlan>, String>`

採用条件（安全ゲート）:
- `domain_plan` が `DomainPlan::Pattern2Break(_)` のときだけ
- `outcome.plan` が `Some(DomainPlan::Pattern2Break(_))` のときだけ（planner 由来のみ採用）
- `outcome.facts` が存在し、`facts.facts.pattern2_break` が `Some` のときだけ

合成:
- 既存の `compose_coreplan_for_pattern2_break_subset(builder, facts, ctx)` を再利用

失敗時（release既定）:
- `Ok(None)` または `Err(_)` はすべて `Ok(None)` に丸めて、従来経路へフォールバック（仕様不変）

### Step 2: router の非strict経路で Pattern2 release adopt を接続

対象:
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

位置:
- `if !strict_or_dev { ... }` ブロック内（Pattern1/6/7 の直後）
- `lower_via_plan(...)` の前

処理:
- `PlanVerifier::verify(&core_plan)?; PlanLowerer::lower(...)`
- タグ出力はしない（strict/dev のみ tag）

### Step 3: 非strict smoke を追加して gate に入れる

狙い:
- 既存の Pattern2 subset smoke は strict/dev タグ検証に寄っており、release adopt 経路が踏まれない可能性がある。
- “strictを付けない” integration smoke を 1 本追加して、非strict 実行でも安定することを固定する。

追加（例）:
- `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern2_release_adopt_vm.sh`

要件:
- `HAKO_JOINIR_STRICT=1` を設定しない
- fixture は既存 subset fixture を使用:
  - `apps/tests/phase29ai_pattern2_break_plan_subset_ok_min.hako`
- 期待:
  - exit code = 15（既存の subset smoke と同じ）

配線:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に filter 1 行追加
- `docs/development/current/main/phases/phase-29ae/README.md` の pack 項目追記

### Step 4: docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P39完了/Next）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p39): release adopt pattern2 break subset"`
