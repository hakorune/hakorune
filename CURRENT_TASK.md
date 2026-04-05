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
5. `phase-139x array owner pilot` (active)
6. `phase-140x map owner pilot` (next)
7. `phase-141x string semantic boundary review` (next)
8. `phase-137x main kilo reopen selection` (successor after architecture)
9. `phase-kx vm-hako small reference interpreter recut` (parked after optimization)

## Current Front

- Active lane: `phase-139x array owner pilot`
- Active front: `ArrayCoreBox` / `ArrayStateCoreBox` を visible semantics owner として固定し、Rust を `ABI facade` + `raw substrate` + `native accelerators` に保つ cutover seam を詰める
- Current blocker: final owner graph は fixed。次は `Array owner` pilot で、`.hako` owner が持つ範囲と Rust 側 compat/runtime forwarding の shrink line を source-backed に決める
- Exact focus: `ArrayBox.{get,set,push,len,length,size}` の policy/fallback/state は `.hako` owner に、`nyash.array.slot_*` と cache/fast-path substrate は Rust に残す stop-line を current phase docs に落とす

## Successor Corridor

1. `phase-139x array owner pilot`
2. `phase-140x map owner pilot`
3. `phase-141x string semantic boundary review`
4. `phase-137x main kilo reopen selection`
5. `phase-kx vm-hako small reference interpreter recut`

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
3. `docs/development/current/main/phases/phase-139x/README.md`
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
  - owner: `lang/src/runtime/collections/array_core_box.hako`
  - substrate: `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - ABI facade: `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - compat/runtime forwarders: `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - accelerators: `crates/nyash_kernel/src/plugin/array_handle_cache.rs`, `crates/nyash_kernel/src/plugin/array_string_slot.rs`
- `phase-137x` is not cancelled:
  - it remains the perf successor after semantic ownership is fixed
- first exact slices:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
