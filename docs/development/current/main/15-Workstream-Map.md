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
| Front | `lifecycle value parent locked -> audits landed -> thin-entry inventory/selection landed -> sum placement pilot landed -> ny-llvmc parity proving slice landed -> tuple compat transport landed` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `canonical multi-slot sum remains deferred; current tuple route stays compat-boxed` |
| Next | `string exact-seed retirement design read` |
| After Next | `where / enum methods / monomorphization stay backlog-only` |

## Current Read

- `phase-132x` landed:
  - `--backend` default is now `mir`
  - explicit `vm` / `vm-hako` proof-debug lanes stay frozen keep
- current pre-optimization cleanup:
  - separate `vm-compat-fallback`
  - separate kernel-side Rust fallback policy
  - keep `vm-hako` as reference/conformance only
- landed optimization sub-corridor:
  - `phase-165x` landed the MIR-side operand-role escape barrier vocabulary cut
  - runtime/helper policy and generic cross-block escape work stay outside that slice
- landed optimization structure follow-on:
  - `phase-166x` completed the structural cleanup corridor
  - landed order is `semantic refresh owner -> generic value_origin / phi_relation owner -> compat semantic recovery quarantine -> boundary/lifecycle extraction decision`
- landed optimization stability follow-on:
  - `phase-167x` repaired user-box method sealing/determinism inside the MIR builder
  - instance methods now go through the shared finalize owner and seed receiver `Box(...)` metadata before known-receiver canonicalization; deterministic lexical traversal is kept as supporting structure
  - repeated release direct emit for `Counter.step_chain` is green again (`6/6`), while the separate pure-first/backend exact build+asm stop-line remains open
- active exact-route follow-on:
  - `phase-168x` is landed
  - the stale pure-first/direct exact contract for `Counter.step_chain` now matches the current narrow forwarding body again, and exact asm/perf evidence is green
- landed string metadata-contract follow-on:
  - `phase-169x` is landed
  - merged header `%21` on `kilo_micro_substring_concat` now carries a narrow `stable_length_scalar` witness while keeping the `stop_at_merge` plan-window contract
  - the live post-sink loop body now collapses to `source_len + const` on the exact front, and direct/pure-first contracts were refreshed together
- landed string bridge-shrink follow-on:
  - `phase-170x` is landed
  - boundary `pure-first` `substring()` on helper-result receivers now reads concat-triplet piece carriers from `direct_kernel_entry.plan.proof`
  - the targeted substring proof, len proof, live direct-emit contracts, exact asm/perf, and `quick` gate are green
- landed string exact follow-on:
  - `phase-171x` is landed as the bottom-tested loop-shape cut
  - `phase-172x` is landed
  - current exact front remains `kilo_micro_substring_concat`
  - the exact seed now consumes the landed `stable_length_scalar` witness through the header string-lane phi and switches to the existing length-only route
  - latest reread after that cut: `ny_aot_instr=1,666,187 / ny_aot_cycles=1,049,205 / ny_aot_ms=4`
  - next string work should finish the final emitted-MIR return-carrier cleanup if that route needs a dedicated guard
- active broader string follow-on:
  - `phase-173x`, `phase-174x`, and `phase-175x` are landed
  - same-block direct-helper `return` publication sink now consumes the landed `publication_sink` plan metadata
  - same-block canonical `Store { value, .. }` / `FieldSet { value, .. }` write boundaries now consume that same landed `publication_sink` plan metadata
  - same-block `RuntimeDataBox.set(...)` now consumes that same landed `publication_sink` plan as the first host-boundary publication slice
  - the cut is explicitly a birth-sink move, not a barrier-relaxation phase
  - remaining string backlog is the final emitted-MIR return-carrier cleanup only
- active generic optimizer follow-on:
  - `phase-176x` is landed as the first reachability-aware DCE cut
  - `phase-177x` is landed as the first effect-sensitive DCE cut
  - next target is broader effect-sensitive / no-dst cleanup
  - do not mix that with unreachable-block deletion
- landed shim-structure follow-on:
  - `phase-178x` is landed
  - split `hako_llvmc_ffi_sum_local_seed.inc` into a facade plus helper/emit/matcher include slices
  - keep this as BoxShape only; no pure-compile dispatch reorder and no semantic change
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
