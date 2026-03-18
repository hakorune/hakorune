---
Status: Draft
Scope: `BoxBase::new` / `StringViewBox::new` の最適化相談前提を repo 内契約として整理する
Related:
- src/box_trait.rs
- crates/nyash_kernel/src/exports/string_view.rs
- crates/nyash_kernel/src/tests.rs
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/substring-view-materialize-boundary-ssot.md
- CURRENT_TASK.md
---

# Box Identity / View Allocation Design Note

## Goal

この文書は、`BoxBase::new` と `StringViewBox::new` が perf top に出たときに、
何が「触ってよい最適化」で、何が「identity 契約を壊す危険が高い最適化」かを
repo 内の言葉で固定するためのメモだよ。

目的は 2 つだけ。

1. `BoxBase::new` を単なる allocation helper と誤認しない。
2. 外部相談をするときに、質問を「この repo の契約」に合わせて狭くする。

## Current Contract

### Box identity

- [`src/box_trait.rs`](/home/tomoaki/git/hakorune-selfhost/src/box_trait.rs)
  - `next_box_id()` は global `AtomicU64`
  - `BoxBase::new()` / `BoxBase::with_parent_type()` は fresh id を発行する
  - `BoxCore::box_id()` はその id を観測面に出す
- visible contract:
  - 各 box instance は生存中に安定した `box_id` を持つ
  - live box 同士で id が衝突しない
  - `is_identity()` は clone/share policy を決めるが、id 発行そのものとは別

### Where `box_id` matters

- GC / finalization
  - [`src/runtime/gc_controller.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/gc_controller.rs)
  - [`src/finalization.rs`](/home/tomoaki/git/hakorune-selfhost/src/finalization.rs)
- identity-sensitive boxes
  - [`src/boxes/array/mod.rs`](/home/tomoaki/git/hakorune-selfhost/src/boxes/array/mod.rs)
  - [`src/boxes/map_box.rs`](/home/tomoaki/git/hakorune-selfhost/src/boxes/map_box.rs)
  - [`src/runtime/plugin_loader_v2/enabled/types.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/plugin_loader_v2/enabled/types.rs)
- value codec / alias wrappers
  - [`crates/nyash_kernel/src/plugin/value_codec/encode.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/value_codec/encode.rs)
  - [`crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs)
- tests
  - [`src/tests/box_tests.rs`](/home/tomoaki/git/hakorune-selfhost/src/tests/box_tests.rs)
  - [`src/box_trait.rs`](/home/tomoaki/git/hakorune-selfhost/src/box_trait.rs)

### `StringViewBox` contract

- [`crates/nyash_kernel/src/exports/string_view.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_view.rs)
  - `StringViewBox` stores `base_handle`, `base_obj`, `[start, end)`
  - read-only helpers may resolve through this metadata without eager copy
  - `clone_box()` / `share_box()` materialize to `StringBox`
  - each new view currently gets a fresh `BoxBase::new()`
- [`docs/development/current/main/design/substring-view-materialize-boundary-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/substring-view-materialize-boundary-ssot.md)
  - view semantics and materialize boundaries are already fixed in SSOT
- [`crates/nyash_kernel/src/tests.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/tests.rs)
  - short slice, nested short slice, mid slice, and fast-off contracts are pinned

## Red Lines

次は、現 wave では触ってはいけない線だよ。

1. `BoxBase::new` の generic id reuse
- 2 つの live box が同じ `box_id` を持つ危険がある
- GC / finalization / identity-sensitive boxes を壊す

2. `StringViewBox` に base id をそのまま流す
- distinct views collapse into one identity
- current `StringViewBox` contract does not define views as mere aliases

3. generic lazy id generation without a uniqueness proof
- `box_id()` が観測される前に必ず unique id を発行できる保証が要る
- repo 内にはその導線の SSOT がまだない

## Existing Special Case

- [`crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs)
  - `BorrowedHandleBox` は `source_handle` を stable id として再利用する
  - これは「既存 handle の alias wrapper」という明示的 special case
  - `StringViewBox` にそのまま一般化してよい根拠にはならない

## Safe Frontier

現時点で安全に触れるのは、`BoxBase::new` 自体ではなく、
`StringViewBox::new` を呼ぶ回数を減らす upstream 条件だけだよ。

具体例:

1. `borrowed_substring_plan_from_handle(...)` の `CreateView` 条件を絞る
2. short slice を materialize に寄せる閾値を見直す
3. nested view で view を増やさず materialize する条件を増やす

ただし 2 と 3 は、いまの substring-view 契約を変更する可能性がある。

## Decision Frame For External Consultation

外に相談するときは、質問を次のどれかに狭める。

1. `StringViewBox` should remain a distinct box with fresh id, but how can we reduce view creation count safely?
2. Is there a safe lazy-id pattern that preserves per-instance uniqueness before any observable `box_id()` use?
3. Under what exact conditions may an alias wrapper reuse an existing id without breaking GC/finalization semantics?

聞いてはいけない曖昧な質問:

- `BoxBase::new` を速くする一般論
- Rust object allocation を速くする一般論
- pooling すればよいか、という広すぎる相談

## Current Perf Reading

- current micro top:
  - `substring_hii`
  - `Registry::alloc`
  - `BoxBase::new`
- interpretation:
  - `BoxBase::new` は hot だが、current wave では stop-line
  - next safe cut is upstream of `StringViewBox::new`, not inside `BoxBase::new`

