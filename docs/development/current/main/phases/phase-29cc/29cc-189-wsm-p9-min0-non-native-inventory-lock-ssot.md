---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P9-min0 として non-native inventory を docs-first で固定し、BridgeRustBackend fallback 境界を監査可能にする。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min0_non_native_inventory_lock_vm.sh
  - tools/checks/phase29cc_wsm_p9_non_native_inventory_guard.sh
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
---

# 29cc-189 WSM-P9-min0 Non-Native Inventory Lock

## Purpose
`default-only` 契約は維持したまま、現時点で native shape table に入らず `BridgeRustBackend` へ落ちる
代表ケースを inventory として固定する。

## Inventory (fixed)
1. `apps/tests/phase29cc_wsm02d_demo_min.hako`
2. `projects/nyash-wasm` の console/math/webcanvas/canvas_advanced 系デモ
3. route-trace 上の `plan=bridge-rust-backend` を出す non-native fixture 群

## Decision
1. bridge fallback は保持する（P8 継続）。
2. non-native inventory は `min1` 以降の shape 追加で段階的に縮退する。
3. inventory lock は portability ガードで常時監査する。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min0_non_native_inventory_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p9_non_native_inventory_guard.sh`
3. `tools/checks/dev_gate.sh portability`
