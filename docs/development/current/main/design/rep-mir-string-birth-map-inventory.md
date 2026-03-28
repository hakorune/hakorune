---
Status: SSOT
Decision: provisional
Date: 2026-03-19
Scope: Shadow RepMIR pilot のために、`substring_concat` 周辺の current birth map を 1 枚で固定する
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/string-birth-sink-ssot.md
- docs/development/current/main/design/rep-mir-string-lowering-ssot.md
- docs/development/current/main/design/string-transient-lifecycle-ssot.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- crates/nyash_kernel/src/exports/string.rs
- crates/nyash_kernel/src/exports/string_view.rs
---

# Shadow RepMIR Birth Map Inventory

## Purpose

この文書は、Shadow RepMIR pilot の前提として、`substring -> concat3 -> length` 周辺の current birth site を明示する。

目的は 2 つだけだよ。

1. どの helper が今どの表現で始まり、どこで birth するかを固定する。
2. future shadow op へ置き換えるとき、`BoxBase::new` / `Registry::alloc` をどこまで避けられるかを見えるようにする。

## Reading Rule

ここでいう "birth" は次のいずれかを含む。

- `StringBox` / `StringViewBox` の物理 birth
- `handles::to_handle_arc(...)` による handle birth
- `string_handle_from_owned(...)` 経由の owned materialize birth
- `BoxBase::new` / `Registry::alloc` へ到達する経路

逆に、observer-only の pure length は birth しない。

## Birth Map

### 1. `substring_hii`

| Field | Current Truth |
| --- | --- |
| Input representation | `i64 handle + start/end i64` |
| Current planner | [`borrowed_substring_plan_from_handle(...)`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_view.rs) |
| Current birth site | `substring_hii` の `match` で `Materialize` は `string_handle_from_owned(...)`、`CreateView` は `handles::to_handle_arc(Arc::new(StringViewBox::new(...)))` |
| Reaches `BoxBase::new` | yes, via `StringViewBox::new(...)` / `StringBox` birth path |
| Reaches `Registry::alloc` | yes, via `handles::to_handle_arc(...)` |
| Future shadow op mapping | `thaw.str -> str.slice -> freeze.str`; read-only slice は `StrView`、escape boundary only で freeze |

Notes:

- current planner still decides `ReturnHandle` / `ReturnEmpty` / `Materialize` / `CreateView`
- this is still a substrate-bearing route, so the shadow pilot should only mirror the exact current boundary, not add a new runtime layer

### 2. `concat3_hhh`

| Field | Current Truth |
| --- | --- |
| Input representation | three `i64 handle` values |
| Current planner | `concat3_plan_from_fast_str(...)`, `concat3_plan_from_spans(...)`, `concat3_plan_from_parts(...)` |
| Current birth site | `freeze_concat3_plan(...)` when `Concat3Plan::Materialize(...)` is selected; handle reuse returns existing handle, materialize uses `string_handle_from_owned(...)` |
| Reaches `BoxBase::new` | yes, when materialize path is taken |
| Reaches `Registry::alloc` | yes, when materialize path is taken |
| Future shadow op mapping | `thaw.str -> str.cat3 -> freeze.str`; `Pieces3` or equivalent transient carrier should stay pass-local and only freeze at escape |

Notes:

- file-local `plan -> freeze -> handle` split is already accepted in `string.rs`
- this helper is the cleanest first place for a shadow lowering pilot because it already separates route selection from birth

### 3. `string_len_from_handle`

| Field | Current Truth |
| --- | --- |
| Input representation | `i64 handle` |
| Current planner | none; this is a read-only observer helper |
| Current birth site | none on the hot path; it only reads spans / strings and returns `i64` |
| Reaches `BoxBase::new` | no, in current accepted shape |
| Reaches `Registry::alloc` | no, in current accepted shape |
| Future shadow op mapping | `str.len` returning `i64` directly from `StrView` / `StrPieces`; no box birth |

Notes:

- this helper is observer-only and should remain so for this wave
- the rejected explicit downcast fast path is intentionally not reopened

### 4. `string_handle_from_owned`

| Field | Current Truth |
| --- | --- |
| Input representation | owned `String` |
| Current planner | none; this is a birth sink helper |
| Current birth site | `gc_alloc(value.len() as u64) -> StringBox::new(value) -> handles::to_handle_arc(arc)` |
| Reaches `BoxBase::new` | yes, through `StringBox::new(...)` |
| Reaches `Registry::alloc` | yes, through `handles::to_handle_arc(...)` |
| Future shadow op mapping | `freeze.str` for `StrOwned` or flattened `StrPieces`; keep as the single birth sink |

Notes:

- this helper is still the canonical freeze sink for owned strings in the current substrate
- it should not be duplicated by a new runtime token layer

### 5. `borrowed_substring_plan_from_handle`

| Field | Current Truth |
| --- | --- |
| Input representation | `i64 handle + start/end i64 + view_enabled bool` |
| Current planner | yes, this is the planner |
| Current birth site | it still constructs `Materialize(String)` and `CreateView(StringViewBox)` today |
| Reaches `BoxBase::new` | yes, via `CreateView(StringViewBox::new(...))` and the later birth sink |
| Reaches `Registry::alloc` | yes, via `CreateView` and later handle creation |
| Future shadow op mapping | planner should emit a recipe only; `substring_hii` or `freeze.str` should perform the birth |

Notes:

- this is the exact helper that should become a recipe emitter in the next structure-first slice
- the rejected `OwnedSubstring/ViewRecipe` shuffle proved that a plan/birth split without a real transient carrier is not enough

## Cross-Helper Summary

Current birth density is split across three places:

1. substring planner
2. concat3 freeze sink
3. owned-string handle sink

The shadow pilot should not add a fourth place.

Current docs-first direction is to collapse these readings under `freeze.str` as the single birth sink, while keeping planner/placement in compile-time and keeping runtime free of new observable token layers.
`concat_hs` and `insert_hsi` already share the `freeze_text_plan(...)` sink helper, but moving the canonical sink into `string_store.rs` was rejected on perf grounds; the next step is still to shrink the planner to recipe-only / boundary-only placement.

The narrowest useful mapping is:

- `substring_hii` -> `thaw.str -> str.slice -> freeze.str`
- `concat3_hhh` -> `thaw.str -> str.cat3 -> freeze.str`
- `string_len_from_handle` -> `str.len`
- `string_handle_from_owned` -> `freeze.str`
- `borrowed_substring_plan_from_handle` -> recipe only, no birth

## Pilot Rule

For `kilo_micro_substring_concat`, the pilot must stay within this birth map.

Not allowed:

- new runtime token types
- new `NyashBox` variants
- widening to VM / plugin / FFI
- backend-wide `RepMIR` semantics owner

Allowed:

- AOT consumer-local shadow lowering
- pass-local `RepKind` / `TStr`
- `freeze.str` as a narrow sink
- `freeze_text_plan(...)` as the current shared helper on the concat/insert pilot path

## Stop Lines

If a future change wants to:

- move birth into a new runtime layer
- expose transient reps through ABI
- change `BoxBase::new` semantics
- widen beyond string hot chain

then this map is no longer sufficient and the design must be re-cut.
