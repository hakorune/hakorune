---
Status: Ready
Scope: Phase 29cb P0 (docs-first)
---

# Phase 29cb P0: in-body step normalization (generic loop v0.2)

## 目的

generic loop v0.1 では受理できない “途中 step” を、
normalizer の正規化で **末尾 step** に寄せて受理できるようにする。

## 基本方針（SSOT）

- continue は増やさない（v0.2 では continue 無しを前提）
- loop body の途中に `step` が 1 回だけ出るケースのみ対象
- if/else の join は持ち込まない（IfEffect は then-only, leaf-only のまま）
- 既定挙動/恒常ログは不変
- strict/dev は FlowBox タグで検証できる形を維持

## 正規化境界（Ok/None/Freeze）

### Ok (受理)

- loop body が “leaf effects + ExitIf + IfEffect(then-only)” のみ
- loop step が **途中に 1 回** だけ現れる
- continue/break/return は ExitIf で表現されている（暗黙ジャンプ禁止）

### Ok(None) / Freeze

- in-body step が複数回
- continue を含む（v0.2 では対象外）
- nested loop / multi-entry
- IfEffect の else / join / nested if

## 実装タスク（P1 でやることの前提）

- facts: in-body step を “許容形” として見える化
- normalizer: in-body step を **末尾 step** に正規化（1 回限定）
- verifier/lowerer: v0.1 の語彙は維持、追加の語彙は入れない

## Gate / 検証

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`（current regression pack entry; exact legacy stems are tracked in the retirement SSOT）
- `./tools/smokes/v2/run.sh --profile quick`

## 付記（禁止事項）

- CorePlan を汎用 CFG にしない
- by-name ハードコードで回避しない
