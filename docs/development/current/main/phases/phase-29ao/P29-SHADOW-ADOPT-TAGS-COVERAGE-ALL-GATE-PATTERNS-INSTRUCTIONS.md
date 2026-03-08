---
Status: Ready
Scope: code+tests+docs（strict/dev のみ、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - tools/smokes/v2/lib/test_runner.sh
---

# Phase 29ao P29: Shadow adopt coverage（全 gate route family にタグ + smoke で必須化）

Date: 2025-12-30  
Status: Ready for execution  
Goal: JoinIR regression gate（phase29ae pack）に含まれる全 route family について、strict/dev の shadow adopt（Facts→CorePlan）が **実際に踏まれている**ことを、安定タグと smoke の期待として固定する。

## 背景

- P28 で scan route family の shadow adopt にタグを付け、専用 smoke で必須化した。
- ただし regression pack は LoopSimpleWhile / LoopBreak / IfPhiJoin / LoopTrueEarlyExit も含むため、「pack が緑でも shadow adopt が踏めていない」抜け道が残りうる。
- P29 は “観測のみ” で、挙動は変えずに「踏んだ」を SSOT 化する。

## 非目的

- release 既定のログ/出力を増やす
- 新しい env var を追加する
- shadow adopt の適用範囲を広げる（P29 はタグとテスト固定だけ）

## 実装方針

### 1) router に stable tag を追加（strict/dev の adopt 成功時のみ）

対象:
- `src/mir/builder/control_flow/joinir/route_entry/router.rs`

追加タグ（SSOT・固定 / historical tag suffix維持）:
- LoopSimpleWhile adopt: `[coreplan/shadow_adopt:pattern1_simplewhile]`（historical tag suffix維持）
- LoopBreak subset adopt: `[coreplan/shadow_adopt:pattern2_break_subset]`（historical tag suffix維持）
- IfPhiJoin adopt: `[coreplan/shadow_adopt:pattern3_ifphi]`（historical tag suffix維持）
- LoopTrueEarlyExit adopt: `[coreplan/shadow_adopt:pattern5_infinite_early_exit]`（historical tag suffix維持）
- （既存）scan route family は P28 のタグを維持

注意:
- `strict_or_dev == true` かつ “Facts→CorePlan を採用して return する直前” に `eprintln!` する
- fallback 経路（DomainPlan→Normalizer）では出さない

### 2) 既存 smoke の期待に「タグ必須」を追加（回帰 pack の行程を増やさない）

P28 で `tools/smokes/v2/lib/test_runner.sh` がタグをノイズとして除去するようになっているため、
**タグ検証は `filter_noise` 前の生出力で行う**こと。

対象（いずれも regression pack に既に含まれる）:

- LoopSimpleWhile strict shadow:
  - `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh`
  - 生出力に `[coreplan/shadow_adopt:pattern1_simplewhile]` が含まれることを必須化

- LoopBreak subset:
  - `tools/smokes/v2/profiles/integration/joinir/loop_break_plan_subset_vm.sh`
  - 生出力に `[coreplan/shadow_adopt:pattern2_break_subset]` が含まれることを必須化
  - 既存の promotion_hint など他の tag 期待は維持

- IfPhiJoin:
  - `tools/smokes/v2/profiles/integration/joinir/if_phi_join_vm.sh`
  - 生出力に `[coreplan/shadow_adopt:pattern3_ifphi]` が含まれることを必須化

- LoopTrueEarlyExit strict shadow:
  - `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh`
  - 生出力に `[coreplan/shadow_adopt:pattern5_infinite_early_exit]` が含まれることを必須化

- scan route strict shadow（P28で追加済み）:
  - `tools/smokes/v2/profiles/integration/joinir/scan_with_init_strict_shadow_vm.sh`
  - `tools/smokes/v2/profiles/integration/joinir/split_scan_strict_shadow_vm.sh`
  - ここはタグ検証が既にある前提。無ければ同様に “生出力で検証” に統一する。

### 3) docs（運用SSOT）の追記

- `docs/development/current/main/phases/phase-29ae/README.md`
  - 「shadow adopt tag は `filter_noise` で落ちる」こと
  - 「tag を検証する current semantic wrapper は生出力を参照する」こと
  を 1〜2 行で明文化（迷子防止）。

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新（追跡）

- `docs/development/current/main/phases/phase-29ao/README.md`（P29 追加、Next 更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`（Current/Next 更新）

## コミット

- `git add -A`
- `git commit -m "phase29ao(p29): require shadow adopt tags for all gate routes"`
