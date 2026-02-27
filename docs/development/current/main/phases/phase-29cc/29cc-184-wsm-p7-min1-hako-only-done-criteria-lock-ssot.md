---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P7-min1 として「wasm `.hako`-only 完了判定」を docs-first で固定し、P7 実装順を lock する。
Related:
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-175-wsm-g4-min5-headless-two-example-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-176-wsm-g4-min6-gate-promotion-closeout-lock-ssot.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
---

# 29cc-184 WSM-P7-min1 Hako-Only Done Criteria Lock

## Purpose
wasm lane が monitor-only に戻っている状態から、`.hako`-only 完了判定を再起動するための基準を固定する。  
実装の可否判定を曖昧にせず、P7 の各 min を順序固定で進める。

## Decision
1. `.hako`-only 完了判定（P7）を次の 3 条件で固定する。
   - default route が `NYASH_WASM_ROUTE_POLICY=default` のみ受理し、legacy 値を fail-fast で拒否する。
   - `projects/nyash-wasm` 由来の 2 デモ（`webcanvas` / `canvas_advanced`）が headless parity で緑。
   - `tools/checks/dev_gate.sh wasm-demo-g3-full` と `tools/checks/dev_gate.sh portability` が緑。
2. compat は即削除しない。
   - default route の挙動は不変。
   - 互換は「内部 bridge 由来の縮退導線（実装上の互換責務）」として期限付き保持する。
3. P7 の固定順は `min2 -> min3 -> min4` とする。

## Fixed Order (P7)
1. `WSM-P7-min2`: default hako-only guard lock
2. `WSM-P7-min3`: two-demo lock（`webcanvas`/`canvas_advanced`）
3. `WSM-P7-min4`: compat retention lock（期限付き保持 + rollback 契約）

## Acceptance
1. `cargo check --features wasm-backend --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh`

## Next
1. `WSM-P7-min2`（default hako-only guard lock）へ進む。
