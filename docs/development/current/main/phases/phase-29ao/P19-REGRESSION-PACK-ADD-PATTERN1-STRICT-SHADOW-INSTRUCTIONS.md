---
Status: Ready
Scope: docs+smokes（回帰ゲート強化、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29ao P19: 回帰ゲートに LoopSimpleWhile strict/dev shadow adopt を含める（historical label 1）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。P17/P18 で導入した strict/dev の shadow adopt を、JoinIR 回帰 SSOT（phase29ae pack）で必ず実行される形にする。

## 目的

- strict/dev の LoopSimpleWhile shadow adopt（Facts→CorePlan(skeleton)）が、回帰ゲートで常に実行されるようにする。
- “実装は入ったが回帰で踏まれない” 状態を防ぐ（SSOTの維持）。

## 非目的

- 新しい言語仕様・最適化の追加
- regression pack を重くする（最小1本だけ追加）
- LLVM EXE 系を pack に入れる（環境/時間依存が大きい）

## 実装方針（最小）

### 追加する smoke（新規）

- 新規ファイル: `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh`
  - 入力: `apps/tests/loop_simple_while_strict_shadow_min.hako` を current semantic fixture alias として使う
  - historical fixture token: `apps/tests/phase286_pattern1_frag_poc.hako`
  - 実行: `NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1` を付けて VM 実行
  - 期待: exit code `3`（既存PoCと同じ）

理由:
- strict/dev の shadow adopt を確実に踏むため（strict=1）。
- fixture を増やさない（既存の LoopSimpleWhile PoC を流用）。

### regression pack に追加

- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に 1 行追加
  - `run_filter "loop_simple_while_strict_shadow_vm" "loop_simple_while_strict_shadow_vm" || exit 1`

### SSOT ドキュメント更新

- `docs/development/current/main/phases/phase-29ae/README.md`
  - Regression pack (SSOT) に current semantic wrapper `loop_simple_while_strict_shadow_vm` を追記

## 検証（必須）

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile quick`

## コミット

- `git add -A`
- `git commit -m "docs(phase29ao): gate loop_simple_while strict shadow adopt in regression pack"`
