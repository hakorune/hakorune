---
Status: investigation
Date: 2026-03-28
Scope: `perf-kilo` current wave の string birth hot path について、accepted / rejected / stop-line を 1 枚で読めるようにする
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/investigations/perf-kilo-string-leaf-rejected-followups-2026-03-28.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_store.rs
  - src/runtime/host_handles.rs
  - src/boxes/basic/string_box.rs
---

# Perf Kilo String Birth Hot Path Summary (2026-03-28)

## Goal

`perf-kilo` の string hot path について、今進んだもの・外れたもの・止めるべきところを 1 枚で読めるようにする。

このノートは shell history の代わりではなく、current lane の summary だよ。

## Current Accepted Slices

- placement helper landed
  - `crates/nyash_kernel/src/exports/string_birth_placement.rs` now owns the compile-time placement vocabulary
  - `substring_hii` / `concat_hs` / `insert_hsi` / `concat3_hhh` now read the same retention classes
- docs-first retained-boundary split landed
  - `retained-boundary-and-birth-placement-ssot.md` now owns `BoundaryKind` vs `RetainedForm`
  - `placement` and `sink` no longer share the same parent explanation
- `freeze.str` は canonical sink として残す
  - `concat_hs` と `insert_hsi` は `freeze_text_plan(...)` を共有
  - `concat3_hhh` は file-local plan/freeze split のまま
- `BorrowedSubstringPlan` は recipe-only / boundary-only まで縮んだ
  - `crates/nyash_kernel/src/exports/string_view.rs` が recipe 側を持つ
  - `crates/nyash_kernel/src/exports/string.rs` が dispatch + freeze を受ける
- store boundary の in-place source borrow は landed
  - `array_set_by_index_string_handle_value(...)` は hot path で一時 `Arc` clone を作らない
- sink-local hot branch の direct cut も landed
  - `Registry::alloc` は hot birth branch を registry 内で直展開
  - `Registry::get` は direct clone path へ縮退
- narrow branch-check trim landed
  - `concat_hs` const-suffix empty-path and `insert_hsi` source-empty lookup are no longer checked twice
  - kept recheck is `kilo_kernel_small_hk = 707 ms`, `kilo_meso_substring_concat_array_set = 68 ms`

## Current Rejected Slices

詳細は [perf-kilo-string-leaf-rejected-followups-2026-03-28.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/perf-kilo-string-leaf-rejected-followups-2026-03-28.md) に置く。

ここでは current wave で再び触らないものだけ要約するよ。

- `freeze.str` canonical sink を `string_store.rs` に移す試行
- direct `concat_hs` / `concat3` copy materialization
- piece-preserving `insert_inline` plus store/freeze reshaping
- blanket `#[inline(always)]` on host registry / hako-forward string wrappers
- `concat_hs` duplicate span-resolution removal plus span-resolver inlining
- specialized `StringBox`-only store leaf under `nyash.array.set_his`

## Current Stop-Line

- `BoxBase::new` は stop-line
  - identity semantics に結びつくので、現 wave の safe cut ではない
- `StringBox::new` も単独 target ではない
  - thin wrapper に近く、ここだけを切っても whole-program の win 根拠が弱い
- `Registry::alloc` / `Registry::get` は landed
  - sink-local lane としての追加安全 cut は現時点で無い
- current placement lane has now landed, but perf has not yet moved
  - the next step is upstream birth-density proof, not another sink-local cut

## Latest ASM Read

2026-03-29 の latest `kilo_meso_substring_concat_array_set` microasm 読みでは、次が top tier だったよ。

- `nyash_rust::runtime::host_handles::get` (`16.62%`)
- `nyash_rust::runtime::host_handles::Registry::alloc` (`14.85%`)
- `nyash_rust::box_trait::BoxBase::new` (`10.93%`)
- `nyash.string.substring_hii` (`7.88%`)
- `nyash.array.string_len_hi` (`5.54%`)
- `nyash.array.set_his` (`5.34%`)
- `nyash.string.insert_hsi` (`4.47%`)
- `nyash_kernel::exports::string::string_is_empty_from_handle` (`4.21%`)

読みとしては、sink-local leaf ではなく、`concat_hs` / birth-boundary / handle registry の組み合わせがまだ支配的だよ。
なのでこの lane で次に触るなら、`string_birth_placement.rs` を retained-boundary parent SSOT に従って再整理し、upstream birth-density を下げる側になる。

## Next Move

- この lane では sink-local tuning を止める
- 次の候補は upstream の birth-density / placement 側
  - `BoundaryKind` と `RetainedForm` を分けた parent contract を維持する
  - `array_set` を first `Store` proof boundary として扱う
  - `StringViewBox::new` / `freeze_text_plan(...)` への到達回数を減らせるかを見る
- そこへ行く前に、docs は current truth に追従しておく

## Acceptance

- accepted / rejected / stop-line を 1 枚で辿れる
- current lane の再実装を避けられる
- 新しい leaf を探す時に、どこを見て何を見ないかが明確になる
