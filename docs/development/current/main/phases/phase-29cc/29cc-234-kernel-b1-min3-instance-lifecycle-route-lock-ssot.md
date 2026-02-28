---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B1-min3 として、instance field bridge の lifecycle fail-fast 境界を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-233-kernel-b1-min2-runtime-state-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/instance.rs
---

# 29cc-234 Kernel B1-min3 Instance Lifecycle Route Lock

## Purpose

`nyash.instance.get_field_h` / `nyash.instance.set_field_h` で、instance lifecycle 境界（finalized / invalid field name）を fail-fast（戻り値 `0`）で固定する。

## Fixed Contract

1. `nyash.instance.get_field_h` は `finalized` instance で常に `0` を返す。
2. `nyash.instance.set_field_h` は `finalized` instance で常に `0` を返す。
3. `name` が invalid UTF-8 のとき、get/set とも `0` を返す（panic 禁止）。
4. exported symbol 名と ABI 戻り値面（`0` / handle）は変更しない。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `tools/checks/dev_gate.sh runtime-exec-zero` green
4. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. B1 closeout lock（instance boundary closeout）
2. B2-min1（value_codec encode/decode line）
