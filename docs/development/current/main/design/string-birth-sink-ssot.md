---
Status: provisional SSOT
Decision: provisional
Date: 2026-04-19
Scope: string hot path の birth/freeze を helper ごとに散らさず、`freeze.str` を唯一の birth sink として読むための正本
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
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

1. `freeze.str` を string birth sink として固定し、objectize/publication とは責務を分ける。
2. `substring / concat / insert` の意味 owner と、birth timing / policy / sink を分離する。
3. 将来 `.hako` owner を広げても、runtime に新しい observable token や Box を増やさない。

## Core Rule

string の birth は op 名で決めない。

- `substring` だから birth、ではない
- `concat` だから birth、でもない
- **retained / substrate-visible になった地点だけ** `freeze.str` を許す

つまり、`TextPlan` / `PiecesN` が読まれるだけの chain では birth しない。
birth は `BoundaryKind` に従って、最後に 1 回だけ行う。

## Publish Boundary Split

この文書では、`publish` と `freeze.str` を同じ責務として読まない。

- `publish`
  - lowering / boundary owner が決める effect
  - 「ここから public/object world に出る」を示す
  - v1 bridge は `publish.text(reason, repr)` に固定する
- `freeze.str`
  - retained string birth の sink
  - bytes birth / reuse / flatten の mechanical owner

Reading lock:

- `freeze.str` は publication policy owner ではない
- `publish` は第二の birth sink ではない
- runtime が `need_stable_object` を再推測して sink semantics を変えてはいけない
- `publish.text` は `TextPlan` を直接 consume しない。plan から publish したいなら先に `freeze.str` を通すか、借用 view replay を明示する
- current backend leaf が `materialize_owned_string -> StringBox::new -> Registry::alloc`
  まで続いて見えても、それは policy truth ではなく boundary-side mechanics として読む

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

`TextCell` はこの層の carrier ではない。
future sink/residence であって、`View1 / PiecesN / OwnedTmp` と同列の corridor value にしない。

### 4. Birth Sink (`freeze.str`)

`freeze.str` は **birth-time concerns だけ** を持つ。

親 SSOT に従って、

- `BoundaryKind` = retained reason
- `RetainedForm` = retained result

を sink の外で決める。この文書は sink 実行だけを持つ。

持つべき責務:

- `shared empty` / `ReuseHandle` / `full-slice reuse`
- `total_len` 決定
- flatten copy を 1 回だけ行う
- retained text birth / reuse
- boundary が既に public を要求している場合の cold handoff

持たないべき責務:

- meaning owner
- threshold policy owner
- publication policy owner
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
  -> MaterializeOwned / TextCell residence
  -> optional publish.text(reason, repr)
  -> optional cold objectize / handle issue
```

perf-kilo の current asm/perf 読みでは、`set_his` の局所分岐よりもこの直列が先に支配している。

current backend leaf may still realize the cold boundary as:

```text
materialize_owned_string
  -> StringBox::new
  -> Registry::alloc
  -> handle
```

ただし、これは mechanical leaf sequence であって、この文書の policy truth ではない。

なので、次の exact cut は `nyash.array.set_his` の monomorphic split ではなく、**`string_birth_placement.rs` による upstream placement proof** だよ。

## Immediate Rollout

1. docs-first
    - `freeze.str` を唯一の birth sink として current docs に固定する
   - `publish.text(reason, repr)` を boundary effect として固定する
2. docs-first parent
    - `BoundaryKind` と `RetainedForm` の split を retained-boundary parent SSOT に固定する
    - `reason` と `repr` の split を string semantic/boundary SSOT に固定する
3. landed
   - `concat_hs` と `insert_hsi` は `freeze_text_plan(...)` を共有し、`plan -> freeze` の形へ入った
4. next
    - `array_set` を first `Store` proof boundary として維持する
    - `substring_hii` を first `StableView` replay boundary として固定する
5. meso proof
   - `kilo_meso_substring_concat_array_set`
6. main proof
   - `kilo_kernel_small_hk`
7. only then
   - sink-local narrow tuning is paused after the direct `Registry::get` clone-path cut; the current summary is in `docs/development/current/main/investigations/perf-kilo-string-birth-hotpath-summary-2026-03-28.md`
   - compile-time placement helper landed in `crates/nyash_kernel/src/exports/string_birth_placement.rs`
    - keep `StringBox::new` out unless new asm evidence appears
    - `publish.any` は string-only `publish.text` が proving ground を通るまで deferred

### Rejected follow-up

- `freeze.str` の canonical sink を `string_store.rs` に移す試行は、stable main を `834 ms` と `909 ms` に悪化させたので rejected。
- そのため、現時点の active lane は `string.rs` 側の shared `freeze_text_plan(...)` helper を維持すること。
- planner cleanup は一段 landed 済みで、const-suffix / insert recipe helpers は `crates/nyash_kernel/src/exports/string_plan.rs` に隔離した。
- substring boundary cleanup is next: `BorrowedSubstringPlan` should become recipe-only / boundary-only.

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
