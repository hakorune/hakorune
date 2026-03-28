---
Status: provisional SSOT
Decision: provisional
Date: 2026-03-28
Scope: string hot path の birth/freeze を helper ごとに散らさず、`freeze.str` を唯一の birth sink として読むための正本
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/string-transient-lifecycle-ssot.md
  - docs/development/current/main/design/transient-text-pieces-ssot.md
  - docs/development/current/main/design/rep-mir-string-birth-map-inventory.md
  - docs/development/current/main/design/recipe-scope-effect-policy-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - crates/nyash_kernel/src/plugin/value_codec/string_store.rs
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - src/runtime/host_handles.rs
  - src/boxes/basic/string_box.rs
---

# String Birth Sink SSOT

## Goal

string の retained value birth を helper ごとに持たせず、`freeze.str` を **唯一の birth sink** として扱う。

この文書の目的は 3 つだけだよ。

1. `materialize_owned_string -> StringBox::new -> Registry::alloc/get` を 1 本の birth 直列として固定する。
2. `substring / concat / insert` の意味 owner と、birth timing / policy / sink を分離する。
3. 将来 `.hako` owner を広げても、runtime に新しい observable token や Box を増やさない。

## Core Rule

string の birth は op 名で決めない。

- `substring` だから birth、ではない
- `concat` だから birth、でもない
- **retained / substrate-visible になった地点だけ** `freeze.str` を許す

つまり、`TextPlan` / `PiecesN` が読まれるだけの chain では birth しない。
birth は `BoundaryKind` に従って、最後に 1 回だけ行う。

## Layer Split

### 1. Meaning / Contract (`.hako` / docs owner)

ここが持つもの:

- `__str.slice`
- `__str.concat2`
- `__str.concat3`
- `__str.insert`
- `__str.len`
- `__str.freeze_policy`
- `__str.escape_rule`

ここが持たないもの:

- `StringBox`
- `StringViewBox`
- handle alloc
- registry access
- raw copy kernel

### 2. Compile-time Planner (AOT-local)

ここは runtime 層ではなく、compile-time の placement 層だよ。

役割:

- `RepInference`
- `BirthPlacement`
- `StringFusion`

ここが決めるもの:

- どこで `freeze.str` を置くか
- どの chain を transient のまま通すか
- `BoundaryKind` の選択

ここが持たないもの:

- runtime visible token
- new `NyashBox`
- public ABI

### 3. Transient Carrier (backend-local, non-Box)

canonical carrier は `TText = View1 | PiecesN | OwnedTmp`

- `View1`: 1 span の borrowed view
- `PiecesN`: normalized small piece list
- `OwnedTmp`: spill only

carrier は identity を持たない。
VM / plugin / FFI / host handle へ見せない。

### 4. Birth Sink (`freeze.str`)

`freeze.str` は **birth-time concerns だけ** を持つ。

持つべき責務:

- `shared empty` / `ReuseHandle` / `full-slice reuse`
- `total_len` 決定
- flatten copy を 1 回だけ行う
- `StringBox` birth
- `Registry::alloc`
- handle return

持たないべき責務:

- meaning owner
- threshold policy owner
- route policy
- benchmark-specific branching

### 5. Native Leaf

leaf に残すもの:

- flat copy / flatten kernel
- `StringBox` object layout
- `BoxBase`
- `Registry::alloc/get`
- `drop_epoch`
- GC / TLS / atomic / ABI glue

## BoundaryKind

birth reason は helper 名ではなく `BoundaryKind` で読む。

v1 では次の 4 種で十分だよ。

- `LoopCarry`
- `Store`
- `AbiVisible`
- `CloneShare`

この文書の current exact lane では、`array_set` は `Store` boundary として扱う。
`substring+concat+len` の chain は birth せず、`dst.set(row, out)` でだけ `freeze.str` を許す。

## Current Reading

現状の hot 直列は次の 1 本として読む。

```text
TextPlan / PiecesN
  -> freeze.str
  -> materialize_owned_string
  -> StringBox::new
  -> Registry::alloc
  -> handle
```

perf-kilo の current asm/perf 読みでは、`set_his` の局所分岐よりもこの直列が先に支配している。

なので、次の exact cut は `nyash.array.set_his` の monomorphic split ではなく、**birth density を下げるための `freeze.str` 共通化** だよ。

## Immediate Rollout

1. docs-first
   - `freeze.str` を唯一の birth sink として current docs に固定する
2. narrow pilot
   - `concat_hs` を `concat3_hhh` と同じ `plan -> freeze` 系へ寄せる
3. narrow pilot
   - `insert_hsi` を同じ `plan -> freeze` 系へ寄せる
4. meso proof
   - `kilo_meso_substring_concat_array_set`
5. main proof
   - `kilo_kernel_small_hk`
6. only then
   - sink-local narrow tuning (`StringBox::new`, `Registry::alloc/get`)

## Non-Goals

- `freeze.str` を policy owner にすること
- runtime に新しい transient token / `NyashBox` を増やすこと
- `set_his` の helper split を benchmark 専用で増やすこと
- `HostHandleAllocPolicy` tuning を current exact lane に戻すこと
- loop-carry shaping を current exact laneへ先戻しすること

## Acceptance

- `freeze.str` が唯一の birth sink として読める
- `materialize_owned_string` は sink leaf としてだけ残る
- `substring / concat / insert` は helper ごとに birth policy を持たない
- runtime / VM / plugin / FFI visible な新 token を増やさない
