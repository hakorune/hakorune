---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min13（canvas 1語彙）として `env.canvas.drawLine` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-148-wsm-g3-min12-canvas-fillcircle-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_drawline_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-149 WSM-G3-min13 Canvas DrawLine Lock

## Purpose
G3 の 1語彙追加として `drawLine` を contract/import/binding/gate まで通し、`strokeStyle + lineWidth + moveTo/lineTo/stroke` の合成呼び出し境界を固定する。

## Decision
1. `env.canvas.drawLine -> canvas_drawLine` を extern contract に追加した。
2. runtime import 定義へ `canvas_drawLine(canvas_id, x1, y1, x2, y2, color, width)` を追加した。
3. browser JS import binding へ `canvas_drawLine` 実装を追加し、`ctx.strokeStyle = color; ctx.lineWidth = width; ctx.beginPath(); ctx.moveTo(...); ctx.lineTo(...); ctx.stroke();` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_drawline_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3-full` に drawLine contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_drawline_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
- `WSM-P1-min1`: `--emit-wat` CLI を追加し、WAT出力入口を固定する。
