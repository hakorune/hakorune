---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B3-min2 として、invoke route の invalid/unresolved 入口契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-239-kernel-b3-min1-future-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/invoke.rs
  - crates/nyash_kernel/src/plugin/invoke_core.rs
---

# 29cc-240 Kernel B3-min2 Invoke Route Lock

## Purpose

`invoke.rs` の各入口（i64/f64/by-name/tagged）で invalid/unresolved receiver を fail-fast（戻り値 `0`）に固定し、entry route の揺れを止める。

## Fixed Contract

1. `nyash_plugin_invoke3_i64` は invalid receiver で `0` を返す。
2. `nyash_plugin_invoke3_f64` は invalid receiver で `0.0` を返す。
3. by-name invoke（`*_name_*`, `invoke_by_name`）は invalid/unresolved input で `0` を返す。
4. tagged invoke（固定長/可変長）は invalid receiver で `0` を返す。
5. panic/silent fallback 追加は禁止。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `cargo test -p nyash_kernel invoke_ -- --nocapture` green
4. `tools/checks/dev_gate.sh runtime-exec-zero` green
5. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. B3-closeout（mod wiring lock）
2. kernel residue closeout（B1/B2/B3 統合）
