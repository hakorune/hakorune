---
Status: Ready
Scope: code+tests+docs（strict/dev のみ、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - src/mir/builder/control_flow/joinir/patterns/router.rs
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29ao P28: Shadow adopt observability（strict/dev tags + gate smokes）

Date: 2025-12-30  
Status: Ready for execution  
Goal: strict/dev の shadow adopt（Facts→CorePlan）経路が **本当に踏まれている**ことを、安定タグと回帰スモークで SSOT 化する。

## 背景

- P17/P23/P24/P25/P26/P27 で複数 pattern に strict/dev shadow adopt を入れた。
- しかし「strict/dev で実行して PASS」だけだと、
  - planner が `Ok(None)` に倒れて fallback が選ばれた
  - ルール順序の都合で別経路になった
  などで “shadow adopt が実際には踏まれていない” 可能性を検知しづらい。

P28 は **挙動を変えずに**「踏んだか」を機械的に検証可能にする。

## 非目的

- release 既定のログ/出力を増やす
- 新しい env var を追加する
- shadow adopt の適用範囲を広げる（P28 は観測のみ）

## 実装方針

### 1) 安定タグを strict/dev のみ出力（SSOT）

対象:
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

方針:
- `strict_or_dev` が true のときに限り、shadow adopt を **実際に採用した**タイミングで `eprintln!` する
- タグ文字列は固定・短い・検索しやすい形にする（SSOT）

推奨タグ:
- Pattern6: `[coreplan/shadow_adopt:pattern6_scan_with_init]`
- Pattern7: `[coreplan/shadow_adopt:pattern7_split_scan]`

（必要なら Pattern1/3/5/2subset も後で追加できるが、P28 では “P24/P27 を確実に踏む” 目的で 6/7 を優先）

注意:
- strict/dev only（release 既定では出ない）
- 既存の promotion hint タグと同じく “診断用” として扱い、恒常ログ増加は起こさない

### 2) adopt 専用 smoke を追加し、回帰パックに組み込む

既存の `phase29ab_pattern6_` / `phase29ab_pattern7_` は複数 fixture を含み、contract/freezes も混ざるため、
“tag が出る fixture” を 1 本に固定する smoke を新設する。

追加ファイル:
- `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern6_strict_shadow_vm.sh`
  - fixture: `apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako`
  - strict: `HAKO_JOINIR_STRICT=1`
  - 期待: exit=1（既存 smoke と同じ）かつ tag を含む
- `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern7_strict_shadow_vm.sh`
  - fixture: `apps/tests/phase29ab_pattern7_splitscan_ok_min.hako`
  - strict: `HAKO_JOINIR_STRICT=1`
  - 期待: exit=3（既存 smoke と同じ）かつ tag を含む

回帰パックに追加:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - `run_filter "pattern6_strict_shadow_vm" "phase29ao_pattern6_strict_shadow_vm"`
  - `run_filter "pattern7_strict_shadow_vm" "phase29ao_pattern7_strict_shadow_vm"`

docs 更新:
- `docs/development/current/main/phases/phase-29ae/README.md` に pack 項目として追記

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P28 追加、Next 更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`（Current/Next 更新）

## コミット

- `git add -A`
- `git commit -m "phase29ao(p28): add shadow adopt tags + gate smokes for p6/p7"`
