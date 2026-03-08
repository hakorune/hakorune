---
Status: Ready
Scope: Stage-2（release既定）を SplitScan（historical label 7）の “planner subset” へ拡張する（仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29ao P38: Stage-2 — release adopt SplitScan planner subset（historical label 7）

## 目的

- P37 で ScanWithInit（historical label 6）の planner subset を release 既定で Facts→CorePlan に寄せた。
- P38 では同じ安全方針で、SplitScan（historical label 7）のうち **planner subset（Facts が取れているケース）** を release 既定で “facts→compose→CorePlan” に寄せる。
- strict/dev の shadow adopt は引き続き “タグ必須” で回帰固定する（観測/Fail-Fast 維持）。

## 非目的

- SplitScan の near-miss / contract 系を release adopt する（subset外、従来経路維持）
- 新しい env var 追加
- release でタグ出力（恒常ログ増加）
- エラー文字列変更

## 実装方針（P37と同型）

### Step 1: composer に SplitScan release adopt 入口を追加

対象:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`
- `src/mir/builder/control_flow/plan/composer/mod.rs`（re-export）

追加する関数（例）:
- `try_release_adopt_core_plan_for_split_scan(...) -> Result<Option<CorePlan>, String>`

採用条件（安全ゲート）:
- `domain_plan` が `DomainPlan::SplitScan(_)` のときだけ
- `outcome.plan` が `Some(DomainPlan::SplitScan(_))` のときだけ（planner 由来のみ採用）
- `outcome.facts` が存在し、`facts.facts.split_scan` が `Some` のときだけ

合成:
- 既存の `compose_coreplan_for_split_scan(builder, facts, ctx)` を再利用

失敗時（release既定）:
- `Ok(None)` または `Err(_)` はすべて `Ok(None)` に丸めて、従来経路へフォールバック（仕様不変）

### Step 2: router で SplitScan release adopt を接続

対象:
- `src/mir/builder/control_flow/joinir/route_entry/router.rs`

位置:
- `try_shadow_adopt_core_plan(...)` が `None` のあと、`lower_via_plan(...)` の前
- `!strict_or_dev` ブロック内（P37 の ScanWithInit と同じ場所）

処理:
- `PlanVerifier::verify(&core_plan)?; PlanLowerer::lower(...)` を通す
- タグ出力はしない（strict/dev のみ tag）

### Step 3: 非strict 経路を回帰で踏む smoke を追加（P37と同型）

狙い:
- strict/dev スモークは tag gate に寄っており、release adopt 経路が踏まれない可能性がある。
- “strictを付けない” integration smoke を 1 本追加して、非strict 実行でも安定することを固定する。

追加（例）:
- `tools/smokes/v2/profiles/integration/joinir/split_scan_release_adopt_vm.sh`

要件:
- `HAKO_JOINIR_STRICT=1` を設定しない
- fixture は既存 OK minimal を使用:
  - `apps/tests/split_scan_ok_min.hako`
- 期待:
  - 既存と同じ RC（既存 smoke の期待値に合わせる）

配線:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に filter を 1 行追加
- `docs/development/current/main/phases/phase-29ae/README.md` の pack 項目を追記

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P38完了/Next）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p38): release adopt split-scan subset"`
