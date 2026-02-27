---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P6-min1 として `NYASH_WASM_ROUTE_POLICY` の default-only no-op 維持方針を docs-first で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-169-wsm-p5-min10-legacy-hard-remove-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/config/env/runner_flags.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-170 WSM-P6-min1 Route Policy Default-Only No-Op Lock

## Purpose
`WSM-P5-min10` で legacy policy 値を hard-remove した後、`NYASH_WASM_ROUTE_POLICY` をどう扱うかを固定する。  
この段階では ENV 名を即削除せず、`default-only` かつ実行上は no-op として維持する。

## Decision
1. `NYASH_WASM_ROUTE_POLICY` は当面残す（互換的な入力面の明示用）。
2. 受理値は `default` のみを維持し、非 `default` は `[freeze:contract][wasm/route-policy]` で fail-fast。
   - fail-fast 文言は `(allowed: default)` を含む。
3. `unset` と `default` の挙動は同一（wasm 出力同値）として契約化し、運用上は no-op とする。
4. 実際の compile route 判断は default lane 固定であり、この ENV は選択分岐を増やさない。
5. 撤去判断（ENV 物理削除）は `projects/nyash-wasm` 側の移植進行を見て別ミニタスクで行う。

## Implemented
1. `tests/wasm_demo_min_fixture.rs`
   - `wasm_demo_min_fixture_route_policy_default_noop_contract` を追加。
   - `NYASH_WASM_ROUTE_POLICY` unset と `default` で `.wasm` 出力が一致することを固定。
2. `src/config/env/runner_flags.rs`
   - `parse_wasm_route_policy_mode(Some(\"default\"))` 受理テストを追加し、default-only 契約を明示。
3. `tools/checks/dev_gate.sh`
   - `phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh` を `wasm-boundary-lite` に追加。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-G4-min1`: `projects/nyash-wasm` の最小移植対象（run loop + 1 fixture parity）を task-set として固定する。
