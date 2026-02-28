---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B3-min1 として、future route の invalid/unresolved 入口契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-238-kernel-b2-closeout-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/future.rs
---

# 29cc-239 Kernel B3-min1 Future Route Lock

## Purpose

`future.rs` の route 入口で invalid/unresolved receiver を fail-fast（戻り値 `0`）に固定し、B3 の async 境界を安定化する。

## Fixed Contract

1. `nyash.future.spawn_method_h` は invalid receiver で `0` を返す。
2. `nyash.future.spawn_instance3_i64` は invalid receiver で `0` を返す。
3. `env.future.set` / `env.future.await` は invalid future handle で `0` を返す。
4. panic/silent fallback 追加は禁止。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `cargo test -p nyash_kernel future_ -- --nocapture` green
4. `tools/checks/dev_gate.sh runtime-exec-zero` green
5. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. B3-min2（invoke route lock）
2. B3-closeout（mod wiring lock）
