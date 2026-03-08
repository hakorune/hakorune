---
Status: Ready
Scope: Stage-2（release既定）を ScanWithInit（historical label 6）の “planner subset” へ拡張する（仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29ao P37: Stage-2 — release adopt ScanWithInit planner subset（historical label 6）

## 目的

- P36 で LoopSimpleWhile（historical label 1）を release 既定で Facts→CorePlan へ寄せる「Stage-2 pilot」を開始した。
- P37 は次の安全ステップとして、ScanWithInit（historical label 6）のうち **planner subset（Facts が取れているケース）** を release 既定で “facts→compose→CorePlan” に寄せる。
- strict/dev の shadow adopt は引き続き「タグ必須」で回帰固定する（観測/Fail-Fast 維持）。

## 非目的

- ScanWithInit の reverse/matchscan variant を release adopt する（subset外なので従来経路維持）
- 新しい env var の追加
- release でタグを出す（恒常ログ増加になるので禁止）
- エラー文字列の変更（strict/dev の Fail-Fast 文言も変えない）

## 現状（前提）

- ScanWithInit strict/dev shadow adopt は回帰で固定済み:
  - `tools/smokes/v2/profiles/integration/joinir/scan_with_init_strict_shadow_vm.sh`
  - タグ: `[coreplan/shadow_adopt:pattern6_scan_with_init]`
- router の採用順（概略）:
  - `single_planner` が返した `domain_plan` を基本経路として `lower_via_plan` で処理
  - strict/dev のときだけ composer で shadow adopt → tag 出力 → CorePlan lower
- composer 側には P36 で LoopSimpleWhile の release adopt 入口が追加済み:
  - `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

## 実装内容

### Step 1: composer に ScanWithInit release adopt 入口を追加

- 対象ファイル:
  - `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

追加する関数（例）:
- `try_release_adopt_core_plan_for_scan_with_init(...) -> Result<Option<CorePlan>, String>`

条件（安全ゲート）:
- `domain_plan` が `DomainPlan::ScanWithInit(_)` のときだけ
- `outcome.plan` が `Some(DomainPlan::ScanWithInit(_))` のときだけ（planner 由来のみ採用）
- `outcome.facts` が存在し、`facts.facts.scan_with_init` が `Some` のときだけ

生成:
- 既存の `compose_coreplan_for_scan_with_init(builder, facts, ctx)` を再利用

失敗時の扱い（release既定の安全策）:
- `Ok(None)` または `Err(_)` は **すべて `Ok(None)`** に丸めて、従来経路へフォールバック（既定挙動不変）

### Step 2: router で ScanWithInit release adopt を接続

- 対象ファイル:
  - `src/mir/builder/control_flow/joinir/route_entry/router.rs`

位置:
- `try_shadow_adopt_core_plan(...)` が `None` のあと、`lower_via_plan(...)` の前
- かつ `strict_or_dev == false` のときのみ

形（例）:
- `if !strict_or_dev { if let Some(core_plan) = composer::try_release_adopt_core_plan_for_scan_with_init(...) { verify+lower } }`

注意:
- release 側ではタグ出力しない（`eprintln!` は strict/dev adopt のみに限定）
- `PlanVerifier::verify(&core_plan)` は通す（Fail-Fast は core 側で担保、release でも verify は構造チェックなのでOK）

### Step 3: Stage-2 の非strict 経路を回帰で踏む smoke を追加

狙い:
- 既存 gate は strict/dev 実行が多く、release adopt 経路が踏まれない可能性がある。
- P37 は “strictを付けない” integration smoke を 1 本追加して、非strict 実行でも安定することを固定する。

追加（例）:
- `tools/smokes/v2/profiles/integration/joinir/scan_with_init_release_adopt_vm.sh`

要件:
- `HAKO_JOINIR_STRICT=1` を設定しない（非strict）
- fixture は既存の OK minimal を使う:
  - `apps/tests/scan_with_init_ok_min.hako`
- 期待:
  - exit code = 1（既存と同じ）

配線:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に filter を 1 行追加
- `docs/development/current/main/phases/phase-29ae/README.md` に pack の項目を追記

### Step 4: docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md` に P37 の項目追加（完了時）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` の Next 更新
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` の Next 更新

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p37): release adopt scan-with-init subset"`
