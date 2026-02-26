---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min7（canvas 1語彙）として `env.canvas.stroke` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-142-wsm-g3-min6-canvas-fill-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_stroke_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-143 WSM-G3-min7 Canvas Stroke Lock

## Purpose
G3 の 1語彙追加を継続し、`stroke` を contract/import/binding/gate まで通して path 描画の境界を固定する。

## Decision
1. `env.canvas.stroke -> canvas_stroke` を extern contract に追加した。
2. runtime import 定義へ `canvas_stroke(canvas_id)` を追加した。
3. browser JS import binding へ `canvas_stroke` 実装を追加し、`ctx.stroke()` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_stroke_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3` に stroke contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_stroke_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3`

## Next
- `WSM-G3-min8`: `env.canvas.setFillStyle` を同じ方式で 1語彙追加する。
