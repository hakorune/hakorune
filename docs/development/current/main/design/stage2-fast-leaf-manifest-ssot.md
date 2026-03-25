---
Status: SSOT
Decision: provisional
Date: 2026-03-25
Scope: `stage2` AOT/native fast-lane の internal-only leaf contract を固定し、public ABI を増やさずに backend-private fast path を生成する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
  - docs/development/current/main/design/stage2-string-route-split-plan.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/abi-export-manifest-v0.toml
  - docs/development/current/main/design/rust-kernel-export-surface-strata-ssot.md
  - docs/development/current/main/design/hako-host-facade-contract-ssot.md
  - src/runner/modes/llvm/method_id_injector.rs
  - src/runtime/extern_registry.rs
---

# Stage2 Fast Leaf Manifest (SSOT)

## Goal

- `AOT/native` hot path だけに使う backend-private fast leaf contract を固定する。
- public canonical ABI を増やさず、existing ABI manifest から internal fast rows を生成できるようにする。
- hot collection/runtime path の route collapse を、manifest-first で進められるようにする。

## Boundary Lock

### Public contract stays fixed

- public canonical ABI is still only:
  - `Core C ABI`
  - `TypeBox ABI v2`
- `FastLeafManifest` は public ABI ではない。
- plugin / host / VM reference lane はこの manifest を external contract として見てはいけない。

### Internal-only contract

- `FastLeafManifest` は build artifact / backend-private table として扱う。
- consumer は `AOT/native` lowering と backend route selection だけに限定する。
- current daily/mainline consumer は `ny-llvm` / `ny-llvmc` だけに固定する。
- `llvmlite` keep lane は consumer ではない。`FastLeafManifest` を理解する必要はなく、shared MIR / ABI / observer contract だけを維持すればよい。
- `rust-vm` / generic host/provider dispatch / plugin reverse-call は consumer ではない。

## Row Shape

### Canonical inherited fields

各 row はまず [`abi-export-manifest-v0.toml`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-manifest-v0.toml) の canonical row を継承する。

- `box_type`
- `method`
- `symbol`
- `args`
- `ret`
- `arg_ownership`
- `ret_ownership`
- `failure_contract`
- `compat_status`

### Backend-private added fields

`FastLeafManifest` で追加してよい row field は次だけに固定する。

- `leaf_id`
  - backend-private deterministic integer id
  - public meaningは持たない
- `value_class_profile`
  - lowering が期待する value-class specialization
  - initial values:
    - `scalar_i64`
    - `handle_any`
    - `borrowed_string`
- `may_alloc`
  - leaf 実行が allocation を起こしうるか
- `may_barrier`
  - leaf 実行が write barrier / GC hook を起こしうるか
- `cold_fallback`
  - monomorphic route 失敗時に落としてよい cold lane 名
  - initial values:
    - `none`
    - `generic_box_call`
    - `host_loader`

追加 field はこの SSOT を更新するまで増やさない。

## Generation Rule

### Source of truth

- row source は existing ABI manifest だよ。
- `FastLeafManifest` は canonical ABI manifest から生成する。
- hand-written second truth を別に持たない。

### Inclusion rule

V0 で fast-leaf に含めてよいのは次だけ。

1. `compat_status = adapter-default` または `active mainline`
2. current source layering で visible semantics owner が既に fixed
3. hot collection/runtime lane に属する
4. `HostFacade/provider/plugin loader` を通さずに leaf まで届く

V0 で除外するもの:

- compat-only rows
- plugin/extern/provider rows
- host reverse-call rows
- dynamic loader rows
- broad runtime-data facade rows

### Leaf ID rule

- `leaf_id` は build-time deterministic であること
- assignment order は次に固定する
  1. `box_type`
  2. `method`
  3. `symbol`
- stable source orderではなく sorted order で振る
- `leaf_id` の値は public compatibility を持たない

## V0 Eligible Rows

V0 fast-leaf 対象は次で固定する。

### Array

- `ArrayBox.len` -> `nyash.array.slot_len_h`
- `ArrayBox.length` -> `nyash.array.slot_len_h`
- `ArrayBox.size` -> `nyash.array.slot_len_h`
- `ArrayBox.get` -> `nyash.array.slot_load_hi`
- `ArrayBox.push` -> `nyash.array.slot_append_hh`

### Map

- `MapBox.size` -> `nyash.map.entry_count_h`
- `MapBox.len` -> `nyash.map.entry_count_h`
- `MapBox.get` -> `nyash.map.slot_load_hh`
- `MapBox.set` -> `nyash.map.slot_store_hhh`
- `MapBox.has` -> `nyash.map.probe_hh`

### String observer

- `StringBox.len` -> `nyash.string.len_h`
- `StringBox.length` -> `nyash.string.len_h`
- `StringBox.size` -> `nyash.string.len_h`

## Initial Metadata Defaults

V0 defaults are fixed like this.

| Row family | value_class_profile | may_alloc | may_barrier | cold_fallback |
| --- | --- | --- | --- | --- |
| `ArrayBox.len/length/size` | `handle_any` | `0` | `0` | `none` |
| `ArrayBox.get` | `scalar_i64` | `0` | `0` | `generic_box_call` |
| `ArrayBox.push` | `handle_any` | `1` | `1` | `generic_box_call` |
| `MapBox.size/len` | `handle_any` | `0` | `0` | `none` |
| `MapBox.get` | `handle_any` | `0` | `0` | `generic_box_call` |
| `MapBox.set` | `handle_any` | `1` | `1` | `generic_box_call` |
| `MapBox.has` | `handle_any` | `0` | `0` | `generic_box_call` |
| `StringBox.len/length/size` | `handle_any` | `0` | `0` | `none` |

`String concat/substring/search` は V0 対象外。`String` 2-wave と `cold dynamic lane split` は landed 済みで、`hako_alloc policy/state contract` も landed stop-line に達したが、widen 判断は `plugin route-manifest hardening` 後の別 wave に保留する。

## Consumer Rule

### Allowed consumers

- LLVM/AOT lowering route tables
- backend-private boundary dispatch
- method-id injection / direct leaf selection

### Forbidden consumers

- `llvmlite` harness / `src/llvm_py/**`
- `HostFacade.call(...)`
- plugin loader metadata/config truth
- runtime generic extern/provider dispatch
- public docs/reference ABI tables

## Next Exact Implementation Order

1. `Array hot path collapse` (landed)
   - first code slice
   - V0 route-table use is fixed for `slot_len/load/append` and `set_hih|set_hii`
2. `Map hot path collapse` (landed)
   - observer route is fixed on `nyash.map.entry_count_h`
   - raw `slot_load_hh` / `slot_store_hhh` / `probe_hh` stay the direct seam
3. `String search/slice route split` (landed)
4. `String concat route split` (landed)
5. `cold dynamic lane split` (landed)
6. `hako_alloc policy/state contract` (landed)
7. `plugin route-manifest hardening`
8. widen fast-leaf eligibility only after plugin metadata/route hardening is fixed

## Acceptance

- `FastLeafManifest` does not create a third public ABI
- inherited row truth still comes from [`abi-export-manifest-v0.toml`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-manifest-v0.toml)
- V0 eligible rows are only Array/Map/String observer hot rows
- cold dynamic lanes stay excluded
- `ny-llvm` / `ny-llvmc` is the only fast-leaf consumer
- `llvmlite` remains a keep lane outside the fast-leaf contract
- docs point to `plugin route-manifest hardening` as the next exact code slice

## Non-Goals

- publishing `leaf_id` outside backend internals
- moving host/provider/plugin rows into fast-leaf V0
- widening to generic `RuntimeDataBox` facade rows in the same wave
- using `FastLeafManifest` as a replacement for public ABI manifest truth
