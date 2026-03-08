---
Status: Ready
Scope: tests+docs（仕様不変）
Related:
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/apps/archive/README.md
  - docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md
  - docs/development/current/main/phases/phase-29ao/README.md
---

# Phase 29ao P34: LoopBreak の誤マッチ防止（freeze / notapplicable で shadow adopt タグが出ないことを回帰で固定）

Date: 2025-12-30  
Status: Ready for execution  
Goal: Phase 29ao P33 で LoopBreak（historical label 2）の match 範囲を広げたので、**本来 LoopBreak として planner 由来になってはいけないケース**でも
誤って shadow adopt してしまう退行を防ぐ。

対象は LoopBreak route の negative archived replay lane:
- NotApplicable case（historical replay token details は retirement SSOT 参照）
- Freeze case（historical replay token details は retirement SSOT 参照）

これらで strict/dev 実行時に `[coreplan/shadow_adopt:pattern2_break_subset]` が出たら FAIL とする。

## 非目的

- 実装変更（facts/normalizer/planner には触れない）
- release の挙動変更

## 変更内容

### 1) NotApplicable smoke に “shadow adopt タグ禁止” を追加

対象:
- archived replay wrapper（NotApplicable case; exact basename は retirement SSOT を参照）

やること:
- raw `OUTPUT` に `[coreplan/shadow_adopt:pattern2_break_subset]` が含まれていたら FAIL。
- 既存の output=2 期待は維持。

### 2) Freeze smoke に “shadow adopt タグ禁止” を追加

対象:
- archived replay wrapper（Freeze case; exact basename は retirement SSOT を参照）

やること:
- raw `OUTPUT` に `[coreplan/shadow_adopt:pattern2_break_subset]` が含まれていたら FAIL。
- 既存の freeze tag 検証（`[joinir/freeze]`）は維持。

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p34): prevent pattern2 shadow-adopt on negative cases"`
