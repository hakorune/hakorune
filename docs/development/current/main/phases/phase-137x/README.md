# Phase 137x: main kilo reopen selection

- Status: Active Guardrail
- 目的: string corridor / borrowed-corridor perf validation を guardrail lane として維持し、current implementation lane `phase-163x` の変更が string hot lane を壊していないかを exact/whole/asm で継続監視する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/observe/backend/tls.rs`

## Decision Now

- fixed perf order remains:
  1. `leaf-proof micro`
  2. `micro kilo`
  3. `main kilo`
- current local rule:
  - build structure before benchmark-driven widening
  - use exact micro + whole-kilo as accept gates after each structural slice
- `phase-134x` structural split is landed
- `phase-138x` / `phase-139x` / `phase-140x` / `phase-141x` semantic-owner corridor is landed
- contract-first corridor は landed
- Birth / Placement vocabulary lock is now landed in design SSOT
- perf consumer は llvmlite object emit retreat の後で reopen
- `vm-hako` stays parked as reference/conformance

## Restart Handoff

- this block is the current truth for restart; if older numbers below disagree, prefer this block
- restart with the code as it is now
- runtime-wide pattern anchor is now:
  - `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
- current upstream string corridor design anchor is now:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- current sibling implementation lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- pre-optimization cleanup anchor is now:
  - `docs/development/current/main/design/vm-fallback-lane-separation-ssot.md`
- perf release gate now builds `ny-llvmc` as well; do not run exact/asm probes after editing compiler sources without refreshing release artifacts first
- mixed accept gate stays `kilo_micro_substring_only`
- split exact fronts are now keeper checks, not active blockers:
  - `kilo_micro_substring_views_only`
  - `kilo_micro_len_substring_views`
- current broader-corridor reopen front is `kilo_micro_substring_concat`
- current broader-corridor genericization rule:
  - do not add a new string-only MIR dialect
  - landed: `string_corridor_candidates` now carry proof-bearing plan metadata for borrowed-slice and concat-triplet routes
  - landed: direct `substring_concat3_hhhii` helper results now stay on the same proof-bearing lane with concat-triplet-backed `publication_sink` plan metadata
  - landed: direct helper-result `length()` / `substring()` now consume that same `publication_sink` plan in `string_corridor_sink`
  - landed: first non-`phi` `materialization_sink` slice now sinks a direct `substring_concat3_hhhii` helper birth to a single local `ArrayBox.set` boundary when only copy aliases separate the helper from the store
  - landed: first post-store observer slice now keeps `array.set` as the first `Store` boundary while rewriting one trailing helper-result `length()` observer to `end - start` and deleting the copy-only observer/store chains
  - landed: first plan-selected `direct_kernel_entry` slice now reads `string_corridor_candidates[*].plan.start/end` on direct helper-result receivers and lowers `length()` as window arithmetic in boundary `pure-first`
  - targeted proof: `apps/tests/mir_shape_guard/string_direct_kernel_plan_len_window_min_v1.mir.json` + `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_direct_kernel_plan_len_min.sh`
  - next: keep loop-carried `phi_merge` outside this cut, shrink the remaining dynamic/exact bridge paths that still do not read the plan directly, and treat any further `array.set + trailing length()` widening as a separate metadata-contract phase only if fresh perf evidence appears
  - migration-safe reading: this lane should keep landing in canonical MIR facts/candidates/sink plus kernel/backend substrate, not in Rust-builder-local shape logic
  - treat exact seed logic in `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc` as temporary bridge surface to shrink after generic plan-selected routes prove out
- pure Rust reference compare lane:
  - `benchmarks/rust/bench_kilo_micro_substring_views_only.rs`
  - `tools/perf/bench_rust_vs_hako_stat.sh kilo_micro_substring_views_only 1 3`
  - latest pure Rust reference: `instr=5,667,104 / cycles=1,572,750 / cache-miss=5,254 / ms=3`
  - latest C-like Rust reference: `instr=12,566,914 / cycles=3,404,383 / cache-miss=5,256 / ms=3`
- latest exact reread on the mixed accept gate:
  - `kilo_micro_substring_only: instr=1,669,659 / cycles=1,077,794 / cache-miss=8,810 / AOT 3 ms`
  - split exact reread:
    - `kilo_micro_substring_views_only: instr=466,001 / cycles=841,958 / cache-miss=9,391 / AOT 3 ms`
    - `kilo_micro_len_substring_views: instr=1,672,096 / cycles=1,009,964 / cache-miss=8,902 / AOT 3 ms`
- current broader-corridor reopen front:
  - `kilo_micro_substring_concat: instr=5,565,655 / cycles=5,816,743 / cache-miss=9,424 / AOT 4 ms`
  - `kilo_micro_array_string_store: c_ms=9 / ny_aot_ms=9`; this family is not the current blocker
- target band for the next keeper:
  - mixed accept gate: hold `instr <= 1.8M`
  - local split `kilo_micro_substring_views_only`: hold `instr <= 0.6M`
  - control split `kilo_micro_len_substring_views`: hold `instr <= 1.8M`
  - broader-corridor reopen `kilo_micro_substring_concat`: first keeper target `instr < 5.5M`
  - whole strict: keep `<= 709 ms`; ideal band is `690-705 ms`
- ideal `len_h` steady-state asm shape:
  - direct `STRING_DISPATCH_FN` load once; no `STRING_DISPATCH_STATE` state machine in `nyash.string.len_h`
  - direct `host_handles::DROP_EPOCH` load once
  - primary/secondary handle compare only
  - `JIT_TRACE_LEN_ENABLED_CACHE` load once with cold init off the hot return path
  - trace-off fast hit returns directly
- current whole-kilo health:
  - `tools/checks/dev_gate.sh quick` is green
  - `kilo_kernel_small_hk` strict accepted reread: `ny_aot_ms=709`
  - parity: `vm_result=1140576`, `aot_result=1140576`
- current landed substring truth:
  - boundary `pure-first` now lands the first retained-view `substring_hii` exact-micro consumer slice:
    - `kilo_micro_substring_views_only` now matches a known-positive loop-bound exit-len shape and collapses before `substring_hii` / `len_h` replay
    - exact reread on `2026-04-10`: `instr=465,637 / cycles=704,757 / cache-miss=8,280 / AOT 3 ms`
    - current microasm dump now shows `ny_main` as `mov $0x20, %eax ; ret`
    - reading: the sibling exact micro is no longer the blocker; the next string keeper must move back to the mixed accept gate and broader corridor rewrite family
  - `substring_hii` can reissue a fresh handle from a cached `StringViewBox` object when the transient result handle dropped but the source handle still points to the same live source object
  - `str.substring.route` observe read is now dominated by the steady-state handle-hit path: `view_arc_cache_handle_hit=599,998 / total=600,000`
  - current keeper removes redundant `view_enabled` state from `SubstringViewArcCache`; the cache only runs under `view_enabled`, so the extra key dimension was dead hot-path work
  - `2026-04-09` perf reread on `kilo_micro_substring_views_only`:
    - exact: `instr=34,363,814 / cycles=6,537,017 / cache-miss=10,232 / AOT 4 ms`
    - top: `nyash.string.substring_hii 87.04%`, `ny_main 6.00%`
    - annotate says the first visible tax is still inside the caller entry:
      1. `SUBSTRING_ROUTE_POLICY_CACHE` load/decode
      2. `substring` provider state read + `SUBSTRING_VIEW_ARC_CACHE` TLS entry/state check
      3. only then the steady-state compare path
      4. slow plan / materialize is not the dominant block on this front
  - latest baseline asm reread still shows the next visible tax before the view-arc cache compare block:
    1. `SUBSTRING_ROUTE_POLICY_CACHE` decode
    2. `substring_view_enabled` / fallback provider state reads
    3. only then `SubstringViewArcCache` steady-state compare
  - boundary `pure-first` now consumes MIR JSON `string_corridor_*` for `substring(...).length()`:
    - direct route trace now hits `string_len_corridor -> substring_len_direct_kernel_plan_window`
    - retained-slice `length()` / `len()` consumers now also rewrite through the same direct entry even when the slice producer dominates from another block through local copy aliases
    - the current bridge shrink also removes the `substring_len_hii` declaration need from this plan-window lane; metadata is now the only direct-kernel proof source here
    - latest exact reread on `kilo_micro_len_substring_views`: `instr=1,672,259 / cycles=1,022,005 / cache-miss=10,525 / AOT 3 ms`
    - latest split-pack reread on `kilo_micro_substring_views_only`: `instr=466,001 / cycles=841,958 / cache-miss=9,391 / AOT 3 ms`
    - reading: the split retained-view fronts are now closed; the next string keeper reopens on broader corridor publication/materialization work
  - boundary `pure-first` now also lands the first generic concat observer pilot:
    - single-use `concat pair/triple -> len()` now defers the concat producer and reads known chain length without forcing handle birth
    - observe direct probe on `kilo_micro_concat_hh_len` now shows:
      - `birth.placement`: all `0`
      - `birth.backend`: `freeze_text_plan_total=0`, `string_box_new_total=0`, `handle_issue_total=0`, `materialize_owned_total=0`, `gc_alloc_called=0`
      - `str.concat2.route=0`, `str.len.route=0`
    - exact reread on `kilo_micro_concat_hh_len`: `instr=7,657,032 / cycles=2,284,266 / cache-miss=8,479 / AOT 4 ms`
    - reading: this closes the first `concat -> len` observer slice
  - boundary `pure-first` now also lands the first generic non-`len` concat consumer slice:
    - compiler-visible `concat pair/triple -> substring(...)` now routes to `nyash.string.substring_concat_hhii` / `nyash.string.substring_concat3_hhhii`
    - dynamic route proof hits `string_substring_route -> substring_concat3_hhhii`
    - reading: this removes the intermediate concat handle birth for substring consumers; remaining concat backlog is `return` / `store` / host-boundary publication
  - broader-corridor keeper repair is now landed:
    - `string_corridor_sink` rewrites `concat(left_slice, const, right_slice).length()` into `substring_len_hii(left) + const_len + substring_len_hii(right)` and keeps `substring(concat3(...))` on `substring_concat3_hhhii`
    - the exact `pure-first` `kilo_micro_substring_concat` seed now accepts both the pre-sink and post-sink body shapes, so this generic sink no longer ejects the exact lane into the slow fallback route
    - latest exact reread on `kilo_micro_substring_concat`: `instr=5,565,655 / cycles=5,816,743 / cache-miss=9,424 / AOT 4 ms`
  - first broader-corridor `publication_sink` inventory slice is now landed:
    - emitted MIR JSON on `kilo_micro_substring_concat` now keeps the direct `substring_concat3_hhhii` helper result on the same corridor lane with `borrowed_corridor_fusion` / `publication_sink` / `materialization_sink` / `direct_kernel_entry` candidates
    - the helper-result plan is concat-triplet-backed and points at the shared source root plus outer `start/end`
    - reading: helper-result inventory is no longer the gap
  - first broader-corridor `publication_sink` actual transform is now landed too:
    - `string_corridor_sink` rewrites direct helper-result `length()` to `end - start`
    - `string_corridor_sink` composes direct helper-result `substring()` back into `substring_concat3_hhhii` by adding the inner window to the helper's outer window
    - reading: the remaining exact-front gap is the loop-carried `text = out.substring(...)` `phi_merge` route, not missing helper-result inventory or direct helper-result consumers
  - current `substring_len_hii` pilot uses `with_text_read_session_ready(...)` to avoid the hot `REG` ready probe; current helper perf is the mixed sink candidate above
  - split exact reread now puts the retained-view exact micro below `0.5M instr`, so `substring_hii` is no longer the active blocker on that split and `len_h` remains the control split
  - current keeper is on `len_h`: hoist one `handles::drop_epoch()` read in `string_len_fast_cache_lookup()` and reuse it for both cache slots
  - current keeper also keeps `len_h` trace-off steady state thin by tail-calling a tiny fast-return helper instead of carrying `trace_len_fast_hit(...)` inline in the hot cache-hit block
  - current keeper removes the `STRING_DISPATCH_STATE` state machine from emitted `len_h` hot asm by probing `STRING_DISPATCH_FN` directly once
  - current keeper also splits trace state into raw-read + cold-init helpers, so the hot cache-hit path sees one `JIT_TRACE_LEN_ENABLED_CACHE` load
  - current keeper also lands the `drop_epoch()` global mirror: `nyash.string.len_h` now reads `host_handles::DROP_EPOCH` directly, and the `host_handles::REG` ready probe is gone from the hot block
  - split exact reread now clears the sibling retained-view exact micro at boundary `pure-first`; next priority moves back to the mixed accept gate and corridor rewrite family
  - pure Rust reference is the current lower bound for this front; current AOT is about `6.06x instr / 4.10x cycles` over it
  - C-like Rust reference is the current contract-aligned comparison point; current AOT is about `2.73x instr / 1.91x cycles` over it
  - upstream corridor pilot is now structurally landed:
    - single-use `substring(...).length()` chains sink to `nyash.string.substring_len_hii`
    - kernel export + MIR interpreter fallback are in place
    - current status is structural plus perf-positive candidate: compile/test are green, and the mixed accept gate now rereads at `instr=47,270,021 / cycles=28,264,307 / cache-miss=9,191 / AOT 8 ms`
  - `nyash.string.substring_hii` / `nyash.string.len_h` / `trace_borrowed_substring_plan` stay as the fallback semantic carrier
  - WSL validation rule stays `3 runs + perf`
- do not reopen for this lane:
  - `OwnedText` backing for substring source lifetime
  - live-source direct-read widening on `as_str_fast()`
  - global `dispatch` / `trace` false-state fast probes outside `string_len_export_impl()`
  - lifting substring runtime cache mechanics into `.hako` or `MIR`
  - widening `@rune` beyond declaration-local metadata for this lane
  - generic scalar/cache/route frameworks before a second lane proves the same keeper pattern
- rejected local probes are now centralized in:
  - [phase137x-substring-rejected-optimizations-2026-04-08.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md)
  - current rejected list:
    1. broad `NyashBox` substring-source contract widening
    2. `substring_view_arc_cache_lookup` / `entry_hit` hot-path fusion
    3. birth-side second `with_handle(...)` removal via planner-local source metadata carry
    4. reissue-side slot carry / `refresh_handle` rematch removal
    5. concrete `Arc<StringViewBox>` cache carrier narrowing
    6. `len_h` cache-first reorder
    7. `drop_epoch_if_ready()` fast accessor probe
    8. global `dispatch` / `trace` false-state fast probes
    9. `len_h` dispatch-hit cold split
    10. `trace_len_state()` helper / trace cache single-load probe
    11. `len_h` two-slot pre-match + single epoch-guard probe
    12. local `dispatch_known_absent_fast` + cold dispatch probe combo
    13. `drop_epoch_after_cache_hit()` ready-after-hit probe
    14. `len_h` dispatch single-probe + raw trace-state split
    15. `len_h` 1-probe hash-slot cache shape
    16. registry-pointer epoch read on len cache hits
    17. `len_h` `ReadOnlyScalarLane` separation-only slice
    18. `len_h` combined `ReadOnlyScalarLane` + entry snapshot slice
    19. `len_h`-specific 4-box slice (`façade + control snapshot + pure cache probe + cold path`)
    20. `SubstringViewArcCache` global compare reorder (`start/end` before `source_handle`)
    21. `SubstringViewArcCache` `same_source_pair` specialization
    22. `substring_hii` common-case body duplication via `route_raw == 0b111`
    23. `substring` provider `raw read + cold init` adoption (`substring_view_enabled` / fallback policy / route policy)
    24. `substring_route_policy()` cold init split while keeping the active caller shape unchanged
    25. `substring_hii` route/provider snapshot + eager `DROP_EPOCH` snapshot
    26. `SubstringViewArcCache::entry_hit` reissue/clear cold split
- next active cut:
  1. keep `kilo_micro_substring_only` as accept gate
  2. use `kilo_micro_substring_views_only` for local `substring_hii` cuts
  3. keep `len_h` runtime mechanics stable unless split fronts move again
  4. latest keeper already removed the remaining `len_h` control-plane hot loads
  5. current pivot is upstream, not another leaf-local `substring_hii` split:
     - `.hako policy -> canonical MIR facts -> placement/effect pass -> Rust microkernel -> LLVM`
  6. do not add a permanent second public MIR dialect for this wave
  7. both `len_lane` separation-only and combined lane+snapshot retries were rejected; lane boundary alone is not the next keeper slice
  8. the earlier `drop_epoch()` global mirror rejection was invalidated by stale release artifacts; the hypothesis is now landed, and future perf reads must rebuild release artifacts first
  9. fixed task order:
     - step 1: docs-first; treat `string-canonical-mir-corridor-and-placement-pass-ssot.md` as the active design owner
     - step 2: landed; inventory canonical string corridor sites and current lowering carriers for `str.slice` / `str.len` / `freeze.str` via `src/mir/string_corridor.rs`
     - step 3: landed; canonical MIR-side fact carrier is `FunctionMetadata.string_corridor_facts`, and verbose dumps plus MIR JSON expose it with no runtime behavior change
     - step 4: landed; `src/mir/string_corridor_placement.rs` now reads `FunctionMetadata.string_corridor_facts`, emits no-op candidate decisions into `FunctionMetadata.string_corridor_candidates`, and exposes them in verbose MIR dumps plus MIR JSON
     - step 5: landed structurally; the first borrowed-corridor sinking pilot now rewrites single-use `substring(...).length()` chains to `nyash.string.substring_len_hii`
     - step 6: landed; `phase-162x vm fallback lane separation cleanup` is complete, so this front now reads through `ny-llvmc(boundary pure-first)` without mixing fallback owners
     - step 7: landed; boundary `pure-first` now consumes MIR JSON `string_corridor_*` metadata for `substring(...).length()` and now reads the route as `string_len_corridor -> substring_len_direct_kernel_plan_window`
     - step 8: landed; boundary `pure-first` now also routes compiler-visible concat pair/triple `substring(...)` consumers to `nyash.string.substring_concat_hhii` / `nyash.string.substring_concat3_hhhii`
     - step 9: landed; `FunctionMetadata.string_corridor_candidates` now carries proof-bearing plan metadata on the broader-corridor reopen front `kilo_micro_substring_concat`, and MIR JSON exports the same plan surface
     - step 10: landed; direct `substring_concat3_hhhii` helper results now stay on the corridor metadata lane with concat-triplet-backed `publication_sink` proof
     - step 11: landed; direct helper-result `length()` / `substring()` now consume that same `publication_sink` proof in `string_corridor_sink`
     - step 12: landed; `materialization_sink` now covers the non-`phi` local `ArrayBox.set` store boundary and the first trailing `length()` post-store observer window on the same canonical MIR lane
     - step 13: landed first plan-selected `direct_kernel_entry` slice; boundary `pure-first` now reads plan windows on direct helper-result receivers, lowers `length()` as window arithmetic, and no longer keeps the `substring_len_hii` declaration bridge on that lane
     - step 14: next shrink the remaining dynamic/exact bridge paths that still bypass the plan
     - step 15: separate phase, not this cut: any `phi_merge` relaxation for the loop-carried `text = out.substring(...)` route
     - step 16: only after that reopen new `substring_hii` runtime leaf cuts, and only with exact/asm proof
     - step 17: do not retry the same `len_h`-specific 4-box slice as-is; it did not clear exact or asm gates
     - step 18: keep this lane specific; do not generalize into a reusable scalar framework until a second lane wins the same pattern
     - step 18: do not swap the active `substring` providers to `raw read + cold init` as one slice; that provider-adoption cut regressed the local split
     - step 19: do not duplicate the common-case `substring_hii` body again; the earlier `route_raw == 0b111` duplication regressed badly
     - step 20: `substring_route_policy()` cold split alone is also blocked; even with the caller unchanged it regressed the local split
     - step 21: any future `len_h` reopen must preserve direct dispatch probe + single trace-state load + direct `DROP_EPOCH` load
     - step 22: do not retry the same `substring_hii` route/provider snapshot with eager `DROP_EPOCH` capture; it widened the caller prologue and regressed exact/whole together
     - step 23: do not cold-split `SubstringViewArcCache::entry_hit` reissue/clear in isolation; it regressed every split front and whole strict
     - step 24: primitive/user-box follow-on work now lives in `phase-163x`; keep this README string-only
  10. next local cut must show an exact-visible or asm-visible change on `substring_hii`, but only after the upstream corridor slices are in place
- safe restart order:
  1. `git status -sb`
  2. `tools/checks/dev_gate.sh quick`
  3. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
  4. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  5. `docs/development/current/main/phases/phase-163x/README.md`
  6. `src/mir/string_corridor.rs`
  7. after any `nyash_kernel` / `hakorune` runtime source edit, rerun `bash tools/perf/build_perf_release.sh` before exact micro / asm probes
  8. `tools/perf/run_kilo_string_split_pack.sh 1 3`
  9. `tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_views_only 'nyash.string.substring_hii' 200`
  10. read the rejected ledger before retrying any substring-local cut
- documentation rule for failed perf cuts:
  1. keep a short current summary in this README
  2. keep exact rejected-cut evidence in one rolling investigation doc per front/family/date
  3. do not create test-by-test folders unless that artifact family itself becomes an independent lane
- promotion policy for this cache family:
  1. the first proven win can stay local in Rust when one exact front is isolated and measurable
  2. once the same alternating-access pattern appears in another exact front, stop adding route-local cache variants and evaluate a shared hot-cache policy above Rust
  3. lift only when the semantics are common and lifetime / ownership boundaries remain explicit at the higher layer
  4. avoid repeating Rust-local cache additions in the same family without rechecking that promotion condition
- immediate substring follow-up:
  1. `substring_hii` is first target again under the split pair
  2. keep runtime cache mechanics as-is; broad provider adoption into the hot caller lost the local split
  3. read the rejected ledger before retrying any substring-local cut
  4. use the split exact pair before and after every provider-side change
  5. use the landed `substring(...).length()` corridor consumer plus the landed `concat -> substring(...)` carrier as the templates for the next retained-view `substring_hii` cut
  6. retained-view `substring_hii` local shapes remain the next string-only keeper front
  7. next cleanup task must stay narrower than the rejected provider-adoption slice
- lifecycle placement is fixed:
  - `.hako`: source-preserve / identity / publication demand
  - `MIR`: visibility carrier and escalation contract
  - `Rust`: mechanics only
- if the next session needs a quick read order, start here:
  1. this block
  2. `CURRENT_TASK.md`
  3. `git status -sb`

## Fresh Read

- current front is structure-first:
  - `kilo_micro_substring_only`
  - `nyash.string.substring_hii` / `nyash.string.len_h` / `trace_borrowed_substring_plan` source contract as fallback semantic carrier
  - `substring` / `len` cache state now uses flattened TLS records to reduce lookup shape overhead
  - `SourceLifetimeKeep`
  - `RetargetAlias` source-lifetime semantics
  - `concat_birth` fresh-box materialization landed
  - AOT compiler-side literal `string + string` fold landed
 - whole-kilo read order is now fixed through a supported contract split ladder:
   - `kilo_micro_concat_hh_len`
   - `kilo_micro_array_string_store`
   - `kilo_meso_substring_concat_len`
   - `kilo_meso_indexof_append_array_set`
   - `kilo_kernel_small_hk`
 - missing whole corridor is now explicit:
   - rotating row `indexOf("line") + append + array.set`
 - exploratory meso shapes remain documented but stay out of the default AOT ladder for now:
   - `kilo_meso_substring_concat_array_set`
   - `kilo_meso_substring_concat_array_set_loopcarry`
   - current `pure-first` route still rejects both
 - use `tools/perf/run_kilo_kernel_split_ladder.sh` when re-reading whole-kilo after a structural slice
- current probe-only split-ladder reread (`repeat=1`):
  - `kilo_micro_concat_hh_len: 61 ms`
  - `kilo_micro_array_string_store: 176 ms`
  - `kilo_meso_substring_concat_len: 33 ms`
  - `kilo_meso_indexof_append_array_set: 152 ms`
  - `kilo_kernel_small_hk: 700 ms`
- current lifecycle visibility lock for `store.array.str`:
  - public row stays `store.array.str`
  - `.hako` owns source-preserve / identity / publication demand
  - MIR carries that visibility through the existing lowering carrier:
    - `GenericMethodRouteState`
    - `GenericMethodEmitPlan`
  - no-behavior carrier fields are now landed:
    - `array_store_string_source_preserve`
    - `array_store_string_identity_demand_stable_object`
    - `array_store_string_publication_demand_publish_handle`
  - `.hako` owner-side policy methods are now landed:
    - `array_store_string_source_preserve(...)`
    - `array_store_string_identity_demand(...)`
    - `array_store_string_publication_demand(...)`
  - current mirror reads those lifecycle policy fields through `set_route`
  - Rust only executes:
    - `SourceKindCheck`
    - `SourceLifetimeKeep`
    - `AliasUpdate`
    - `NeedStableObject`
- benchmark numbers stay current truth, but they are now validation, not the driver for widening Rust transport
- latest structural visibility split:
  - `value_codec/string_materialize.rs` now owns `OwnedBytes -> StableBoxNow -> FreshRegistryHandle`
  - `value_codec/string_store.rs` now stays on store-from-source execution
  - accept-gate reread:
    - `kilo_micro_array_string_store: 179 ms`
    - `kilo_meso_indexof_append_array_set: 150 ms`
    - `kilo_kernel_small_hk: 695 ms`
- `exports/string.rs` is now a thin export shell with helpers split out
- `plugin/map_substrate.rs` is now raw substrate helpers only
- `plugin/map_aliases.rs` now owns the ABI alias surface
- `nyash_kernel` is ready to be re-baselined under the new responsibility split
- `src/tests.rs` has been split into `tests/filebox.rs` and `tests/string.rs`, so the root test module is no longer a 1000+ line monolith
- reopened perf read:
  - baseline: `kilo_kernel_small_hk`: `c_ms=81 / ny_aot_ms=1529`
  - after string const-path branch collapse: `c_ms=82 / ny_aot_ms=775`
  - after const-handle cache follow-up: `c_ms=84 / ny_aot_ms=731`
  - after const empty-flag cache: `c_ms=81 / ny_aot_ms=723`
  - after shared text-based const-handle helper: `c_ms=80 / ny_aot_ms=903`
  - after single-closure const suffix fast path: `c_ms=83 / ny_aot_ms=820`
  - latest whole-kilo reread after visibility lock: `c_ms=83 / ny_aot_ms=762`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_concat_const_suffix`: `c_ms=3 / ny_aot_ms=36`
  - `kilo_micro_concat_hh_len`: `c_ms=3 / ny_aot_ms=4`
  - `kilo_micro_concat_birth`: `c_ms=6 / ny_aot_ms=3`
  - `kilo_micro_array_string_store`: `c_ms=9 / ny_aot_ms=173`
 - latest whole-kilo reread after keep API narrowing: `c_ms=77 / ny_aot_ms=708`
  - latest whole-kilo reread after keep-anchor cold fallback narrowing: `c_ms=79 / ny_aot_ms=696`
- latest bundle read:
  - string contracts remain `keep_transient -> fresh_handle` for non-empty const concat/insert
  - `20260406-024104` still shows `crates/nyash_kernel/src/exports/string_helpers.rs::concat_const_suffix_fallback` as the top explicit hot symbol (`11.70%`)
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs::array_string_store_handle_at` remains second (`5.68%`)
  - exact micro gap is currently larger on `array_string_store`
 - deeper observe drill-down now exists for:
   - `store.array.str`: `existing_slot / append_slot / source_string_box / source_string_view / source_missing`
   - `const_suffix`: `empty_return / cached_fast_str_hit / cached_span_hit`
   - generic string consumer:
     - `str.concat2.route`: `total / dispatch_hit / fast_str_owned / fast_str_return_handle / span_freeze / span_return_handle / materialize_fallback / unclassified`
     - `str.len.route`: `total / dispatch_hit / fast_str_hit / fallback_hit / miss / latest_fresh_handle_fast_str_hit / latest_fresh_handle_fallback_hit / unclassified`
   - `birth.placement`: `return_handle / borrow_view / freeze_owned / fresh_handle / materialize_owned / store_from_source`
   - `birth.backend`: `freeze_text_plan_total / view1 / pieces2 / pieces3 / pieces4 / owned_tmp / materialize_owned_total / materialize_owned_bytes / string_box_new_total / string_box_new_bytes / string_box_ctor_total / string_box_ctor_bytes / arc_wrap_total / handle_issue_total / gc_alloc_called / gc_alloc_bytes / gc_alloc_skipped`
 - exact observe read:
   - `kilo_micro_array_string_store` AOT direct probe is saturated on one shape:
     - `cache_hit=800000`
     - `retarget_hit=800000`
     - `existing_slot=800000`
     - `source_string_box=800000`
   - current cache-churn hypothesis is not supported on that exact micro
   - `kilo_micro_concat_const_suffix` AOT direct probe does not hit `const_suffix`
   - `kilo_micro_concat_hh_len` isolated the generic `concat -> len` consumer without substring carry, and that slice is now landed
   - `kilo_micro_concat_birth` now isolates fresh concat birth/materialize with only final `len`
   - `kilo_micro_concat_birth` direct probe currently shows:
     - `birth.placement`: `fresh_handle=800000`
     - `birth.backend`: `materialize_owned_total=800000`, `materialize_owned_bytes=14400000`, `gc_alloc_called=800000`, `gc_alloc_bytes=14400000`
     - `str.concat2.route`: `fast_str_owned=800000`, other classified routes `0`, `unclassified=0`
     - `str.len.route`: `fast_str_hit=1`, `latest_fresh_handle_fast_str_hit=1`, other classified routes `0`, `unclassified=0`
   - `kilo_micro_concat_hh_len` observe direct probe now shows:
     - `birth.placement`: `return_handle=0 / borrow_view=0 / freeze_owned=0 / fresh_handle=0 / materialize_owned=0 / store_from_source=0`
     - `birth.backend`: `freeze_text_plan_total=0`, `string_box_new_total=0`, `handle_issue_total=0`, `materialize_owned_total=0`, `gc_alloc_called=0`
     - `str.concat2.route`: `total=0`
     - `str.len.route`: `total=0`
     - latest exact reread: `instr=7,657,032 / cycles=2,284,266 / cache-miss=8,479 / AOT 4 ms`
   - `NYASH_PERF_BYPASS_GC_ALLOC=1` diagnostic observe lane still matters only for `kilo_micro_concat_birth`:
     - `kilo_micro_concat_birth`: `50 -> 51 ms`
     - observe-build `kilo_kernel_small_hk`: `1077 -> 1084 ms`
     - direct probe cleanly flips:
       - `gc_alloc_called=800000 -> 0`
       - `gc_alloc_skipped=0 -> 800000`
   - current evidence keeps `kilo_micro_concat_birth` as the remaining concat birth front; the landed `concat_hh_len` observer slice no longer exercises runtime concat/len routes
   - external design lock after the latest exact/whole split:
     - do not treat birth as one fused event
     - read current backend as:
       - byte birth = `MaterializeOwned`
       - object birth = `StableBoxNow`
       - publication birth = `FreshRegistryHandle`
     - next backend-private carriers are:
       - `OwnedBytes`
       - `TextReadSession`
     - next structural goal is to reduce `StableBoxNow` demand before trying to
       make `next_box_id` or registry issue cheaper again
   - source-backed private seam slice is now in place:
     - `OwnedBytes` exists in `string_store.rs`
     - `TextReadSession` exists in `host_handles.rs`
     - `string_len_from_handle(...)`, `string_is_empty_from_handle(...)`,
       `concat_pair_from_fast_str(...)`, and `concat3_plan_from_fast_str(...)`
       now read through the session seam
     - this slice does not reintroduce deferred objectization behavior
   - `StableBoxNow` demand probe now also exists:
     - `kilo_micro_concat_birth`
       - `object_get_latest_fresh=0`
       - `object_with_handle_latest_fresh=0`
       - `object_pair_latest_fresh=0`
       - `object_triple_latest_fresh=0`
       - `text_read_handle_latest_fresh=1`
       - `text_read_pair_latest_fresh=0`
       - `text_read_triple_latest_fresh=0`
     - `kilo_micro_concat_hh_len`
       - `object_get_latest_fresh=0`
       - `object_with_handle_latest_fresh=0`
       - `object_pair_latest_fresh=0`
       - `object_triple_latest_fresh=0`
       - `text_read_handle_latest_fresh=800000`
       - `text_read_pair_latest_fresh=0`
       - `text_read_triple_latest_fresh=0`
     - latest fresh handles are staying inside the single-handle text-read seam on the current exact fronts
     - exact micro evidence does not support object-world leakage as the current first cause
   - delayed `StableBoxNow` retry truth:
     - exact micro improved:
       - `kilo_micro_concat_birth`: `50 -> 37 ms`
       - `kilo_micro_concat_hh_len`: `67 -> 57 ms`
     - whole-kilo still regressed:
       - `kilo_kernel_small_hk`: `764 ms`
     - whole observe probe points at early object-world escalation instead of exact-path leakage:
       - `stable_box_demand.object_with_handle_latest_fresh=540000`
       - `stable_box_demand.object_get_latest_fresh=0`
       - `stable_box_demand.object_pair_latest_fresh=0`
       - `stable_box_demand.object_triple_latest_fresh=0`
       - `stable_box_demand.text_read_handle_latest_fresh=0`
       - `stable_box_demand.text_read_pair_latest_fresh=938`
     - current read:
       - exact micro stays inside the single-handle text-read seam
       - whole-kilo quickly promotes latest fresh string handles into generic object `with_handle(...)`
       - delayed objectization must not be relanded until that consumer is widened or bypassed
       - target assembly shape:
         - `concat_hh + len_h` should stay on text/materialize paths for as long as possible
         - registry/object traffic should appear only at sink/object boundaries, not between concat and immediate len
     - caller-attributed whole-kilo truth:
       - `stable_box_demand.object_with_handle_array_store_str_source_latest_fresh=540000`
       - `stable_box_demand.object_with_handle_substring_plan_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_array_fast_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_any_arg_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_any_index_latest_fresh=0`
     - source-backed `store.array.str` split confirms that this whole-kilo latest-fresh demand is entirely retarget-side:
       - `store.array.str latest_fresh_retarget_hit=540000`
       - `store.array.str latest_fresh_source_store=0`
     - no-behavior-change planner truth now confirms the source-contract mismatch itself:
       - whole-kilo:
         - `plan.source_kind_string_like=540000`
         - `plan.source_kind_other_object=0`
         - `plan.source_kind_missing=0`
         - `plan.slot_kind_borrowed_alias=540000`
         - `plan.slot_kind_other=0`
         - `plan.action_retarget_alias=540000`
         - `plan.action_store_from_source=0`
         - `plan.action_need_stable_object=0`
       - exact `kilo_micro_array_string_store`:
         - `plan.source_kind_string_like=800000`
         - `plan.slot_kind_borrowed_alias=800000`
       - `plan.action_retarget_alias=800000`
       - `plan.action_store_from_source=0`
       - `plan.action_need_stable_object=0`
     - no-behavior-change reason truth now clarifies the remaining contract:
       - whole-kilo:
         - `reason.source_kind_via_object=540000`
         - `reason.retarget_keep_source_arc=540000`
         - `reason.retarget_alias_update=540000`
       - exact `kilo_micro_array_string_store`:
         - `reason.source_kind_via_object=800000`
         - `reason.retarget_keep_source_arc=800000`
         - `reason.retarget_alias_update=800000`
     - borrowed alias whole-kilo truth:
       - `borrowed.alias.borrowed_source_fast=540000`
       - `borrowed.alias.as_str_fast=540064`
       - `borrowed.alias.as_str_fast_live_source=540064`
       - `borrowed.alias.as_str_fast_stale_source=0`
       - `borrowed.alias.array_len_by_index_latest_fresh=1`
       - `borrowed.alias.array_indexof_by_index_latest_fresh=938`
       - `borrowed.alias.encode_epoch_hit=0`
       - `borrowed.alias.encode_ptr_eq_hit=0`
       - `borrowed.alias.encode_to_handle_arc=0`
       - `borrowed.alias.encode_to_handle_arc_array_get_index=0`
       - `borrowed.alias.encode_to_handle_arc_map_runtime_data_get_any=0`
     - current read:
       - retargeted latest-fresh aliases are not escaping through encoder fallback
       - caller-attributed encode-to-handle paths are also closed in current behavior
       - `BorrowedHandleBox::as_str_fast()` stays entirely on the live-source side in whole-kilo
       - `array_string_len_by_index(...)` / `array_string_indexof_by_index(...)` are not the 540k latest-fresh culprit
       - the remaining stable object pressure stays on `store.array.str -> with_handle(ArrayStoreStrSource)` itself, not alias runtime encode
     - full object API demand also stays closed on the current culprit:
       - `borrowed.alias.to_string_box_latest_fresh=0`
       - `borrowed.alias.equals_latest_fresh=0`
       - `borrowed.alias.clone_box_latest_fresh=0`
     - latest landed keep-anchor cold fallback narrowing:
       - `BorrowedHandleBox::{to_string_box,type_name,is_identity}` now derive cold semantics from verified text anchor + keep class instead of stable object fallback
       - `to_string_box` now uses cold owned copy-out
       - `type_name` is derived from `TextKeepClass`
       - `is_identity` is fixed `false`
       - current exact/whole reread:
         - `kilo_micro_array_string_store`: `182 ms`
         - `kilo_micro_concat_hh_len`: `65 ms`
         - `kilo_kernel_small_hk`: `696 ms`
       - read:
         - this is a structure-first slice, not a hot-path trim
         - it removes more object-like behavior from the keep surface
         - remaining cold object fallback work is mostly `equals` and explicit promotion paths
       - current hot path is not using `BorrowedHandleBox` full stable-object APIs at all
     - latest landed encode object-demand sealing:
       - borrowed-alias encode planning/fallback execution now stays inside `value_codec/borrowed_handle.rs`
       - `encode.rs` no longer reaches into alias cold object helpers for:
         - fallback scalar check
         - pointer-equality reuse
         - fallback handle issue
       - removed encode-only cross-module helper surface:
         - `encode_fallback_box_ref()`
         - `clone_stable_box_for_encode_fallback()`
         - `ptr_eq_source_object()`
       - probe-only split-ladder reread (`repeat=1`):
         - `kilo_micro_concat_hh_len: 62 ms`
         - `kilo_micro_array_string_store: 183 ms`
         - `kilo_meso_substring_concat_len: 40 ms`
         - `kilo_meso_indexof_append_array_set: 148 ms`
         - `kilo_kernel_small_hk: 693 ms`
     - latest landed borrowed-alias equals cold split:
       - `BorrowedHandleBox::equals` now routes through a cold helper in `value_codec/borrowed_handle.rs`
       - `maybe_borrow_string_handle_with_epoch(...)` and `maybe_borrow_string_keep_with_epoch(...)` now use cold owned-box promotion helpers for `StringView` / non-borrowable keep paths
       - regression tests now pin:
         - borrowed-alias equality against plain `StringBox`
         - borrowed-alias equality across distinct source handles with the same text
         - `StringView` store-from-source materialization into owned `StringBox`
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib store_string_box_from_source` -> 4 passed
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib borrowed_alias_equals_same_text_from_distinct_sources` -> 1 passed
     - latest landed `store.array.str` non-string source-presence split:
       - `ArrayStoreStrSource::OtherObject` no longer transports `Arc<dyn NyashBox>` across the executor seam
       - `with_array_store_str_source(...)` still classifies under `with_handle(...)`, but the non-string branch now carries presence-only contract
       - `maybe_store_non_string_box_from_verified_source(...)` now consumes only `source_handle` / `drop_epoch`
       - regression tests now pin:
         - `with_array_store_str_source(...)` -> `OtherObject` for live non-string handles
         - `with_array_store_str_source(...)` -> `Missing` for dropped handles
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 19 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
     - latest landed source-lifetime helper sealing:
       - `array_string_slot.rs` no longer clones keep / rebuilds proof directly from `VerifiedTextSource`
       - string-like retarget/store now go back through `value_codec` helpers:
         - `try_retarget_borrowed_string_slot_take_verified_text_source(...)`
         - `store_string_box_from_verified_text_source(...)`
       - `VerifiedTextSource` now owns keep-preserving rewrite helpers for the retry path
       - regression tests now pin:
         - retarget success from verified string source into borrowed alias slot
         - retry `Err` path preserves `StringView` semantics before store fallback
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 21 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
     - latest landed by-value `VerifiedTextSource` consumption:
       - hot retarget path no longer clones keep before calling `try_retarget_borrowed_string_slot_take_keep(...)`
       - `VerifiedTextSource` now hands off keep by value:
         - `into_keep()`
       - string-like store now also consumes keep by value through:
         - `store_string_box_from_source_keep_owned(...)`
         - `store_string_box_from_verified_text_source(...)`
       - regression tests now pin:
         - retarget success from verified string source into borrowed alias slot
         - retry `Err` path preserves `StringView` semantics before store fallback
         - owned keep store path keeps borrowed alias for string handles
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 22 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
       - exact front probe (`repeat=3`):
         - `kilo_micro_array_string_store: 174 ms`
     - latest landed source-kind observation split:
       - `with_array_store_str_source(...)` now returns `StringHandleSourceKind` alongside the payload
       - `array_string_slot.rs` now consumes the kind explicitly for planning instead of asking the payload again
       - regression tests now pin:
         - `StringLike`
         - `OtherObject`
         - `Missing`
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 22 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
       - exact front probe (`repeat=5`):
         - `kilo_micro_array_string_store: 178 ms`
    - latest landed const-suffix cache split:
      - `execute_const_suffix_contract(...)` now uses module-level cache helpers instead of carrying the text-cache closure shape inside the function body
      - hot cached-handle lookup stays on the same semantics, but the cache/read structure is flatter
       - accept-gate reread:
         - `kilo_micro_concat_const_suffix: 81 ms`
         - `kilo_meso_indexof_append_array_set: 166 ms`
         - `kilo_kernel_small_hk: 696 ms`
     - latest landed const-suffix meta/text cache split:
       - cached metadata and cached suffix text are now stored in separate TLS caches
       - hot cached-handle lookup reads only metadata:
         - `ptr`
         - `handle`
         - `is_empty`
       - `RefCell<Option<String>>` text cache is only touched on reload or cold text fallback
       - accept-gate reread:
         - `kilo_micro_concat_const_suffix: 74 ms`
         - `kilo_meso_indexof_append_array_set: 149 ms`
         - `kilo_kernel_small_hk: 695 ms`
     - landed structural slice:
       - `ArrayStoreStrSource` now owns the source `Arc`
       - `with_array_store_str_source(...)` completes host-handle source read before `arr.with_items_write(...)`
       - `store.array.str` no longer nests host-handle read-lock across planner/retarget execution
     - current 3-run plain-release recheck on the landed slice:
       - `kilo_micro_array_string_store: 189 ms`
       - `kilo_micro_concat_hh_len: 67 ms`
       - `kilo_kernel_small_hk: 745 ms`
     - current read:
       - this is not a large exact-front win
       - but it is a cleaner source-contract split and keeps whole-kilo near the good end of the current band
     - target assembly shape remains:
         - planner-proved `RetargetAlias` should become metadata-heavy code
         - generic object fetch/downcast should disappear from the hot retarget path except for true source-lifetime keep
  - current design freeze:
      - do not add a public MIR op for `RetargetAlias`
      - carry only:
        - `source_preserve`
        - `identity_demand`
  - latest keep stop-line slice:
    - `SourceLifetimeKeep` is now opaque on its surface; backing representation stays internal
    - target reading is now explicit:
      - `TextKeep`
      - `AliasSourceMeta`
      - cold copy-out to owned text
    - `ArrayStoreStrSource::StringLike(...)` now carries a typed `VerifiedTextSource`
    - `StringView` keep fallback now uses cold owned text copy-out instead of object-like fallback cloning through the keep surface
    - current accept gate:
      - `kilo_micro_array_string_store: 170 ms`
      - `kilo_micro_concat_hh_len: 61 ms`
      - `kilo_kernel_small_hk: 715 ms`
         - `publication_demand`
         above Rust
       - keep runtime narrowables as backend-private seams:
         - `StringLikeProof`
         - `TextKeep`
         - `AliasSourceMeta`
     - latest landed structural split:
       - `BorrowedHandleBox` now separates `TextKeep` from `AliasSourceMeta`
       - `SourceLifetimeKeep` remains the current keep carrier, still backed by `StableBox(...)`
       - this is a no-behavior split to narrow the next cut to keep semantics
       - accept-gate reread:
         - `kilo_micro_array_string_store: 175 ms`
         - `kilo_micro_concat_hh_len: 63 ms`
         - `kilo_kernel_small_hk: 703 ms`
     - closed follow-up:
       - replacing `with_handle(ArrayStoreStrSource)` with direct `get()` source load regressed slightly
       - 3-run plain release:
         - `kilo_micro_array_string_store: 192 ms`
         - `kilo_micro_concat_hh_len: 69 ms`
         - `kilo_kernel_small_hk: 747 ms`
       - revert the behavior change; keep `with_handle_caller(...)` for now
        - planner says this hot path is pure `RetargetAlias`
        - the expensive escalation therefore happens before action selection, not because planner asked for `NeedStableObject`
        - but current `retarget` still needs source-object keep:
          - `source_kind_via_object`
         - `retarget_keep_source_arc`
         - `retarget_alias_update`
       - no-behavior-change `source_kind_check` split is now landed:
         - `StringHandleSourceKind`
         - `classify_string_handle_source(...)`
         - `array_string_slot.rs` planning now reads that contract instead of open-coding string-like checks
       - next structural slice is therefore:
         - split `source_kind_check` from `keep_source_arc`
         - do not assume object entry can simply disappear
     - current first widening target is therefore:
       - `store.array.str` source read under `array_string_slot.rs`
     - attempted widening truth:
       - redirecting `store.array.str` source read into `TextReadSession` moved latest fresh demand out of `object_with_handle(...)`
       - but plain release regressed:
         - `kilo_micro_array_string_store: 181 -> 187 ms`
         - `kilo_kernel_small_hk: 757 -> 916 ms`
       - the behavior change is reverted; keep the caller attribution only
     - narrow `retarget` retry truth:
       - a no-op guard in `try_retarget_borrowed_string_slot_verified(...)` for unchanged `(source_handle, source_drop_epoch)` did not materially move the front
       - plain release recheck:
         - `kilo_micro_array_string_store: 183 ms`
         - `kilo_kernel_small_hk: 746 ms`
       - the behavior change is reverted; keep the counter truth only
     - latest-fresh stable object cache truth:
       - caching the newest `Arc<dyn NyashBox>` in TLS and short-circuiting `with_handle(ArrayStoreStrSource)` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 210 ms`
         - `kilo_micro_concat_hh_len: 78 ms`
         - `kilo_kernel_small_hk: 760 ms`
       - the behavior change is reverted
     - borrowed alias raw string cache truth:
       - caching source string addr/len inside `BorrowedHandleBox` and bypassing `inner.as_str_fast()` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 196 ms`
         - `kilo_micro_concat_hh_len: 69 ms`
         - `kilo_kernel_small_hk: 798 ms`
       - the behavior change is reverted
     - typed string payload truth:
       - issuing fresh string handles through a typed `StringBox` payload and using a typed retarget fast path regressed the exact fronts immediately
       - plain release 3-run:
         - `kilo_micro_array_string_store: 201 ms`
         - `kilo_micro_concat_hh_len: 72 ms`
       - whole-kilo was not pursued; the behavior change is reverted
     - cloned source-arc retarget truth:
       - hot `RetargetAlias` was retried with a narrow `clone source Arc first, then retarget` slice
       - plain release 3-run regressed across exact and whole:
         - `kilo_micro_array_string_store: 205 ms`
         - `kilo_micro_concat_hh_len: 91 ms`
         - `kilo_kernel_small_hk: 959 ms`
       - the behavior change is reverted
       - keep only the no-behavior structural split:
         - `StoreArrayStrPlan` separates planner from executor
         - borrowed retarget now exposes `keep_source_arc` / `alias_update` helpers
     - typed `ArrayStoreStrSource` helper is now landed:
       - `with_array_store_str_source(...)` wraps `with_handle_caller(ArrayStoreStrSource)`
       - `store.array.str` now consumes a typed source contract instead of open-coding generic object entry at the executor callsite
       - this remains the landed no-behavior seam
     - typed helper internal bypass truth:
       - trying `with_str_handle(...)` first inside `with_array_store_str_source(...)` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 190 ms`
         - `kilo_micro_concat_hh_len: 67 ms`
         - `kilo_kernel_small_hk: 783 ms`
       - the behavior change is reverted
     - `keep_source_arc` ptr-eq truth:
       - observe direct probe now runs again via sync-stamp aligned perf-observe lane
       - exact `kilo_micro_array_string_store`:
         - `reason.retarget_keep_source_arc_ptr_eq_hit=0`
         - `reason.retarget_keep_source_arc_ptr_eq_miss=800000`
       - whole `kilo_kernel_small_hk`:
         - `reason.retarget_keep_source_arc_ptr_eq_hit=0`
         - `reason.retarget_keep_source_arc_ptr_eq_miss=540000`
     - `keep_source_arc` always sees a different source object on the current culprit path
     - clone-elision / ptr-eq guard ideas are closed
     - borrowed string keep seam is now landed:
       - `BorrowedHandleBox` keep-side contract is explicit as `BorrowedStringKeep`
       - current behavior still uses `StableBox(...)` only
       - this is still no-behavior-change
       - next structural cut can target source-lifetime keep without widening generic object payloads
     - closed follow-up:
       - a typed `BorrowedStringKeep::StringBox` fast path regressed on both exact and whole
       - 3-run plain release:
         - `kilo_micro_array_string_store: 198 ms`
         - `kilo_micro_concat_hh_len: 71 ms`
         - `kilo_kernel_small_hk: 777 ms`
       - behavior change is reverted
       - current read:
         - transport-only typed keep is not enough
         - source-lifetime keep semantics must move before keep representation changes again
     - `TextSnapshot` keep retry truth:
       - narrow retarget-only `TextSnapshot` keep improved exact fronts:
         - `kilo_micro_array_string_store: 178 ms`
         - `kilo_micro_concat_hh_len: 65 ms`
       - but whole-kilo collapsed:
         - `kilo_kernel_small_hk: 1792 ms`
       - the behavior change is reverted
       - current read:
         - snapshot keep can win on the exact retarget micro
         - but mixed generic consumers still force enough on-demand objectization to lose badly on whole-kilo
    - latest landed retarget success slice:
      - move source `Arc` into alias keep on successful `RetargetAlias`
      - this removes one extra clone from the hot retarget path without widening host-handle payloads
      - 3-run plain release:
        - `kilo_micro_array_string_store: 178 ms`
        - `kilo_micro_concat_hh_len: 65 ms`
        - `kilo_kernel_small_hk: 740 ms`
    - latest landed live-source alias slice:
      - gate `BorrowedHandleBox::as_str_fast()` live/stale epoch probe behind `observe::enabled()`
      - this keeps observe truth unchanged while removing the plain-release hot-path epoch read
      - 3-run plain release:
        - `kilo_micro_array_string_store: 169 ms`
        - `kilo_micro_concat_hh_len: 61 ms`
        - `kilo_kernel_small_hk: 717 ms`
    - latest landed source-lifetime keep split:
      - `ArrayStoreStrSource::StringLike(...)` now carries `SourceLifetimeKeep`
      - retarget success path now consumes `try_retarget_borrowed_string_slot_take_keep(...)`
      - this is still no-behavior-change and keeps `StableBox(...)` underneath; it only fixes the next cut onto keep semantics
      - 3-run plain release reread:
        - `kilo_micro_array_string_store: 169 ms`
        - `kilo_micro_concat_hh_len: 63 ms`
        - `kilo_kernel_small_hk: 703 ms`
    - latest landed string-like proof split:
      - `SourceKindCheck` now carries `StringLikeProof` separately from `SourceLifetimeKeep`
      - `ArrayStoreStrSource::StringLike(...)` now keeps both:
        - `proof: StringLikeProof`
        - `keep: SourceLifetimeKeep`
      - `execute_store_array_str_slot(...)` now records string-like source observe truth from the typed source contract instead of repeating local downcasts
      - this is still no-behavior-change; it narrows the next cut to keep semantics rather than source-kind transport
      - 3-run plain release reread:
        - `kilo_micro_array_string_store: 173 ms`
        - `kilo_micro_concat_hh_len: 68 ms`
        - `kilo_kernel_small_hk: 713 ms`
    - latest landed keep API narrowing:
      - `SourceLifetimeKeep` now exposes only text/lifetime-side API on the keep seam
      - full object API stays on `BorrowedHandleBox` through `stable_box_ref()` instead of the keep carrier
      - representation is still `StableBox(...)`; this is API narrowing only
      - 3-run plain release reread:
        - `kilo_micro_array_string_store: 173 ms`
        - `kilo_micro_concat_hh_len: 63 ms`
        - `kilo_kernel_small_hk: 708 ms`
    - closed proof-carrying keep direct path:
      - carrying `StringLikeProof` inside `TextKeep` and using proof-specific `as_str_fast()` regressed
      - 3-run plain release:
        - `kilo_micro_array_string_store: 178 ms`
        - `kilo_micro_concat_hh_len: 67 ms`
        - `kilo_kernel_small_hk: 730 ms`
      - current read:
        - keep proof on the source-contract side
        - do not widen alias keep semantics with proof transport again
    - read-contract freeze:
      - `BorrowedHandleBox::as_str_fast()` stays a stable-object read only
      - `host_handles::with_str_handle(...)` / `with_text_read_session(...)` stay live-source session reads only
      - do not push registry-backed direct read into `as_str_fast()`
    - latest landed read-contract naming cleanup:
      - `SourceLifetimeKeep` stable-object text read is now named as stable-object read rather than generic fast read
      - `ArrayStoreStrSource::object_ref()` is now `stable_object_fallback_ref()`
      - this is still no-behavior-change; it aligns backend naming with the read-contract split
      - accept-gate reread:
        - `kilo_micro_array_string_store: 173 ms`
        - `kilo_micro_concat_hh_len: 62 ms`
        - `kilo_kernel_small_hk: 698 ms`
    - latest landed typed store-from-source split:
      - `store.array.str` now sends the string-like store path through `SourceLifetimeKeep` directly
      - generic object fallback remains only for `OtherObject / Missing`
      - this is still no-behavior-change at the representation layer; it narrows the next actual cut away from object-centric store fallback
      - accept-gate reread:
        - `kilo_micro_array_string_store: 175 ms`
        - `kilo_micro_concat_hh_len: 65 ms`
        - `kilo_kernel_small_hk: 699 ms`
    - latest landed object-fallback API narrowing:
      - removed the unified `ArrayStoreStrSource` object-fallback accessor
      - `StringLike` and `OtherObject` no longer rejoin through one object-ref API
      - this is still no-behavior-change; it keeps the string-like branch and object fallback branch structurally separate
      - accept-gate reread:
        - `kilo_micro_array_string_store: 171 ms`
        - `kilo_micro_concat_hh_len: 64 ms`
        - `kilo_kernel_small_hk: 700 ms`
    - latest landed object-demand API narrowing:
      - raw `stable_box_ref()` access no longer crosses module boundaries from keep/alias internals into encode/store callers
      - encode-side object demand now goes through intent helpers:
        - `encode_fallback_box_ref()`
        - `clone_stable_box_for_encode_fallback()`
        - `ptr_eq_source_object()`
      - store-from-source keep demand now goes through:
        - `clone_stable_box_for_store_fallback()`
      - this is still no-behavior-change; it narrows object-demand API shape without changing keep representation
      - accept-gate reread:
        - `kilo_micro_array_string_store: 172 ms`
        - `kilo_micro_concat_hh_len: 68 ms`
        - `kilo_kernel_small_hk: 711 ms`
    - latest landed compatibility shim removal:
      - removed unused pre-keep string/source helpers:
        - `try_retarget_borrowed_string_slot_with_source(...)`
        - `try_retarget_borrowed_string_slot_verified(...)`
        - `keep_borrowed_string_slot_source_arc(...)`
        - `store_string_box_from_string_source(...)`
      - current structural path is now explicit:
        - retarget: `try_retarget_borrowed_string_slot_take_keep(...)`
        - store-from-source: `store_string_box_from_source_keep(...)`
      - this is still no-behavior-change; it removes compatibility entry points
        that were keeping the old object-centric shape visible
      - accept-gate reread:
        - `kilo_micro_array_string_store: 176 ms`
        - `kilo_micro_concat_hh_len: 63 ms`
        - `kilo_kernel_small_hk: 691 ms`
    - latest landed module string dispatch cleanup:
      - removed `plugin/module_string_dispatch.rs` direct `host_handles::get()/to_handle_arc()` bypass for compat string handle decode/encode
      - compat decode now goes through `value_codec::owned_string_from_handle(...)`
      - compat encode now goes through `materialize_owned_string(...)`
      - this keeps the compat path inside the `value_codec` seam and removes the review's last direct `host_handles` bypass from `module_string_dispatch.rs`
      - accept-gate reread:
        - `kilo_micro_array_string_store: 176 ms`
        - `kilo_micro_concat_hh_len: 67 ms`
        - `kilo_kernel_small_hk: 712 ms`
      - physical `string_store.rs` file split remains deferred until the keep semantics change lands
    - latest landed encode planner/executor split:
      - `runtime_i64_from_box_ref_caller(...)` no longer mixes borrowed-alias reuse planning and fallback handle issue in one block
      - planner now decides:
        - `ReuseSourceHandle`
        - `ReturnScalar`
        - `EncodeFallback`
      - executor now performs only the fallback publication mechanics
      - this is no-behavior-change structure cleanup for the review's `encode.rs` concern
      - accept-gate reread:
        - `kilo_micro_array_string_store: 179 ms`
        - `kilo_kernel_small_hk: 739 ms`
      - one `1014 ms` whole-kilo outlier was discarded after the immediate reread returned to the current WSL band
    - latest landed `SourceLifetimeKeep` subtype semantics:
      - keep contract now distinguishes verified string-like source subtype:
        - `StringBox`
        - `StringView`
      - borrowed alias creation now consumes keep semantics directly:
        - `maybe_borrow_string_keep_with_epoch(...)`
      - `store.array.str` string-like path now constructs keep from the verified subtype instead of treating keep as a generic stable object
      - representation is still `Arc<dyn NyashBox>` underneath; this is a keep-semantics cut, not a transport widening
      - accept-gate reread:
        - `kilo_micro_array_string_store: 175 ms`
        - `kilo_kernel_small_hk: 699 ms`
    - latest landed `string_classify.rs` split:
      - moved `SourceKindCheck` and typed `ArrayStoreStrSource` construction out of `string_store.rs` into `value_codec/string_classify.rs`
      - `string_store.rs` now keeps materialize/objectize/publication and store fallback execution only
      - this is a no-behavior physical split after the module-string layer bypass removal
      - accept-gate reread:
        - `kilo_micro_array_string_store: 174 ms`
        - `kilo_kernel_small_hk: 715 ms`
      - one `1894 ms` whole-kilo outlier was discarded after the immediate reread returned to the current band
    - next observation order is fixed:
     1. split the `store.array.str -> with_handle(ArrayStoreStrSource)` object contract again before changing behavior
     2. keep borrowed alias string-read trimming closed; live-source fast read was not enough
     3. keep typed `StringBox` payload widening closed at the host-handle layer
     4. keep `keep_source_arc` clone-elision ideas closed; ptr-eq never hits on the current culprit
     5. keep typed `BorrowedStringKeep::StringBox` fast path closed; transport-only specialization still loses
     6. do not add more typed-helper transport; move the next cut to the source-lifetime contract side
     7. use `BorrowedStringKeep` as the backend-private seam, but change keep semantics before keep representation changes again
     8. only then retry delayed `StableBoxNow`
   - `DeferredString` experiment truth:
     - exact micro improved:
       - `kilo_micro_concat_hh_len`: `57 -> 51 ms`
       - `kilo_micro_concat_birth`: `47 -> 35 ms`
     - whole-kilo probe regressed:
       - `kilo_kernel_small_hk`: `741 -> 952 ms`
     - code was reverted
     - next widening choice is now:
       1. explain the whole-kilo regression first
       2. only then reconsider pair/span widening
   - `host_handles` now has a source-backed payload seam:
     - slot storage reads through `HandlePayload::StableBox(...)`
     - public registry APIs still return `Arc<dyn NyashBox>`
     - this does not change behavior yet; it only narrows the future widening point for `DeferredStableBox`
     - single-handle string-only access is also separated now:
       - `host_handles::with_str_handle(...)`
       - `string_len_from_handle(...)` and `string_is_empty_from_handle(...)` consume that seam
   - current exact backend front is therefore:
     - `FreshHandle`
     - `MaterializeOwned`
   - target string-chain assembly shape:
     - `concat_hh + len_h` should spend most cycles in text/materialize work, not registry/object machinery
     - `StableBoxNow` and `FreshRegistryHandle` should move to sink/object boundaries only
   - current birth backend split now reads:
     - `StringBox` ctor side before registry issue
     - direct probe now also shows:
       - `string_box_ctor_total=800000`
       - `string_box_ctor_bytes=14400000`
       - `arc_wrap_total=800000`
     - observe-build `kilo_micro_concat_birth` microasm top:
       - `birth_string_box_from_owned`: `38.23%` to `41.46%`
       - `issue_string_handle_from_arc`: `27.66%` to `31.54%`
       - `__memmove_avx512_unaligned_erms`: `9.10%` to `10.88%`
       - `string_concat_hh_export_impl`: `11.53%` to `12.73%`
   - release observe direct probe now confirms second-axis counters too:
     - `objectize_stable_box_now_total=800000`
     - `objectize_stable_box_now_bytes=14400000`
     - `issue_fresh_handle_total=800000`
   - `kilo_micro_concat_birth` observe-build microasm after backend split now reads:
     - `materialize_owned_bytes`: `25.81%`
     - `issue_fresh_handle`: `24.62%`
     - `StringBox::perf_observe_from_owned`: `21.27%`
     - `__memmove_avx512_unaligned_erms`: `14.67%`
     - `nyash.string.concat_hh`: `5.81%`
   - annotate for `issue_fresh_handle(...)` shows the dominant local leaf is the final registry unlock/release path
   - next backend front is therefore:
     1. `materialize_owned_bytes`
     2. `issue_fresh_handle`
     3. `StringBox::perf_observe_from_owned`
   - do not spend more time on concat/len route guessing for these exact fronts unless a future counter contradicts the current read
   - `objectize_stable_string_box` stays as the seam name, but most runtime cost is currently absorbed by ctor/issue leaves
   - backend second-axis lock:
     - top-level Birth / Placement vocabulary stays unchanged
     - `box_id` is not promoted into that vocabulary
     - backend-only reading is now:
       - `Objectization = None | StableBoxNow | DeferredStableBox`
       - `RegistryIssue = None | ReuseSourceHandle | FreshRegistryHandle`
     - current `concat_birth` path still couples:
       - `FreshHandle`
       - `MaterializeOwned`
       - `StableBoxNow`
       - `FreshRegistryHandle`
     - current source-backed backend split is now visible in `string_store.rs`:
       - `materialize_owned_bytes`
       - `objectize_stable_string_box`
       - `issue_fresh_handle`
     - second-axis counters now also exist for:
       - `objectize_stable_box_now_total / bytes`
       - `issue_fresh_handle_total`
     - observe lane contract is now fail-fast:
       - default perf AOT lane aborts unless `target/release/.perf_release_sync` is newer than both `target/release/libnyash_kernel.a` and `target/release/hakorune`
       - `NYASH_PERF_COUNTERS=1` / `NYASH_PERF_TRACE=1` still require `target/release/.perf_observe_release_sync`
       - canonical rebuild orders are fixed in `tools/perf/build_perf_release.sh` and `tools/perf/build_perf_observe_release.sh`
       - helper-local ranking rule:
         - plain release asm = real cost ranking
         - observe build = counts and symbol split
         - `materialize_owned_bytes(...)` observe annotate is currently dominated by TLS counter work, so it is not sufficient by itself for first-front ordering
  - current microasm read:
     - `string_concat_hh_export_impl`: `54.04%`
     - `string_len_from_handle`: `21.37%`
     - `__memmove_avx512_unaligned_erms`: `15.40%`

## Next

1. keep canonical contract corridor landed and immutable
2. treat `kilo_micro_substring_only` as the current exact front
   - current AOT consumer: the preloop `nyash.box.from_i8_string_const` setup plus one `nyash.string.len_h` read
   - current executor: landed borrowed-corridor sink plus the DCE-triggered second corridor sweep
   - use `3 runs + perf` before judging any WSL delta
   - read this front through the substring/len boundary first
    - latest exact reread after the second corridor sweep is `C 3 ms / AOT 3 ms`, with perf counters `instr 1,669,909`, `cycles 1,061,204`, `cache-miss 8,516`
    - emitted MIR on this front now contains no `substring_len_hii` / `substring_hii`
    - current microasm keeps one preloop `len_h` read and the hot loop is scalar `add %rax,%rcx`
    - current validation blockers are cleared:
      1. quick gate is green again after retargeting the stale RawMap clear/delete guard grep from `map_substrate.rs` to `map_aliases.rs`
      2. whole-kilo strict accept is green again after the `concat_hs` deadlock fix plus the const/dynamic split
    - next backend trim order now that the sink is landed:
      1. broader string corridor placement/effect rewrite on the mixed gate family
      2. `nyash.string.len_h` only if a post-startup split says the preloop read reopened
      3. `nyash.box.from_i8_string_const` startup duplication only if the exact front reopens after the corridor rewrite
3. keep canonical `concat_birth` / fresh-box materialization as the secondary front
   - current AOT consumer: `nyash.string.concat_hh` plus the `materialize_owned_string(...)` backend seam
   - current executor: `string_concat_hh_export_impl(...)` + `materialize_owned_string(...)`
   - use `kilo_micro_concat_birth` as the exact isolated repro before changing this front
4. keep canonical `store.array.str` as the fallback exact front when the seam reopens
   - current executor: `array_string_store_handle_at(...)`
5. keep canonical `const_suffix` / `thaw.str + lit.str + str.concat2 + freeze.str` as the second route, but do not assume the current exact micro exercises it
6. use exact micro + whole-kilo together before moving to a new leaf
