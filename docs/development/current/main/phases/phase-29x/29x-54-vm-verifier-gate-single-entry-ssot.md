---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X28 verifier gate 一本化（vm / vm-fallback / vm-hako の verify 入口統一）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-53-vm-route-compat-bypass-failfast-ssot.md
  - src/runner/modes/common_util/verifier_gate.rs
  - src/runner/modes/vm.rs
  - src/runner/modes/vm_fallback.rs
  - src/runner/modes/vm_hako.rs
  - tools/checks/vm_verifier_gate_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_verifier_gate_single_entry_vm.sh
---

# Phase 29x X28: VM Verifier Gate Single-Entry SSOT

## 0. Goal

VM lane の verify 実装重複を止め、`NYASH_VM_VERIFY_MIR=1` 時の契約検査を
単一入口に固定する。対象は `vm` / `vm-fallback` / `vm-hako`。

## 1. Contract

1. `MirVerifier::new()` の callsite は `src/runner/modes/common_util/verifier_gate.rs` のみ許可。
2. `vm` / `vm-fallback` / `vm-hako` は
   `enforce_vm_verify_gate_or_exit(&module, "<route>")` を必ず通す。
3. `NYASH_VM_VERIFY_MIR=1` かつ verifier failure 発生時は
   `[freeze:contract][vm-route/verifier-gate]` で fail-fast する（warn-only 継続禁止）。
4. 詳細行は `[vm-route/verifier-detail]` の安定タグで 1行ずつ出力する。

## 2. Ownership Check

`tools/checks/vm_verifier_gate_guard.sh` で次を機械判定する。

1. `MirVerifier::new()` が `verifier_gate.rs` 以外に存在しない
2. `vm.rs` / `vm_fallback.rs` / `vm_hako.rs` に gate hook が存在する
3. 上記 route mode に `NYASH_VM_VERIFY_MIR` の直読みが残っていない

## 3. Evidence (X28)

1. `cargo check -q --bin hakorune`
2. `cargo test -q verifier_gate -- --nocapture`
3. `cargo test -q route_orchestrator -- --nocapture`
4. `bash tools/checks/vm_verifier_gate_guard.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_verifier_gate_single_entry_vm.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
7. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`

## 4. Next Step

X29 で safety gate（lifecycle/unsafe 境界）を同様に単一入口化する。
