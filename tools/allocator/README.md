# Allocator Comparison Tools

This directory contains small phase-295x comparison helpers. They are local
evidence tools, not allocator-provider activation paths.

## Mimalloc Direct-Exact Evidence

Use the direct-exact wrappers when investigating current `.hako` mimalloc
parity. They source `tools/allocator/mimalloc_direct_exact_env.sh` so worker
runs do not accidentally measure the default/safe front.

Tooling boundary:

```text
hakozuna_mixed_ws_ldpreload_compare.py:
  runner, subject orchestration, report fields

replacement_front_templates.py:
  benchmark-only replacement-front C templates, deterministic workload helpers,
  and .hako SizeClassBox mirror helpers
```

Before claiming that an allocator benchmark is measuring the full `.hako`
mimalloc algorithm, run the algorithm coverage report:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py
```

To overlay an already generated Hakozuna mixed-ws compare report onto the
static inventory, pass it explicitly:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py \
  --benchmark-report target/hakozuna-mixed-ws-page-bins-current/report.out
```

The report separates `.hako` policy/model coverage from benchmark-only
replacement-front execution. The current expected state is:

```text
replacement_front_is_full_hako_algorithm=0
benchmark_report_consumed=0
size_class_policy_product_bins_connected=0
size_class_policy_single_class_benchmark_bridge_supported=1
page_model_hot_array_bridge_plan_v0=1
page_model_hot_array_access_plan_v0=1
page_model_hot_array_source_migration_selected=1
page_model_hot_array_source_type_ready=1
page_model_hot_array_birth_contract_ready=1
page_model_hot_array_source_migration_blocker=none
page_model_hot_array_next_bridge=source_migration_measurement
page_model_hot_array_seed_push_blocker=0
replacement_front_product_pages_bridge_plan_v0=1
replacement_front_product_pages_bridge_report_only=1
replacement_front_product_pages_consumer_enabled=0
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
replacement_front_locked_global_multithread_supported=1
replacement_front_thread_local_multithread_supported=1
replacement_front_multithread_claim=0
provider_activation=0
production_replacement_active=0
winner_claim=0
```

Use this to avoid reading the fixed-slot replacement front as a product
allocator or full `.hako` algorithm claim.

With `--benchmark-report`, the report overlays the executed benchmark-only
route while preserving the no-product-claim boundary:

```text
benchmark_report_consumed=1
benchmark_replacement_subject=hakorune_replacement_front_ldpreload
size_class_policy_product_bins_connected=1
replacement_front_product_bins_consumer_enabled=1
replacement_front_product_bins_route=benchmark_page_bins
replacement_front_page_bins_consumer_enabled=1
replacement_front_page_bins_route=benchmark_page_bins
replacement_front_product_pages_consumer_enabled=0
replacement_front_product_pages_source_ready=1
replacement_front_product_pages_bridge_blocker=consumer_not_enabled
replacement_front_product_pages_next_bridge=page_map_backed_replacement_front_plan
hotcore_replacement_consumer_enabled=0
hotcore_replacement_shape_ready=1
hotcore_replacement_bridge_blocker=consumer_not_enabled
hotcore_replacement_next_bridge=replacement_front_consume_hotcore_page_model
hotcore_replacement_route=not_consumed_by_replacement_front
replacement_front_page_bins_product_claim=0
replacement_front_is_full_hako_algorithm=0
```

This overlay is evidence that the benchmark-only front consumed the selected
route in that report. It is not product allocator activation and it does not
turn benchmark page-bins into the full `.hako` mimalloc algorithm.

`page_model_hot_array_access_plan_v0` is a source-readiness scan. It reports
`free` / `local_free` / `block_used` `get` / `set` / `push` calls separately.
The seed path now uses append-or-overwrite `set(i, ...)` shape, so the old
seed-time `push` blocker is closed. PageModel hot arrays are now source-level
`DirectArrayI64` fields. The current bridge is source migration measurement,
not another hot `get/set` route or constructor fixture.

The Hakozuna mixed-ws compare report also emits a report-only size-class bridge
view:

```text
replacement_front_size_class_bridge_plan_v0=1
replacement_front_size_class_policy_bridge=0
workload_size_class_distinct_count=...
```

This mirrors `SizeClassBox` for workload classification only. It does not make
the fixed-slot replacement front consume `.hako` size classes.

For the benchmark-only replacement front, use the narrow size-class bridge when
the owner evidence needs the fixed slot size to come from the `.hako`
`SizeClassBox` mirror:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-match-hako-size-class \
  --out target/hakozuna-mixed-ws-sizeclass-bridge/report.out \
  --out-dir target/hakozuna-mixed-ws-sizeclass-bridge/artifacts \
  --sample-count 3
```

This sets the benchmark-only slot size to
`SizeClassBox.good_size(max-size + 16)` and reports:

```text
replacement_front_size_class_policy_bridge=1
replacement_front_size_class_bridge_mode=hako_good_size_request_ceiling
```

It is still a single fixed-slot benchmark front, not product bins/pages.

The compare report also emits the product bins/pages readiness boundary:

```text
replacement_front_product_bins_plan_v0=1
replacement_front_product_bins_consumer_enabled=0
replacement_front_product_bins_required_regular_bins=...
replacement_front_product_pages_plan_v0=1
replacement_front_product_pages_consumer_enabled=0
replacement_front_page_bins_plan_v0=1
replacement_front_page_bins_supported=...
replacement_front_page_bins_consumer_enabled=0
replacement_front_page_bins_owner=benchmark_only
replacement_front_page_bins_product_claim=0
```

These fields are report-only inputs for the future multi-class/page front.
They do not mean product bins/pages are connected.

The algorithm coverage report also exposes the product-pages source-readiness
bridge. This is still report-only and keeps `consumer_enabled=0`; it only says
that the `.hako` PageMap/release/realloc/huge/OSVM seams are present enough for
the next benchmark-only bridge design:

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

For the first benchmark-only multi-bin prototype, use:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-bins-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-native-bins/report.out \
  --out-dir target/hakozuna-mixed-ws-native-bins/artifacts \
  --sample-count 3
```

This generates only the regular `.hako` size-class bins required by the
deterministic workload prefix and reports:

```text
replacement_front_algorithm_shape=multi_bin_native_benchmark_front
replacement_front_product_bins_consumer_enabled=1
replacement_front_product_bins_route=benchmark_native_bins
replacement_front_product_pages_consumer_enabled=0
```

It remains single-thread-only in v0 and still keeps product pages, activation,
hooks, globals, and winner claims closed.

The next bridge after native-bins is `page_bins`: a benchmark-only page-shaped
bin route. It may consume workload regular bins plus page-shaped owner storage,
but it must keep product replacement and full `.hako` algorithm claims closed
until the algorithm coverage report proves the executed route is no longer
split from the `.hako` model.

For the first page-shaped benchmark-only multi-bin prototype, use:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-page-bins/report.out \
  --out-dir target/hakozuna-mixed-ws-page-bins/artifacts \
  --sample-count 3
```

This keeps the same workload regular `.hako` size-class bins as
`--replacement-front-native-bins-mode`, but stores each bin in a page-shaped
owner struct. It reports:

```text
replacement_front_algorithm_shape=page_bin_benchmark_front
replacement_front_product_bins_consumer_enabled=1
replacement_front_product_bins_route=benchmark_page_bins
replacement_front_page_bins_consumer_enabled=1
replacement_front_page_bins_route=benchmark_page_bins
replacement_front_page_bins_lookup_route=range_scan
replacement_front_product_pages_consumer_enabled=0
replacement_front_page_bins_product_claim=0
```

It remains single-thread-only in v0 and still keeps product pages, activation,
hooks, globals, and winner claims closed.

The same compare reports also emit the current HotCore bridge boundary:

```text
replacement_front_hotcore_bridge_plan_v0=1
replacement_front_hotcore_consumer_enabled=0
hotcore_replacement_shape_ready=1
hotcore_replacement_bridge_blocker=consumer_not_enabled
hotcore_replacement_next_bridge=replacement_front_consume_hotcore_page_model
hotcore_page_model_source_ready=1
hotcore_small_alloc_calls_acquire_fresh_small=1
hotcore_release_calls_release_local_known_live=1
page_model_hot_methods_ready=1
```

This means `.hako` `objectLifecycleSmallAlloc` /
`objectLifecycleReleaseBlock` and their PageModel hot calls are source-ready,
but remain model/plan evidence until the replacement front consumes that route.

For the first benchmark-only HotCore/PageModel bridge, keep page-bins mode and
add the HotCore wrapper mode:

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

This routes the benchmark-only page-bin alloc/free core through
HotCore/PageModel-shaped acquire/release helpers and reports:

```text
replacement_front_algorithm_shape=page_bin_hotcore_page_model_benchmark_front
replacement_front_product_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_page_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_hotcore_consumer_enabled=1
replacement_front_hotcore_route=benchmark_page_bins_hotcore_page_model
```

The boundary remains narrow: product pages, activation, hooks, globals, winner
claims, and full `.hako` algorithm claims stay closed.

The current malloc-owner keeper for this bridge is the benchmark-only
SizeClassBox table lookup. It keeps the same page-bin HotCore/PageModel route,
but lowers the request-size to bin mapping through an 8-byte bucket table
instead of the generated ordered range scan:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --replacement-front-hotcore-page-model-mode \
  --replacement-front-size-class-table-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-hotcore-size-table/report.out \
  --out-dir target/hakozuna-mixed-ws-hotcore-size-table/artifacts \
  --sample-count 7
```

Expected report fields:

```text
replacement_front_size_class_table_mode=1
replacement_front_size_class_lookup_route=table_8byte_bucket
replacement_front_algorithm_shape=page_bin_hotcore_page_model_benchmark_front
replacement_front_hotcore_consumer_enabled=1
replacement_front_is_full_hako_algorithm=0
```

This is still a benchmark-only replacement-front lowering probe. It is not a
new source syntax, product page bridge, allocator activation, or winner claim.

```bash
tools/allocator/hako_mimalloc_direct_exact_app_perf_stat.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-public.stat.txt \
  --runs 5
```

For owner-first assembly evidence, use the perf/asm wrapper. It keeps the built
EXE, `perf.data`, annotate output, and objdump next to the report.

```bash
tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-public.asm.txt \
  --symbol ny_main
```

## Hakmem External Bench Bridge

Use `hakmem_external_bench.py` to run selected benchmarks from the extracted
`hakmem_20260525` corpus while keeping copied binaries and mutable output under
`target/`.

Default source:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

Default target:

```text
target/hakmem-bench/
```

List the supported local bridge inputs:

```bash
tools/allocator/hakmem_external_bench.py --list
```

Prepare the target-local executable copy without running benchmarks:

```bash
tools/allocator/hakmem_external_bench.py --prepare-only
```

Run a small smoke benchmark:

```bash
tools/allocator/hakmem_external_bench.py \
  --bench cfrac \
  --allocator sys \
  --allocator mimalloc \
  --out target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

Mutable output:

```text
target/hakmem-bench/out/bench/benchres.csv
```

Snapshot output:

```text
target/hakmem-bench/results/*.benchres.csv
```

### Minimal LD_PRELOAD Fixture

For daily LD_PRELOAD allocator replacement checks, use the repo-local minimal
random-mixed fixture instead of the full extracted corpus:

```bash
make -C benchmarks/external/hakmem/random-mixed-system
```

The LD_PRELOAD pilot tools default to:

```text
benchmarks/external/hakmem/random-mixed-system/build/bench_random_mixed_system
```

Pass `--hakmem-root /path/to/hakmem` only when intentionally running against the
full extracted corpus.

For the Ubuntu-side mixed working-set subject, use the repo-local Hakozuna
fixture:

```bash
make -C benchmarks/external/hakozuna/mixed-ws
```

The provider replacement decision ladder can select it with:

```bash
--ldpreload-benchmark hakozuna-mixed-ws
```

For same-machine allocator comparison, use the Hakozuna mixed-ws compare tool.
It runs the same repo-local CRT benchmark under system malloc, C mimalloc
through LD_PRELOAD, and optionally the Hakorune provider LD_PRELOAD package:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --out target/hakozuna-mixed-ws-compare/report.out \
  --out-dir target/hakozuna-mixed-ws-compare/artifacts \
  --sample-count 5
```

Pass `--mimalloc-library /path/to/libmimalloc.so.2` to avoid `ldconfig`
discovery. Pass `--manifest target/.../provider/pkg/hakorune_provider.json`
when intentionally adding the Hakorune provider subject. The report uses C
mimalloc as the local reference subject and keeps all product replacement and
winner-claim fields closed. Provider-subject reports also emit manifest build
metadata and bridge-interpretation fields:

```text
provider_ldpreload_measurement_interpretation=provider_abi_wrapper_and_shim_bridge
provider_ldpreload_is_hako_core_speed_claim=0
provider_manifest_hako_provider_alloc_free_route=host_malloc_free_wrapper
provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle=0
subject_N_shim_init_real_fallback_per_provider_operation=...
subject_N_next_owner_family=provider_alloc_free_internal_real_malloc_boundary
```

Use those fields to avoid reading the current provider LD_PRELOAD bridge as a
direct `.hako` allocator-core speed claim.

Additional repo-local Hakmem fixtures are available when the owner refresh
needs a wider shape than random-mixed or Hakozuna mixed-ws:

```bash
make -C benchmarks/external/hakmem/tiny-hot-system
benchmarks/external/hakmem/tiny-hot-system/build/bench_tiny_hot_system \
  64 100 1000
```

```bash
make -C benchmarks/external/hakmem/mid-large-mt-system
benchmarks/external/hakmem/mid-large-mt-system/build/bench_mid_large_mt_system \
  2 1000 128 42
```

`tiny-hot-system` focuses on small malloc/free hot-path overhead.
`mid-large-mt-system` focuses on 8-32KiB multi-thread allocation/free traffic.
Both are minimal system-malloc fixtures copied from the extracted Hakmem
corpus; do not vendor the full corpus for routine development.

Compare repo-local Hakmem fixtures under system malloc and C mimalloc:

```bash
python3 tools/allocator/hakmem_fixture_ldpreload_compare.py \
  --fixture tiny-hot-system \
  --allow-ldconfig-discovery \
  --out target/hakmem-fixture-tiny-hot/report.out \
  --out-dir target/hakmem-fixture-tiny-hot/artifacts \
  --sample-count 3
```

Add the benchmark-only Hakorune replacement front only when the fixed-slot
shape is intentional for that fixture:

```bash
python3 tools/allocator/hakmem_fixture_ldpreload_compare.py \
  --fixture tiny-hot-system \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-slot-size 64 \
  --out target/hakmem-fixture-tiny-hot-replacement/report.out \
  --out-dir target/hakmem-fixture-tiny-hot-replacement/artifacts \
  --sample-count 3
```

For mid/large fixtures, start with system/C mimalloc comparison and open a
replacement-front size-class row only if fresh owner evidence selects it.

For the benchmark-only Hakorune replacement front subject, keep smoke/evidence
and performance distribution separate. First run the counter-enabled thread
local front with focused cross-thread smokes:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-cross-thread-smoke \
  --replacement-front-match-workload-realloc-size \
  --out target/hakozuna-mixed-ws-replacement-smoke/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-smoke/artifacts \
  --sample-count 5
```

Then use the current multithread performance owner for distribution. The local
v0 owner is the counterless locked global front; it is benchmark-only and still
does not claim product replacement:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-match-workload-realloc-size \
  --out target/hakozuna-mixed-ws-replacement-perf/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf/artifacts \
  --sample-count 7
```

For a stable local distribution run, keep the same subject and increase the
operation count:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-match-workload-realloc-size \
  --threads 4 \
  --iters-per-thread 10000000 \
  --working-set 8192 \
  --min-size 16 \
  --max-size 1024 \
  --out target/hakozuna-mixed-ws-replacement-perf-40m/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf-40m/artifacts \
  --sample-count 5 \
  --warmup-count 1
```

Reports classify the selected replacement-front evidence owner so that smoke
and performance runs are not confused:

```text
replacement_front_evidence_owner=locked_global_multithread_front
replacement_front_multithread_perf_candidate=1
replacement_front_thread_local_perf_candidate=0
replacement_front_correctness_smoke=0
```

Thread-local reports remain useful for correctness and remote-free evidence,
but are not the current performance keeper unless fresh perf/asm evidence
selects them:

```text
replacement_front_evidence_owner=thread_local_multithread_front
replacement_front_thread_local_perf_candidate=1
```

`--replacement-front-match-workload-realloc-size` is a benchmark fixture probe,
not a product size-class claim. It chooses a fixed replacement slot size large
enough for the benchmark request range, for example `1040` bytes for the
default `16..1024` mixed-ws workload. The report must keep:

```text
workload_realloc_request_gt_replacement_slot_size=0
subject_N_replacement_front_match_workload_realloc_size=1
subject_N_replacement_front_inplace_realloc_within_slot_plan=1
```

Counter-enabled smokes should also show in-place realloc coverage and no copy
traffic:

```text
subject_N_replacement_front_realloc_inplace_count_total>0
subject_N_replacement_front_realloc_copy_bytes_total=0
```

`--replacement-front-skip-hot-counters` is incompatible with
counter-validating smokes by design. Slot metadata/header shortcut probes are
not part of the current keeper path; keep them out unless a new owner-first
row reopens that subject.

Run the current no-product-default provider replacement decision ladder:

```bash
tools/allocator/hako_mimalloc_provider_replacement_decision_ladder.sh \
  --out target/provider-replacement-decision/report.out \
  --skip-build-release
```

This consumes Hako/C repeated evidence, provider explicit evidence, repeated
repo-local hakmem LD_PRELOAD evidence, and the generated Rust global allocator
smoke. It records readiness only; product allocator replacement, production
hooks, production `#[global_allocator]`, and winner claims stay closed.

The LD_PRELOAD repeated report also carries shim overhead diagnostics:

```text
shim_runtime_real_fallback_count_total
shim_init_real_fallback_count_total
shim_host_passthrough_count_total
shim_pointer_table_overflow_total
```

`shim_runtime_real_fallback_count_total` and
`shim_pointer_table_overflow_total` are correctness gates and must stay zero.
`shim_init_real_fallback_count_total` is a performance diagnostic: a large
value means the replacement path is running through shim/provider boundary
work even when the provider is bound successfully.

To intentionally compare against a full extracted corpus build, pass:

```bash
--hakmem-root /home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

For an external Hakozuna mixed-ws build, pass:

```bash
--ldpreload-benchmark hakozuna-mixed-ws \
--hakozuna-root /path/to/hakozuna/hz3/out/linux/x86_64
```

Compare two decision reports without changing product defaults:

```bash
python3 tools/allocator/provider_replacement_decision_pair_compare.py \
  --left target/provider-replacement-decision-s5/report.out \
  --right target/provider-replacement-decision-external-s5/report.out \
  --out target/provider-replacement-decision-pair/report.out
```

Export a provider package for handoff:

```bash
python3 tools/allocator/provider_package_export_bundle.py \
  --package-dir target/provider-replacement-decision-s5/report.out.artifacts.d/provider/pkg \
  --out-dir dist/provider-handoff \
  --force \
  --out dist/provider-handoff/export.out
```

The output includes `hakorune-mimalloc-provider.zip`. By default the bundle
contains both the Hakorune provider shared library and a generated
malloc-family LD_PRELOAD shim:

```text
dist/provider-handoff/hakorune-mimalloc-provider/
  hakorune_provider.json
  hakorune_provider.sha256
  libhakorune_provider.so
  libhakorune_provider_ldpreload.so
  run_ldpreload_example.sh
```

Run the handoff bundle through LD_PRELOAD:

```bash
dist/provider-handoff/hakorune-mimalloc-provider/run_ldpreload_example.sh \
  benchmarks/external/hakmem/random-mixed-system/build/bench_random_mixed_system \
  1000 128 42
```

The helper sets `HAKORUNE_PROVIDER_LIBRARY`,
`HAKORUNE_PROVIDER_LDPRELOAD_REPORT`, and `LD_PRELOAD` for that process only.
It is still handoff evidence, not product allocator replacement.

## Stop Lines

- Do not commit copied benchmark executables or generated `benchres.csv`.
- Do not import historical `hakmem` CSV/log rows as current phase repeated
  measurement evidence without a schema-adapter row.
- Do not claim speed or RSS winners from this bridge.
- Do not use this bridge to open provider activation, process replacement,
  hooks, backend matchers, or `#[global_allocator]`.

The bridge emits `winner_claim=0` and the provider/replacement stop-line fields
so downstream scripts can keep the boundary explicit.

## Hakmem Result Adapters

Convert a `mimalloc-bench` `benchres.csv` file into key-value evidence:

```bash
tools/allocator/hakmem_benchres_adapter.py \
  --in target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

Convert a `hakozuna_compare_*.log` file into key-value evidence:

```bash
tools/allocator/hakmem_hakozuna_compare_log_adapter.py \
  --in /home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/hakozuna_compare_20260118_034633/hakozuna_compare_20260118_034633_mimalloc_e165faccc.log
```

Both adapters emit external historical corpus evidence only. They are useful for
schema alignment and workload selection, not for phase-295x winner claims.
