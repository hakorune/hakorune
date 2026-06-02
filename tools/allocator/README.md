# Allocator Comparison Tools

This directory contains small phase-295x comparison helpers. They are local
evidence tools, not allocator-provider activation paths.

## Mimalloc Direct-Exact Evidence

Use the direct-exact wrappers when investigating current `.hako` mimalloc
parity. They source `tools/allocator/mimalloc_direct_exact_env.sh` so worker
runs do not accidentally measure the default/safe front.

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

Then use the counterless variant only for performance distribution:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
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
  --replacement-front-thread-local-mode \
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
