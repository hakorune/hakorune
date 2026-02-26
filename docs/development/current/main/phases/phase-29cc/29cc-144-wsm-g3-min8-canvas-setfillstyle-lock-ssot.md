---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min8（canvas 1語彙）として `env.canvas.setFillStyle` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-143-wsm-g3-min7-canvas-stroke-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setfillstyle_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-144 WSM-G3-min8 Canvas SetFillStyle Lock

## Purpose
G3 の 1語彙追加を継続し、`setFillStyle` を contract/import/binding/gate まで通して canvas style 設定境界を固定する。

## Decision
1. `env.canvas.setFillStyle -> canvas_setFillStyle` を extern contract に追加した。
2. runtime import 定義へ `canvas_setFillStyle(canvas_id, color)` を追加した。
3. browser JS import binding へ `canvas_setFillStyle` 実装を追加し、`ctx.fillStyle = color` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_setfillstyle_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3-full` に setFillStyle contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setfillstyle_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
- `WSM-G3-min9`: `env.canvas.setStrokeStyle` を同じ方式で 1語彙追加する。
