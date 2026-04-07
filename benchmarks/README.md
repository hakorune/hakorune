# Hakorune Benchmarks

This repository now bundles a light micro/synthetic benchmark suite to keep an eye on low-level optimizations. All cases are emitted on the fly by `tools/perf/microbench.sh` so they do not interfere with normal apps, but the generated Nyash code mirrors patterns we see in real programs.

## Included cases

| Case        | Notes                                                                 |
|-------------|-----------------------------------------------------------------------|
| `loop`      | Plain integer accumulation                                            |
| `strlen`    | String length in tight loop (`nyrt_string_length` path)               |
| `box`       | StringBox allocation/free                                             |
| `branch`    | Dense conditional tree (modulo & arithmetic)                          |
| `call`      | Helper function dispatch (mix/twist)                                  |
| `stringchain`| Substring concatenation + length accumulation                        |
| `arraymap`  | ArrayBox + MapBox churn                                               |
| `chip8`     | Simplified CHIP-8 style fetch/decode loop (derived from apps/chip8)   |
| `kilo`      | Text-buffer edits/search (inspired by enhanced_kilo_editor)           |
| `sieve`     | Integer-heavy Sieve of Eratosthenes (prime count)                     |
| `matmul`    | NxN integer matrix multiply (3 nested loops)                          |
| `linidx`    | Linear index array ops: idx=i*cols+j (CSE/hoist検証)                  |
| `maplin`    | Integer-key Map get/set with linear keys (auto key heuristics)        |
| `numeric_mixed_medium` | Integer arithmetic + branching + mod (800k iterations, Phase 21.5) |

Each case has a matching C reference, so the script reports both absolute time and the Hakorune/C ratio. Scenario-style cases (`chip8`, `kilo`) still keep all logic deterministic and print-free to make timings stable.

## Machine-Code Micro Lane (kilo/text)

For unstable environments (for example WSL wall-clock jitter), use fixed micro cases + `perf stat` counters first, then validate on `kilo_kernel_small`.

Leaf-proof cases (run these first when adding/changing one observer/mutator leaf):

- `kilo_leaf_array_rmw_add1`: integer `get -> +1 -> set` without trailing reread
- `kilo_leaf_array_string_len`: array string `get -> length` observer only
- `kilo_leaf_array_string_indexof_const`: array string `get -> indexOf("line")` observer only
- `kilo_leaf_map_getset_has`: string-key `MapBox.get -> set -> has` on a stable key array
- `kilo_leaf_map_get_missing`: missing-key `MapBox.get` on a stable integer key

Fixed micro cases (files live in `benchmarks/` + `benchmarks/c/`):

- `kilo_micro_indexof_line`: indexOf-heavy loop with stable array routing
- `kilo_micro_substring_concat`: substring + concat tight loop
- `kilo_micro_array_getset`: integer-key array get/set loop

Meso split cases (use these before `kilo_kernel_small_hk` when the whole app is still too coarse):

- `kilo_meso_substring_concat_len`: pure substring + concat + len chain
- `kilo_meso_indexof_append_array_set`: rotating row `indexOf("line") + append + array.set`

Exploratory meso shapes (keep these out of the default pure-first ladder until AOT support catches up):

- `kilo_meso_substring_concat_array_set`: substring + concat + array.set, no loop-carry
- `kilo_meso_substring_concat_array_set_loopcarry`: substring + concat + array.set with loop-carried text

Commands:

```bash
# Leaf-proof ladder first
tools/perf/run_kilo_leaf_proof_ladder.sh 1 15

# Single case: C vs Nyash AOT counters (median)
tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 15

# Run all three fixed cases
tools/perf/run_kilo_micro_machine_ladder.sh 1 15

# Run the meso split ladder
tools/perf/run_kilo_meso_machine_ladder.sh 1 15

# Run the contract split ladder up to whole kilo
tools/perf/run_kilo_kernel_split_ladder.sh 1 15

# Assembly-first probe (top report + annotate + objdump snippet)
tools/perf/bench_micro_aot_asm.sh \
  kilo_micro_indexof_line \
  'nyash_kernel::exports::string::find_substr_byte_index' \
  20
```

`bench_micro_c_vs_aot_stat.sh` outputs:

- `c_instr/c_cycles/c_cache_miss` and `ny_aot_*` medians
- `ratio_instr` (`c_instr / ny_aot_instr`)
- `ratio_cycles` (`c_cycles / ny_aot_cycles`)
- `ratio_ms` (`c_ms / ny_aot_ms`)

AOT runs in this lane use deterministic defaults unless explicitly overridden:

- `NYASH_GC_MODE=off`
- `NYASH_SCHED_POLL_IN_SAFEPOINT=0`
- `NYASH_DISABLE_PLUGINS=1`
- `NYASH_SKIP_TOML_ENV=1`

## Notes

- `tools/perf/dump_mir.sh` can optionally write the MIR(JSON) for a given `.hako` and print a block/op histogram. It tries the normal provider path first and falls back to the minimal `jsonfrag` version (while-form) when needed, so you can inspect both the structural skeleton and the full lowering.
- Current baseline observations (LLVM/EXE, `NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1`): `call`, `stringchain`, and `kilo` already beat the C reference (ratio < 100%), while `branch`, `arraymap`, and `chip8` remain near ≈200%—they are targets for the upcoming hoisting/array-map hot-path work.

### MIR emit stabilization (2025-11-13)

The `--exe` mode now uses a robust Python3-based JSON extraction in `tools/hakorune_emit_mir.sh` to handle stdout noise from Stage-B. When Stage-B is unavailable (using resolution issues), the script automatically falls back to:
1. Direct `--emit-mir-json` CLI path
2. Minimal jsonfrag MIR generation (FORCE mode)

This ensures that `tools/perf/microbench.sh --exe` always produces a ratio measurement, even when the full selfhost MIR builder path is unavailable. For production use, `PERF_USE_PROVIDER=1` can force the provider path (with automatic jsonfrag fallback).

## Latest fast-path measurements

The following numbers were recorded on 2025-11-12 with the opt-in work enabled:

```bash
export NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1 \
       NYASH_LLVM_SKIP_BUILD=1 NYASH_LLVM_FAST=1 NYASH_LLVM_FAST_INT=1 \
       NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_COLLECTIONS_HOT=1
tools/perf/microbench.sh --case <case> --backend llvm --exe --runs 3
```

Goal: bring the Hakorune EXE ratio to ≤125% of the reference C time (roughly C's 80%).  
Current effort: keep baking new hoist/CSE patterns so `arraymap`, `matmul`, and `maplin` fall below that bar while `sieve` and the other cases stay stable.

| Case      | EXE ratio (C=100%) | Notes |
|-----------|--------------------|-------|
| `branch`   | 75.00%             | 目標達成（≤125%）。 |
| `arraymap` | 150.00%            | Array/Map hot-path + binop CSE をさらに磨いて ≤125% を目指す。 |
| `chip8`    | 25.00%             | 十分速い。FAST_INT/hoist が効いている。 |
| `kilo`     | 0.21% (N=200,000)  | LLVM backend では EXE 経路を強制し、既定 N を 200k に自動調整。C 参照の方が重い構成のため比率は極小。 |
| `sieve`    | 200.00%            | `NYASH_VERIFY_RET_PURITY=1` ON での測定。auto キー判定がまだ保守的。 |
| `matmul`   | 300.00%            | まだ 3 重ループの Array/Map get/set が支配。自動 CSE と auto map key を詰める予定。 |
| `linidx`   | 100.00%            | Linear index case is at parity; hoist + CSE already helps share SSA. |
| `maplin`   | 200.00%            | auto Map キー判定（linear/const拡張）で _h を選択しやすくなった。 さらに詰める余地あり。 |

### Notes

- You can rerun any case with the command above; `NYASH_LLVM_SKIP_BUILD=1` keeps repeated ny-llvmc builds cheap once the binaries are ready.
- `kilo` は C 参照側が重く既定 N=5,000,000 だと長時間化するため、LLVM backend では常に EXE 経路＋既定 N=200,000 で測定するようにしました（`tools/perf/microbench.sh` が `--backend llvm` 時に自動で `--exe` + `N=200000` 相当へ調整します）。必要なら `--n <value>` で上書きしてください。
- `lang/src/llvm_ir/boxes/aot_prep/README.md` に StrlenFold / LoopHoist / ConstDedup / CollectionsHot のパス一覧をまとめ、NYASH_AOT_MAP_KEY_MODE={h|i64|hh|auto} の切り替えも説明しています。今後は hoist/collections の強化で arraymap/matmul/maplin/sieve を 125% 以内に引き下げる施策を続けます。
- 代表測定では `NYASH_VERIFY_RET_PURITY=1` を有効化し、Return 直前の副作用を Fail-Fast で検出しながら回しています（ごく軽微なハンドル・boxcallの変化が 2～3× に跳ねることがある点をご留意ください）。

## Running

Build `hakorune` in release mode first:

```bash
cargo build --release --bin hakorune
```

Then pick a case:

```bash
# LLVM EXE vs C (default n/runs are tuned per case)
tools/perf/microbench.sh --case chip8 --exe --n 200000 --runs 3

# VM vs C for a string-heavy micro
tools/perf/microbench.sh --case stringchain --backend vm --n 100000 --runs 3
```

The script takes care of generating temporary Nyash/C sources, building the C baseline, piping through the MIR builder (with FAST toggles enabled), and printing averages + ratios. Set `NYASH_SKIP_TOML_ENV=1` / `NYASH_DISABLE_PLUGINS=1` before calling the script if you want a fully clean execution environment.

## Phase 21.5 Progressive Ladder

For incremental tracking (small -> medium -> app), use:

```bash
# fast smoke-style run
tools/perf/run_progressive_ladder_21_5.sh quick

# heavier run with more repeats
tools/perf/run_progressive_ladder_21_5.sh default
```

### Medium Benchmarks

| Key | Description |
|-----|-------------|
| `box_create_destroy` | Box allocation/deallocation churn |
| `method_call_only` | Method call dispatch (StringBox.length) |
| `numeric_mixed_medium` | Integer arithmetic + branching + mod (800k iterations, **opt-in**) |
| `chip8_kernel_small` | CHIP-8 style deterministic kernel (400k iterations, cross-language baseline) |
| `kilo_kernel_small` | Kilo-inspired text-buffer edit kernel (60k iterations, cross-language baseline) |

### Cross-Language Baseline (APP-PERF-01 / APP-PERF-02)

4系統比較（C / Python / Hakorune VM / Hakorune LLVM-AOT）:

```bash
# 4-way comparison
tools/perf/bench_compare_c_py_vs_hako.sh chip8_kernel_small 1 1
tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 1

# Output format:
# [bench4] name=<key> c_ms=<n> py_ms=<n> ny_vm_ms=<n> ny_aot_ms=<n> ratio_c_vm=<r> ratio_c_py=<r> ratio_c_aot=<r> aot_status=<ok|skip|fail>
#
# Ratio definitions (1.00 = parity with C):
#   ratio_c_vm  = c_ms / ny_vm_ms
#   ratio_c_py  = c_ms / py_ms
#   ratio_c_aot = c_ms / ny_aot_ms

# Contract smoke
bash tools/smokes/v2/profiles/integration/phase21_5/perf/chip8/phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh
bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh
```

Files:
- `benchmarks/bench_chip8_kernel_small.hako` - Nyash version
- `benchmarks/c/bench_chip8_kernel_small.c` - C reference (with volatile)
- `benchmarks/python/bench_chip8_kernel_small.py` - Python reference
- `benchmarks/bench_kilo_kernel_small.hako` - Nyash version
- `benchmarks/c/bench_kilo_kernel_small.c` - C reference (with volatile)
- `benchmarks/python/bench_kilo_kernel_small.py` - Python reference

### Cross-Language + App Bundle (APP-PERF-03)

micro cross-language baseline（chip8/kilo）と app wallclock（apps/tools）を単一ハーネスで再生:

```bash
tools/perf/bench_crosslang_apps_bundle.sh 1 1 1 1

# Output format:
# [bench4-app] chip8_aot_status=<ok|skip|fail> chip8_ratio_c_aot=<r> chip8_ny_aot_ms=<n> \
#              kilo_aot_status=<ok|skip|fail> kilo_ratio_c_aot=<r> kilo_ny_aot_ms=<n> \
#              apps_total_ms=<n> apps_hotspot_case=<name> apps_hotspot_ms=<n> \
#              entry_source_total_ms=<n> entry_prebuilt_total_ms=<n> entry_delta_ms=<n> entry_winner=<name>

# Contract smoke
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_apps_crosslang_bundle_contract_vm.sh
```

### Opt-in Extra Benchmarks

To add extra medium benchmarks without affecting default ladder weight:

```bash
# Run with numeric_mixed_medium (opt-in)
PERF_LADDER_EXTRA_MEDIUM_KEYS=numeric_mixed_medium \
  tools/perf/run_progressive_ladder_21_5.sh quick

# Run with compare_reuse_small (opt-in)
PERF_LADDER_EXTRA_MEDIUM_KEYS=compare_reuse_small \
  tools/perf/run_progressive_ladder_21_5.sh quick

# Multiple extra benchmarks
PERF_LADDER_EXTRA_MEDIUM_KEYS="numeric_mixed_medium compare_reuse_small another_bench" \
  tools/perf/run_progressive_ladder_21_5.sh quick
```

### MIR Hotops Report (opt-in)

MIR op histogram can be emitted from a bench key or a `.hako` file:

```bash
# bench key
tools/perf/report_mir_hotops.sh numeric_mixed_medium

# direct file path + function override
tools/perf/report_mir_hotops.sh benchmarks/bench_method_call_only_small.hako --function main --top 10
```

Output format:

- `[mir-shape] input=<...> function=<...> blocks=<n> inst_total=<n> unique_ops=<n>`
- `[mir-shape/op] rank=<k> op=<name> count=<n> pct=<x.y>`

In ladder runs, enable optional report step:

```bash
PERF_LADDER_MIR_HOTOPS=1 \
PERF_LADDER_MIR_HOTOPS_KEYS="numeric_mixed_medium method_call_only_small" \
tools/perf/run_progressive_ladder_21_5.sh quick
```

### Compile/Run Split Probe (bench-level)

Use this probe to separate `source total` vs `emit` vs `prebuilt MIR run` for one bench key.
Default emit route is `direct` (binary-only stage1 route).
The probe fail-fasts when emitted MIR cannot execute via `--mir-json-file`.

```bash
# text one-line contract
tools/perf/bench_compare_compile_run_split.sh numeric_mixed_medium 1 1

# JSON output
PERF_SPLIT_OUTPUT=json tools/perf/bench_compare_compile_run_split.sh numeric_mixed_medium 1 1

# helper route comparison (optional)
PERF_SPLIT_EMIT_ROUTE=helper tools/perf/bench_compare_compile_run_split.sh numeric_mixed_medium 1 1
```

Decision fields:
- `status=ok`: split probe completed with executable prebuilt route
- `decision=vm_hotpath_priority`: optimize VM runtime first
- `decision=json_opt_candidate`: compile (`emit`) share is high enough to prioritize JSON/emit-path optimization

### Useful toggles

Perf env の正本（SSOT）は `docs/reference/environment-variables.md` の `Perf Lane (tools/perf)`。この節は日常運用でよく触る項目だけを載せる。

#### Ladder / Baseline

- `PERF_LADDER_*` の boolean トグルは `0|1` のみ受理（無効値は fail-fast）
- `PERF_LADDER_AOT_SMALL=1` (default) to print AOT on small benches
- `PERF_LADDER_AOT_MEDIUM=0` (default) to skip expensive AOT on medium benches
- `PERF_LADDER_AOT_SENTINEL=0|1` to force AOT sentinel OFF/ON (profile default: quick=0, default=1)
- `PERF_LADDER_REGRESSION_GUARD=0|1` to force regression guard OFF/ON (profile default: quick=0, default=1)
- `PERF_LADDER_MIR_HOTOPS=0|1` to enable optional MIR hotops report step in ladder (default: `0`)
- `PERF_LADDER_MIR_HOTOPS_KEYS="k1 k2"` to choose keys for MIR hotops report
- `PERF_LADDER_APPS=1` (default) to include `apps/tools/*` wall-clock
- `PERF_LADDER_STABILITY=1` (default) to run baseline drift summary

#### Split / Bench Runtime

- Core LLVM env (`NYASH_LLVM_FAST*`, `NYASH_LLVM_OPT_LEVEL`, `NYASH_LLVM_HOT_TRACE`) is documented in `docs/reference/environment-variables.md` (SSOT). This section focuses on perf-lane usage/toggles.

- `PERF_SPLIT_OUTPUT=text|json` to choose compile/run split output format (`bench_compare_compile_run_split.sh`, default: `text`)
- `PERF_SPLIT_EMIT_ROUTE=helper|direct` to choose split emit route (default: `direct`)
- `PERF_SPLIT_JSON_OPT_IN_RATIO_PCT=<N>` to set split decision threshold (%), default `40`
- `PERF_SPLIT_MIN_TOTAL_MS=<N>` to ignore very small benches for split decision, default `100`
- `PERF_SPLIT_EMIT_TIMEOUT=<duration>` to override emit timeout for split probe (default: `PERF_VM_TIMEOUT`)
- `HAKO_EMIT_MIR_MAINLINE_ONLY=1` to keep helper route fail-fast (optional when route=`helper`)
- `HAKO_VM_MAX_STEPS=<N>` to raise VM step budget for long-running VM benches (`tools/perf/lib/bench_env.sh` default: `100000000`; `0` is unlimited for diagnostics only)
- `NYASH_VM_FAST_REGFILE=0|1` to toggle dense ValueId register slots in VM perf profile (`tools/perf/lib/bench_env.sh` default: `1`; set `0` to compare legacy HashMap-only register writes)
- `PERF_VM_TIMEOUT=<duration>` to override per-run VM timeout used by perf scripts (default is resolved in `tools/perf/lib/bench_env.sh`: `fast=20s`, `heavy=60s`)
- `PERF_AOT_TIMEOUT_SEC=<N>` to override AOT timeout seconds used by perf scripts (default: `20`)
- `PERF_AOT_SKIP_BUILD=auto|0|1` to control rebuild behavior in AOT helper flow (`auto`: skip cargo rebuild only when required release artifacts already exist; `0`: always rebuild; `1`: always skip; invalid values fail-fast)
- `PERF_AOT_AUTO_SAFEPOINT=0|1` to control `NYASH_LLVM_AUTO_SAFEPOINT` in `bench_compare_c_py_vs_hako.sh` AOT lane (default: `0`, perf-oriented). If unset, `NYASH_LLVM_AUTO_SAFEPOINT` is used as fallback; invalid values fail-fast.
- `PERF_SKIP_VM_PREFLIGHT=0|1` to skip VM preflight fail-fast probe when you only need AOT contract checks (`0|1` 以外は fail-fast)
- `NYASH_LLVM_FAST_INT=0|1` to control AOT integer-fast lowering in perf helper flow (`tools/perf/lib/aot_helpers.sh` default: `1`; set `0` to compare baseline lowering)
- `NYASH_LLVM_FAST=1` in ny-llvmc fast lane links AOT executable with `-no-pie` on Linux (micro-bench launch overhead reduction)
- `NYASH_LLVM_FAST_IR_PASSES=0|1` to disable/enable FAST-only IR pass pipeline in `src/llvm_py/llvm_builder.py` (default: `1` when `NYASH_LLVM_FAST=1`)
- `NYASH_LLVM_HOT_TRACE=1` to emit one-line `[llvm/hot]` per-function counters (`%/compare/resolve`) during LLVM lowering (perf diagnosis)
- `PERF_NUMERIC_HOT_TRACE_BACKEND=llvmlite` to pin backend used by hot-trace contract smoke (`phase21_5_perf_numeric_hot_trace_contract_vm.sh`)
- `PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_BINOP=<N>` / `PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_COMPARE=<N>` to set allowed fallback ceilings in hot-trace contract (default: `0/0`)
- `PERF_METHOD_CALL_HOT_TRACE_BACKEND=llvmlite` to pin backend used by method-call hot-trace contract smoke (`phase21_5_perf_method_call_hot_trace_contract_vm.sh`)
- `PERF_METHOD_CALL_HOT_TRACE_MAX_FALLBACK_CALL=<N>` to set allowed `resolve_fallback_call` ceiling in method-call hot-trace contract (default: `0`)
- `PERF_COPY_CONST_HOTSPOT_MAX_AOT_MS=<N>` to set allowed `ny_aot_ms` ceiling for copy/const hotspot contract (`phase21_5_perf_copy_const_hotspot_contract_vm.sh`, default: `20`)

#### Apps / Entry / MIR Mode

- `PERF_APPS_OUTPUT=text|json` to choose app wallclock output format (`bench_apps_wallclock.sh`, default: `text`)
- `PERF_APPS_SUBTRACT_STARTUP=1` to report startup-subtracted app wallclock (`startup_ms` / `net_total_ms` / `net_cases` in JSON)
- `PERF_APPS_ENTRY_MODE=source|mir_shape_prebuilt` to switch app execution entry (`bench_apps_wallclock.sh`, default: `source`)
- `PERF_APPS_MIR_SHAPE_INPUT_MODE=emit|prebuilt` to select mir_shape_guard input path mode (`bench_apps_wallclock.sh`, default: `emit`)
- `PERF_APPS_MIR_SHAPE_PREBUILT=<path>` to override prebuilt MIR file path when mode is `prebuilt`
- `PERF_APPS_MIR_SHAPE_SOURCE=<path>` to override emit source `.hako` when mode is `emit`
- `PERF_MIR_SHAPE_SCAN_MAX_MS=<N>` to set scan budget (ms) for `phase21_5_perf_mir_shape_profile_contract_vm.sh` (default: `50`)
- `PERF_APPS_ENTRY_MODE_DELTA_SAMPLES=<N>` to control sample count for `bench_apps_entry_mode_compare.sh --json-lines` (default: `1`)
- `PERF_APPS_ENTRY_MODE_SIGNIFICANCE_MS=<N>` to set significance threshold for `bench_apps_entry_mode_compare.sh` (default: `10`)
- `PERF_APPS_MIR_MODE_SIGNIFICANCE_MS=<N>` to control winner significance threshold (ms) for `bench_apps_mir_mode_compare.sh`

#### Gate Toggles

- `PERF_GATE_APPS_CASE_BREAKDOWN_CHECK=1` to enable per-app JSON contract check in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_MIR_SHAPE_PROFILE_CHECK=1` to enable `mir_shape_guard` profile contract check in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_SUBTRACT_CHECK=1` to enable app startup-subtract JSON contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_MIR_MODE_CHECK=1` to enable app mir-shape input-mode contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_MIR_MODE_DELTA_CHECK=1` to enable emit/prebuilt delta contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_MIR_MODE_SPREAD_CHECK=1` to enable emit/prebuilt spread contract (`--json-lines N`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_MIR_MODE_SIGNIFICANCE_CHECK=1` to enable emit/prebuilt significance contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_MIR_MODE_CASE_HOTSPOT_CHECK=1` to enable per-case hotspot contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_COMPILE_RUN_SPLIT_CHECK=1` to enable app compile/run split contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_EMIT_ROUTE_CHECK=1` to enable app `--emit-mir-json` -> `--mir-json-file` route contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_ENTRY_MODE_CHECK=1` to enable app entry mode (`source|mir_shape_prebuilt`) contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_ENTRY_MODE_DELTA_CHECK=1` to enable app entry mode delta contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_ENTRY_MODE_SIGNIFICANCE_CHECK=1` to enable app entry mode significance contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_ENTRY_MODE_SPREAD_CHECK=1` to enable app entry mode spread contract (`--json-lines`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_ENTRY_MODE_CASE_HOTSPOT_CHECK=1` to enable app entry mode case hotspot contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_GUARD_LIB_CHECK=1` to enable perf guard helper contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_SHELLCHECK_CHECK=1` to enable optional shellcheck contract in `phase21_5_perf_gate_vm.sh` (skip-pass when shellcheck is not installed)
- `PERF_GATE_BENCH_ENV_CHECK=1` to enable bench-env timeout SSOT contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1` to enable bench_compare env fail-fast contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_BENCH_COMPILE_RUN_SPLIT_CHECK=1` to enable bench compile/run split contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_AOT_SKIP_BUILD_CHECK=1` to enable `PERF_AOT_SKIP_BUILD` contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1` to enable AOT safepoint env contract in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_AOT_LINK_MODE_CHECK=1` to enable AOT fast-link mode contract (`phase21_5_perf_aot_link_mode_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_NUMERIC_COMPARE_CHAIN_CHECK=1` to enable `%`/compare chain contract (`phase21_5_perf_numeric_compare_chain_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_FAST_IR_PASSES_CHECK=1` to enable FAST IR passes ON/OFF contract (`phase21_5_perf_fast_ir_passes_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_NUMERIC_HOT_TRACE_CHECK=1` to enable numeric hot-trace contract (`phase21_5_perf_numeric_hot_trace_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_METHOD_CALL_HOT_TRACE_CHECK=1` to enable method-call hot-trace contract (`phase21_5_perf_method_call_hot_trace_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_COPY_CONST_HOTSPOT_CHECK=1` to enable copy/const hotspot contract (`phase21_5_perf_copy_const_hotspot_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_FAST_NATIVE_CODEGEN_CHECK=1` to enable FAST native codegen ON/OFF contract (`phase21_5_perf_fast_native_codegen_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_NUMERIC_ARITH_CSE_CHECK=1` to enable arithmetic CSE contract (`phase21_5_perf_numeric_arith_cse_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_OPT_LEVEL_CHECK=1` to enable AOT opt-level pin contract (`phase21_5_perf_opt_level_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_COMPARE_EXPR_CSE_CHECK=1` to enable compare expr-cache CSE contract (`phase21_5_perf_compare_expr_cse_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_COMPARE_REUSE_AOT_CEILING_CHECK=1` to enable compare_reuse AOT ceiling contract (`phase21_5_perf_compare_reuse_aot_ceiling_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_LADDER_EXTRA_MEDIUM_CHECK=1` to enable ladder extra-medium key contract (`phase21_5_perf_ladder_extra_medium_key_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_KILO_CROSSLANG_CHECK=1` to enable kilo cross-language contract (`phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_GATE_APPS_CROSSLANG_BUNDLE_CHECK=1` to enable APP-PERF-03 bundle contract (`phase21_5_perf_apps_crosslang_bundle_contract_vm.sh`) in `phase21_5_perf_gate_vm.sh`
- `PERF_COMPARE_REUSE_AOT_MAX_MS=<N>` to set allowed `ny_aot_ms` ceiling for compare_reuse AOT contract (default: `20`)

#### Regression Thresholds

- `PERF_REG_GUARD_APPS_MAX_DEGRADE_PCT=<N>` / `PERF_REG_GUARD_PER_APP_MAX_DEGRADE_PCT=<N>` to tune total/per-app guard thresholds
- `PERF_STABILITY_INCLUDE_APPS_ENTRY_MODE=1` to include entry-mode compare summary in stability baseline output
- `PERF_STABILITY_ENTRY_MODE_SAMPLES=<N>` to set sample count for entry-mode compare during baseline stability recording
- `PERF_REG_GUARD_ENTRY_SOURCE_MAX_DEGRADE_PCT=<N>` / `PERF_REG_GUARD_ENTRY_PREBUILT_MAX_DEGRADE_PCT=<N>` to tune entry-mode source/prebuilt total thresholds
- `PERF_REG_GUARD_ENTRY_DELTA_MIN_RATIO=<f>` to require minimum retained delta advantage (`current_delta_abs / baseline_delta_abs`)

### AOT Sentinel

default profile では AOT 経路の健全性確認として `numeric_mixed_medium` を AOT で実行する（quick は既定OFF）。

```bash
# quick で AOT sentinel を明示的に有効化
PERF_LADDER_AOT_SENTINEL=1 tools/perf/run_progressive_ladder_21_5.sh quick

# default では AOT sentinel が既定ON
tools/perf/run_progressive_ladder_21_5.sh default
```

### Regression Guard

default profile では medium/app regression guard が既定ON（quick は既定OFF）。

```bash
# quick で regression guard を明示的に有効化
PERF_LADDER_REGRESSION_GUARD=1 tools/perf/run_progressive_ladder_21_5.sh quick

# 単体契約スモーク
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_regression_guard_contract_vm.sh

# app case breakdown 契約スモーク
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_apps_case_breakdown_contract_vm.sh
```

### Perf Gate (quick contract bundle)

For a single-entry Phase 21.5 perf contract replay:

```bash
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh
```

Preset wrapper (recommended for daily/segment runs):

```bash
# core-only (same as direct gate)
tools/perf/run_phase21_5_perf_gate_bundle.sh quick

# llvm/aot hotpath contracts
tools/perf/run_phase21_5_perf_gate_bundle.sh hotpath

# app entry/mode contracts
tools/perf/run_phase21_5_perf_gate_bundle.sh apps

# heavy full replay (hotpath + apps + regression helpers)
tools/perf/run_phase21_5_perf_gate_bundle.sh full
```

Optional Step-5 regression check (requires baseline file):

```bash
PERF_GATE_REGRESSION_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# app case breakdown も同時に確認
PERF_GATE_APPS_CASE_BREAKDOWN_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# mir_shape profile 契約も同時に確認
PERF_GATE_MIR_SHAPE_PROFILE_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# startup subtract 契約も同時に確認
PERF_GATE_APPS_SUBTRACT_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# mir_shape input mode 契約も同時に確認
PERF_GATE_APPS_MIR_MODE_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# emit/prebuilt 差分契約も同時に確認
PERF_GATE_APPS_MIR_MODE_DELTA_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# emit/prebuilt 複数サンプル差分契約も同時に確認
PERF_GATE_APPS_MIR_MODE_SPREAD_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# 有意差しきい値契約も同時に確認
PERF_GATE_APPS_MIR_MODE_SIGNIFICANCE_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# ケース別hotspot契約も同時に確認
PERF_GATE_APPS_MIR_MODE_CASE_HOTSPOT_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# compile/run split契約も同時に確認
PERF_GATE_APPS_COMPILE_RUN_SPLIT_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# emit->mir-json-file ルート契約も同時に確認
PERF_GATE_APPS_EMIT_ROUTE_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# app entry mode(source/prebuilt)契約も同時に確認
PERF_GATE_APPS_ENTRY_MODE_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# app entry mode差分契約も同時に確認
PERF_GATE_APPS_ENTRY_MODE_DELTA_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# app entry mode 有意差しきい値契約も同時に確認
PERF_GATE_APPS_ENTRY_MODE_SIGNIFICANCE_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# app entry mode spread契約も同時に確認
PERF_GATE_APPS_ENTRY_MODE_SPREAD_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# app entry mode ケース別hotspot契約も同時に確認
PERF_GATE_APPS_ENTRY_MODE_CASE_HOTSPOT_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# perf guard helper 契約も同時に確認
PERF_GATE_GUARD_LIB_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# shellcheck 契約も同時に確認（shellcheck未導入時はskip-pass）
PERF_GATE_SHELLCHECK_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# bench_env timeout SSOT 契約も同時に確認
PERF_GATE_BENCH_ENV_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh

# bench compile/run split 契約も同時に確認
PERF_GATE_BENCH_COMPILE_RUN_SPLIT_CHECK=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh
```

Entry mode compare helper:

```bash
tools/perf/bench_apps_entry_mode_compare.sh 1 1 vm
PERF_APPS_ENTRY_MODE_SIGNIFICANCE_MS=15 tools/perf/bench_apps_entry_mode_compare.sh 1 1 vm
PERF_APPS_ENTRY_MODE_DELTA_SAMPLES=3 tools/perf/bench_apps_entry_mode_compare.sh 1 1 vm --json-lines 3
```

### Step-5 Baseline / Regression Guard

Seed baseline (includes medium + apps):

```bash
PERF_STABILITY_INCLUDE_MEDIUM=1 \
PERF_STABILITY_INCLUDE_APPS=1 \
PERF_STABILITY_INCLUDE_APPS_ENTRY_MODE=1 \
PERF_STABILITY_WRITE_BASELINE=1 \
tools/perf/record_baseline_stability_21_5.sh 2 1 1
```

Run regression guard:

```bash
bash tools/checks/phase21_5_perf_regression_guard.sh
```

Entry-mode threshold override example:

```bash
PERF_REG_GUARD_ENTRY_SOURCE_MAX_DEGRADE_PCT=30 \
PERF_REG_GUARD_ENTRY_PREBUILT_MAX_DEGRADE_PCT=30 \
PERF_REG_GUARD_ENTRY_DELTA_MIN_RATIO=0.40 \
bash tools/checks/phase21_5_perf_regression_guard.sh
```

Standalone helpers:

```bash
tools/perf/bench_apps_wallclock.sh 1 3 vm
PERF_APPS_OUTPUT=json tools/perf/bench_apps_wallclock.sh 1 3 vm
PERF_APPS_OUTPUT=json PERF_APPS_SUBTRACT_STARTUP=1 tools/perf/bench_apps_wallclock.sh 1 3 vm
PERF_APPS_OUTPUT=json PERF_APPS_MIR_SHAPE_INPUT_MODE=prebuilt tools/perf/bench_apps_wallclock.sh 1 3 vm
tools/perf/bench_apps_mir_mode_compare.sh 1 1 vm
tools/perf/bench_apps_mir_mode_compare.sh 1 1 vm --json-lines 3
PERF_APPS_MIR_MODE_SIGNIFICANCE_MS=15 tools/perf/bench_apps_mir_mode_compare.sh 1 1 vm
PERF_APPS_MIR_MODE_SIGNIFICANCE_MS=15 tools/perf/bench_apps_mir_mode_compare.sh 1 1 vm --json-lines 3
tools/perf/record_baseline_stability_21_5.sh 3 1 3
```

`bench_apps_wallclock.sh` JSON には `hotspot` が常に含まれる:
- `PERF_APPS_SUBTRACT_STARTUP=0` のとき `hotspot.metric=raw`（`cases` 基準）
- `PERF_APPS_SUBTRACT_STARTUP=1` のとき `hotspot.metric=net`（`net_cases` 基準）
- `mir_shape_input_mode` が常に含まれる（`emit` or `prebuilt`）
- case 追加/変更は `tools/perf/lib/apps_wallclock_cases.sh` のレジストリを更新する（`bench_apps_wallclock.sh` 本体の編集は不要）

`bench_apps_mir_mode_compare.sh` summary JSON には case-level 比較が含まれる:
- `emit_cases_median_ms` / `prebuilt_cases_median_ms`
- `case_delta_ms`（`prebuilt - emit`）
- `case_winner`（`emit|prebuilt`）
- `hotspot_case_delta`（`delta_ms_abs` 最大ケース。`significant` は `PERF_APPS_MIR_MODE_SIGNIFICANCE_MS` で判定）
