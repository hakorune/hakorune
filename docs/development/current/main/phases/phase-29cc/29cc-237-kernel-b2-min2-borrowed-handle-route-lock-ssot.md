---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B2-min2 として、borrowed_handle profile route をテストで固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-236-kernel-b2-min1-value-codec-encode-decode-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs
  - crates/nyash_kernel/src/plugin/value_codec/decode.rs
  - crates/nyash_kernel/src/plugin/value_codec/tests.rs
---

# 29cc-237 Kernel B2-min2 Borrowed Handle Route Lock

## Purpose

`CodecProfile::ArrayFastBorrowString` だけが borrowed handle metadata を付与する契約を固定し、generic/profile route の混線を防ぐ。

## Fixed Contract

1. `ArrayFastBorrowString` profile で string handle を decode すると `borrowed_handle_source_fast` が `Some` になる。
2. `Generic` profile では string handle を decode しても `borrowed_handle_source_fast` は `None` のまま。
3. borrowed metadata の source handle は入力 handle と一致する。
4. panic/silent fallback 追加は禁止。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `cargo test -p nyash_kernel any_arg_to_box_ -- --nocapture` green
4. `tools/checks/dev_gate.sh runtime-exec-zero` green
5. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. B2-closeout（value_codec mod wiring lock）
2. B3-min1（future route lock）
