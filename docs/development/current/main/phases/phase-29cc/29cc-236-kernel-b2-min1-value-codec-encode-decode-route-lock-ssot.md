---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B2-min1 として、value_codec encode/decode の未解決handle境界を route lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-235-kernel-b1-closeout-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/value_codec/decode.rs
  - crates/nyash_kernel/src/plugin/value_codec/encode.rs
  - crates/nyash_kernel/src/plugin/value_codec/tests.rs
---

# 29cc-236 Kernel B2-min1 Value Codec Encode/Decode Route Lock

## Purpose

`value_codec` の encode/decode で、未解決 handle（drop 後など）を即値として扱う契約を固定し、route drift を防ぐ。

## Fixed Contract

1. `any_arg_to_index` は正の入力でも handle 未解決時は即値 `arg` を返す。
2. `any_arg_to_box_with_profile` は handle 未解決時に profile を問わず `IntegerBox(arg)` として扱う。
3. `box_to_runtime_i64` に戻した結果は未解決 handle 入力時に `arg` と一致する。
4. panic/silent fallback 追加は禁止（戻り値契約で fail-fast）。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `cargo test -p nyash_kernel any_arg_to_ -- --nocapture` green
4. `tools/checks/dev_gate.sh runtime-exec-zero` green
5. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. B2-min2（borrowed_handle route lock）
2. B2-closeout（value_codec mod wiring lock）
