---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min10（canvas 1語彙）として `env.canvas.setLineWidth` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-145-wsm-g3-min9-canvas-setstrokestyle-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setlinewidth_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-146 WSM-G3-min10 Canvas SetLineWidth Lock

## Purpose
G3 の 1語彙追加を継続し、`setLineWidth` を contract/import/binding/gate まで通して canvas style 設定境界を固定する。

## Decision
1. `env.canvas.setLineWidth -> canvas_setLineWidth` を extern contract に追加した。
2. runtime import 定義へ `canvas_setLineWidth(canvas_id, width)` を追加した。
3. browser JS import binding へ `canvas_setLineWidth` 実装を追加し、`ctx.lineWidth = width` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_setlinewidth_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3-full` に setLineWidth contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_setlinewidth_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
- `WSM-G3-min11`: gap inventory を更新して次の 1語彙（`fillCircle` or `drawLine`）候補を固定する。
