---
Status: SSOT
Decision: accepted
Date: 2026-03-17
Scope: `ArrayBox` / `MapBox` の current owner truth と、`0rust` に向けた ring/owner cutover 順序を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/ring1-core-provider-scope-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - lang/src/runtime/collections/README.md
---

# Array/Map Owner And Ring Cutover (SSOT)

## 0. Conclusion

- `ArrayBox` / `MapBox` は `ring0` ではなく `ring1` の責務である。
- ただし current mainline owner はまだ `.hako` 単独ではなく、runtime lane と AOT/LLVM lane で Rust owner が残っている。
- `0rust` の target は `ring0` へ移すことではない。
  - `ring0` は OS API abstraction に限定する。
  - collection mainline owner は `.hako` ring1 collection layer へ寄せる。
- Rust の `ArrayBox` / `MapBox` / kernel plugin / builtin residue は、daily owner から外したあとに compat/archive keep へ後退させる。

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

- `.hako` 側は current mainline では thin wrapper / adapter であり、まだ storage/primitive owner ではない。
- exact owners:
  - `lang/src/runtime/collections/**`
  - `lang/src/vm/boxes/abi_adapter_registry.hako`
- meaning:
  - `.hako` 側は ABI vocabulary を thin に束ねる。
  - latest visible-owner slice:
    - `lang/src/runtime/collections/map_core_box.hako` now owns adapter-on `MapBox` size/state helpers consumed by `lang/src/vm/boxes/mir_call_v1_handler.hako`
  - collection semantics の最終 owner ではなく、現時点では Rust owner への adapter surface である。

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
  2. Rust provider/kernel plugin は thin ABI or compat keep に後退させる
  3. daily path が Rust collection semantics を直接 owner しなくなってから、archive/preservation-first retire を検討する

## 4. Fixed Cutover Order

1. current owner truth を docs で固定する
   - `ring1 accepted` と `still-Rust mainline` を同時に読めるようにする
2. runtime provider の mainline owner を `.hako ring1` 側へ寄せる
   - `provider_lock` / `plugin_host` から見た visible owner を `.hako` collection layer に近づける
3. AOT/LLVM collection path を `.hako ring1`-compatible boundary へ寄せる
   - `nyash.array.*` / `nyash.map.*` / `nyash.runtime_data.*` の daily dependency を thin keep に縮める
4. Rust concrete births/plugins/builtin residue を compat/archive keep に限定する
5. preservation-first rule を満たした後だけ delete/retire を再判定する

## 5. Non-goals

1. `ring0` に collection semantics を持ち込むこと
2. `ring1 accepted` を理由に current Rust owner を見えなくすること
3. backend-zero の current blocker を runtime collection cutover で上書きすること
