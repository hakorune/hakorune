# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-05
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
10. `phase-144x string semantic owner follow-up` (active)
11. `phase-137x main kilo reopen selection` (successor after owner cutover)
12. `phase-kx vm-hako small reference interpreter recut` (parked after optimization)

## Current Front

- Active lane: `phase-144x string semantic owner follow-up`
- Active front: String semantic owner を `.hako` 側に保ったまま、`StringCoreBox` の visible wrapper residue を owner helper / string-kernel delegation に押し戻す
- Current blocker: String seam は landed したが、`StringCoreBox` には `size alias / indexOf / substring / lastIndexOf` の inline visible branch が残っている
- Exact focus: `StringBox.size/indexOf/substring` を `.hako` helper / string-kernel delegation に寄せ、`lastIndexOf` を含む残り wrapper residue を narrow shell に閉じ込める。Rust `string_view.rs` / `string_helpers.rs` / `string_plan.rs` は動かさない

## Successor Corridor

1. `phase-137x main kilo reopen selection`
2. `phase-kx vm-hako small reference interpreter recut`

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
3. `docs/development/current/main/phases/phase-144x/README.md`
4. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`

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
- `phase-144x` current follow-up:
  - `String` is not a full owner move
  - current work is wrapper-level semantic-owner enforcement, not Rust-zero
- `phase-137x` is intentionally delayed:
  - perf reopen waits until owner implementation is clean enough not to create rework
- first exact slices:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
