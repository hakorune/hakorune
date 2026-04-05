---
Status: Active
Date: 2026-04-06
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane: `phase-150x array string-store vertical slice`
- current front: `ArrayStoreString` route を `.hako owner -> MIR canonical reading -> Rust executor` の第2 consumer として通す
- blocker: current concrete symbol `nyash.array.set_his` を authority ではなく ABI/executor detail に押し込むこと
- landed:
  - `phase-140x map owner pilot`
  - `phase-139x array owner pilot`
  - `phase-138x nyash_kernel semantic owner cutover`
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`
- active next:
  - `phase-151x canonical lowering visibility lock`
  - `phase-137x main kilo reopen selection`
  - `phase-kx vm-hako small reference interpreter recut`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
4. `docs/development/current/main/phases/phase-150x/README.md`
5. `docs/development/current/main/phases/phase-137x/README.md`
6. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`

## Decision Lock

- fixed perf order remains:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is closed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3`
- `phase-134x` landed the split:
  - `keep / thin keep / compat glue / substrate candidate`
- `phase-138x` landed the final owner model:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade`
  - `compat quarantine`
- `phase-139x` landed the first pilot:
  - `ArrayCoreBox` / `ArrayStateCoreBox` hold visible semantics
  - `RawArrayCoreBox` / `PtrCoreBox` stay substrate
  - Rust `array_substrate.rs` stays thin ABI facade
  - Rust `array_runtime_facade.rs` stays compat/runtime forwarding
  - Rust cache/fast-path leaves stay native accelerators
- `phase-142x` landed:
  - visible `ArrayBox.{push,get,set,len/length/size,pop}` behavior now reads through `.hako` owner helpers
  - `array_handle_cache.rs` / `array_string_slot.rs` remain Rust accelerators
  - `array_substrate.rs` stays thin and array forwarding is split into runtime-any / idx facade / substrate shells
- `phase-140x` landed the second pilot:
  - `MapCoreBox` / `MapStateCoreBox` hold visible semantics
  - `RawMapCoreBox` stays substrate
  - Rust `map_aliases.rs` stays thin facade
  - Rust `map_runtime_facade.rs` stays compat/runtime forwarding
  - Rust `map_probe.rs` / `map_slot_load.rs` / `map_slot_store.rs` stay native/raw leaves
- `phase-143x` landed:
  - visible `MapBox.{set,get,has,len/length/size}` behavior now reads through `.hako` owner helpers
  - Rust map surface stays thin facade / observer shim / forwarding / accelerators
- `phase-141x` landed the final boundary review:
  - `string.rs` stays thin ABI facade
  - `string_view.rs` / `string_helpers.rs` / `string_plan.rs` stay Rust lifetime/native substrate
  - `.hako` semantic owner lives under `runtime/kernel/string/**`
  - `string_core_box.hako` is the VM-facing runtime wrapper
  - `module_string_dispatch/**` stays quarantine, not owner
- `phase-145x` landed:
  - host-side glue:
    - `crates/nyash_kernel/src/hako_forward_bridge.rs`
    - `crates/nyash_kernel/src/plugin/future.rs`
    - `crates/nyash_kernel/src/plugin/invoke_core.rs`
  - quarantine:
    - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`
- `phase-146x` landed:
  - tighten string semantic owner / wrapper / native substrate wording and helper boundaries
  - close the wrapper-vs-owner naming gap in `StringCoreBox`
- `phase-147x` landed lock:
  - `.hako` keeps route / retained-form / boundary authority
  - MIR keeps canonical optimization contract
  - Rust keeps executor / accelerator only
  - LLVM keeps generic optimization / codegen only
- `phase-148x` landed freeze:
  - `const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`
  - `ArrayStoreString -> store.array.str`
  - `MapStoreAny -> store.map.value`
- `phase-149x` landed:
  - `const_suffix` current lowering now reads as canonical executor detail

## First Design Slices

- `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
- `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
- `lang/src/runtime/collections/map_core_box.hako`
- `lang/src/runtime/collections/map_state_core_box.hako`
- `crates/nyash_kernel/src/plugin/map_runtime_facade.rs`
- `crates/nyash_kernel/src/plugin/map_aliases.rs`

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
tools/checks/dev_gate.sh quick
git diff --check
```
- paused perf truth:
  - baseline: `kilo_kernel_small_hk`: `c_ms=81 / ny_aot_ms=1529`
  - string const fast-path: `c_ms=82 / ny_aot_ms=775`
  - const-handle cache follow-up: `c_ms=84 / ny_aot_ms=731`
  - const empty-flag cache: `c_ms=81 / ny_aot_ms=723`
  - shared text-based const-handle helper: `c_ms=80 / ny_aot_ms=903`
  - single-closure const suffix fast path: `c_ms=83 / ny_aot_ms=820`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
- next contract-first slice:
  - first vertical slice: `concat const-suffix`
  - second vertical slice: `array string-store`
  - then canonical lowering visibility lock
  - perf consumer stays blocked until that lands
- `phase-144x` landed:
  - `StringCoreBox.{size,indexOf,lastIndexOf,substring}` now reads through helperized wrapper paths
  - `indexOf(search, fromIndex)` delegates to `StringSearchKernelBox.find_index_from(...)`
  - `lastIndexOf(needle)` delegates to `StringSearchKernelBox.last_index(...)`
  - no lifetime substrate move was made
