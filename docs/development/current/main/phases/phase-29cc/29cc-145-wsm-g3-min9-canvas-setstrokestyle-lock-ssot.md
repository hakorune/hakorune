---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min9（canvas 1語彙）として `env.canvas.setStrokeStyle` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-144-wsm-g3-min8-canvas-setfillstyle-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-145 WSM-G3-min9 Canvas SetStrokeStyle Lock

## Purpose
G3 の 1語彙追加を継続し、`setStrokeStyle` を contract/import/binding/gate まで通して canvas style 設定境界を固定する。

## Decision
1. `env.canvas.setStrokeStyle -> canvas_setStrokeStyle` を extern contract に追加した。
2. runtime import 定義へ `canvas_setStrokeStyle(canvas_id, color)` を追加した。
3. browser JS import binding へ `canvas_setStrokeStyle` 実装を追加し、`ctx.strokeStyle = color` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3-full` に setStrokeStyle contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setstrokestyle_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
- `WSM-G3-min10`: `env.canvas.setLineWidth` を同じ方式で 1語彙追加する。
