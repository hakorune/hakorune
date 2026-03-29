---
Status: provisional SSOT
Decision: provisional
Date: 2026-03-29
Scope: `array.set` などの retained boundary の後ろにある observer を、boundary 自体へ同化せずに扱うための compile-time facts
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
  - docs/development/current/main/design/string-birth-placement-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/transient-text-pieces-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/exports/string_plan.rs
---

# Post Store Observer Facts SSOT

## Goal

`array.set` を first `Store` boundary として保ったまま、その直後に来る observer を別契約で読む。

この文書は `store` の semantic boundary を広げるためのものではなく、**store の後ろにある observer を compile-time facts として扱うためのもの** だよ。

## Why this exists

`array.set(...)` の後ろに `length()` が続く case を 1 つの retained/store boundary に同化しようとした retry は、WSL 3-run / 20-run の両方で main を悪化させた。

この lane で必要なのは「store を広げること」ではなく、

- `array.set` を first Store proof boundary として維持すること
- trailing `length()` を post-store observer として別扱いすること
- observer が carrier facts から答えられるなら、そこで direct lookup を減らすこと

だよ。

## Core Contract

### BoundaryKind

retain する理由は親 SSOT の `BoundaryKind` で読む。

- `Store`
- `LoopCarry`
- `AbiVisible`
- `CloneShare`

`array.set` は `Store` boundary のまま固定する。

### PostStoreUse

store の直後に何が来るかは、boundary とは別に `PostStoreUse` で読む。

- `None`
- `LenObserver`
- `ReadbackObserver`
- `EscapingConsumer`

`length()` は semantic boundary ではなく、`PostStoreUse::LenObserver` として読む。

### PlacedCarrierFacts

placement は、observer を store に混ぜる代わりに carrier facts を持てる。

```text
PlacedCarrierFacts = {
  boundary_kind: BoundaryKind,
  retained_form: RetainedForm,
  post_store_use: PostStoreUse,
  known_len: Option<usize>,
}
```

この `known_len` があるときだけ、`length()` は runtime materialize に戻らずに答えられる。

## What this doc owns

- `array.set` を first Store boundary として固定すること
- trailing `length()` を post-store observer として読むこと
- carrier facts で observer を減らす方針

## What this doc does not own

- `RetainedForm` の retained result taxonomy
- `BoundaryKind` の retained reason taxonomy
- `freeze.str` の sink implementation
- `StringBox::new`
- `Registry::alloc/get`
- `BoxBase::new`

## Current Reading

現 wave の読みはこうだよ。

- `array.set` は first Store proof boundary のまま
- `length()` は post-store observer として別扱い
- `TextPlan` / `PiecesN` の `known_len` があれば observer は carrier facts で答える
- それでも足りない case だけが runtime observer lookup になる

## Non-Goals

- `array.set + length()` を semantic boundary として同一化すること
- direct-store consumer widening を再開すること
- `StoreBoundaryKind` のような新しい boundary taxonomy を runtime に増やすこと
- `array.set` の owner を benchmark 名で増やすこと

## Current Task Order

1. `array.set` を first Store boundary として維持する
2. trailing `length()` を post-store observer として扱う
3. `TextPlan` / `PiecesN` の carrier facts から答えられる observer を増やす
4. meso/main proof は同じ artifact pair で見る
5. それでも必要なら、その時だけ `RetainedForm` wiring を code-side で広げる

## Acceptance

- `array.set` と trailing `length()` が同じ semantic boundary にされない
- `PostStoreUse` と `PlacedCarrierFacts` で observer を別契約として読める
- direct-store consumer widening を再開せずに、store boundary は first proof のまま保てる
- これ以降の placement work が、leaf tweak ではなく carrier facts の拡張として読める
