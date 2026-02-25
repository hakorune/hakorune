---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X29 safety gate 一本化（unsafe route boundary + lifecycle boundary の fail-fast 契約統一）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-54-vm-verifier-gate-single-entry-ssot.md
  - src/runner/modes/common_util/safety_gate.rs
  - src/runner/modes/vm.rs
  - src/runner/modes/vm_fallback.rs
  - src/runner/modes/vm_hako.rs
  - tools/checks/vm_safety_gate_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_safety_gate_single_entry_vm.sh
---

# Phase 29x X29: VM Safety Gate Single-Entry SSOT

## 0. Goal

safety 判定を mode ごとに重複実装しないため、
unsafe route boundary と lifecycle boundary の fail-fast 契約を
`common_util/safety_gate` の 1 箇所へ集約する。

## 1. Contract

1. `vm` / `vm-fallback` の Hako-like source fail-fast は
   `enforce_vm_source_safety_or_exit()` のみ許可。
2. `vm` / `vm-fallback` / `vm-hako` は
   `enforce_vm_lifecycle_safety_or_exit()` を必ず通す。
3. lifecycle violation（現行: `ReleaseStrong { values=[] }`）は
   `[freeze:contract][vm-route/safety-lifecycle]` で fail-fast する。
4. source boundary violation は
   `[freeze:contract][vm-route/safety-hako-source]` で fail-fast する。

## 2. Ownership Check

`tools/checks/vm_safety_gate_guard.sh` で次を機械判定する。

1. `vm.rs` / `vm_fallback.rs` に source safety hook が存在する
2. `vm.rs` / `vm_fallback.rs` / `vm_hako.rs` に lifecycle safety hook が存在する
3. `vm.rs` / `vm_fallback.rs` へ direct fail-fast 実装が残っていない
4. lifecycle reason 語彙（`release_strong-empty-values`）が `safety_gate` 所有である

## 3. Evidence (X29)

1. `cargo check -q --bin hakorune`
2. `cargo test -q safety_gate -- --nocapture`
3. `cargo test -q verifier_gate -- --nocapture`
4. `cargo test -q route_orchestrator -- --nocapture`
5. `bash tools/checks/vm_safety_gate_guard.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_safety_gate_single_entry_vm.sh`
7. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_verifier_gate_single_entry_vm.sh`
8. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
9. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`

## 4. Next Step

X30 で thin-rust Core C ABI 最小面（route / verify / safety）を docs/header/code で同期する。
