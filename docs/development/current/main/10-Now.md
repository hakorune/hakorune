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
  - owner-target reminder:
    - except for OS / process / file / env boundaries, backend / ABI / alloc / GC / kernel substrate, and explicit compat/bootstrap keeps already named in SSOT, implementation should move toward `.hako`
    - do not leave new compiler meaning / route policy / semantic helper logic in Rust for convenience
    - mirbuilder migration double-work guard:
      - stop adding new syntax/AST-close lowering, source-shape recognizers, or builder-local sugar to `src/mir/builder/**` unless it is required for current canonical MIR parity/cleanup
      - keep new optimization work on canonical MIR contracts, MIR-to-MIR passes, and backend lowering where `.hako` and Rust builders share the same asset
      - next selfhost structural move is `.hako` builder authority replacement; Rust builder should trend toward oracle/fallback, not renewed mainline growth
      - fixed structural task order when that lane reopens:
        1. inventory Rust builder growth surfaces (`src/mir/mod.rs` `detect_*`, `src/mir/builder/**`, `src/host_providers/mir_builder*.rs`)
        2. lock `.hako builder` vs `Rust oracle` canonical MIR compare pack
        3. keep helper routing selfhost-first and Rust route explicit oracle/compat only
        4. retire builder-intelligence families one by one
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
    - metadata-bearing Point local-i64, Flag local-bool, and PointF local-f64 user-box fixtures are now green on `phase163x_boundary_user_box_metadata_keep_min.sh` via boundary `pure-first` owner lane without compat replay, including the Point, Flag, and PointF single-copy alias routes
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
    - first thin-entry method actual-consumer slice is now landed too:
      - LLVM/Python `mir_call.method_call` now consults `user_box_method.known_receiver` selector rows before the direct known-box fallback
      - when the selector chooses `thin_internal_entry`, lowering now takes a dedicated thin-known-receiver direct method route beneath canonical `Call`
      - the previous direct known-box call remains as compatibility fallback, so existing lowered user-box methods keep working while the selector becomes a real consumer
      - native-driver/shim now also has a first narrow boundary pure-first consumer slice for the same selector contract:
        - `tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_method_known_receiver_min.sh` now pins metadata-bearing `Counter.step`, `Counter.step_chain`, and `Point.sum` fixtures
        - `lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed.inc` now consumes `user_box_method.known_receiver` together with the matching scalar field selections for `Counter.value` and `Point.{x,y}`
        - the widened owner-lane slice now also accepts one local receiver `copy` and the one-hop recursive delegate, while still staying local-i64 + known-receiver only; broader generic local-method parity remains separate
      - canonical known-receiver callsite rewrite is now landed:
        - `callsite_canonicalize` rewrites known user-box receiver calls from `RuntimeDataBox`/union and `Global <Box>.<method>/<arity>` into canonical `Call(Method{box_name=<Box>, certainty=Known, box_kind=UserDefined})`
        - `tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_counter_step_contract.sh` pins the current direct-route `Counter.step` contract on `bench_kilo_micro_userbox_counter_step.hako`
      - first measured local-method keeper is now landed:
        - `benchmarks/bench_kilo_micro_userbox_counter_step.hako` + `benchmarks/c/bench_kilo_micro_userbox_counter_step.c`
        - exact reread: `kilo_micro_userbox_counter_step` = `c_instr=127,242 / c_cycles=208,224 / c_ms=3` vs `ny_aot_instr=465,881 / ny_aot_cycles=794,663 / ny_aot_ms=3`
        - current `ny_main` object snippet is `mov $0x52041ab, %eax ; ret`, so the current stop-line reads as startup/process overhead rather than loop churn
      - second measured local-method keeper is now landed:
        - `benchmarks/bench_kilo_micro_userbox_point_sum.hako` + `benchmarks/c/bench_kilo_micro_userbox_point_sum.c`
        - `tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_point_sum_contract.sh` pins the current direct-route `Point.sum` contract
        - exact reread: `kilo_micro_userbox_point_sum` = `c_instr=127,235 / c_cycles=216,542 / c_ms=3` vs `ny_aot_instr=465,837 / ny_aot_cycles=1,127,654 / ny_aot_ms=3`
        - current `ny_main` object snippet is `mov $0x5b8d83, %eax ; ret`
      - recursive one-hop delegate keeper is now landed:
        - `benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako` + `benchmarks/c/bench_kilo_micro_userbox_counter_step_chain.c`
        - `phase163x_direct_emit_user_box_counter_step_chain_contract.sh` pins the current direct-route `Counter.step_chain` contract
        - `phase163x_boundary_user_box_method_known_receiver_min.sh` now keeps `Counter.step_chain` green on boundary pure-first without compat replay
        - exact reread: `kilo_micro_userbox_counter_step_chain` = `c_instr=127,245 / c_cycles=230,857 / c_cache_miss=3,693 / c_ms=3` vs `ny_aot_instr=466,852 / ny_aot_cycles=836,012 / ny_aot_cache_miss=8,495 / ny_aot_ms=4`
        - current `ny_main` object snippet is `mov $0x2b, %eax ; ret`
    - portability-ci on `public-main` succeeded for commit `6b91896c0` (run `24211665863`), covering Windows check and macOS build (release)
  - verified post-Variant optimization order is now locked:
    1. `ny-llvmc` parity wave for the already-landed local enum/user-box routes
    2. sibling string retained-view `substring_hii` consumer expansion on the landed boundary `pure-first` corridor family
    3. broader string corridor genericization on the existing metadata path (do not add a new string-only MIR dialect):
       - landed: widen `string_corridor_candidates` into proof-bearing plan metadata
       - landed: keep direct `substring_concat3_hhhii` helper results on the same proof-bearing lane so `publication_sink` can read concat-triplet proof from the helper result itself
       - landed: helper-result `length()` / `substring()` now consume that same `publication_sink` plan in `string_corridor_sink` without crossing `phi_merge`
       - landed: boundary `pure-first` now reads `direct_kernel_entry` plan windows on helper-result receivers and lowers `length()` as window arithmetic on the landed non-`phi` corridor route
       - next: shrink the remaining dynamic and exact bridge paths that still do not read the plan directly
       - separate follow-on: any further `materialization_sink` widening across `phi_merge` or broader post-store windows only with a metadata-contract update first
       - shrink the temporary exact-seed bridge in `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`
    4. actual-consumer switch for selected thin-entry user-box method routes that are still metadata-only today (`user_box_method.known_receiver` first)
      - landed:
        - first LLVM/Python consumer slice for `user_box_method.known_receiver`
        - first broader native-driver/shim boundary pure-first consumer slice for the same selector contract (`Counter.step` + `Point.sum` local-i64 fixtures, plus their single-copy receiver aliases)
        - first measured local-method keeper (`kilo_micro_userbox_counter_step`)
        - second measured local-method keeper (`kilo_micro_userbox_point_sum`)
      - next follow-on: continue widening from measured contracts only; keep `ArrayBox` read-side observer evidence separate
    5. `ArrayBox` typed-slot expansion beyond the landed `InlineI64` pilot
       - landed next narrow slices: existing `slot_store_hih` / `slot_append_hh` any routes now birth/preserve `InlineBool` / `InlineF64` for `BoolBox` / `FloatBox` payloads without widening ABI rows
       - current stop-line: keep reads on the existing encoded-any `slot_load_hi` contract; do not add a new typed load row without observer evidence first (`kilo_micro_array_getset` still does not justify it)
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
    - Variant* inventory for this cut is now exhausted; the current `ny-llvmc` parity-wave keeper slice now covers Point / Flag / PointF direct+single-copy local keep routes
    - sibling string retained-view exact-micro consumer expansion is now landed:
      - boundary `pure-first` recognizes the current `kilo_micro_substring_views_only` exit-len shape and collapses it before `substring_hii` / `len_h` replay
      - latest exact reread: `instr=465,637 / cycles=704,757 / cache-miss=8,280 / AOT 3 ms`
      - latest microasm: `ny_main = mov $0x20, %eax ; ret`
    - sibling string mixed accept guardrail is now landed too:
      - `string_corridor_sink` now runs a second sweep after DCE so complementary `substring_len_hii` pairs can fuse once dead borrowed-string temps drop out
      - `kilo_micro_substring_only` now emits no `substring_len_hii` / `substring_hii`
      - latest exact reread: `instr=1,669,909 / cycles=1,061,204 / cache-miss=8,516 / AOT 3 ms`
      - latest microasm: `ny_main` now keeps only the preloop source-length read and the loop body is scalar `add %rax,%rcx`
    - sibling string retained-slice length consumer expansion is now landed too:
      - `string_corridor_sink` now rewrites `length()` / `len()` on retained slice values into `substring_len_hii` even when the slice producer lives in a dominating block and is only reached through local copy aliases
      - `kilo_micro_len_substring_views` now compiles without loop `RuntimeDataBox.length` / `substring_len_hii` consumers
      - latest exact reread: `instr=1,672,259 / cycles=1,022,005 / cache-miss=10,525 / AOT 3 ms`
      - latest split-pack reread keeps all three string split fronts in the same 3 ms band:
        - `kilo_micro_substring_only = instr=1,669,659 / cycles=1,077,794 / cache-miss=8,810`
        - `kilo_micro_substring_views_only = instr=466,001 / cycles=841,958 / cache-miss=9,391`
        - `kilo_micro_len_substring_views = instr=1,672,096 / cycles=1,009,964 / cache-miss=8,902`
    - next substep after the current parity-wave keeper: broader string corridor genericization on the mixed gate family
    - keeper repair landed: the exact `pure-first` `kilo_micro_substring_concat` seed now accepts the post-sink body shape (`substring_len_hii` pair + `substring_concat3_hhhii`), so the generic concat-observer rewrite keeps the exact lane instead of falling back
    - proof-bearing plan metadata widening is now landed: `StringCorridorCandidate` carries `plan` metadata for borrowed-slice and concat-triplet proofs, and MIR JSON exposes it for downstream consumers
    - first `publication_sink` inventory slice is now landed too: emitted MIR JSON on `kilo_micro_substring_concat` keeps the direct `substring_concat3_hhhii` helper result on the corridor lane with concat-triplet-backed `publication_sink` plan metadata
    - first actual `publication_sink` transform is now landed too: `string_corridor_sink` rewrites direct helper-result `length()` to `end - start` and composes direct helper-result `substring()` back into `substring_concat3_hhhii` from the same plan metadata
    - first non-`phi` `materialization_sink` slice is now landed too: when a direct `substring_concat3_hhhii` helper result has a single local `ArrayBox.set` consumer through copy aliases, `string_corridor_sink` now sinks that birth to the store boundary instead of keeping the helper earlier in the block
    - first post-store observer slice is now landed too: when that same direct helper result also has one trailing `length()` observer after the local `ArrayBox.set`, `string_corridor_sink` now keeps `array.set` as the first `Store` boundary, rewrites the observer to `end - start`, and removes copy-only store/observer chains
    - first plan-selected `direct_kernel_entry` slice is now landed too: boundary `pure-first` reads `string_corridor_candidates[*].plan.start/end` for direct helper-result receivers and lowers `length()` as the same window arithmetic (`end - start`) instead of rediscovering the route from legacy remembered substring calls
    - targeted boundary proof is now pinned on `string_direct_kernel_plan_len_window_min_v1.mir.json`; the smoke `phase137x_boundary_string_direct_kernel_plan_len_min.sh` must hit `substring_len_direct_kernel_plan_window` and keep `nyash.string.len_h` / `nyash.string.substring_len_hii` out of the lowered IR
    - latest bridge shrink removes the old `STRING_LEN -> string_substring_len` declaration path on this lane; direct-kernel len now trusts corridor plan metadata rather than substring-call re-inference
    - current `./target/release/hakorune --emit-mir-json{,-minimal}` probe on `bench_kilo_micro_substring_concat.hako` reads `interesting_n = 17`, and the active `phase29x_backend_owner_daily_substring_concat_loop_min` smoke now points at `apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1_post_sink.mir.json`, so exact-seed narrowing can follow the aligned post-sink body shape
    - the daily-owner route blocker is now cleared too: default `backend=mir` executes the compiled module again, and the `.hako ll emitter` runtime decl manifest now includes `nyash.string.substring_len_hii` / `nyash.string.substring_concat3_hhhii`, so the phase29x daily smoke reaches `[hako-ll/daily] ... acceptance_case=substring-concat-loop-v1`
    - trustworthy current-shape probe for this front is `tools/smokes/v2/lib/emit_mir_route.sh --route direct ...`; do not use `tools/hakorune_emit_mir.sh` as the shape oracle here because the helper can persist a non-strict JSON payload from selfhost stdout capture
    - current live post-sink benchmark body is now pinned separately by `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_post_sink_shape.sh`; that smoke requires the helper-result `%36` to keep `publication_sink` / `direct_kernel_entry` plans and the scalar consumers `%88/%89` to keep `direct_kernel_entry` candidates on the live MIR, and the exact seed now trusts those metadata-backed helper/scalar contracts instead of re-proving shared `source_root`, raw helper names/args, or the intermediate raw `substring` producers from emitted MIR
    - the same post-sink probe now also pins the exact-seed preheader/exit semantics (`StringBox.length()` on the seed input, then trailing `length() + ... + ret` on exit), so those truths are visible outside the seed even though the seed still owns the current guard
    - the first narrow `phi_merge` handoff is now pinned too by `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_phi_merge_contract.sh`; live direct MIR still carries `%21 = phi([4,0], [22,20])` and `%22 = phi([36,19])`, helper-result `%36` still owns the proof-bearing plan window, relation metadata now makes the stop line explicit (`%22 = preserve_plan_window`, `%21 = stop_at_merge`), and merged header phi `%21` still keeps only non-window `publication_sink` / `materialization_sink` / `direct_kernel_entry` continuity
    - the same phi smoke now also pins the exact-seed loop semantics (`phi/phi/phi`, positive loop bound, compare `<`, branch, and the latch `const 1` increment), so the remaining exact-seed work moved to a semantic-boundary decision rather than more raw body-shape cleanup
    - structure lock: loop-carried corridor continuity now consumes the generic MIR seam in `src/mir/phi_query.rs`; `src/mir/string_corridor_relation.rs` is now the string-side relation layer, and `string_corridor_placement` only maps stored `facts -> relations -> candidates` continuity to optimization candidates
    - decision now fixed: stop shrinking the exact seed at the semantic-guard boundary for this phase
      - keep preheader/exit `length` truth plus header/latch loop truth in the seed as the current miscompile-prevention owner
      - treat any future retirement of those semantic guards as a separate contract phase, not as more bridge cleanup in this wave
    - fresh broader-corridor reread keeps `kilo_micro_substring_concat` (`instr=5,565,547 / cycles=5,907,473 / cache-miss=8,629 / AOT 4 ms`) as the current exact front, while exploratory `kilo_meso_substring_concat_array_set` stayed essentially flat (`instr=384,347,679 / cycles=185,582,276 / AOT 42 ms`), so this cut is a canonical-MIR/kernel asset landing rather than a meso perf keeper by itself
    - first direct-set insert-mid smoke is now pinned too: `phase137x_boundary_string_insert_mid_direct_set_min.sh` uses the synthetic direct-set probe to observe `string_insert_mid_window`, keep `nyash.string.insert_hsi` in the lowered IR, and require the plan-backed `plan_window_match` route on the synthetic fixture
    - string genericization order is now fixed: keep canonical MIR as the only IR truth, land proof-bearing plan metadata first, then land helper-result `publication_sink` inventory, then helper-result actual `publication_sink`, then `materialization_sink`, then select `direct_kernel_entry` from that plan near lowering, then shrink the remaining bridge paths
    - migration-safe reading: keep this lane in canonical MIR facts/candidates/sink and kernel/backend substrate only; do not reopen Rust-builder-local shape logic while `.hako` builder authority replacement is open
    - the exact `pure-first` seed in `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc` is temporary bridge surface and should shrink only after the generic plan-selected route proves out
    - separate follow-on phase, not this cut: carry the actual plan window across `phi_merge`, or relax `call` / `boxcall` / `return` barriers, only with another metadata-contract update first; `phase137x_direct_emit_substring_concat_phi_merge_contract.sh` is now the live guard for the first handoff on that loop-carried route, and the post-store observer lane still owes `array.set + trailing length()` facts before widening beyond the new store-only sink
    - fixed return order:
      1. continue shrinking exact-seed structural checks only where the live post-sink metadata contract already proves the route
      2. landed: loop-carried base/root interpretation now sits behind the generic MIR seam `src/mir/phi_query.rs`
      3. landed first narrow `plan window across phi_merge` cut on the single-input backedge phi `%22`; merged `%21` is now explicitly `stop_at_merge` and any broader widening stays in a separate metadata-contract phase
    - sibling string follow-on after that: move from the landed exact micro to the broader corridor rewrite family on the mixed accept gate
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
