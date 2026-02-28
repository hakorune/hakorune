---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B1-min2 として、runtime state shims（runtime_data/semantics/instance）の fail-fast route 契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-232-kernel-b1-min1-closeout-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/runtime_data.rs
  - crates/nyash_kernel/src/plugin/semantics.rs
  - crates/nyash_kernel/src/plugin/instance.rs
---

# 29cc-233 Kernel B1-min2 Runtime State Route Lock

## Purpose

`runtime_data`/`semantics`/`instance` の handle-based runtime state shims における fail-fast 境界（invalid/unresolved route の戻り値）をテストで固定する。

## Fixed Contract

1. `runtime_data.*` は invalid handle / unsupported route で `0` を返す（panic禁止）。
2. `semantics.add_hh` は invalid handle で `0` を返す（silent fallback 追加禁止）。
3. `instance.get_field_h/set_field_h` は invalid handle / null name / unresolved value で `0` を返す。
4. exported symbol と戻り値 ABI（`0`/`1`/handle）を変更しない。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `tools/checks/dev_gate.sh runtime-exec-zero` green
4. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. B1-min3（instance lifecycle deeper cutover）
2. B2（value codec line）
3. source 削除（no-delete-first 維持）
