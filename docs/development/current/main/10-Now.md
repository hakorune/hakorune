---
Status: SSOT
Date: 2026-04-09
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- lane: `phase-163x primitive and user-box fast path`
- current implementation focus:
  - parent design locked: `lifecycle-typed-value-language-ssot.md`
  - keep `field_decls` as authority
  - keep names-only `fields` as compatibility mirror
  - aggregate/objectization audits landed:
    - `phase163x-aggregate-truth-audit-2026-04-09.md`
    - `phase163x-early-objectization-audit-2026-04-09.md`
  - thin-entry inventory + first manifest-driven selection pilot landed in MIR metadata / verbose MIR / MIR JSON for known user-box + enum/sum local routes
  - the sum placement/effect pilot now also lands its inspection chain on top of that route:
    - `sum_placement_facts`
    - `sum_placement_selections`
    - `sum_placement_layouts`
  - `Program(JSON v0)` bridge now refreshes the same thin-entry + sum-placement metadata chain
  - LLVM now keeps selected local sum aggregates boxless through `sum_make` / `sum_tag` / `sum_project` and only materializes runtime `__NySum_*` compat boxes at `return` / `call` / `boxcall` escape barriers
  - the `ny-llvmc` parity proving slice is also landed:
    - product LLVM/Python lowering now seeds `thin_entry_selections` into the resolver alongside `sum_placement_selections` / `sum_placement_layouts`
    - metadata-bearing product smoke is green on `phase163x_boundary_sum_metadata_keep_min.sh` via boundary compat replay -> harness keep lane
    - native-driver metadata awareness remains canary-only backlog, not the current blocker
  - recommended post-selection order is now locked:
    1. `sum placement/effect pilot` (`sum outer-box sinking` first proving slice)
    2. `ny-llvmc` parity wave
    3. `tuple multi-payload` compat transport
  - tuple multi-payload compat transport is now landed:
    - parser/AST accept `Variant(T, U, ...)` while keeping tuple payload truth above canonical MIR
    - Stage1 lowers tuple ctors/matches through `__NyEnumPayload_<Enum>_<Variant>` with synthetic `_0`, `_1`, ... fields
    - canonical `EnumCtor` / `EnumMatch` / `SumMake` / `SumProject` stay single-slot
  - remaining follow-ons stay backlog-only: `where`, enum methods, full monomorphization
- sibling string guardrail:
  - `phase-137x main kilo reopen selection`
  - `kilo_micro_substring_views_only`
- prerequisite cleanup:
  - `phase-162x vm fallback lane separation cleanup` is landed and no longer the current blocker
- current landed upstream slices:
  - string corridor facts inventory
  - placement/effect scaffold
  - first borrowed-corridor sink pilot for single-use `substring(...).length()`
  - typed `field_decls` carrier + canonical `field.get` / `field.set`
  - declared-field storage bridge
  - narrow typed primitive pilot for `IntegerBox` / `BoolBox`
- observe lane:
  - `--features perf-observe`
  - `NYASH_PERF_COUNTERS=1`
  - TLS exact counter backend
  - `--features perf-trace`
  - `NYASH_PERF_TRACE=1`
  - trace lane is now parked placeholder
  - contract identity:
    - `store.array.str`
    - `const_suffix`
- latest bundle anchor:
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/summary.txt`
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/asm/perf_report.txt`
- recent landed:
  - `phase-140x map owner pilot`
  - `phase-139x array owner pilot`
  - `phase-138x nyash_kernel semantic owner cutover`
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`

## Current Read

- `vm` removal is not current work
- but `vm fallback` owner split cleanup is now inserted before the next perf proof
- fixed perf order stays:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is closed:
  - `kilo_micro_substring_concat`: parity locked
  - `kilo_micro_array_getset`: parity locked
  - `kilo_micro_indexof_line`: frozen faster than C
- `phase-134x` re-cut `nyash_kernel` into four buckets:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- current architecture target is fixed:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade` as thin keep
  - `compat quarantine` as non-owner
- landed final string seam:
  - semantic owner: `runtime/kernel/string/**`
  - VM-facing wrapper: `string_core_box.hako`
  - thin facade: `string.rs`
  - lifetime/native substrate: `string_view.rs` / `string_helpers.rs` / `string_plan.rs`
  - quarantine: `module_string_dispatch/**`
- current architecture follow-up is implementation-first:
  - `phase-142x` = landed Array owner cutover implementation
  - `phase-143x` = landed Map owner cutover implementation
  - `phase-144x` = landed String semantic owner follow-up
- current cleanup lane:
  - `phase-145x` = landed compat quarantine shrink
  - `phase-146x` = landed string semantic boundary tighten
- current optimization authority lock:
  - `.hako` owns route / retained-form / boundary
  - MIR owns canonical substrate contract
  - Rust owns executor / accelerator only
- landed contract freeze:
  - `const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`
  - `ArrayStoreString -> store.array.str`
  - `MapStoreAny -> store.map.value`
- landed first consumer:
  - `const_suffix` current lowering now reads as executor detail under the canonical contract
- landed second consumer:
  - `ArrayStoreString` current lowering now reads as ABI/executor detail under canonical `store.array.str`
- landed visibility lock:
  - `const_suffix`, `ArrayStoreString`, `MapStoreAny` all read through owner -> canonical -> concrete lowering -> executor
- current stop-line:
  - observer stays compile-out by default and feature-on by choice
  - observer must not look like a fifth authority layer
  - exact counter backend must not keep shared atomic cost on the hot path
  - heavy trace must not piggyback on exact counter backend or sink
- perf lane is active again:
  - capability lock is landed:
    - `phase-160x capability-family inventory`
    - `phase-161x hot-path capability seam freeze`
  - current perf truth:
    - whole `kilo_kernel_small_hk = 703ms`
    - exact micro `kilo_micro_concat_birth = 3ms`
    - exact micro `kilo_micro_concat_const_suffix = 36ms`
    - exact micro `kilo_micro_concat_hh_len = 3ms`
    - exact micro `kilo_micro_array_string_store = 169ms`
  - current rule:
    - structure first
    - exact/whole benchmark second

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
3. `docs/development/current/main/phases/phase-163x/README.md`
4. `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
5. `docs/development/current/main/phases/phase-137x/README.md`
