---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の kernel B2（value_codec）完了条件を固定し、B3 async/entry line に遷移する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-236-kernel-b2-min1-value-codec-encode-decode-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-237-kernel-b2-min2-borrowed-handle-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
---

# 29cc-238 Kernel B2 Closeout Lock

## Purpose

kernel residue B2（`value_codec/*`）の route 契約を closeout し、B3（`future.rs` / `invoke.rs` / `mod.rs`）へ着手する境界を固定する。

## B2 Closeout Criteria (fixed)

1. B2-min1（encode/decode immediate fallback）と B2-min2（borrowed_handle profile route）の lock が揃っている。
2. `value_codec` の route 契約はテストで固定され、profile 混線がない。
3. ABI 戻り値面（immediate / handle）を変更しない。
4. source 削除は行わない（no-delete-first 維持）。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `tools/checks/dev_gate.sh runtime-exec-zero` green
4. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. B3-min1（future route lock）
2. B3-min2（invoke route lock）
3. B3-closeout（mod wiring lock）
