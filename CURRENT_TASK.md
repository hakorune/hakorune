# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-06
Scope: repo root から current lane / next lane / restart read order に最短で戻るための薄い anchor。

## Purpose

- root から current lane と current front を最短で読む
- landed history や implementation detail は phase docs を正本にする
- `CURRENT_TASK.md` は pointer に徹し、ledger にはしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `git status -sb`
4. `tools/checks/dev_gate.sh quick`

## Order At A Glance

1. `phase-132x vm default backend decision` (landed)
2. `phase-133x micro kilo reopen selection` (landed)
3. `phase-134x nyash_kernel layer recut selection` (landed)
4. `phase-138x nyash_kernel semantic owner cutover` (landed)
5. `phase-139x array owner pilot` (landed)
6. `phase-140x map owner pilot` (landed)
7. `phase-141x string semantic boundary review` (landed)
8. `phase-142x array owner cutover implementation` (landed)
9. `phase-143x map owner cutover implementation` (landed)
10. `phase-144x string semantic owner follow-up` (landed)
11. `phase-145x compat quarantine shrink` (landed)
12. `phase-146x string semantic boundary tighten` (landed)
13. `phase-137x main kilo reopen selection` (paused after reopen proof)
14. `phase-147x semantic optimization contract selection` (landed)
15. `phase-148x borrowed text and sink contract freeze` (landed)
16. `phase-149x concat const-suffix vertical slice` (active)
17. `phase-kx vm-hako small reference interpreter recut` (parked after optimization)

## Current Front

- Active lane: `phase-149x concat const-suffix vertical slice`
- Active front: `const_suffix` route を `.hako owner -> MIR canonical reading -> Rust executor` で最初に通す
- Current blocker: current concrete helper `nyash.string.concat_hs` を semantic source of truth にせず、executor detail へ押し込むこと
- Exact focus:
  - `.hako` owner route `const_suffix`
  - canonical MIR reading `thaw.str + lit.str + str.concat2 + freeze.str`
  - current executor path `concat_const_suffix_fallback(...)`

## Successor Corridor

1. `phase-150x array string-store vertical slice`
2. `phase-137x main kilo reopen selection`
3. `phase-kx vm-hako small reference interpreter recut`

## Parked After Optimization

- `phase-kx vm-hako small reference interpreter recut`
  - keep `vm-hako` as reference/conformance only
  - do not promote to product/mainline
  - revisit after the optimization corridor, not before

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep
- `nyash_kernel`
  - keep `Rust host microkernel`, ABI thin facade, lifetime-sensitive hot leaf, and native accelerator leaves in Rust
  - move semantic ownership, collection owner policy, and route semantics toward `.hako`
  - do not turn compat quarantine into a permanent owner layer

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
5. `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
6. `docs/development/current/main/phases/phase-147x/README.md`

## Notes

- `phase-132x` landed:
  - remove `vm` from the default backend
  - keep explicit `vm` / `vm-hako` proof-debug callers alive
  - do not wait for full vm source retirement before resuming mainline work
- fixed perf reopen order remains:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is landed:
  - `kilo_micro_substring_concat`: parity locked
  - `kilo_micro_array_getset`: parity locked
  - `kilo_micro_indexof_line`: frozen faster than C
- `phase-134x` landed the refactor split:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- `phase-138x` is the next design corridor:
  - landed: final shape is `Rust host microkernel` + `.hako semantic kernel` + `native accelerators`
  - landed: `ABI facade` is thin keep
  - landed: `compat quarantine` is non-owner and shrink-only
  - landed: `Array owner` is the first cutover pilot
- `phase-139x` current seam:
  - landed: owner = `lang/src/runtime/collections/array_core_box.hako`
  - landed: substrate = `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - landed: ABI facade = `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - landed: compat/runtime forwarders = `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - landed: accelerators = `crates/nyash_kernel/src/plugin/array_handle_cache.rs`, `crates/nyash_kernel/src/plugin/array_string_slot.rs`
- `phase-140x` landed seam:
  - landed: owner = `lang/src/runtime/collections/map_core_box.hako`, `lang/src/runtime/collections/map_state_core_box.hako`
  - landed: substrate = `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`
  - landed: thin facade = `crates/nyash_kernel/src/plugin/map_aliases.rs`
  - landed: observer shim = `crates/nyash_kernel/src/plugin/map_substrate.rs`
  - landed: compat/runtime forwarding = `crates/nyash_kernel/src/plugin/map_runtime_facade.rs`
  - landed: accelerators = `crates/nyash_kernel/src/plugin/map_probe.rs`, `crates/nyash_kernel/src/plugin/map_slot_load.rs`, `crates/nyash_kernel/src/plugin/map_slot_store.rs`
- `phase-141x` landed seam:
  - semantic owner: `lang/src/runtime/kernel/string/README.md`, `lang/src/runtime/kernel/string/chain_policy.hako`, `lang/src/runtime/kernel/string/search.hako`
  - VM-facing wrapper: `lang/src/runtime/collections/string_core_box.hako`
  - thin facade: `crates/nyash_kernel/src/exports/string.rs`
  - Rust keep: `crates/nyash_kernel/src/exports/string_view.rs`, `crates/nyash_kernel/src/exports/string_helpers.rs`, `crates/nyash_kernel/src/exports/string_plan.rs`
  - quarantine: `crates/nyash_kernel/src/plugin/module_string_dispatch/**`
- `phase-142x` landed cutover:
  - `ArrayBox.{push,get,set,len/length/size,pop}` visible semantics now sit on `.hako` owner helpers
  - Rust array surface is split into compat aliases, runtime any-key shell, idx forwarding, substrate forwarding, and accelerators
- `phase-143x` landed cutover:
  - visible `MapBox.{set,get,has,len/length/size}` behavior now reads through `.hako` owner helpers
  - Rust map surface remains thin facade / observer shim / forwarding / raw leaves
- `phase-144x` landed follow-up:
  - `StringCoreBox.{size,indexOf,lastIndexOf,substring}` now reads through helperized wrapper paths
  - `lastIndexOf` now delegates to `.hako` search owner helper instead of wrapper-local search
  - `indexOf(search, fromIndex)` now delegates to `.hako` search owner via `StringSearchKernelBox.find_index_from(...)`
- `phase-145x` landed target:
  - host-side glue:
    - `crates/nyash_kernel/src/hako_forward_bridge.rs`
    - `crates/nyash_kernel/src/plugin/future.rs`
    - `crates/nyash_kernel/src/plugin/invoke_core.rs`
  - quarantine:
    - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`
  - goal:
    - host service contract と compat quarantine を source 上で取り違えない状態にする
- `phase-146x` landed target:
  - string semantic owner / wrapper / native substrate の stop-line を source 上で tighten
  - target files:
    - `lang/src/runtime/kernel/string/README.md`
    - `lang/src/runtime/collections/string_core_box.hako`
    - `crates/nyash_kernel/src/exports/string_view.rs`
    - `crates/nyash_kernel/src/exports/string_plan.rs`
    - `crates/nyash_kernel/src/exports/string_helpers.rs`
- `phase-147x` landed design lock:
  - authority order is `.hako owner / policy -> MIR canonical contract -> Rust executor / accelerator -> LLVM generic optimization / codegen`
  - `BorrowedText` / `TextSink` may exist only as Rust internal executor protocol
  - first canonical-op candidates:
    - `lit.str`
    - `str.concat2`
    - `store.array.str`
    - `store.map.value`
  - first vertical slice stays `concat const-suffix`
- `phase-148x` landed contract freeze:
  - owner route `const_suffix` now freezes the canonical MIR reading `thaw.str + lit.str + str.concat2 + freeze.str`
  - owner route `ArrayStoreString` now freezes the canonical MIR reading `store.array.str`
  - owner route `MapStoreAny` now freezes the canonical MIR reading `store.map.value`
  - current concrete executor paths remain `nyash.string.concat_hs`, `nyash.array.set_his`, and `nyash.map.slot_store_hhh`
- `phase-137x` current baseline and first reopen wins:
  - baseline: `kilo_kernel_small_hk`: `c_ms=81 / ny_aot_ms=1529`
  - after string const-path branch collapse: `c_ms=82 / ny_aot_ms=775`
  - after const-handle cache follow-up: `c_ms=84 / ny_aot_ms=731`
  - after const empty-flag cache: `c_ms=81 / ny_aot_ms=723`
  - after shared text-based const-handle helper: `c_ms=80 / ny_aot_ms=903`
  - after single-closure const suffix fast path: `c_ms=83 / ny_aot_ms=820`
  - latest sampled whole-kilo reread: `c_ms=82 / ny_aot_ms=775`
  - exact micro `kilo_micro_concat_const_suffix`: `c_ms=2 / ny_aot_ms=85`
  - exact micro `kilo_micro_array_string_store`: `c_ms=9 / ny_aot_ms=217`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
- latest bundle read:
  - string trace contract unchanged for `concat_hs` / `insert_hsi`
  - `20260406-010145` bundle still has `concat_const_suffix_fallback` as the top explicit hot symbol (`10.80%`)
  - `array_string_store_handle_at` is still second (`6.33%`) after string const-path
- `phase-137x` is paused, not canceled:
  - perf reopen stays the first implementation consumer after contract freeze
- first exact slices:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
