---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min2（canvas 1語彙）として `env.canvas.clear` を実装・固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-137-wsm-g3-min1-gap-inventory-lock-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/runtime.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_clear_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-138 WSM-G3-min2 Canvas Clear Lock

## Purpose
G3 の「1 blocker = 1 shape」方針に従い、Canvas語彙を 1件だけ追加して extern contract から runtime binding までを固定する。

## Decision
1. `env.canvas.clear -> canvas_clear` を extern contract に追加した。
2. runtime import 定義へ `canvas_clear(canvas_id_ptr, canvas_id_len)` を追加した。
3. browser JS import binding へ `canvas_clear` 実装を追加し、`ctx.clearRect(0, 0, canvas.width, canvas.height)` を実行する。
4. gate:
   - `phase29cc_wsm_g3_canvas_clear_contract_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-demo-g3` を追加（`wasm-demo-g2` + `canvas.clear` contract）

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g3_canvas_clear_contract_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g3`

## Next
- `WSM-G3-min3`: `env.canvas.strokeRect` を同じ方式で 1語彙追加する。
