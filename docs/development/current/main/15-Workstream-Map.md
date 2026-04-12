---
Status: Active
Date: 2026-04-12
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
| Now | `phase-163x primitive and user-box fast path` |
| Front | `layer roadmap regroup landed -> semantic simplification bundle partial through phase196x -> next narrow slice is lane B0 generic memory Store/Load docs-facts` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `generic memory Store/Load observer-owner contract is not fixed yet, so lane B cannot widen safely` |
| Next | `semantic simplification bundle lane B0 (generic memory Store/Load docs-facts phase)` |
| After Next | `lane B1 dead Load pruning -> lane B2 overwritten Store pruning` |

## Current Read

- design owners:
  - implementation lane: `docs/development/current/main/phases/phase-163x/README.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
  - roadmap SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- landed structure anchors:
  - `phase-165x` / `phase-166x`: semantic refresh and generic relation ownership are fixed
  - `phase-167x` / `phase-168x`: user-box method determinism and exact-route refresh are fixed
  - `phase-169x` through `phase-180x`: string guardrail, `StringKernelPlan`, publication slices, and seam cleanup are landed; only the final emitted-MIR return-carrier cleanup stays parked on `phase-137x`
  - `phase-176x` / `phase-177x` / `phase-181x` / `phase-182x` / `phase-183x` / `phase-184x` / `phase-185x` / `phase-186x` / `phase-187x` / `phase-188x` / `phase-189x` / `phase-190x` / `phase-191x` / `phase-192x` / `phase-196x`: semantic simplification bundle is landed through DCE lane A2
  - `phase-178x` / `phase-193x` / `phase-194x`: BoxShape splits are landed and stay behavior-preserving
  - `phase-195x` / `phase-197x`: roadmap regroup and pointer hygiene are landed; current docs agree on lane B0
- immediate sequence:
  - lane B0 generic memory `Store` / `Load` docs-facts phase
  - lane B1 dead `Load` pruning
  - lane B2 overwritten `Store` pruning
- stop-lines:
  - keep lane B separate from `Debug` / terminator observer cleanup
  - keep lane B separate from `generic placement / effect`
  - keep parked `phase-96x` out of the active optimization lane
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
  4. `phase-137x main kilo reopen selection` (historical reopen lane; now sibling string guardrail)
- `phase-154x` landed current-facing wording slice:
  - `docs/guides/exe-first-wsl.md`
  - `docs/guides/selfhost-pilot.md`
  - `docs/reference/environment-variables.md`
  now treat llvmlite as explicit keep-lane only
- `phase-155x` freezes canonical perf front:
  - `concat_birth` first
  - `const_suffix` / `thaw.str + lit.str + str.concat2 + freeze.str` second
  - latest bundle anchor = `20260406-024104`
- current local execution rule:
  - structure before benchmark-driven widening
  - source-lifetime contract before helper-local transport trim
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
- `phase-159x` landed:
  - exact counter remains `perf-observe`
  - trace/debug-only lane is `perf-trace`
  - trace lane is parked placeholder and no longer blocks perf reopen
- `phase-164x` landed:
  - repo-wide `cargo fmt --check` drift cleanup is complete
  - the cleanup corridor stayed separate from `phase-163x`
- current exact front truth:
  - `kilo_micro_concat_birth = 3ms`
  - `kilo_micro_concat_const_suffix = 36ms` (WSL lane: recheck with 3 runs)
  - `kilo_micro_concat_hh_len = 4ms` (landed `concat -> len` observer slice)
  - compiler-visible `concat pair/triple -> substring(...)` is now also landed on the same pure-first route; remaining concat backlog is the final emitted-MIR return-carrier cleanup only
  - sibling exact keeper front: `kilo_micro_substring_concat = 1,666,187 instr / 1,049,205 cycles / 4 ms` after the landed `phase-172x` stable-length exact-route cut
- landed capability lock before perf reopen:
  1. `phase-160x capability-family inventory`
  2. `phase-161x hot-path capability seam freeze`
  3. `phase-137x main kilo reopen selection` (historical reopen lane; now sibling string guardrail)
- paused reopen truth:
  - baseline: `kilo_kernel_small_hk = 1529ms`
  - string const fast-path: `775ms`
  - const-handle cache follow-up: `731ms`
  - const empty-flag cache: `723ms`
  - shared text-based const-handle helper: `903ms`
  - single-closure const suffix fast path: `820ms`
  - latest sampled whole-kilo reread: `703ms`
  - first implementation consumer: `array string-store`
  - second implementation consumer: `concat const-suffix`
  - exact micro:
    - `kilo_micro_concat_const_suffix = 84ms`
    - `kilo_micro_concat_hh_len = 4ms`
    - `kilo_micro_array_string_store = 169ms`

## Successor Corridor

1. `phase-163x primitive and user-box fast path`
2. `phase-137x` string guardrail / borrowed-corridor perf validation

## Parked After Optimization

- `phase-96x vm_hako LLVM acceptance cutover`
  - the active vm_hako capability gate is retired to a compatibility stub
  - `vm-hako-core.txt` is frozen as the 4-row monitor pack (`compare`, `env`, `file_close`, `file_read`)
  - `mapbox` mirror cleanup is complete; the remaining retirement work is runtime bridge separation
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
