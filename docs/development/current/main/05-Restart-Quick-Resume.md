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

- lane: `phase-160x capability-family inventory`
- current front: hot Rust helpers を future capability family ごとに棚卸しし、perf front がどの seam に属するかを lock する
- blocker: seam が曖昧なまま最適化を続けると、後で capability family 化するときに hot path を再整理する二度手間が出る
- landed:
  - `phase-140x map owner pilot`
  - `phase-139x array owner pilot`
  - `phase-138x nyash_kernel semantic owner cutover`
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`
- active next:
  - `phase-137x main kilo reopen selection`
  - `phase-kx vm-hako small reference interpreter recut`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
4. `docs/development/current/main/phases/phase-152x/README.md`
5. `docs/development/current/main/phases/phase-154x/README.md`
6. `docs/development/current/main/phases/phase-160x/README.md`

## Decision Lock

- llvmlite retreat order is fixed:
  1. runner object emit cutover
  2. `ny_mir_builder` harness drop
  3. llvmlite keep/archive lock
  4. perf reopen
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
- `phase-150x` landed:
  - `ArrayStoreString` current lowering now reads as ABI/executor detail under canonical `store.array.str`
- `phase-151x` landed lock:
  - `const_suffix`
  - `ArrayStoreString`
  - `MapStoreAny`
  must all be readable as:
  - `.hako owner`
  - `MIR canonical reading`
  - `current concrete lowering`
  - `Rust executor`
- `phase-154x` landed current-facing wording slice:
  - `docs/guides/exe-first-wsl.md` now reads `ny-llvmc` as the daily EXE-first owner route
  - `docs/guides/selfhost-pilot.md` no longer presents llvmlite as a daily selfhost/product requirement
  - `docs/reference/environment-variables.md` labels `NYASH_LLVM_USE_HARNESS=1` examples as explicit keep-lane
- `phase-155x` landed:
  - perf front is now read canonical reading first
- `phase-156x` landed:
  - first counter surface exists for `store.array.str` and `const_suffix`
  - first exact probe disproved cache-churn on `kilo_micro_array_string_store`
- `phase-157x` landed:
  - `perf-observe` feature controls compile-in / compile-out
  - `NYASH_PERF_COUNTERS=1` is runtime gate only inside feature-on build
  - default release stays zero-cost
- `phase-158x` landed:
  - exact counter backend is TLS-first
  - summary remains stderr sink based
  - current-thread flush is the active exact counter truth
- `phase-159x` landed:
  - exact counter lane と trace lane の split place は source-backed
  - `perf-trace` is keep-only placeholder, not the current blocker
- latest bundle anchor:
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/summary.txt`
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/asm/perf_report.txt`
- current perf reopen truth:
  - `kilo_kernel_small_hk`: latest reread `ny_aot_ms=741`
  - `kilo_micro_concat_const_suffix`: `ny_aot_ms=84`
  - `kilo_micro_array_string_store`: `ny_aot_ms=181`
- immediate capability map order:
  1. `phase-160x capability-family inventory`
  2. `phase-161x hot-path capability seam freeze`
  3. `phase-137x main kilo reopen selection`

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
- perf consumer is paused behind llvmlite retreat:
  - `phase-137x` resumes only after object emit reads `ny-llvmc` first
- `phase-144x` landed:
  - `StringCoreBox.{size,indexOf,lastIndexOf,substring}` now reads through helperized wrapper paths
  - `indexOf(search, fromIndex)` delegates to `StringSearchKernelBox.find_index_from(...)`
  - `lastIndexOf(needle)` delegates to `StringSearchKernelBox.last_index(...)`
  - no lifetime substrate move was made
