---
Status: Active
Date: 2026-06-02
Scope: active mimalloc migration, optimization, and provider-benchmark workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
---

# Mimalloc Current Workstream

This is the active restart card. It intentionally stays compact.

Full historical MIM-001..MIM-146 prose was archived to:

```text
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
```

Use that archive for exact old evidence. Use this file for the current decision
surface, next task order, and parking lot.

## Goal

Keep proving that `.hako` mimalloc can be built, packaged, and compared against
C mimalloc without opening product allocator replacement prematurely.

Current focus:

```text
provider/DLL benchmark bridge
same-machine C mimalloc comparison
next owner selection from local evidence
algorithm-port coverage separation
```

## Stop Line

- no new numbered row for inventory-only work
- no row-specific `.sh` guard
- no full external benchmark corpus import
- no copied benchmark executables in git
- no provider activation as product default
- no allocator replacement claim
- no hook installation claim
- no production `#[global_allocator]` claim
- no winner claim
- no source syntax expansion unless a tracked reference decision accepts it

## Current Decisions

```text
parity/front:
  direct exact remains the .hako mimalloc optimization front

public/default front:
  compatibility reference only; not the parity owner

provider package:
  handoff artifact and benchmark smoke path only

LD_PRELOAD shim:
  experiment/measurement bridge only

Hakozuna mixed-ws:
  repo-local CRT fixture is connected
  compare same-machine system malloc / C mimalloc / optional Hakorune provider
  do not compare Ubuntu numbers horizontally while CPU differs

record/state direction:
  PageModel remains box owner
  primitive mutable state may become record-shaped residence only through
  RecordStateResidencePlanV0-style metadata/plans

Inline(required):
  small receiver-local leaf helper only
  multi-block hot paths use HotCore/direct-exact plans instead

algorithm-port coverage:
  `.hako` hako_alloc policy/model coverage and benchmark-only replacement-front
  execution coverage are different surfaces. Do not read the fixed-slot
  replacement front as proof that the full `.hako` mimalloc algorithm is wired
  into LD_PRELOAD/product replacement.
```

## Algorithm Port Coverage

Use this section before selecting the next implementation owner.

Current reading:

```text
.hako model/policy coverage:
  size_class_policy=modeled
  page_local_free_stack=modeled
  same_thread_local_free=modeled
  object_lifecycle_hot_core=modeled
  page_map/realloc/huge/osvm/remote_free=policy_or_seam_modeled

benchmark-only replacement front execution:
  fixed_slot_native_free_stack=executed
  matched_fixed_slot_size=executed for selected fixtures
  hako_size_class_good_size_slot=executed only when
    --replacement-front-match-hako-size-class is passed
  product_bins=benchmark_native_bins_v0 when
    --replacement-front-native-bins-mode is passed
  product_pages=report_only_plan
  in_place_realloc_within_fixed_slot=executed
  thread_local_arena_remote_free_bridge=executed

not yet bridged:
  size_class_policy_to_product_replacement_pages
  .hako PageModel arrays to source DirectArrayI64 storage
  .hako HotCore/PageModel plans to replacement-front lowering
  general page queue / segment / OSVM product allocator front
```

Stable report entry:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py
```

Overlay a generated mixed-ws compare report when selecting the next bridge
owner from executed benchmark evidence:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py \
  --benchmark-report target/hakozuna-mixed-ws-page-bins-current/report.out
```

The overlay may show `replacement_front_product_bins_consumer_enabled=1` for a
benchmark-only route, while `replacement_front_is_full_hako_algorithm=0` and
`replacement_front_product_pages_consumer_enabled=0` keep product allocator
claims closed.

The first benchmark-only HotCore/PageModel bridge is available as:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --replacement-front-hotcore-page-model-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-hotcore-page-model/report.out \
  --out-dir target/hakozuna-mixed-ws-hotcore-page-model/artifacts \
  --sample-count 3
```

Expected route fields:

```text
replacement_front_algorithm_shape=page_bin_hotcore_page_model_benchmark_front
replacement_front_product_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_page_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_hotcore_consumer_enabled=1
replacement_front_hotcore_route=benchmark_page_bins_hotcore_page_model
replacement_front_product_pages_consumer_enabled=0
replacement_front_is_full_hako_algorithm=0
```

This bridge consumes the HotCore/PageModel-shaped alloc/free helper route in the
benchmark-only replacement front. It is still not product pages, activation,
hook installation, or a full `.hako` allocator algorithm claim.

Local 3-sample refresh:

```text
page_bins_refresh:
  report=target/hakozuna-mixed-ws-page-bins-refresh/report.out
  median_ops_per_sec=8,136,100.692
  hotcore_consumer_enabled=0

hotcore_page_model:
  report=target/hakozuna-mixed-ws-hotcore-page-model/report.out
  median_ops_per_sec=8,945,904.118
  hotcore_consumer_enabled=1
  route=benchmark_page_bins_hotcore_page_model
```

Interpretation: HotCore/PageModel wrapper mode is a structural bridge keeper
because it consumes the next `.hako` semantic boundary and improves over the
same-run page-bins refresh. It is not a winner/performance claim against the
older page-bins best sample or C mimalloc.

Rejected local probe:

```text
free_release_owned_direct_probe:
  attempted_change=replace free() out-param find_owned call with generated release_owned(ptr)
  asm_effect=free stack canary/out-param call removed
  median_ops_per_sec=8,522,097.800
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper

hotcore_cold_init_probe:
  attempted_change=mark generated init_bins as cold,noinline
  median_ops_per_sec=8,735,302.853
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper

page_map_backed_bridge_probe:
  attempted_change=add benchmark-only page-map register/lookup/unregister
    ownership table to page-bins replacement front
  report=target/hakozuna-mixed-ws-page-map-current/report.out
  product_pages_consumer_enabled=1
  page_bins_lookup_route=page_map_lookup
  median_ops_per_sec=4,021,684.925
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper
```

Interpretation: local C-shape cleanups can improve isolated assembly while
regressing end-to-end mixed-ws throughput. The naive page-map-backed ownership
bridge proves the report shape can consume product pages, but its linear lookup
is too expensive for the current optimization owner. Keep the existing
HotCore/PageModel bridge and do not re-open these probes without new perf owner
evidence.

The same algorithm coverage overlay now reports product-pages bridge readiness
without opening product replacement:

```text
replacement_front_product_pages_bridge_plan_v0=1
replacement_front_product_pages_bridge_report_only=1
replacement_front_product_pages_consumer_enabled=0
replacement_front_product_pages_route=not_consumed
replacement_front_product_pages_source_ready=1
replacement_front_product_pages_full_source_ready=1
replacement_front_product_pages_bridge_blocker=consumer_not_enabled
replacement_front_product_pages_next_bridge=page_map_backed_replacement_front_plan
page_map_source_ready=1
page_map_release_source_ready=1
realloc_same_class_source_ready=1
realloc_grow_copy_release_source_ready=1
huge_page_source_ready=1
osvm_page_source_pilot_ready=1
```

Interpretation: `.hako` PageMap, page-map release, same-class realloc,
grow/copy/release, huge model, and OSVM pilot seams are source-present for the
next bridge design. The replacement front still does not consume product pages,
and `replacement_front_is_full_hako_algorithm=0` remains the claim boundary.
The next implementation owner is a narrow page-map-backed benchmark bridge
plan, not product activation or a full allocator claim.

The same report carries the current PageModel hot-array readiness view:

```text
size_class_policy_product_bins_connected=0
size_class_policy_single_class_benchmark_bridge_supported=1
size_class_policy_single_class_bridge_mode=hako_good_size_request_ceiling
page_model_hot_array_bridge_plan_v0=1
page_model_hot_array_access_plan_v0=1
page_model_hot_array_source_migration_selected=1
page_model_hot_array_source_type_ready=1
page_model_hot_array_birth_contract_ready=1
page_model_hot_array_source_migration_blocker=none
page_model_hot_array_next_bridge=source_migration_measurement
page_model_hot_array_candidate_type=DirectArrayI64
page_model_hot_array_arraybox_fields=none
page_model_hot_array_directarray_fields=free,local_free,block_used
page_model_hot_array_directarray_supported_ops=get,set
page_model_hot_array_directarray_missing_ops=none
page_model_hot_array_seed_push_blocker=0
page_model_hot_array_op_summary=free:get=...:set=...:push=...
hotcore_replacement_bridge_plan_v0=1
hotcore_replacement_consumer_enabled=0
hotcore_replacement_shape_ready=1
hotcore_replacement_bridge_blocker=consumer_not_enabled
hotcore_replacement_next_bridge=replacement_front_consume_hotcore_page_model
hotcore_page_model_source_ready=1
hotcore_small_alloc_calls_acquire_fresh_small=1
hotcore_release_calls_release_local_known_live=1
page_model_hot_methods_ready=1
hotcore_source_methods=objectLifecycleSmallAlloc,objectLifecycleReleaseBlock
```

This is bridge-readiness reporting, not source migration or replacement-front
lowering consumption. The `.hako` HotCore/PageModel shape is ready; the current
gap is that the replacement front still does not consume it.

The PageModel hot-array access scan distinguishes hot `get/set` traffic from
seed-time initialization traffic. `seedFreeBlocks` uses append-or-overwrite
`set(i, ...)` shape for all three arrays, so the old `ArrayBox.push` blocker is
closed. The PageModel `free` / `local_free` / `block_used` fields are now
`DirectArrayI64` source fields.

Current implementation note: DirectArrayI64 source receivers are accepted by
the generic `get/set` route producer and by DirectArrayAccessPlan /
DirectArrayExtentFact metadata.

Explicit `new DirectArrayI64()` also lowers to the direct-i64 birth symbol in
the LLVM exact front. The next bridge is source migration measurement, not
another route/birth fixture.

Acceptance for claiming algorithmic completeness stays closed until the report
can show the `.hako` size-class/page-local/HotCore route as the executed
replacement-front path. Until then, replacement-front benchmark results are
thin-front evidence only:

```text
replacement_front_is_full_hako_algorithm=0
size_class_policy_product_bins_connected=0
provider_activation=0
production_replacement_active=0
winner_claim=0
```

## Current Evidence Anchors

Recent completed slices:

```text
MIM-116..MIM-123:
  state bucket / record-state residence / hako_check state-explain /
  source-surface responsibility cleanup

MIM-124..MIM-134:
  owner-first perf refreshes, body-timer control, workload matrix,
  direct-exact app perf/asm tooling, observer-light closeout

MIM-135..MIM-146:
  provider package explicit measurement, hakmem and hakozuna LD_PRELOAD
  bridge, generated global allocator smoke, provider export bundle,
  same-machine Hakozuna mixed-ws C mimalloc comparison
```

Latest local benchmark bridge:

```text
tool:
  tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py

fixture:
  benchmarks/external/hakozuna/mixed-ws/build/bench_mixed_ws_crt

reference_subject:
  c_mimalloc_ldpreload

optional_subject:
  hakorune_provider_ldpreload when --manifest is passed

report_contract:
  hakozuna-mixed-ws-ldpreload-compare-v0

interpretation:
  provider subject is provider ABI wrapper + LD_PRELOAD shim bridge evidence
  provider_ldpreload_is_hako_core_speed_claim=0
  provider_ldpreload_is_product_allocator_claim=0
  provider_manifest_hako_provider_alloc_free_route=host_malloc_free_wrapper
  provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle=0

key shim diagnostic:
  shim_init_real_fallback_per_provider_operation

owner hint:
  provider_alloc_free_internal_real_malloc_boundary when init fallback
  dominates provider operations
```

Provider package route split:

```text
object-lifecycle-small-alloc-release-v0:
  verifies selected .hako object-lifecycle MIR call chain
  provider alloc/free route = host_malloc_free_wrapper

object-lifecycle-native-slot-bridge-v0:
  verifies the same selected .hako object-lifecycle MIR call chain
  provider alloc/free route = native_static_slot_bridge_from_object_lifecycle_shape
  uses_host_malloc = 0
  use only as explicit provider-package / LD_PRELOAD bridge evidence
  product activation / hook / global allocator / winner claim remain closed
```

## Next Task Order

1. Keep algorithm-port coverage visible.
   - Run `tools/allocator/hako_mimalloc_algorithm_coverage.py` before opening a
     new allocator-algorithm claim.
   - If the report still says `replacement_front_is_full_hako_algorithm=0`,
     treat the benchmark-only front as thin-front evidence, not product
     allocator evidence.

2. Bridge `.hako` algorithm pieces only by explicit owner evidence.
   - First candidate: size-class policy to replacement bins/pages.
     `--replacement-front-match-hako-size-class` is the benchmark-only
     single-class bridge and may be used for local evidence. It does not claim
     product bins/pages.
     The compare report now emits product bins/pages fields. Without bins mode:
     `replacement_front_product_bins_consumer_enabled=0` and
     `replacement_front_product_pages_consumer_enabled=0`.
     `--replacement-front-native-bins-mode` is the next benchmark-only v0:
     it consumes workload regular bins but keeps pages/product activation
     closed.
   - Second candidate: `HakoAllocPageModel` hot arrays to DirectArrayI64-backed
     storage. This source migration is complete for `free` / `local_free` /
     `block_used`; do not keep probing local get/set shape unless new owner
     evidence selects it.
   - Third candidate, now primary after page-bins local probes:
     HotCore/PageModel plan consumption by replacement-front lowering. The
     benchmark-only HotCore/PageModel wrapper mode is connected; next decide
     keeper/nonkeeper from same-machine measurement.
   - Do not weaken ProviderFront or add source syntax to force the bridge.

3. Measure the native slot bridge provider route.
   - Build with `--provider-package-hako-semantic-codegen
     object-lifecycle-native-slot-bridge-v0`.
   - Confirm `hako_provider_alloc_free_uses_host_malloc=0`.
   - Compare against the previous host-wrapper provider route and C mimalloc.

4. Run a stable same-machine Hakozuna mixed-ws comparison.
   - Use enough iterations/samples to reduce startup noise.
   - Compare system malloc, C mimalloc LD_PRELOAD, and optional Hakorune
     provider LD_PRELOAD.
   - Treat the result as local evidence only.

5. Classify the remaining gap.
   - If provider shim counters dominate, optimize shim/provider boundary first.
   - If `.hako` allocator core dominates, return to direct-exact app perf/asm.
   - If benchmark setup noise dominates, improve measurement before code edits.
   - Do not call provider LD_PRELOAD evidence a `.hako` core speed result while
     `provider_ldpreload_is_hako_core_speed_claim=0`.
   - Use `HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE=1` only as a
     measurement-only native-slot shim variant. It bypasses shim pointer
     tracking through the private `hakorune_provider_usable_size_v0` export and
     exists to distinguish tracking tax from provider ABI call-boundary tax.
   - Use `HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED=1` only with
     usable-size mode in controlled benchmarks. It skips provider `owns` checks
     to isolate the remaining call-boundary tax and is not a replacement
     contract.

6. Continue provider replacement ladder only as smoke/readiness.
   - Keep `provider_activation=0`, `production_replacement_active=0`,
     `hook_installed=0`, `global_allocator_product_claim=0`,
     `winner_claim=0`.

7. Add a benchmark-only `HakoAllocReplacementFront` probe if C-like thinness is
   still required.
   - Do not weaken `HakoAllocProviderFront`.
   - The probe must bypass provider API table dispatch, hot function-pointer
     calls, shim pointer tracking, and hot `owns` checks.
   - Required report fields:

```text
provider_table_dispatch=0
function_pointer_hot_call=0
owns_check_hot_path=0
tracking_hot_path=0
direct_core_call=1
activation=0
benchmark_only=1
summary=ok
```

   - Tool entry:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --out-dir /tmp/hakorune_replacement_front_compare \
  --out /tmp/hakorune_replacement_front_compare/report.out
```

   - Locked multithread smoke entry:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --threads 4 \
  --out-dir /tmp/hakorune_replacement_front_locked_mt_compare \
  --out /tmp/hakorune_replacement_front_locked_mt_compare/report.out
```

   - Interpret the locked front as the first thread-safety shape only. It is
     allowed to lose throughput to C mimalloc because the point is to prove that
     the benchmark-only replacement front no longer uses an unsynchronized
     global free stack.

   - Thread-local arena probe entry:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --threads 4 \
  --out-dir /tmp/hakorune_replacement_front_tls_mt_compare \
  --out /tmp/hakorune_replacement_front_tls_mt_compare/report.out
```

   - Interpret the thread-local arena probe as the first scalable shape. It is
     still benchmark-only. Cross-thread free routes through a remote queue;
     cross-thread realloc remains unsupported and must stay visible in counters.
     Remote-free publication is serialized with the arena registry so owner
     thread exit cannot race with a foreign free touching owner arena storage.
     Use `--replacement-front-skip-hot-counters` only as a measurement-only
     counter-tax probe; it is incompatible with the focused counter smokes and
     is not allocator readiness evidence.
     Use `--replacement-front-tls-counter-mode` as the benchmark-only keeper
     direction for preserving counters while avoiding hot atomic increments in
     the thread-local replacement front.
     GNU/Linux thread-local replacement fronts use the initial-exec TLS model
     for the benchmark-only shim. This removes hot `__tls_get_addr` calls from
     the generated replacement front and remains benchmark-only evidence.
     Same-thread free uses a local-only fast path; remote owner publication is
     only consulted after the local slot check fails.

```text
thread_local_replacement_front_smoke=1
thread_local_arena=1
cross_thread_free_policy=remote_queue
replacement_front_arena_registry_overflow_count_total=0
activation=0
benchmark_only=1
winner_claim=0
summary=ok
```

   - The Hakozuna mixed-ws workload may not exercise cross-thread free. Use a
     focused cross-thread free smoke when changing this path:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-cross-thread-smoke \
  --threads 4 \
  --out-dir /tmp/hakorune_replacement_front_tls_cross_thread_compare \
  --out /tmp/hakorune_replacement_front_tls_cross_thread_compare/report.out
```

   - The focused smoke must require:

```text
replacement_front_cross_thread_free_smoke_ok=1
replacement_front_cross_thread_free_remote_free_push_count>=1
replacement_front_cross_thread_free_remote_free_drain_count>=1
replacement_front_cross_thread_free_arena_registry_overflow_count=0
replacement_front_cross_thread_realloc_smoke_ok=1
replacement_front_cross_thread_realloc_unsupported_count>=1
replacement_front_cross_thread_realloc_host_passthrough_count=0
```

   - If the owner thread has already exited, the benchmark-only front must not
     pass a recognized Hakorune pointer to host `free`. Mark it abandoned,
     count it, and keep product activation closed:

```text
replacement_front_abandoned_owner_smoke_ok=1
replacement_front_abandoned_owner_abandoned_arena_count>=1
replacement_front_abandoned_owner_abandoned_remote_free_count>=1
replacement_front_abandoned_owner_host_passthrough_count=0
activation=0
benchmark_only=1
winner_claim=0
```

6. Finish the Hakorune mimalloc lane before any victory claim.

## Replacement Front Hot-Path Plan

Historical REPL-001..REPL-018 evidence was archived to:

```text
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
```

Current replacement-front truth:

```text
fixed_slot_native_front=available
matched_hako_good_size_slot=available
multi_bin_native_benchmark_front=available_single_thread_v0
page_bin_benchmark_front=available_single_thread_v0
locked_global_multithread_front=positive_local_evidence_v0
thread_local_multithread_front=correctness_smoke_available_not_perf_keeper
product_pages=not_connected
provider_activation=0
production_replacement_active=0
winner_claim=0
```

Next replacement-front order:

1. Keep `--replacement-front-native-bins-mode` benchmark-only and single-thread
   until a thread/page plan is selected.
2. Treat the locked global counterless front as the current local multithread
   performance evidence owner; keep thread-local as correctness/smoke evidence
   until perf/asm selects a concrete thread-local hot cost.
3. Open product pages only after bins evidence says pages are the owner.
4. Reopen `.hako` core optimization only with fresh owner evidence.
5. Keep detailed evidence in the investigation archive, not this active card.

Product pages v0 boundary:

```text
replacement_front_page_bins_plan_v0=1
replacement_front_page_bins_supported=1
replacement_front_page_bins_consumer_enabled=0 by default; 1 only when
  --replacement-front-page-bins-mode is selected
replacement_front_page_bins_route=not_consumed | benchmark_page_bins
replacement_front_page_bins_owner=benchmark_only
replacement_front_page_bins_threading=single_thread_until_plan_selected
replacement_front_page_bins_product_claim=0
```

The first implementation is a benchmark-only page/bin-backed route that keeps
provider activation, product replacement, hooks, global allocator, and winner
claims closed. It consumes the workload regular bins and adds a page-shaped
owner structure, but it must not claim full `.hako` mimalloc until the coverage
report stops saying `replacement_front_is_full_hako_algorithm=0`.

## Daily Commands

Build the Hakozuna mixed-ws fixture:

```bash
make -C benchmarks/external/hakozuna/mixed-ws
```

Run same-machine C mimalloc comparison:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --out target/hakozuna-mixed-ws-compare/report.out \
  --out-dir target/hakozuna-mixed-ws-compare/artifacts \
  --sample-count 5
```

Add the optional Hakorune provider subject:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --manifest target/.../provider/pkg/hakorune_provider.json \
  --out target/hakozuna-mixed-ws-compare-provider/report.out \
  --out-dir target/hakozuna-mixed-ws-compare-provider/artifacts \
  --sample-count 5
```

Run the benchmark-only replacement-front smoke/evidence subject:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-cross-thread-smoke \
  --replacement-front-slot-size 1024 \
  --out target/hakozuna-mixed-ws-replacement-smoke/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-smoke/artifacts \
  --sample-count 5
```

Run the replacement-front performance distribution subject after smoke:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-slot-size 1024 \
  --threads 2 \
  --out target/hakozuna-mixed-ws-replacement-perf/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf/artifacts \
  --sample-count 7
```

Run the single-thread benchmark-only multi-bin front:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-bins-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-native-bins/report.out \
  --out-dir target/hakozuna-mixed-ws-native-bins/artifacts \
  --sample-count 5
```

Run pointer guard after docs pointer edits:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Parking Lot

- DirectMemory / Span / Bytes / LayoutSpan remain future substrate work.
- `direct {}` remains parked; use RequiredFastPathRegion diagnostics first.
- `DirectArray<T>` generic source form remains parked; v0 source-visible type is
  concrete `DirectArrayI64`.
- `RecordStateResidencePlanV0` stays a narrow box-private primitive residence
  plan, not record-as-box or ordinary-box auto-recordification.
- Mixed-base helper extraction stays parked unless `EffectSummary` /
  `ReceiverSnapshotPublicationPlanV0` evidence selects it again.
- External Ubuntu benchmark numbers remain non-horizontal until CPU and run
  conditions are aligned.
