---
Status: Done
Decision: accepted
Date: 2026-02-28
Scope: WSM-Freeze-min3 として `NYASH_WASM_ROUTE_POLICY=rust_native` の適用範囲を compile-wasm のみに固定し、emit-wat 側を fail-fast で lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-208 WSM-Freeze-min3 Route Policy Scope Lock

## Purpose
Freeze 段の route policy (`default|rust_native`) を維持しつつ、`rust_native` の利用範囲を比較 lane（compile-wasm）へ限定する。  
`--emit-wat` での silent ignore を禁止し、scope mismatch は fail-fast で止める。

## Decision
1. `NYASH_WASM_ROUTE_POLICY=rust_native` は compile-wasm 専用とする。
2. `--emit-wat` 実行時に `rust_native` が指定された場合は `[freeze:contract][wasm/route-policy-scope]` で fail-fast する。
3. fail-fast 文言は `NYASH_WASM_ROUTE_POLICY=rust_native is compile-wasm only` を固定する。
4. `default` は `--emit-wat` でも従来通り許可する（挙動変更なし）。

## Implemented
1. `src/runner/modes/wasm.rs`
   - `enforce_wasm_route_policy_scope_for_emit_wat()` を追加。
   - `execute_emit_wat_mode()` 入口で scope guard を実行。
2. `tests/wasm_demo_min_fixture.rs`
   - `wasm_demo_emit_wat_rejects_rust_native_policy_scope_contract` を追加。
3. `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm.sh`
   - min3 契約 smoke を追加。
4. `tools/checks/dev_gate.sh`
   - `wasm-freeze-core` へ min3 smoke を追加。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_freeze_min3_route_policy_scope_emit_wat_vm.sh`
- `tools/checks/dev_gate.sh wasm-freeze-core`
- `tools/checks/dev_gate.sh wasm-freeze-parity`

## Next
- `WSM-Freeze-min4`（必要時）: freeze lane を monitor-only のまま維持し、route 追加禁止を guard で監査する。
