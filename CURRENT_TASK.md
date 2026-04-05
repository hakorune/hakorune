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
4. `phase-138x nyash_kernel semantic owner cutover` (active)
5. `phase-139x array owner pilot` (next)
6. `phase-137x main kilo reopen selection` (successor after architecture)
7. `phase-kx vm-hako small reference interpreter recut` (parked after optimization)

## Current Front

- Active lane: `phase-138x nyash_kernel semantic owner cutover`
- Active front: `Rust host microkernel` / `.hako semantic kernel` / `native accelerators` の最終 owner model を固定し、`Array owner` pilot を次の実装 lane にする
- Current blocker: `nyash_kernel` の4層 split は landed したが、final architecture はまだ中間形のまま。`main kilo` を reopen する前に semantic ownership の stop-line を固定する
- Exact focus: Rust から semantic ownership を外し、`host/kernel keep` と `native accelerator` は Rust に、`collection / route / adapter semantics` は `.hako` に寄せる最終形を current SSOT に落とす

## Successor Corridor

1. `phase-138x nyash_kernel semantic owner cutover`
2. `phase-139x array owner pilot`
3. `phase-140x map owner pilot`
4. `phase-141x string semantic boundary review`
5. `phase-137x main kilo reopen selection`
6. `phase-kx vm-hako small reference interpreter recut`

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
3. `docs/development/current/main/phases/phase-138x/README.md`
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
  - final shape is `Rust host microkernel` + `.hako semantic kernel` + `native accelerators`
  - `ABI facade` is thin keep
  - `compat quarantine` is non-owner and shrink-only
- `phase-137x` is not cancelled:
  - it remains the perf successor after semantic ownership is fixed
- first exact slices:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
