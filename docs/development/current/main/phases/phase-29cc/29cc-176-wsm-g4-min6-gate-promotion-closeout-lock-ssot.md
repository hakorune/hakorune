---
Status: Accepted
Phase: 29cc
Task: WSM-G4-min6
Title: Nyash Playground G4 Gate Promotion Closeout Lock
Depends:
  - docs/development/current/main/phases/phase-29cc/29cc-175-wsm-g4-min5-headless-two-example-parity-lock-ssot.md
  - tools/checks/dev_gate.sh
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh
---

# 29cc-176 WSM-G4-min6 Gate Promotion Closeout Lock

## Goal

G4 min1..min5 の lock を `wasm-demo-g2` 標準ゲートへ昇格し、G4 を closeout する。

## Scope

1. `wasm-demo-g2` に min6 closeout smoke を含める。
2. `wasm-boundary-lite` は既存運用を維持する（回帰なし）。
3. phase/docs pointer を monitor-only へ同期する。

## Implementation Contract

1. `phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh` を追加し、次を固定:
   - lock doc keyword contract
   - `dev_gate.sh` に min5/min6 の step が存在すること
   - `phase29cc_wsm_g4_min5_headless_two_examples_vm.sh` 回帰緑
2. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` を G4 closeout 状態へ同期する。
3. wasm lane active next は `none`（monitor-only）へ戻す。

## Acceptance

- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g2`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next

- wasm lane active next: `none`（monitor-only, failure-driven）。
