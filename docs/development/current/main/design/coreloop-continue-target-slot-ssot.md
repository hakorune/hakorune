---
Status: SSOT
Scope: CoreLoopPlan continue_target slot
Related:
- docs/development/current/main/design/coreloop-loopframe-v1-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# ContinueTarget slot (SSOT)

> **注意(2026-11-17、メンテナ決定):** この文書には、[MirBuilder の設計図](mirbuilder-blueprint.md) が上書きした所がある。特に「loop の家族・route・Recipe が、source を受け付けるかを決める」という記述と、段の順(`Facts -> RoutePolicy -> Recipe -> ...`)は、もう設計ではない。食い違う時は、設計図が勝つ。

## Current

- 現状: 全ノーマライザは `continue_target = step_bb` を設定（挙動不変）

## Unlock conditions

- 解禁: strict/dev-only box でのみ `continue_target != step_bb` を使う
- 最初の解禁対象: `StepMode::InlineInBody` の一般化（SSOT: `docs/development/current/main/design/coreloop-stepmode-inline-in-body-ssot.md`）

## Fail-fast

- `continue_target` が Frag wiring / block_effects のどちらにも存在しない場合は contract error
