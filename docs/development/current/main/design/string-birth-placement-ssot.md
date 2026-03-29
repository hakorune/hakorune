---
Status: provisional SSOT
Decision: provisional
Date: 2026-03-29
Scope: `freeze.str` より upstream の compile-time placement を、`TextPlan` / `PiecesN` と birth sink の間で固定する
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/concat3-array-store-placement-window-ssot.md
  - docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
  - docs/development/current/main/design/post-store-observer-facts-ssot.md
  - docs/development/current/main/design/string-transient-lifecycle-ssot.md
  - docs/development/current/main/design/transient-text-pieces-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/rep-mir-string-birth-map-inventory.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/exports/string_plan.rs
---

# String Birth Placement SSOT

## Goal

`TextPlan` / `PiecesN` を backend-local transient carrier として維持したまま、
`freeze.str` に入る前の **placement decision** を 1 箇所で読む。

この文書は sink の正本ではなく、sink の手前の placement の正本だよ。

## Why this exists

今の perf-kilo は sink-local tuning をかなり消化した。

- `Registry::alloc` は hot birth branch を registry 内で直展開済み
- `Registry::get` は direct clone path まで landed 済み
- `BoxBase::new` は stop-line

なので次に必要なのは sink の中ではなく、**どの chain を birth せずに運び、どの chain を view にし、どの chain を freeze するか** だよ。

## Placement Contract

compile-time placement は runtime helper ではなく、AOT consumer 側の decision だよ。

### Retention Class

この文書の `TextRetentionClass` は current code vocabulary だよ。
親 SSOT では、これを

- retained reason = `BoundaryKind`
- retained result = `RetainedForm`

へ分けて読む。

この wave では string chain を次の 4 クラスで読む。

- `ReturnHandle`
  - 既存 handle をそのまま返す
- `KeepTransient`
  - carrier のまま通す
- `RetainView`
  - view として残すが owned birth はしない
- `MustFreeze(BoundaryKind)`
  - `freeze.str` を許す

### BoundaryKind

現時点で必要なのは次の区別だよ。

- `ObserverOnly`
- `Store`
- `LoopCarry`
- `AbiVisible`
- `CloneShare`

### Decision rule

- `substring / concat / insert / concat3` は op 名だけで birth しない
- `BoundaryKind` が `MustFreeze` を要求した地点だけ `freeze.str` を置く
- `TextPlan` / `PiecesN` が `ObserverOnly` なら carrier のまま残す
- `RetainView` は `StringViewBox` birth ではなく、transient view で留める

次 wave の docs-first target は、これを `RetainedForm` まで分けて読むことだよ。

## Current Landing

現コードでは、この placement 語彙が `crates/nyash_kernel/src/exports/string_birth_placement.rs` に landed しているよ。

- `substring_hii`
  - `substring_retention_class(...)` を通して `RetainView` / `MustFreeze(Store)` を読む
- `concat_hs` / `insert_hsi`
  - `concat_suffix_retention_class(...)` / `insert_middle_retention_class(...)` を通して `ReturnHandle` / `KeepTransient` / `MustFreeze(Store)` を読む
- `concat3_hhh`
  - `concat3_retention_class(...)` を通して `ReturnHandle` / `KeepTransient` を読む
  - まだ file-local plan/freeze split のままだけど、decision 語彙は placement helper に寄せた

## What this doc owns

- `substring_hii` の `ViewSpan` / `FreezePlan` 分岐の placement 語彙
- `concat_hs` / `insert_hsi` / `concat3_hhh` の `plan -> freeze` placement 語彙
- `BorrowedSubstringPlan` を recipe-only / boundary-only に読む基準
- `freeze.str` の手前で birth しない chain を明文化すること

## What this doc does not own

- `TextPlan` / `PiecesN` の carrier 形そのもの
- `freeze.str` の sink leaf
- `materialize_owned_string`
- `StringBox::new`
- `Registry::alloc/get`
- `BoxBase::new`
- trailing `length()` などの post-store observer facts

## Current Reading

現コードの読みはこうだよ。

- `string.rs`
  - `concat_hs` / `insert_hsi` は `freeze_text_plan(...)` を共有
  - `concat3_hhh` は `plan -> freeze` の file-local split
- `string_view.rs`
  - `BorrowedSubstringPlan` は `ReturnHandle / ReturnEmpty / FreezePlan / ViewSpan`
- `string_store.rs`
  - birth sink は残るが canonical owner はここに移していない

つまり、現在の upstream placement は **helper-local** ではなく **boundary-local** に寄せるのが正解だよ。

## Current Next Move

1. placement helper の語彙を current truth として維持する
2. `TextRetentionClass` を parent SSOT の `BoundaryKind` / `RetainedForm` split で読む
3. `array_set` を first `Store` proof boundary として維持する
4. `concat3-array-store-placement-window-ssot.md` で compiler-local placement window を固定する
5. meso/main proof の後にだけ code-side enum split を検討する

## Non-Goals

- 新しい observable Box を増やすこと
- sink-local leaf tuning を再開すること
- `BoxBase::new` を micro-cost としていじること
- `freeze.str` を policy owner にすること

## Acceptance

- `TextPlan` / `PiecesN` と `freeze.str` の間に placement 語彙がある
- `substring_hii` の `ViewSpan` / `FreezePlan` が helper-local ではなく decision で読める
- placement helper が `string_birth_placement.rs` として landed している
- C 並みを狙う時に、birth density を upstream から減らす導線がある
