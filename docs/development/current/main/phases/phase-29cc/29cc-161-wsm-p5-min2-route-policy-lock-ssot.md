---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P5-min2（.hako-only roadmap P5）として route policy の判断源を最小実装し、cutover routing gate を lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-160-wsm-p5-min1-default-cutover-doc-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/config/env/runner_flags.rs
  - src/runner/modes/wasm.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min2_route_policy_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-161 WSM-P5-min2 Route Policy Lock

## Purpose
`WSM-P5-min1` で固定した default cutover 境界に対し、route policy の判断源を 1 箇所へ集約する。  
`default` route policy を ENV で明示し、無効値は fail-fast で停止する。  
（注: legacy 値の受理は `WSM-P5-min10` で hard-remove 済み）

## Decision
1. `NYASH_WASM_ROUTE_POLICY` を route policy の SSOT とする。
2. 受理値は `default` のみ。その他は `[freeze:contract][wasm/route-policy]` で fail-fast（`allowed: default`）。
3. default route policy は hako-lane cutover の判断源として維持する。
4. cutover の本体（default を `.hako` emitter/binary writer 実装へ切替）は次タスクで行う。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min2_route_policy_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min3` は `29cc-162` で完了（default=hako-lane bridge lock）。
- `WSM-P5-min10` は `29cc-169` で完了（legacy policy hard-remove lock）。
- `WSM-P5-min4` は `29cc-163` で完了（native/bridge plan 境界 lock）。
- `WSM-P5-min5` は `29cc-164` で完了（native helper 1-shape lock）。
- 次は `WSM-P5-min6`: pilot 以外の 1 shape を `.hako` 実体路へ拡張し、fallback 範囲をさらに縮退する。
