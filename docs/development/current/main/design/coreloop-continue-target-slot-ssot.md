---
Status: SSOT
Scope: CoreLoopPlan continue_target slot
Related:
- docs/development/current/main/design/coreloop-loopframe-v1-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# ContinueTarget slot (SSOT)

## Current

- 現状: 全ノーマライザは `continue_target = step_bb` を設定（挙動不変）

## Unlock conditions

- 解禁: strict/dev-only box でのみ `continue_target != step_bb` を使う
- 最初の解禁対象: `StepMode::InlineInBody` の一般化（SSOT: `docs/development/current/main/design/coreloop-stepmode-inline-in-body-ssot.md`）

## Fail-fast

- `continue_target` が Frag wiring / block_effects のどちらにも存在しない場合は contract error
