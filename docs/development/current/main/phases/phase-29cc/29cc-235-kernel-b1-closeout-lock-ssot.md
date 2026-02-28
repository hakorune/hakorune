---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の kernel B1（invoke/runtime/instance）完了条件を固定し、B2 codec boundary へ遷移する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-231-kernel-b1-min1-invoke-birth-route-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-232-kernel-b1-min1-closeout-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-233-kernel-b1-min2-runtime-state-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-234-kernel-b1-min3-instance-lifecycle-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
---

# 29cc-235 Kernel B1 Closeout Lock

## Purpose

kernel residue の B1 境界（`invoke_core/birth` + `runtime_data/semantics` + `instance`）を closeout し、次の B2 codec line を唯一の active next に固定する。

## B1 Closeout Criteria (fixed)

1. B1-min1/2/3 lock が揃い、境界契約が docs とテストで固定されている。
2. kernel plugin の B1 対象で panic/silent fallback を追加しない。
3. exported symbol と ABI 面（戻り値 `0/1/handle`）を変更しない。
4. source 削除は行わない（no-delete-first 維持）。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `tools/checks/dev_gate.sh runtime-exec-zero` green
4. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. B2-min1（value_codec encode/decode route lock）
2. B2-min2（borrowed_handle route lock）
3. B2-closeout（mod wiring lock）
