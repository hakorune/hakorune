---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min6（canvas 1語彙）として `env.canvas.fill` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-141-wsm-g3-min5-canvas-arc-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_fill_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-142 WSM-G3-min6 Canvas Fill Lock

## Purpose
G3 の 1語彙追加を継続し、`fill` を contract/import/binding/gate まで通して path 描画の境界を固定する。

## Decision
1. `env.canvas.fill -> canvas_fill` を extern contract に追加した。
2. runtime import 定義へ `canvas_fill(canvas_id)` を追加した。
3. browser JS import binding へ `canvas_fill` 実装を追加し、`ctx.fill()` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_fill_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3` に fill contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_fill_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3`

## Next
- `WSM-G3-min7`: `env.canvas.stroke` を同じ方式で 1語彙追加する。
