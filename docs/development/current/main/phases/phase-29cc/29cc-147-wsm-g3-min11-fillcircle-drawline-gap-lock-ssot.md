---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min11（docs-first）として `fillCircle -> drawLine` の固定順を定義する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-146-wsm-g3-min10-canvas-setlinewidth-lock-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md
---

# 29cc-147 WSM-G3-min11 FillCircle/DrawLine Gap Lock

## Purpose
WSM-G3 の次段 2語彙を迷わず進めるため、`fillCircle -> drawLine` の実装順と語彙契約を docs-first で固定する。

## Decision
1. WSM-G3-min12 は `env.canvas.fillCircle` を対象にする。
2. WSM-G3-min13 は `env.canvas.drawLine` を対象にする。
3. 両語彙とも extern 先行（contract/runtime/smoke/docs）で固定し、BoxCall 拡張は本タスク外とする。
4. gate は既存方針を維持し、`tools/checks/dev_gate.sh wasm-demo-g3-full` に 1step ずつ追加する。

## Acceptance
- `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` が `min12(fillCircle)` を active next として同期される。
- `unsupported_features.md` が `fillCircle -> drawLine` の順序で更新される。

## Next
- `WSM-G3-min12`: `env.canvas.fillCircle` を 1語彙実装して fixture/gate lock。
