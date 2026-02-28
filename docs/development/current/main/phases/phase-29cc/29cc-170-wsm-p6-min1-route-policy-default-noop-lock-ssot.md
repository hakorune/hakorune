---
Status: Active
Decision: accepted
Date: 2026-02-27
Scope: WSM-P6-min1 として `NYASH_WASM_ROUTE_POLICY` の `default + rust_native` freeze 方針を docs-first で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-169-wsm-p5-min10-legacy-hard-remove-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/config/env/runner_flags.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-170 WSM-P6-min1 Route Policy Freeze Lock

## Purpose
`WSM-P5-min10` で legacy policy 値を hard-remove した後、`NYASH_WASM_ROUTE_POLICY` を freeze 段へ更新して扱いを固定する。  
この段階では ENV 名を即削除せず、`default + rust_native` の 2 値で運用する。

## Decision
1. `NYASH_WASM_ROUTE_POLICY` は当面残す（Freeze 段の route 明示用）。
2. 受理値は `default | rust_native` のみとし、それ以外は `[freeze:contract][wasm/route-policy]` で fail-fast。
   - fail-fast 文言は `(allowed: default|rust_native)` を含む。
3. `default` は `hako_native` 優先（shape未一致時のみ rust route へ縮退）として運用する。
4. `rust_native` は parity/診断専用 route とし、主実装先にしない。
5. 撤去判断（ENV 物理削除）は Retire Done Criteria 到達後に別ミニタスクで行う。

## Implemented
1. `tests/wasm_demo_min_fixture.rs`
   - `wasm_demo_route_trace_reports_rust_native_forced_contract` を追加。
   - `wasm_demo_route_trace_is_emitted_without_trace_env_contract` を追加。
2. `src/config/env/runner_flags.rs`
   - `parse_wasm_route_policy_mode(Some(\"rust_native\"))` 受理テストを追加し、freeze 契約を明示。
3. `tools/checks/dev_gate.sh`
   - `phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh` で freeze 契約を継続監査。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-Freeze-min2`: rust_native parity lane と hako_native mainline lane の gate 分離を固定する。
