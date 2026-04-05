---
Status: Active
Date: 2026-04-06
Scope: current mainline / next lane / parked corridor の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Workstream Map

## Current Lane

| Item | State |
| --- | --- |
| Now | `phase-159x observe trace split` |
| Front | exact counter と heavy trace を分け、default release / observe release / trace debug の役割を混ぜない |
| Blocker | exact counter と future trace の plane がまだ同じ observe lane に見えること |
| Next | `phase-137x main kilo reopen selection` |
| After Next | `phase-kx vm-hako small reference interpreter recut` |

## Current Read

- `phase-132x` landed:
  - `--backend` default is now `mir`
  - explicit `vm` / `vm-hako` proof-debug lanes stay frozen keep
- `phase-133x` landed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3`
- the structural cut of `crates/nyash_kernel` is landed:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- `phase-138x` landed the final owner graph:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade` thin keep
  - `compat quarantine` non-owner
- current implementation corridor:
  - `phase-142x` = landed Array owner cutover implementation
  - `phase-143x` = landed Map owner cutover implementation
  - `phase-144x` = landed String semantic owner follow-up
  - `phase-145x` = landed compat quarantine shrink
  - `phase-146x` = landed string semantic boundary tighten
- `phase-147x` landed lock:
  - `.hako` keeps owner policy and route vocabulary
  - MIR keeps canonical optimization contract
  - Rust keeps executor / accelerator only
  - LLVM stays generic
- `phase-148x` landed freeze:
  - `const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`
  - `ArrayStoreString -> store.array.str`
  - `MapStoreAny -> store.map.value`
- `phase-149x` landed first consumer:
  - `const_suffix` route is now shaped as executor detail under the canonical contract
- `phase-150x` landed second consumer:
  - `ArrayStoreString` route is now shaped as ABI/executor detail under canonical `store.array.str`
- `phase-151x` landed visibility lock:
  - canonical MIR readings are now visible against current concrete lowering
- next fixed corridor:
  1. `phase-152x llvmlite object emit cutover`
  2. `phase-153x ny_mir_builder harness drop`
  3. `phase-154x llvmlite archive lock`
  4. `phase-137x main kilo reopen selection`
- `phase-154x` landed current-facing wording slice:
  - `docs/guides/exe-first-wsl.md`
  - `docs/guides/selfhost-pilot.md`
  - `docs/reference/environment-variables.md`
  now treat llvmlite as explicit keep-lane only
- `phase-155x` freezes canonical perf front:
  - `store.array.str` first
  - `const_suffix` / `thaw.str + lit.str + str.concat2 + freeze.str` second
  - latest bundle anchor = `20260406-024104`
- `phase-156x` landed:
  - route-tagged counters exist for `store.array.str` and `const_suffix`
  - first exact probe on `store.array.str` showed `cache_hit=800000`, `cache_miss_epoch=0`
- `phase-157x` landed:
  - observer is feature-gated and out-of-band
  - default build compiles observer out
  - `perf-observe` build + `NYASH_PERF_COUNTERS=1` is the canonical observe lane
- `phase-158x` current:
  - exact counter backend is TLS-first
  - stderr summary stays the current sink
  - hot path should not pay shared atomic cost in the observe lane
- `phase-159x` current:
  - exact counter remains `perf-observe`
  - heavy trace is the next split target
  - trace/debug-only observer semantics must not contaminate the exact counter lane
- paused reopen truth:
  - baseline: `kilo_kernel_small_hk = 1529ms`
  - string const fast-path: `775ms`
  - const-handle cache follow-up: `731ms`
  - const empty-flag cache: `723ms`
  - shared text-based const-handle helper: `903ms`
  - single-closure const suffix fast path: `820ms`
  - latest sampled whole-kilo reread: `745ms`
  - first implementation consumer: `array string-store`
  - second implementation consumer: `concat const-suffix`
  - exact micro:
    - `kilo_micro_concat_const_suffix = 85ms`
    - `kilo_micro_array_string_store = 207ms`

## Successor Corridor

1. `phase-137x main kilo reopen selection`
2. `phase-kx vm-hako small reference interpreter recut`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep
- `nyash_kernel`
  - `Rust host microkernel` stays in Rust
  - `ABI facade` stays thin keep in Rust
  - lifetime-sensitive hot leaves and native accelerators stay in Rust until proven otherwise
  - semantic ownership moves toward `.hako`
- compat quarantine must not become a permanent owner layer
  - do not reopen broad perf tuning before optimization authority contract freeze, canonical-lowering visibility lock, counter proof, and observe feature split are complete

## Reference

- current lane docs:
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
  - `docs/development/current/main/phases/phase-148x/README.md`
  - `docs/development/current/main/phases/phase-150x/README.md`
  - `docs/development/current/main/phases/phase-151x/README.md`
  - `docs/development/current/main/design/canonical-lowering-visibility-ssot.md`
  - `docs/development/current/main/phases/phase-146x/README.md`
  - `docs/development/current/main/phases/phase-145x/README.md`
  - `docs/development/current/main/phases/phase-141x/README.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
