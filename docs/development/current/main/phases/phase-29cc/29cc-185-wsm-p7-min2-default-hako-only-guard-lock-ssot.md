---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P7-min2 として default route の hako-only guard を lock し、portability gate に接続する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-184-wsm-p7-min1-hako-only-done-criteria-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md
  - tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-185 WSM-P7-min2 Default Hako-Only Guard Lock

## Purpose
default route が `.hako`-only 契約から逸脱していないことを、軽量 guard で継続監査できるように固定する。
この lock は `default-only` 契約の継続監査を主目的とする。

## Decision
1. default route の guard は次を監査する。
   - `NYASH_WASM_ROUTE_POLICY` は `default` のみ受理（`allowed: default`）。
   - `wasm_hako_default_lane_trace_` 契約テストが緑。
   - docs/guard/smoke の導線が 1 入口で辿れる。
2. guard は `tools/checks/dev_gate.sh portability` に接続する。
3. guard は mutating な経路変更を行わず、契約監査のみを行う。

## Implemented
1. guard 追加:
   - `tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh`
2. smoke 追加:
   - `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh`
3. gate 接続:
   - `tools/checks/dev_gate.sh portability`

## Acceptance
1. `cargo check --features wasm-backend --bin hakorune`
2. `bash tools/checks/phase29cc_wsm_p7_default_hako_only_guard.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh`
4. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P7-min3` two-demo lock（`projects/nyash-wasm` 由来 2 ケース）へ進む。
