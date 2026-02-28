---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: runtime cutover Step-1 として V0 ABI slice（3語彙）を docs-first で固定する。
Related:
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/abi/nyrt_c_abi_v0.md
  - tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
---

# 29cc-216 Runtime V0 ABI Slice Lock

## Purpose

execution-path-zero へ向けた最初の ABI 実装範囲を 3 語彙に固定し、実装拡散を防ぐ。

## Slice vocabulary (fixed)

1. `string_len`
2. `array_get_i64`
3. `array_set_i64`

## Contract (language-agnostic)

1. canonical ABI surfaces は Core C ABI / TypeBox ABI v2 の 2 面を維持する。
2. 関数 ABI は `args borrowed / return owned` を維持する。
3. strict/dev では契約違反を fail-fast し、silent fallback を禁止する。
4. この lock では語彙追加を行わない（3語彙固定）。

## Per-vocabulary intent

1. `string_len`
   - 入力: String-like handle（borrowed）
   - 出力: i64 length（owned primitive）
2. `array_get_i64`
   - 入力: Array handle + index i64（borrowed）
   - 出力: i64 value（owned primitive）
3. `array_set_i64`
   - 入力: Array handle + index i64 + value i64（borrowed）
   - 出力: status / void 相当（owned primitive）

## `.hako` entry lock (Step-3 implementation anchor)

- VM low-level で `array_get_i64` / `array_set_i64` を受ける正本入口は
  `lang/src/runtime/collections/array_core_box.hako` とする。
- VM low-level で `string_len` を受ける正本入口は
  `lang/src/runtime/collections/string_core_box.hako` とする。
- `apps/std/array.hako` は高レイヤの std helper として扱い、VM core からは参照しない。
- `apps/std/string.hako` は高レイヤの std helper として扱い、VM core からは参照しない。

## Acceptance

1. `hako-runtime-c-abi-cutover-order-ssot.md` の Step-1 語彙と一致する。
2. `ABI_BOUNDARY_MATRIX.md` に V0 ABI slice 3語彙の境界が明記されている。
3. `phase29cc_runtime_v0_abi_slice_guard.sh` が green。
4. `phase29cc_runtime_v0_adapter_fixtures_vm.sh` で
   `string_len` の adapter route 契約（registry + handler + core box）が監査される。

## Not in this lock

1. Rust thin wrapper 実装本体
2. `.hako` adapter 実装本体
3. 4語彙目以降の追加
