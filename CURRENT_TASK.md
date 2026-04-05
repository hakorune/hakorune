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
3. `phase-134x nyash_kernel layer recut selection` (active)
4. `phase-135x string export split` (queued)
5. `phase-136x map substrate thin-alias recut` (queued)
6. `phase-137x main kilo reopen selection` (queued)

## Current Front

- Active lane: `phase-134x nyash_kernel layer recut selection`
- Active front: `exports/string.rs` split inventory + `plugin/map_substrate.rs` thin-alias inventory
- Current blocker: `.hako` 移植を先に始めない。先に `ABI / glue / substrate` を Rust 側で切り分ける
- Exact focus: `keep / thin keep / compat glue / substrate candidate` の4層で `nyash_kernel` を再分類し、最初の source slice を `string export split` と `map substrate thin-alias` に固定する

## Successor Corridor

1. `phase-135x string export split`
2. `phase-136x map substrate thin-alias recut`
3. `phase-137x main kilo reopen selection`
4. `phase-kx vm-hako small reference interpreter recut`

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
  - keep host/kernel keep, ABI keep-thin, lifetime-sensitive hot leaf in Rust
  - do not broaden `.hako` migration before `ABI / glue / substrate` separation is source-backed

## Read Next

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-134x/README.md`

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
- `phase-134x` is a design/source corridor before the next optimization cut
- first exact slices:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
