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

1. Measure the native slot bridge provider route.
   - Build with `--provider-package-hako-semantic-codegen
     object-lifecycle-native-slot-bridge-v0`.
   - Confirm `hako_provider_alloc_free_uses_host_malloc=0`.
   - Compare against the previous host-wrapper provider route and C mimalloc.

2. Run a stable same-machine Hakozuna mixed-ws comparison.
   - Use enough iterations/samples to reduce startup noise.
   - Compare system malloc, C mimalloc LD_PRELOAD, and optional Hakorune
     provider LD_PRELOAD.
   - Treat the result as local evidence only.

3. Classify the remaining gap.
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

4. Continue provider replacement ladder only as smoke/readiness.
   - Keep `provider_activation=0`, `production_replacement_active=0`,
     `hook_installed=0`, `global_allocator_product_claim=0`,
     `winner_claim=0`.

5. Add a benchmark-only `HakoAllocReplacementFront` probe if C-like thinness is
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

Next slices:

```text
REPL-001:
  ReplacementFrontHotPathPlanV0 report vocabulary
  behavior change: none

REPL-002:
  TlsArenaFastAllocPlanV0 consumer
  direct_alloc fast/cold split

REPL-003:
  TlsArenaLocalFreePlanV0 consumer
  local free path thinning

REPL-004:
  ReplacementEntryInlinePlanV0
  malloc/free entry boundary thinning

REPL-005:
  owner refresh
  open TlsArenaReallocPlanV0 only if fresh evidence selects realloc

REPL-006:
  RequestedSizeTableTaxProbeV0
  nonkeeper: fixed-slot usable-size realloc mode
  attempted to remove requested_size store/clear from alloc/free hot path

REPL-007:
  ReplacementSlotSizeClassProbeV0
  keeper: benchmark-only slot-size override for mixed-ws max-size matching
  checks whether fixed 2048-byte slots are the remaining cache/footprint owner

REPL-008:
  ReplacementCounterlessPerfProbeV0
  keeper as benchmark/perf-distribution mode only
  skip malloc/free hot-path counters after smoke has already validated behavior

REPL-009:
  TrustedLocalSlotProbeV0
  nonkeeper: benchmark-only local-thread trusted slot mode
  attempted to remove `used[]` store/check/clear from malloc/free hot path

REPL-010:
  HotResolvingRealGuardProbeV0
  nonkeeper: benchmark-only omission of malloc/free/realloc hot reentrant-real guard
  kept fallback resolver protection cold-side only

REPL-011:
  MixedWsSizeDistributionExplainV0
  report-only workload request histogram for the Hakozuna mixed-ws fixture
  fixes the measurement boundary before opening size-class or slot-metadata work

REPL-012:
  SlotMetaWordProbeV0
  nonkeeper: benchmark-only coalescing of live state and requested size into one
  metadata word
  cross-thread semantics stayed intact, but counterless throughput regressed

REPL-013:
  WorkloadReallocMatchedSlotProbeV0
  keeper: benchmark-only slot-size selection for mixed-ws default realloc grow
  uses slot_size=max_size+16 so default realloc grow requests stay covered
```

`REPL-001` reports the current hot-path shape without claiming it is already
C-thin. `REPL-002` consumes that report vocabulary for the generated replacement
front `direct_alloc` fast/cold split. The important fields are:

```text
replacement_front_hotpath_plan_v0=1
replacement_front_hotpath_report_only=1
tls_get_addr_hot_path=0|1
hot_atomic_rmw=0|1
remote_free_drain_hot_path=0|1
remote_owner_publication_after_local_fail=0|1
cold_init_in_hot_path=0|1
register_thread_arena_hot_path=0|1
fast_cold_split_plan=0|1
tls_arena_fast_alloc_plan=0|1
tls_arena_local_free_plan=0|1
free_local_first=0|1
free_remote_path_after_local_fail=0|1
free_hot_remote_queue_call=0|1
replacement_entry_inline_plan=0|1
malloc_to_direct_alloc_boundary=always_inline|tailcall|call
free_to_direct_free_boundary=always_inline|tailcall|call
replacement_front_slot_size=<bytes>
replacement_front_match_workload_realloc_size=0|1
workload_size_histogram_source=deterministic_prefix_exact|deterministic_prefix_sampled
workload_size_histogram_sample_exact=0|1
workload_alloc_request_count=<sampled-count>
workload_realloc_request_count=<sampled-count>
workload_realloc_request_gt_replacement_slot_size=<sampled-count>
workload_request_le_64=<sampled-count>
workload_request_le_128=<sampled-count>
workload_request_le_256=<sampled-count>
workload_request_le_512=<sampled-count>
workload_request_le_1024=<sampled-count>
workload_request_gt_1024=<sampled-count>
```

The current keeper state is expected to show `tls_get_addr_hot_path=0`,
`hot_atomic_rmw=0`, `remote_owner_publication_after_local_fail=1`,
`remote_free_drain_hot_path=0`, `cold_init_in_hot_path=0`,
`register_thread_arena_hot_path=0`, `fast_cold_split_plan=1`, and
`tls_arena_fast_alloc_plan=1`. `REPL-003` also fixes
`tls_arena_local_free_plan=1`, `free_local_first=1`,
`free_remote_path_after_local_fail=1`, and `free_hot_remote_queue_call=0`.
`REPL-004` fixes `replacement_entry_inline_plan=1` with
`malloc_to_direct_alloc_boundary=always_inline` and
`free_to_direct_free_boundary=always_inline`.

`REPL-005` owner refresh after `REPL-004` showed `malloc` and `free` as the
remaining replacement-front owners; `direct_alloc_slow` is cold. `realloc` was
visible but not first owner. `REPL-006` tested fixed slot usable size as the
replacement-front realloc copy extent while skipping `requested_size` writes on
alloc/free. It was rejected as nonkeeper: median throughput fell from about
`172.6M ops/s` to `156.1M ops/s` on the same mixed-ws shape because the larger
realloc copy extent outweighed the removed metadata stores. Keep
`requested_size` on the current hot path until a more precise realloc plan is
selected by fresh evidence.

`REPL-007` is a keeper. The current replacement front used a fixed 2048-byte
slot even when the mixed-ws fixture was capped at 1024 bytes. A benchmark-only
1024-byte slot-size override improved replacement-front median throughput from
about `172.6M ops/s` to `283.9M ops/s` on the same mixed-ws shape, reaching
about `55.1%` of the local C mimalloc reference in that run. This proves the
fixed-slot footprint/cache shape is a major owner. It is still a
benchmark-only storage-layout probe, not a product allocator size-class claim.

`REPL-008` is a keeper only for benchmark/perf-distribution mode. With the
1024-byte slot probe and no focused counter-validating smoke, skipping hot-path
replacement counters improved Hakorune median throughput from about
`283.3M ops/s` to `297.7M ops/s` on the same mixed-ws shape. This mode is
counterless by design: use normal counter-enabled runs for smoke/evidence, then
use counterless runs for performance distribution. Do not combine this with
cross-thread counter smokes or product readiness claims.

`REPL-009` tested a trusted local-thread mode that accepts in-range aligned
pointers without `used[]` store/check/clear. It was rejected as nonkeeper:
median throughput fell from about `297.7M ops/s` to `273.7M ops/s` in the
counterless 1024-byte slot front. The mode is also unsafe for cross-thread and
double-free semantics, so do not reopen it without a real owner token or
size-class metadata plan.

`REPL-010` tested whether the `resolving_real` reentrancy guard still belongs on
the malloc/free/realloc hot entry path after fast/cold split. It was rejected as
nonkeeper: median throughput fell from about `297.7M ops/s` to
`253.8M ops/s` in the counterless 1024-byte slot front. Keep the hot guard in
the current generated shape.

`REPL-011` is report-only. It adds a deterministic Hakozuna mixed-ws request
histogram to the comparison report so size-class and slot-metadata probes do
not accidentally compare a lighter workload. Long runs use a bounded
deterministic prefix sample and mark `workload_size_histogram_sample_exact=0`.
The key warning field is
`workload_realloc_request_gt_replacement_slot_size`: a 1024-byte replacement
slot covers the default `malloc` request range `16..1024`, but every 64th
allocation can request `size + 16` through `realloc`, so some default-shape
realloc requests exceed 1024. Treat this as measurement context, not a product
semantics claim.

`REPL-012` tested the narrow slot metadata traffic candidate from the worker
inventory. The probe coalesced live/free/remote state and requested size into
one metadata word while preserving requested-size and cross-thread semantics.
Focused cross-thread free / abandoned-owner / cross-thread realloc smokes passed,
but the counterless 1024-byte slot front regressed on the same local run:

```text
baseline_counterless_slot1024_median_ops_per_sec=662109806.277
slot_meta_word_counterless_slot1024_median_ops_per_sec=643591446.219
slot_meta_word_delta_pct=-2.80
```

Do not keep the slot-meta-word implementation path in the tool. The remaining
metadata owner is real, but this coalescing shape did not improve the current
generated replacement front.

`REPL-013` is a keeper for the benchmark tool. `REPL-011` showed that
`--replacement-front-slot-size 1024` covers the default `malloc` request range
but not every default mixed-ws `realloc(size + 16)` grow request. A 1040-byte
replacement slot covers that grow request (`workload_realloc_request_gt_replacement_slot_size=0`)
and still kept the benchmark-only replacement front above the local C mimalloc
reference in the same short run:

```text
slot_size=1040
replacement_front_match_workload_realloc_size=1
c_mimalloc_median_ops_per_sec=349180817.438
hakorune_replacement_front_median_ops_per_sec=637099210.841
hakorune_vs_c_mimalloc=1.824554
cross_thread_smoke=ok
```

This is still a benchmark-front result, not a product allocator winner claim.
Use `--replacement-front-match-workload-realloc-size` to request this
workload-matched slot size instead of hard-coding `1040` at call sites.
   - Treat `HakoAllocReplacementFront` as the thin-front direction for
     C-like malloc/free speed.
   - Keep `HakoAllocProviderFront` as explicit-provider infrastructure.
   - Required before claiming allocator readiness:

```text
single_thread_replacement_front_smoke=1
multithread_replacement_front_smoke=1
thread_safety_claim=measured
provider_api_hot_path_required=0
activation=0
benchmark_only=1
winner_claim=0
summary=ok
```

   - Multithread work must not silently share the current single-thread static
     free stack without a synchronization or thread-local plan.
   - Acceptable first multithread shapes:
     - locked global native-slot front, benchmark-only
     - thread-local slot arenas with explicit cross-thread free policy
   - Rejected first shapes:
     - unsynchronized global free stack
     - product activation hidden behind benchmark mode
     - `winner_claim=1` before multithread evidence

7. Reopen `.hako` core optimization only with fresh owner evidence.
   - Candidate families: route-aware materialization/copy, HotCore direct-exact
     call boundary, record-state residence, DirectArray proof/lowering.
   - Do not source-hand-expand helpers to satisfy the compiler.

8. Keep docs lean.
   - Record small inventories in commit messages or this checklist.
   - Move full evidence prose to investigations/archive, not this active card.

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
  --replacement-front-thread-local-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-slot-size 1024 \
  --out target/hakozuna-mixed-ws-replacement-perf/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf/artifacts \
  --sample-count 7
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
