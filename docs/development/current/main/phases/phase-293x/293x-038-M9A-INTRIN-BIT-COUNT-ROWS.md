---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-038-M9A-INTRIN-BIT-COUNT-ROWS
Scope: M9a hako.intrin bit-count rows
---

# 293x-038 M9a Intrin Bit-Count Rows

## Decision

`hako.intrin` now exposes only the first truthful numeric bit-count rows:

```text
IntrinCoreBox.clz_i64(value)
IntrinCoreBox.ctz_i64(value)
IntrinCoreBox.popcnt_i64(value)
externcall "hako_intrin_clz_i64"(value)
externcall "hako_intrin_ctz_i64"(value)
externcall "hako_intrin_popcnt_i64"(value)
```

The current contract is deliberately narrow: values are current-lane
non-negative `i64`. Full unsigned-width runtime semantics remain a future
numeric row.

## Responsibility

- `lang/src/runtime/substrate/intrin/` owns the `.hako` intrinsic capability
  facade.
- `lang/c-abi/shims/hako_kernel.c` owns the native bit-count helpers.
- `MirI64IntrinsicsBox` owns deterministic VM-hako mirror behavior.
- `@rune IntrinsicCandidate` metadata remains separate and is not activated by
  this row.

## Live Surface

```text
clz_i64(value)
ctz_i64(value)
popcnt_i64(value)
```

## Non-Goals

- No `prefetch`, `assume`, or `unreachable` rows in this card.
- No backend optimization/export use.
- No full `u64` runtime value semantics.
- No `@rune IntrinsicCandidate` registry-consistency activation.

## Acceptance

- C ABI declares and implements `hako_intrin_clz_i64`,
  `hako_intrin_ctz_i64`, and `hako_intrin_popcnt_i64`.
- `IntrinCoreBox` routes to those helpers and fail-fasts negative values with
  `[freeze:contract][intrin/non-negative-i64]`.
- VM-hako subset accepts `boxcall(IntrinCoreBox.clz_i64/ctz_i64/popcnt_i64)`
  with one register argument.
- VM-hako subset accepts `externcall(hako_intrin_*_i64/1)`.
- unknown `hako_intrin_*` extern rows fail fast.
- compile v0 emits `mir_call(Extern:hako_intrin_clz_i64)`.

## Gates

```bash
bash tools/checks/k2_wide_intrin_first_row_guard.sh
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
