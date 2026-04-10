---
Status: SSOT
Date: 2026-04-10
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
  - thin-entry inventory + first manifest-driven selection pilot landed in MIR metadata / verbose MIR / MIR JSON for known user-box + enum local routes
  - the enum placement/effect pilot now also lands its inspection chain on top of that route:
    - `sum_placement_facts`
    - `sum_placement_selections`
    - `sum_placement_layouts`
  - `Program(JSON v0)` bridge now refreshes the same thin-entry + sum-placement metadata chain
  - LLVM now keeps selected local enum aggregates boxless through `variant_make` / `variant_tag` / `variant_project` and only materializes runtime `__NyVariant_*` compat boxes at `return` / `call` / `boxcall` escape barriers
  - llvm_py entry no longer synthesizes enum-facing `__NyVariant_*` user box declarations; runtime fallback materialization still happens on demand in lowering/escape barriers
  - VM enum runtime fallback no longer writes payloads through `InstanceBox::set_field_dynamic_legacy`; payload carriers now use the interpreter `obj_fields` compatibility store
  - `InstanceBox` no longer gates box-valued fields behind `NYASH_LEGACY_FIELDS_ENABLE`; dedicated `box_fields` are always present for identity-carrying handles, and `InstanceBox.size` / debug field listing now read the unified field-name union (`fields_ng` + `box_fields`)
  - dead unified/weak InstanceBox facades are gone; `host_box_ops` now uses the canonical `get_field_ng` / `set_field_ng` path directly
  - sum fallback bridge is now isolated in `sum_bridge`; `__NyVariant_*`, `__variant_tag`, and `__variant_payload` helpers no longer leak across handlers
  - interpreter object-field access is now wrapped by `object_field_store`; `get_object_field` / `set_object_field` / `object_field_root_count` are the only live entry points
  - array/string source cleanup landed: `StringHandleSourceKind` is gone, `with_array_store_str_source` now returns only `ArrayStoreStrSource`, and `array_string_slot` derives source-kind from the enum directly
  - the `ny-llvmc` parity proving slice is also landed:
    - product LLVM/Python lowering now seeds `thin_entry_selections` into the resolver alongside `sum_placement_selections` / `sum_placement_layouts`
    - product LLVM/Python lowering now also keeps selected primitive user-box bodies boxless through `newbox` / `field_get` / `field_set` when the birth block fully initializes the declared primitive fields
    - the same selected user-box route now materializes a compat runtime box only at `call` / `boxcall` / `ret`
    - metadata-bearing Point local-i64 user-box fixtures are now green on `phase163x_boundary_user_box_metadata_keep_min.sh` via boundary `pure-first` owner lane without compat replay, including the single-copy alias route
    - metadata-bearing enum smoke is now green on `phase163x_boundary_sum_metadata_keep_min.sh` via boundary `pure-first` owner lane without compat replay, and the keep fixture set now covers `variant_project`, direct `variant_tag`, and single-copy `variant_tag` aliases without `Option::Some`-specific naming
    - thin-entry inventory now normalizes boxed primitive `declared_type` hints back to inline scalar classes for user-box field routes
    - the current Point/Flag `ny-llvmc(boundary pure-first)` keeper seeds now require `user_box_field_{get,set}.inline_scalar` selector rows before firing
    - latest WSL `3 runs + asm` reread on the actual AOT route stays call-free:
      - point-add keeper seed now carries only the loop-visible `sum` lane plus the volatile accumulator anchor, matching the C-style bottom-tested induction loop
      - `kilo_micro_userbox_point_add`: `ny_aot_instr=8,456,727 / ny_aot_cycles=2,756,274 / ny_aot_ms=3`
      - `kilo_micro_userbox_flag_toggle`: `ny_aot_instr=16,457,454 / ny_aot_cycles=3,369,293 / ny_aot_ms=4`
    - three-lane measurement split is now landed:
      - `tools/perf/bench_micro_c_vs_aot_lanes.sh` reports `total CLI` / `startup baseline (ret0)` / `kernel-only`
      - point-add latest `1/3/10` reread: `ny_total_ms=3 / ny_startup_ms=3 / ny_kernel_ms=0.700`, with kernel cycles `c=2,025,422` vs `ny=2,046,604`
      - flag-toggle latest `1/3/10` reread: `ny_total_ms=4 / ny_startup_ms=3 / ny_kernel_ms=0.800`, with kernel cycles `c=4,053,730` vs `ny=2,837,417`
      - current read: keep codegen decisions on `kernel-only`; treat remaining total CLI delta as startup/runtime budget
    - minimal startup route is now landed:
      - `--emit-mir-json-minimal` skips using/prelude resolution and plugin init while keeping the small `.hako` parser normalizations used by the perf fixtures
      - use it for front-end startup checks; the existing three-lane AOT split stays the kernel/startup companion
    - generic native-driver / `ny-llvmc` parity for the broader user-box local-body route remains canary-only backlog, not the current blocker
    - portability-ci on `public-main` succeeded for commit `6b91896c0` (run `24211665863`), covering Windows check and macOS build (release)
  - verified post-Variant optimization order is now locked:
    1. `ny-llvmc` parity wave for the already-landed local enum/user-box routes
    2. sibling string retained-view `substring_hii` consumer expansion on the landed boundary `pure-first` corridor family
    3. broader string corridor placement/effect rewrite using the existing candidate vocabulary:
       - `borrowed_corridor_fusion`
       - `publication_sink`
       - `materialization_sink`
       - `direct_kernel_entry`
    4. actual-consumer switch for selected thin-entry user-box method routes that are still metadata-only today (`user_box_method.known_receiver` first)
    5. `ArrayBox` typed-slot expansion beyond the landed `InlineI64` pilot
  - tuple multi-payload compat transport is now landed:
    - parser/AST accept `Variant(T, U, ...)` while keeping tuple payload truth above canonical MIR
    - Stage1 lowers tuple ctors/matches through `__NyVariantPayload_<Enum>_<Variant>` with synthetic `_0`, `_1`, ... fields
    - canonical `EnumCtor` / `EnumMatch` / `VariantMake` / `VariantProject` stay single-slot
  - void/null cleanup is now landed too:
    - executable surface now accepts both `null` and `void`
    - literal-match parsing accepts `void` arms
    - direct compat null checks treat `VoidBox` and `NullBox` as the same no-value family
  - pre-optimization cleanup/doc sync is now landed too:
    - LLVM/Python local-enum escape barriers now share one helper instead of repeating the same materialization wrapper in `call` / `boxcall` / `ret`
    - runtime nullish checks now converge on `NullBox::check_null()` for the safe compat/tolerance paths touched in this slice
    - MIR reference docs are now split into instruction SSOT + metadata SSOT, while stale "all-in-one" references are reduced to thin pointers
    - next ready follow-on is `phase163x-optimization-resume`
    - immediate cut: `phase163x-sum-thin-entry-cutover`
    - landed proof for this cut:
      - `sum_result_ok_project_copy_local_i64_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias
      - `sum_result_ok_project_copy_local_f64_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias on the Float payload lane
      - `sum_result_ok_project_copy_local_handle_min.prebuilt.mir.json` now proves the same cutover when `variant_project` reads through a single local `copy` alias on the handle payload lane
      - `sum_result_ok_project_local_f64_min.prebuilt.mir.json` now proves the same cutover for `variant_project` on a Float payload lane
      - `sum_result_ok_project_local_handle_min.prebuilt.mir.json` now proves the same cutover for `variant_project` on a handle payload lane
      - `sum_result_ok_tag_only_local_min.prebuilt.mir.json` now proves the same cutover for a payload-less `variant_tag` keep-lane proof
      - `sum_result_ok_tag_local_f64_min.prebuilt.mir.json` now proves the same cutover for `variant_tag` on a Float payload lane
      - `sum_result_ok_tag_local_handle_min.prebuilt.mir.json` now proves the same cutover for `variant_tag` on a handle payload lane
    - Variant* inventory for this cut is now exhausted; next substep is `ny-llvmc` parity wave
    - next parity box in that wave: widen the same metadata-bearing local user-box keep smoke from Point/i64 into Bool/Float declared-field routes
    - separate phase, not this cut: relax `phi_merge` or `call` / `boxcall` / `return` barriers only with a metadata-contract update first
    - sibling string follow-on after that: extend the landed boundary `pure-first` consumer family from `substring(...).length()` plus `concat -> substring(...)` into retained-view `substring_hii` local shapes
    - restart handoff: cleanup queue is empty; continue `phase163x-optimization-resume` next; `phase137x-substring-retained-view-consumer` remains in progress as the sibling string lane
  - verified backlog-only follow-ons:
    - semantic/generic backlog: `where`, enum methods, full monomorphization
    - generic optimizer backlog: stronger cross-block/partial DCE beyond current pure-instruction DCE, and a generic LLVM-side escape pass beyond the already-landed narrow local objectization-at-boundary route
  - not yet fixed as current SSOT tasks:
    - `MapBox` typed value slots
    - float niche tuning (`fast-math` / `FMA` / SIMD-style follow-ons)
    - closure/lambda optimization
- sibling string guardrail:
  - `phase-137x main kilo reopen selection`
  - `kilo_micro_substring_views_only`
- prerequisite cleanup:
  - `phase-162x vm fallback lane separation cleanup` is landed and no longer the current blocker
- current landed upstream slices:
  - string corridor facts inventory
  - placement/effect scaffold
  - MIR JSON carrier for `string_corridor_facts` / `string_corridor_candidates`
  - boundary `pure-first` consumer for `substring(...).length()` via `substring_len_hii`
  - boundary `pure-first` consumer for compiler-visible `concat pair/triple -> substring(...)` via `substring_concat_hhii` / `substring_concat3_hhhii`
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
    - exact micro `kilo_micro_concat_hh_len = 4ms`
    - exact micro `kilo_micro_array_string_store = 169ms`
  - landed concat observer pilot:
    - generic `concat -> len` on `kilo_micro_concat_hh_len` now stays fully boxless in the observe lane
    - direct probe now shows `birth.placement=0`, `materialize_owned_total=0`, `string_box_new_total=0`, `handle_issue_total=0`
    - remaining concat barrier work stays on non-`len` consumers (`substring` / `return` / `store` / host boundary)
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
