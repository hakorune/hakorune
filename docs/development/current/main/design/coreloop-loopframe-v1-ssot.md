---
Status: SSOT
Scope: CorePlan LoopFrame v1 (Loop structural box with Break/Continue depth)
Related:
- docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/phases/archive/phase-29bs/README.md
---

# CoreLoop LoopFrame v1 (SSOT)

## Goal

- Loop を構造箱として扱い、`Loop.body` に `CorePlan` の木（Seq/If/Loop/Effects/Exit）を持てるようにする。
- `Break/Continue(depth)` を LoopFrame stack で解決できるようにする（最内=1）。

## Non-goals

- 任意ラベル / goto の導入
- raw rewrite（AST/CFG の実行コード書き換え）
- 既定挙動変更 / silent fallback

## Core contract

- LoopFrame は `break_bb` / `continue_bb` を提供（ID は debug 表示用のみ）。
- ExitKind は `Break(depth)` / `Continue(depth)` を持つ（`depth=1` が最内）。
- label は AST では許容しても、freeze/verify で depth に解決する（CorePlan に by-name を持ち込まない）。

### ContinueTarget slot (planned, foundation)

`continue` が飛ぶ先は “常に step_bb” とは限らない（step を body に残す/while 風/将来の最適化など）。
CorePlan の語彙を増やさずに合成を保つため、LoopFrame は設計上 `continue_target` を **設定可能なスロット**として扱う。

- `continue_bb` は「次イテレーションへ進む入口」を指す（header 直行か step 経由かは plan のモードで決まる）
- strict/dev の段階導入では、まず `InlineInBody` は continue 禁止で開始してよい（後から slot を使って解禁する）
- 実装: `CoreLoopPlan.continue_target` を追加し、既定は `step_bb` を設定（挙動不変）

## Verifier invariants (minimal)

- `depth` は `1..=loop_depth`
- loop body は CorePlan 木（Seq/If/Loop/Effects/Exit）を持てる（v1）
- cleanup/defer は ExitKind 経由（将来項目として予定。CleanupWrap + region 境界を SSOT 化してから導入する）

## Lowerer outline

- LoopFrame stack を導入して `Break/Continue(depth)` を target へ解決する。
- v0 と v1 を段階的に切り替えられるように、入口で判別できる構成にする。

## GenericLoop v1 rationale

- GenericLoopV1 は v0 の IfEffect 制約回避ではなく、loop body を CorePlan 木（If + Exit）で表現するために導入。
- strict/dev + planner_required 限定で候補化し、既定挙動は不変のまま。
- 証跡: `./tools/smokes/v2/profiles/integration/joinir/phase29bs_loopframe_v1_nested_loop_strict_gate_vm.sh`
  と `apps/tests/phase29bs_nested_loop_break_continue_depth_min.hako`。

### StepMode (planned): Extract vs Inline

“step（loop_increment）を step_bb に移す” のは意味論が変わり得るため、Facts が step placement を保持し、
plan/lowerer が次の 2 モードを選べるようにする（strict/dev + planner_required 限定で段階導入）:

- `ExtractToStepBB`: 安全に移動できる場合のみ（既存 v0/v1 の主経路）
- `InlineInBody`: step を body の元の位置で lower（no rewrite）
  - 初期の verifier 契約は「continue 禁止 + step_bb 空」で conservative に開始する

## Example

- outer loop 内で inner loop から `break 2` / `continue 2` 相当を出す。
- depth が無いと “どの loop に脱出するか” を表現できず、構造箱合成が破綻する。
