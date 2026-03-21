---
Status: SSOT
Decision: accepted
Date: 2026-03-17
Scope: `ArrayBox` / `MapBox` の current owner truth と、`0rust` に向けた ring/owner cutover 順序を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/ring1-core-provider-scope-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - lang/src/runtime/collections/README.md
---

# Array/Map Owner And Ring Cutover (SSOT)

## 0. Conclusion

- `ArrayBox` / `MapBox` は `ring0` ではなく `ring1` の責務である。
- current active direction is to move `ArrayBox` / `MapBox` user-visible semantics into `.hako` ring1 collection core.
- current status is done-enough owner shift, not end-state completion; raw substrate still remains Rust-owned.
- `RuntimeDataBox` is not the target owner for collection semantics; it stays protocol / facade only.
- `0rust` の target は `ring0` へ移すことではない。
  - `ring0` は OS API abstraction に限定する。
  - collection mainline owner は `.hako` ring1 collection layer へ寄せる。
- Rust の `ArrayBox` / `MapBox` / kernel plugin / builtin residue は、daily owner から外したあとに raw substrate / compat/archive keep へ後退させる。

## 1. Ring Lock

1. `array` / `map` は `ring1` domain である。
2. `ring0` は `mem/io/time/fs/log/thread` の OS-facing API に限定し、collection semantics を持たない。
3. `.hako` 側で owner を増やす場合も、置き場は `ring1` collection/runtime layer であり `ring0` ではない。

## 2. Current Truth

### 2.1 Runtime provider lane

- runtime provider の accepted wiring は Rust 側 `ring1` provider で動いている。
- exact owners:
  - `src/providers/ring1/array/mod.rs`
  - `src/providers/ring1/map/mod.rs`
  - `src/runtime/provider_lock/{array,map}.rs`
  - `src/runtime/plugin_host.rs`
  - `src/runtime/core_services.rs`
- meaning:
  - `Ring1ArrayService` / `Ring1MapService` が `provider_lock` に登録され、runtime lane の collection service SSOT を持つ。
  - ただし service implementation 自体はまだ Rust `ArrayBox` / `MapBox` を直接 downcast している。
  - latest provider-lane thin slice:
    - `src/providers/ring1/array/mod.rs` now keeps `type-gate` / `index boxing` behind owner-local helpers and fixes invalid-type contract in unit tests without changing service semantics
    - `src/providers/ring1/map/mod.rs` now keeps `type-gate` / `key boxing` / `size-bool extraction` behind owner-local helpers and fixes invalid-type contract in unit tests without changing service semantics

### 2.2 AOT/LLVM and RuntimeData lane

- AOT/LLVM で visible な collection ABI は still-Rust kernel/plugin owner である。
- exact owners:
  - `crates/nyash_kernel/src/exports/birth.rs`
  - `crates/nyash_kernel/src/plugin/array.rs`
  - `crates/nyash_kernel/src/plugin/map.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data.rs`
- meaning:
  - `nyash.array.birth_h` / `nyash.map.birth_h` は still-Rust concrete box creation を行う。
  - `nyash.array.*` / `nyash.map.*` / `nyash.runtime_data.*` の mainline execution も still-Rust plugin owner に依存している。

### 2.3 `.hako` collection layer

- `.hako` 側は current mainline では still-thin だが、next owner-growth frontier として扱う。
- exact owners:
  - `lang/src/runtime/collections/**`
  - `lang/src/vm/boxes/abi_adapter_registry.hako`
  - meaning:
    - `.hako` 側は ABI vocabulary を thin に束ねるだけで止めず、ring1 collection semantics owner へ成長させる。
  - latest visible-owner slice:
    - `lang/src/runtime/collections/map_core_box.hako` now owns adapter-on `MapBox.{set,get,has,size/len/length}` orchestration plus size/state helpers consumed by `lang/src/vm/boxes/mir_call_v1_handler.hako`
    - `lang/src/vm/boxes/mir_call_v1_handler.hako` no longer carries inline `MapBox.set` fallback logic; handler-side MapBox routing now goes through `MapCoreBox.try_handle(...)`
    - `crates/nyash_kernel/src/plugin/map_slot_load.rs` / `map_slot_store.rs` / `map_probe.rs` now hold the raw Rust `MapBox` load/store/probe substrate, while legacy `nyash.map.{get,set,has}_*` exports remain thin wrappers
    - `lang/src/runtime/collections/array_core_box.hako` now owns adapter-on `ArrayBox.{set,get,push,len/length/size}` orchestration plus len/state helpers consumed by the same handler
    - `lang/src/runtime/collections/runtime_data_core_box.hako` now owns narrow `RuntimeDataBox.{get,set,has,push}` method dispatch plus the same extern routes consumed by `lang/src/vm/boxes/mir_call_v1_handler.hako`
    - `lang/src/runtime/collections/string_core_box.hako` now owns adapter-on `StringBox.length/len/size` orchestration plus the `nyash.string.len_h` thin extern route consumed by the same handler
  - current proof lock:
    - `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` pins the source-contract (`registry/handler/core-box`) wiring for the current `.hako` collection owner slice
    - `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` pins the standalone AOT/runtime-data e2e fixture
  - next owner-growing slice:
    - `ArrayCoreBox` is the first active owner-growth slice
    - `MapCoreBox` follows after `ArrayCoreBox`
    - `RuntimeDataCoreBox` is cleanup-only after `array/map`; do not grow it into a collection owner
  - caution:
    - `verify_v1_inline_file()` / `HAKO_VERIFY_PRIMARY=hakovm` still routes through Rust `hv1_inline::run_json_v1_inline(...)`; those canaries are not `.hako` `MirCallV1HandlerBox` owner proofs
  - current slices are not yet the final collection semantics owner, but they are the intended ring1 owner frontier for `array/map`.

### 2.4 Legacy residue

- legacy builtin residue もまだ残る。
- exact owners:
  - `src/box_factory/builtin_impls/array_box.rs`
  - `src/box_factory/builtin_impls/map_box.rs`
- rule:
  - これらは daily owner に戻さず、compat/archive residue としてのみ扱う。

## 3. 0rust Target

### 3.1 Final direction

- target daily shape:
  - `.hako ring1 collection owner`
  - thin ABI/boundary keep
  - explicit compat/archive keeps
- non-target:
  - `ring0` へ collection owner を移すこと
  - Rust provider/kernel plugin を final owner として残すこと

### 3.2 What “move to .hako” means here

- `.hako` 側に thin wrapper を増やすだけでは不十分。
- move の意味は次の順で固定する。
  1. visible/mainline caller ownership を `.hako` ring1 collection layer に寄せる
  2. `ArrayBox` / `MapBox` の method-shaped semantics を `.hako` ring1 collection core が持つ
  3. Rust provider/kernel plugin は raw substrate or thin ABI / compat keep に後退させる
  4. `RuntimeDataBox` は protocol / facade に固定し、array/map semantics を吸い込まない
  5. daily path が Rust collection semantics を直接 owner しなくなってから、archive/preservation-first retire を検討する

### 3.2b Owner litmus test

- `.hako` に置く:
  - `get/set/push/has/len/length/size`
  - bounds policy
  - index/key normalization
  - visible fallback / error contract
  - smoke/docs/tests がそのまま語る collection semantics
- Rust に残す:
  - `encode/decode`
  - cache / downcast
  - slot load/store
  - growth / probe / rehash
  - object layout / allocator / GC barrier
  - ABI marshal

Short rule:
- method-shaped names belong to `.hako`
- substrate-shaped names belong to Rust

### 3.2c First cutover order

- first:
  - `ArrayCoreBox` / `array_state_core_box.hako`
- second:
  - Rust `array` helpers are renamed/reduced behind raw substrate verbs
- third:
  - `MapCoreBox`
- fourth:
  - `RuntimeDataCoreBox` cleanup as protocol / facade only
- fifth:
  - deepen the boundary below the remaining method-shaped Rust exports used by `.hako` owners:
    - `nyash.array.push_hh`
    - `nyash.map.size_h`
  - landed:
    - daily array observer route now uses `nyash.array.slot_len_h`
    - `nyash.array.len_h` is compat-only
- sixth:
  - only then reopen raw substrate perf
- details for raw naming and demotion live in:
  - `docs/development/current/main/design/collection-raw-substrate-contract-ssot.md`

### 3.3 Promotion Trigger: defer から dedicated kernel module へ移すタイミング

- `defer` は「今は collections ring1 の wrapper だけで contract を保てる」状態を意味する。
- dedicated `.hako` kernel module へ昇格するタイミングは calendar ではなく trigger-based で決める。
- promote してよい条件は、少なくとも次のどれかが true になった時だけ。
  1. wrapper-only では policy / normalization / bounds / birth-materialize の責務を薄く保てなくなった
  2. 同じ collection 形が複数 caller に広がり、ring1 wrapper が transport-only ではなく policy owner になってしまう
  3. dedicated fixture + smoke row が必要になり、collections ring1 の既存 owner では acceptance case を薄く表せなくなった
  4. `.hako` 側で owner-local の契約差分が必要になり、ring1 wrapper が単なる forwarder 以上の責務を持つようになった
- 逆に、thin wrapper のまま contract を保てる限りは defer のまま維持する。
- historical defer rule:
  - `array` was kept in `lang/src/runtime/collections/array_core_box.hako` until this trigger clearly fired
  - no dedicated `lang/src/runtime/kernel/array/` was required
- current decision:
  - `array` / `map` are promoted within `lang/src/runtime/collections/` ring1 collection core now.
  - this is not a `ring0` move and not a new `lang/src/runtime/kernel/{array,map}/` requirement.
  - `runtime_data` does not share that promotion; it stays facade-only.

## 4. Fixed Cutover Order

1. current owner truth を docs で固定する
   - `ring1 accepted` と `still-Rust mainline` を同時に読めるようにする
2. `array` の visible/mainline owner を `.hako ring1` collection core へ寄せる
   - `ArrayCoreBox` / `array_state_core_box.hako` が user-visible semantics を持ち、Rust `array` plugin は raw substrate へ後退する
3. `map` の visible/mainline owner を `.hako ring1` collection core へ寄せる
   - `MapCoreBox` が user-visible semantics を持ち、Rust `map` plugin は raw substrate へ後退する
   - current vm-hako-visible stateful methods now live in `lang/src/runtime/collections/map_state_core_box.hako`, not inline in `mir_vm_s0_boxcall_builtin.hako`
4. `runtime_data` を protocol / facade に retarget する
   - `RuntimeDataCoreBox` は route/dynamic dispatch owner に留め、array/map semantics owner にはしない
   - current first slice: `crates/nyash_kernel/src/plugin/runtime_data.rs` is already a dispatch shell over `runtime_data_array_route.rs` / `runtime_data_map_route.rs`
5. Rust concrete births/plugins/builtin residue を raw substrate / compat/archive keep に限定する
6. preservation-first rule を満たした後だけ delete/retire を再判定する

## 5. Non-goals

1. `ring0` に collection semantics を持ち込むこと
2. `ring1 accepted` を理由に current Rust owner を見えなくすること
3. backend-zero の current blocker を runtime collection cutover で上書きすること
4. `RuntimeDataBox` を collection semantics owner へ育てること
