---
Status: Done
Decision: accepted-but-blocked
Date: 2026-02-27
Scope: WSM-P8-min1 として compat bridge retire execution 判定を lock し、default-only 主経路を崩さず段階撤去条件を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md
  - tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p8/phase29cc_wsm_p8_min1_bridge_retire_readiness_vm.sh
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
---

# 29cc-188 WSM-P8-min1 Bridge Retire Readiness Lock

## Purpose
`default-only` を維持したまま、non-native shape の compat bridge (`BridgeRustBackend` / `bridge-rust-backend`) を
いつ retire execution できるかの判定境界を docs-first で固定する。

## Decision
1. route policy は引き続き `default-only`（`NYASH_WASM_ROUTE_POLICY` は `default` のみ許可）。
2. non-native shape での `BridgeRustBackend` は現時点で必要な互換責務として保持する。
3. retire execution は accepted-but-blocked とし、bridge coverage が native shape table に吸収されるまでは削除しない。
4. lock は portability guard で常時監査し、誤って bridge 境界を崩した変更を fail-fast で止める。

## Implemented
1. lock smoke 追加:
   - `tools/smokes/v2/profiles/integration/phase29cc_wsm/p8/phase29cc_wsm_p8_min1_bridge_retire_readiness_vm.sh`
2. guard 追加:
   - `tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh`
3. smoke は次を固定:
   - `phase29cc_wsm_p7_min4_compat_retention_lock_vm.sh`
   - `wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract`
   - `wasm_hako_default_lane_trace_has_none_shape_id_for_bridge_contract`
   - `wasm_demo_default_hako_lane_bridge_non_pilot_contract`

## Acceptance
1. `bash tools/checks/phase29cc_wsm_p8_bridge_retire_readiness_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p8/phase29cc_wsm_p8_min1_bridge_retire_readiness_vm.sh`
3. `tools/checks/dev_gate.sh portability`
4. `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
1. wasm lane active next は `none`（monitor-only）。
2. 実際の bridge retire execution は blocker 再発時のみ failure-driven で再起動する。
