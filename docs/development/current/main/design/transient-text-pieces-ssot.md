---
Status: provisional SSOT
Decision: provisional
Date: 2026-03-28
Scope: string hot path の transient carrier を、operation tree ではなく normalized small piece list (`TextPlan` / `PiecesN`) として読むための正本
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/string-transient-lifecycle-ssot.md
  - docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
  - docs/development/current/main/design/substring-view-materialize-boundary-ssot.md
  - docs/development/current/main/design/rep-mir-string-lowering-ssot.md
  - docs/development/current/main/design/recipe-scope-effect-policy-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_store.rs
  - benchmarks/bench_kilo_micro_substring_concat.hako
---

# Normalized Transient Text Pieces SSOT

## Goal

string hot path の transient carrier を、concat / insert / substring の操作木ではなく、**normalized small piece list** として固定する。

この文書の目的は 3 つだけだよ。

1. `TextPlan` を observable Box にしない。
2. `Concat2` / `Concat3` / `Insert` を transient operator として見つつ、実体はすぐ `PiecesN` に正規化する。
3. `freeze.str` だけが `StringBox` / `StringViewBox` birth を担当する。

## Core Rule

- transient は backend-local / pass-local / non-Box だよ
- runtime / plugin / FFI visible token contract にしない
- benchmark 名は owner にしない
- `PiecesN` は operation tree ではなく、**normalize 済み carrier** だよ

## Carrier Vocabulary

### Piece

最小の piece は次の 2 種類にする。

```text
Piece =
  RootSlice { base_handle, start, len }
  InlineLit  { ptr, len }
```

- `RootSlice` は既存の `StringSpan` / borrowed view を base にする
- `InlineLit` は小さい literal / inline piece を表す
- raw pointer は carrier の意味 owner ではなく、leaf で解決する補助情報として扱う

### TextPlan / TText

```text
TText =
  View1    { base_slice }
  PiecesN   { pieces[<=4], total_len }
  OwnedTmp  { ptr, len, cap }
```

- `View1` は 1 span の transient view
- `PiecesN` は 2〜4 piece 程度の normalized list
- `OwnedTmp` は spill / freeze sink のみ

### First Pilot Shape

v1 の first pilot は次の形で十分だよ。

- `substring` -> `View1`
- `concat2` / `concat3` -> `PiecesN`
- `insert` -> `PiecesN`
- `length` / `size` / `indexOf` -> read-only observer on transient

`Insert` ノードを owner として残さず、normalize した piece list にすぐ潰すのがポイントだよ。

## Boundary Rule

### freeze してよい地点

- loop-carried assignment
- array / map store
- FFI / C ABI visible point
- clone / share / retention boundary

### transient のままでよい地点

- read-only `substring`
- `concat2` / `concat3` の chain 内
- `length` / `size` / `indexOf` の observer path

### freeze sink

`freeze.str` は次だけを担当する。

- `StringBox` birth
- `StringViewBox` birth
- `handles::to_handle_arc(...)`
- `string_handle_from_owned(...)`
- materialize / copy / flatten

具体的な birth-time responsibilities は [`string-birth-sink-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/string-birth-sink-ssot.md) を正本にする。

## Owner Split

### `.hako` / docs owner

- string algorithm control structure
- escape / freeze policy
- `str.slice`
- `str.concat2`
- `str.concat3`
- `str.insert`
- `str.len`
- `freeze.str`

### Rust substrate owner

- hidden leaf copy / flatten
- handle registry
- `StringBox`
- `StringViewBox`
- `BoxBase`
- `host_handles`
- `freeze.str` leaf

Rust は carrier の実体を持ってよいけれど、**meaning owner** にはならない。

## Relationship To Other SSOTs

- [`string-transient-lifecycle-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/string-transient-lifecycle-ssot.md)
  - 4 層の読みを固定する親 SSOT
- [`transient-string-chain-boxless-wave-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md)
  - `substring -> concat3 -> length` pilot の wave SSOT
- [`substring-view-materialize-boundary-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/substring-view-materialize-boundary-ssot.md)
  - substring-only の view/materialize contract

この文書は上 2 つの SSOT をつなぐ carrier 定義だよ。

## Non-Goals

- rope / cons tree の導入
- runtime / VM / plugin visible new token layer
- `TextPlan` を public owner Box にすること
- benchmark-specific hardcode
- route / fallback policy の再設計
- `StringViewBox` の alias owner 化

## Rollout Order

1. docs-first
   - `TextPlan` / `PiecesN` の語彙を current docs に固定する
2. inventory
   - `substring_hii`, `concat_hs`, `insert_hsi`, `concat3_hhh`, `string_handle_from_owned` の birth map を棚卸しする
3. narrow AOT pilot
   - AOT backend-local のみで carrier を試し、VM / plugin / FFI に見せない
4. perf proof
   - stable / micro の両方で keep / discard を判断する
5. widen only on proof
   - generic scope / policy / recipe family の文脈にだけ広げる

## Acceptance

- `TextPlan` が Box として observable にならない
- `PiecesN` が operation tree ではなく normalized carrier として読める
- `freeze.str` の外で birth が増えない
- new transient token が runtime / plugin / FFI visible にならない
