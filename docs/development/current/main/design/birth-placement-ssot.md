---
Status: SSOT
Decision: current
Date: 2026-04-06
Scope: string/collection hot path で helper 名ではなく Birth / Placement outcome を正本にし、`.hako owner -> MIR canonical contract -> Rust birth backend` の責務を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
  - docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/exports/string_plan.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_store.rs
  - src/runtime/host_handles.rs
---

# Birth / Placement SSOT

## Goal

- helper 名ではなく Birth / Placement outcome を正本にする
- `string_handle_from_owned(...)` や `freeze_text_plan(...)` を semantic source of truth にしない
- `.hako owner / policy -> MIR canonical contract -> Rust birth backend` の読みを固定する
- string hot path の最適化を局所 hack ではなく placement seam 単位で扱う
- Rust/C++/C/LLVM の borrowed/materialize/storage discipline を層ごとに分けて使う

## Why Now

Current exact front は `const_suffix` route ではなく generic string consumer である。

- `kilo_micro_concat_hh_len`
  - current AOT consumer: `nyash.string.concat_hh` + `nyash.string.len_h`
  - current executor detail:
    - `string_concat_hh_export_impl(...)`
    - `string_len_from_handle(...)`
- current microasm read:
  - `string_concat_hh_export_impl`: `54.04%`
  - `string_len_from_handle`: `21.37%`
  - `__memmove_avx512_unaligned_erms`: `15.40%`

The next generic seam is not a new route name.
It is the Birth / Placement outcome that decides whether a value:

- reuses an existing handle
- stays borrowed
- freezes into owned form
- creates a fresh handle
- materializes from borrowed data
- stores from source

## Vocabulary

Use these six outcome names as the SSOT vocabulary:

1. `ReturnHandle`
2. `BorrowView`
3. `FreezeOwned`
4. `FreshHandle`
5. `MaterializeOwned`
6. `StoreFromSource`

Interpretation:

- `ReturnHandle`
  - rewrite / elision outcome
  - not a standalone executor op
- `BorrowView`
  - borrowed/view retained form
  - lifetime-sensitive, non-owning
- `FreezeOwned`
  - sink outcome
  - borrowed / planned text becomes owned string output
- `FreshHandle`
  - new host handle issue
  - runtime backend event, not owner policy
- `MaterializeOwned`
  - owned string realization in native runtime
  - backend leaf below policy/contract
- `StoreFromSource`
  - collection sink that preserves source-based string storage when allowed

## Layer Responsibilities

### `.hako owner / policy`

Owns:

- birth trigger
- retained-form choice
- boundary choice
- visible collection/string route semantics

Examples:

- `concat_suffix_retention_class(...)`
- substring retained-form rules
- route vocabulary such as `const_suffix`

`.hako` decides **whether** a route should reuse, borrow, freeze, or materialize.
It borrows Rust-like ownership vocabulary as semantic meaning only.

### MIR canonical contract

Owns:

- canonical naming of the chosen outcome
- rewrite / elision reading
- stable transport from owner choice to backend execution

Examples:

- `str.concat2`
- `str.concat3`
- `str.len`
- `freeze.str`
- `store.array.str`
- `store.map.value`

MIR carries **what outcome was chosen**, not the runtime mechanics of issuing a handle.
It is also the right place to keep delayed-materialization reading stable.

### Rust birth backend family

Owns:

- freeze/materialize execution
- fresh handle issue
- registry alloc/write
- borrowed lifetime substrate
- store-from-source backend

Current backend leaves include:

- `string_handle_from_owned(...)`
- `freeze_text_plan(...)`
- `materialize_owned_string(...)`
- `string_handle_from_span(...)`
- `store_string_box_from_source(...)`

These are backend leaves only.
They must not become public policy vocabulary.
Rust keeps C-like storage/lifetime mechanics here.

## Current Source Mapping

| Outcome vocabulary | Current Rust detail | Scope |
| --- | --- | --- |
| `ReturnHandle` | reuse branch in string concat/substr helpers | rewrite outcome |
| `BorrowView` | `StringSpan` / `StringViewBox` | borrowed/lifetime substrate |
| `FreezeOwned` | `freeze_text_plan(...)` | string sink backend |
| `FreshHandle` | `string_handle_from_owned(...)` | fresh handle backend |
| `MaterializeOwned` | `materialize_owned_string(...)` | registry/alloc backend |
| `StoreFromSource` | `store_string_box_from_source(...)` | collection sink backend |

## First Vertical Slice

The first slice is **generic `concat_hh + len_h`**, not `const_suffix`.

Reason:

- current exact front is `kilo_micro_concat_hh_len`
- current AOT consumer is `nyash.string.concat_hh` + `nyash.string.len_h`
- recent read-side small seams regressed; the next generic question is how often this path lands in:
  - `ReturnHandle`
  - `BorrowView`
  - `FreezeOwned`
  - `FreshHandle`
  - `MaterializeOwned`

### Reading Lock for the First Slice

Read `concat_hh + len_h` as:

- owner choice above
- canonical contract in the middle
- Birth / Placement outcome at the seam
- backend leaves below

Do **not** read it as:

- “the optimization of `string_concat_hh_export_impl(...)`”
- “the optimization of `string_len_from_handle(...)`”

Those are implementation details, not the seam vocabulary.

## Optimization Rule

Before adding a new hot-path optimization:

1. identify the Birth / Placement outcome being exercised
2. decide whether the issue is
   - owner choice
   - MIR reading
   - backend leaf cost
3. optimize only at that layer

This prevents:

- helper-local policy growth
- route-specific hacks becoming accidental authorities
- mixing read-side and birth-side optimizations without a seam name

## Current Direction

For the current `concat_hh + len_h` front:

- read-side small seams have been tried and reverted
- the next likely generic seam is Birth / Placement backend cost
- specifically:
  - `FreshHandle`
  - `MaterializeOwned`

That means the next optimization work should target birth backend leaves while
keeping this SSOT vocabulary fixed.

## First Exact Probe Read

Current direct AOT probe for `bench_kilo_micro_concat_hh_len.hako` shows:

- `birth.placement`
  - `fresh_handle=800000`
  - `return_handle=0`
  - `borrow_view=0`
  - `freeze_owned=0`
- `birth.backend`
  - `materialize_owned_total=800000`
  - `materialize_owned_bytes=14400000`
  - `gc_alloc_called=800000`
  - `gc_alloc_bytes=14400000`

So the current generic exact front is not a `FreezeOwned`-heavy path.
It is a `FreshHandle -> MaterializeOwned` path.
