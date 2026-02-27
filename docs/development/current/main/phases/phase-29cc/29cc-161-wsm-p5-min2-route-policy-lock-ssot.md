---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P5-min2（.hako-only roadmap P5）として default/legacy route policy を最小実装し、cutover routing gate を lock する。
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
`default` / `legacy-wasm-rust` の切替は ENV で明示し、無効値は fail-fast で停止する。

## Decision
1. `NYASH_WASM_ROUTE_POLICY` を route policy の SSOT とする。
2. 受理値は `default|legacy|legacy-wasm-rust` のみ。その他は `[freeze:contract][wasm/route-policy]` で fail-fast。
3. 現段階の `default` と `legacy-wasm-rust` は同一 Rust backend 実装へ束ねる（挙動変更なし）。
4. cutover の本体（default を `.hako` emitter/binary writer 実装へ切替）は次タスクで行う。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min2_route_policy_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min3` は `29cc-162` で完了（default=hako-lane bridge lock）。
- 次は `WSM-P5-min4`: bridge 依存を縮退し、`.hako` 実体路の lock を 1 shape ずつ追加する。
