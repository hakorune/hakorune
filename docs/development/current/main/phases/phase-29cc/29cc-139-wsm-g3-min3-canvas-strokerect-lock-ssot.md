---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min3（canvas 1語彙）として `env.canvas.strokeRect` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-138-wsm-g3-min2-canvas-clear-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_strokerect_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-139 WSM-G3-min3 Canvas StrokeRect Lock

## Purpose
G3 の 1語彙追加を継続し、`strokeRect` を contract/import/binding/gate まで通して canvas API の外形を段階的に拡張する。

## Decision
1. `env.canvas.strokeRect -> canvas_strokeRect` を extern contract に追加した。
2. runtime import 定義へ `canvas_strokeRect(canvas_id, x, y, w, h, color)` を追加した。
3. browser JS import binding へ `canvas_strokeRect` 実装を追加し、`ctx.strokeStyle` + `ctx.strokeRect` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_strokerect_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3` に strokeRect contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_strokerect_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3`

## Next
- `WSM-G3-min4`: `env.canvas.beginPath` を同じ方式で 1語彙追加する。
