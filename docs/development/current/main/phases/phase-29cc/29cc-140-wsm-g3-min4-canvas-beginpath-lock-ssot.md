---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min4（canvas 1語彙）として `env.canvas.beginPath` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-139-wsm-g3-min3-canvas-strokerect-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_beginpath_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-140 WSM-G3-min4 Canvas BeginPath Lock

## Purpose
G3 の 1語彙追加を継続し、`beginPath` を contract/import/binding/gate まで通して canvas path 系の入口を作る。

## Decision
1. `env.canvas.beginPath -> canvas_beginPath` を extern contract に追加した。
2. runtime import 定義へ `canvas_beginPath(canvas_id)` を追加した。
3. browser JS import binding へ `canvas_beginPath` 実装を追加し、`ctx.beginPath()` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_beginpath_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3` に beginPath contract step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_beginpath_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3`

## Next
- `WSM-G3-min5`: `env.canvas.arc` を同じ方式で 1語彙追加する。
