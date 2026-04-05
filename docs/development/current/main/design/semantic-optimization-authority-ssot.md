---
Status: SSOT
Decision: current
Date: 2026-04-06
Scope: semantic optimization authority を `.hako owner -> MIR canonical contract -> Rust executor -> LLVM generic optimization/codegen` で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md
  - docs/development/current/main/design/canonical-lowering-visibility-ssot.md
  - lang/src/runtime/kernel/string/README.md
  - lang/src/runtime/kernel/string/chain_policy.hako
  - lang/src/runtime/kernel/string/search.hako
  - lang/src/runtime/collections/array_core_box.hako
  - lang/src/runtime/collections/map_core_box.hako
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/exports/string_plan.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/array_string_slot.rs
  - src/runtime/host_handles.rs
---

# Semantic Optimization Authority SSOT

## Goal

- `.hako` に semantic ownership を保ったまま、optimization authority も `.hako` 側に寄せる
- Rust helper が policy owner に見えない構造を固定する
- shape-specific fast path を authority にせず、canonical contract の sugar に閉じ込める

## Final Stack

1. `.hako owner / policy`
2. `MIR canonical contract`
3. `Rust executor / accelerator`
4. `LLVM generic optimization / codegen`

This is the only intended authority order.

## `.hako owner / policy`

`.hako` が持つもの:

- route vocabulary
- retained-form / boundary choice
- visible collection contract
- adapter / route semantics

`.hako` は 「何を選ぶか」 を決める。
Rust helper に semantic branching を戻さない。

## MIR canonical contract

MIR が持つもの:

- owner choice を backend へ運ぶ canonical op 名
- `.hako` policy を Rust executor に渡す stable contract

Keep existing canonical ops:

- `thaw.str`
- `str.slice`
- `str.concat3`
- `str.len`
- `str.find_byte_from`
- `str.eq_at`
- `freeze.str`

Next candidate ops:

- `lit.str`
- `str.concat2`
- `store.array.str`
- `store.map.value`

Current source-backed mapping:

- `.hako` route `const_suffix`
  - current concrete path: `nyash.string.concat_hs`
  - intended canonical MIR reading: `thaw.str + lit.str + str.concat2 + freeze.str`
- `.hako` route `ArrayStoreString`
  - current concrete path: `nyash.array.set_his`
  - intended canonical MIR reading: `store.array.str`
- `.hako` route `MapStoreAny`
  - current concrete path: `nyash.map.slot_store_hhh`
  - intended canonical MIR reading: `store.map.value`

The candidate MIR names are current docs/SSOT truth.
They are not first-class MIR enum variants yet.

Do not encode `return_handle` as a standalone executor op.
It is a rewrite / elision outcome.

## Rust executor / accelerator

Rust が持つもの:

- borrowed view/span lifetime
- raw copy/search/compare
- freeze/materialize leaf
- direct slot/probe/store leaves
- drop-epoch invalidation
- single-lock borrowed access

`BorrowedText` と `TextSink` を使う場合でも、位置づけは Rust 内部 protocol のみ。
Public authority 名にしない。

## LLVM generic optimization / codegen

LLVM が持つもの:

- generic SSA optimization
- alloca / scalar / aggregate friendly lowering
- target codegen

LLVM に owner-aware placement を発見させない。
`borrow / freeze / store` の semantic choice は MIR までで確定する。

## Stop Lines

- `.hako` は owner / policy / route semantics まで
- MIR は canonical contract まで
- Rust は executor / accelerator に徹する
- LLVM は generic optimization と codegen に留める

Reopen gate:

- do not reopen `phase-137x` while canonical readings only exist as doc-level names
- first make the current concrete lowering visibly answer to:
  - `thaw.str + lit.str + str.concat2 + freeze.str`
  - `store.array.str`
  - `store.map.value`
- only then treat perf work as the next consumer

Do not:

- make Rust helper names the semantic source of truth
- grow shape-specific fast paths into independent policy systems
- let ABI facade absorb route semantics

## Implementation Order

1. freeze `.hako` route vocabulary as SSOT
2. freeze MIR canonical op set
3. adopt the first vertical slice on `concat const-suffix`
4. adopt the second vertical slice on `array string-store`
5. lock canonical-lowering visibility against current concrete symbols
6. only then generalize helper naming inside Rust
7. only then reopen perf consumers
