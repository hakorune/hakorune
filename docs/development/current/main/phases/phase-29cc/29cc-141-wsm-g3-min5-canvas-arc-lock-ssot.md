---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min5（canvas 1語彙）として `env.canvas.arc` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-140-wsm-g3-min4-canvas-beginpath-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_arc_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-141 WSM-G3-min5 Canvas Arc Lock

## Purpose
G3 の 1語彙追加を継続し、`arc` を contract/import/binding/gate まで通して canvas path 構文の中核を固定する。

## Decision
1. `env.canvas.arc -> canvas_arc` を extern contract に追加した。
2. runtime import 定義へ `canvas_arc(canvas_id, x, y, radius, start_angle, end_angle)` を追加した。
3. browser JS import binding へ `canvas_arc` 実装を追加し、`ctx.arc(...)` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_arc_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3` に arc contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_arc_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3`

## Next
- `WSM-G3-min6`: `env.canvas.fill` を同じ方式で 1語彙追加する。
