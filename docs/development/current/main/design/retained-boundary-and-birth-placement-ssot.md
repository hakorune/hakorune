---
Status: provisional SSOT
Decision: provisional
Date: 2026-03-29
Scope: string hot path の `BoundaryKind` と retained representation 要求を分離し、placement と sink の責務境界を固定する
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/recipe-scope-effect-policy-ssot.md
  - docs/development/current/main/design/transient-text-pieces-ssot.md
  - docs/development/current/main/design/string-birth-placement-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - crates/nyash_kernel/src/exports/string_birth_placement.rs
---

# Retained Boundary And Birth Placement SSOT

## Goal

`freeze.str` の前段で、次の 2 つを別の質問として扱う。

- `BoundaryKind`
  - なぜ transient chain が retained boundary に到達したのか
- `RetainedForm`
  - retained 側でどの representation が必要か

この文書は `placement` と `sink` の間にある境界契約の親 SSOT だよ。

## Why this split exists

いまの `TextRetentionClass` は実用上は足りているけれど、次の 2 つが少し混ざっている。

- `Store` / `LoopCarry` / `AbiVisible` のような retained reason
- `ReuseHandle` / `RetainView` / `NeedOwnedHandle` のような retained result

この 2 つを分けると、compile-time placement は「なぜ retained になるか」を決め、runtime sink は「何を作るか」を実行するだけになる。

## Core Contract

### BoundaryKind

`BoundaryKind` は retained reason だけを持つ。

- `Store`
- `LoopCarry`
- `AbiVisible`
- `CloneShare`

`ObserverOnly` は retained boundary ではないので、`KeepTransient` のまま通す。

### RetainedForm

`RetainedForm` は retained result だけを持つ。

- `ReuseHandle`
  - 既存 handle を再利用する
- `RetainView`
  - view carrier を retained 側へ渡す
- `NeedOwnedHandle`
  - owned `StringBox` / handle birth が必要

### BirthPlacement

compile-time placement の結果は、概念上は次の 2 分岐で読む。

- `KeepTransient`
- `Retained { boundary: BoundaryKind, form: RetainedForm }`

現コードではこの概念を `TextRetentionClass` で近似しているが、docs 上の正本はこの split とする。

## Current Mapping

現コードの `TextRetentionClass` は、当面は次の対応で読む。

- `ReturnHandle`
  - `Retained { form: ReuseHandle }`
- `RetainView`
  - `Retained { form: RetainView }`
- `MustFreeze(Store)`
  - `Retained { boundary: Store, form: NeedOwnedHandle }`
- `KeepTransient`
  - `KeepTransient`

つまり、今は code が 1 enum、docs は 2 軸という状態だよ。次 wave で code もこの split に近づける。

## Ownership Split

### This doc owns

- retained reason と retained result を分けること
- `array_set` を first `Store` proof boundary として扱うこと
- `placement` と `sink` の責務境界

### This doc does not own

- `PiecesN` の carrier shape
- `freeze.str` の sink implementation
- `StringBox::new`
- `Registry::alloc/get`
- `BoxBase::new`

## Current Task Order

1. docs-first で `BoundaryKind` と `RetainedForm` の split を固定する
2. `string_birth_placement.rs` は引き続き `TextRetentionClass` で動かしつつ、code-side mapping を明文化する
3. first proof boundary は `array_set`
4. meso/main proof を先に見て、それでも必要なら code enum を split する

## Non-Goals

- `freeze.any` のような family 横断 sink を作ること
- runtime-visible な新 token を増やすこと
- `Registry::alloc/get` の micro tuning をこの文書の主語にすること
- benchmark 名で owner を作ること

## Acceptance

- `BoundaryKind` と `RetainedForm` が別の概念として読める
- `placement` は retained reason を決め、`sink` は retained result を実行する
- `array_set` が first `Store` proof boundary として固定される
- `string-birth-placement` / `string-birth-sink` がこの文書を親として参照する
