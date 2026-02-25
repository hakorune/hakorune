---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X20 strict/dev における VM route 優先順を固定。
Related:
  - docs/development/current/main/phases/phase-29x/29x-40-vm-route-cutover-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-41-vm-route-observability-ssot.md
  - src/config/env/vm_backend_flags.rs
  - src/runner/dispatch.rs
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh
---

# Phase 29x X20: VM Route Strict/Dev Priority SSOT

## 0. Goal

strict/dev では `backend=vm` の既定を `vm-hako` 優先へ切り替え、
compat は明示 opt-in (`NYASH_VM_USE_FALLBACK=1`) のみに固定する。

## 1. Priority Contract

`backend=vm` の分岐順は次を SSOT とする。

1. `NYASH_VM_USE_FALLBACK=1`:
   - `lane=compat-fallback`
   - tag: `[vm-route/select] backend=vm lane=compat-fallback reason=env:NYASH_VM_USE_FALLBACK=1`
2. strict/dev 優先が有効（既定）:
   - `lane=vm-hako`
   - tag: `[vm-route/select] backend=vm lane=vm-hako reason=strict-dev-prefer`
3. それ以外:
   - `lane=vm`
   - tag: `[vm-route/select] backend=vm lane=vm reason=default`

## 2. Env Contract

- strict/dev 優先の既定値:
  - `HAKO_JOINIR_STRICT=1` または `NYASH_JOINIR_STRICT=1`
  - または `NYASH_JOINIR_DEV=1` / joinir debug 有効
- override:
  - `NYASH_VM_HAKO_PREFER_STRICT_DEV=0|1`

## 3. Acceptance (X20)

- smoke:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
- 判定:
  - strict/dev (`NYASH_JOINIR_STRICT=1`) で `backend=vm` は `lane=vm-hako`
  - strict/dev + `NYASH_VM_USE_FALLBACK=1` は `lane=compat-fallback`
  - compat は常に明示時のみ

## 4. Notes

- X19 の observability smoke は `lane=vm` 観測時に
  `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` を使って固定する。
- 既定 route の変更時も tag 語彙（`lane` / `reason`）は追加せず再利用する。
