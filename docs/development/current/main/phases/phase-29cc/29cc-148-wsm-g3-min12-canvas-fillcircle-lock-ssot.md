---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min12（canvas 1語彙）として `env.canvas.fillCircle` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-147-wsm-g3-min11-fillcircle-drawline-gap-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_fillcircle_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-148 WSM-G3-min12 Canvas FillCircle Lock

## Purpose
G3 の 1語彙追加として `fillCircle` を contract/import/binding/gate まで通し、`fillStyle + beginPath + arc + fill` の合成呼び出し境界を固定する。

## Decision
1. `env.canvas.fillCircle -> canvas_fillCircle` を extern contract に追加した。
2. runtime import 定義へ `canvas_fillCircle(canvas_id, x, y, radius, color)` を追加した。
3. browser JS import binding へ `canvas_fillCircle` 実装を追加し、`ctx.fillStyle = color; ctx.beginPath(); ctx.arc(...); ctx.fill();` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_fillcircle_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3-full` に fillCircle contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_fillcircle_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
- `WSM-G3-min13`: `env.canvas.drawLine` を 1語彙実装して fixture/gate lock。
