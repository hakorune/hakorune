# APP-PERF-02 指示書 — kilo kernel crosslang baseline（VM/LLVM-AOT/C/Python）

Status: Implemented  
Scope: Phase 21.5 perf lane（real-app style baseline）

## Context

目的は「text-edit 系 workload を 4 系統で比較する導線」を APP-PERF-01 と同じ契約で固定すること。

- Nyash/Hakorune: VM と LLVM-AOT
- 参照: C と Python
- 対象: `kilo` 由来の deterministic kernel（I/O なし、固定ループ）

このタスクは **1ブロッカー=1受理形** として、dataset `kilo_kernel_small` を追加する。
運用キーは lane を分けて扱う:

- `kilo_kernel_small_hk`（kernel-mainline / strict no-fallback）
- `kilo_kernel_small_rk`（kernel-bootstrap）
- `kilo_kernel_small`（legacy alias）

## Deliverables（6 files）

1. `benchmarks/bench_kilo_kernel_small.hako`（新規）
2. `benchmarks/c/bench_kilo_kernel_small.c`（新規）
3. `benchmarks/python/bench_kilo_kernel_small.py`（新規）
4. `tools/perf/bench_compare_c_py_vs_hako.sh`（更新、`kilo_kernel_small` 対応）
5. `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh`（新規）
6. `benchmarks/README.md`（更新）

## Benchmark Logic Contract（3言語同一）

```
rows = 64
ops = 60000
lines = ["line-*"] * rows
undo = 0

loop while i < ops:
  row = i % rows
  line = lines[row]
  split = len(line) // 2
  lines[row] = line[:split] + "xx" + line[split:]
  undo = undo + 1

  if i % 8 == 0:
    for each line in lines:
      if "line" in line:
        line = line + "ln"

result = sum(len(line) for line in lines) + undo
```

- Hako: `return result`
- Python: `print(result)`
- C: `return (int)(result & 0xFF)`（既存 perf ルールに合わせる）

## Script Contract

### `tools/perf/bench_compare_c_py_vs_hako.sh`

Usage:

```bash
tools/perf/bench_compare_c_py_vs_hako.sh <bench_key> [warmup] [repeat]
```

APP-PERF-02時点の許可キー:

- `chip8_kernel_small`
- `chip8_kernel_small_hk`
- `chip8_kernel_small_rk`
- `kilo_kernel_small`
- `kilo_kernel_small_hk`
- `kilo_kernel_small_rk`

出力契約（1行）:

```text
[bench4] name=kilo_kernel_small c_ms=<n> py_ms=<n> ny_vm_ms=<n> ny_aot_ms=<n> ratio_c_vm=<r> ratio_c_py=<r> ratio_c_aot=<r> aot_status=<ok|skip|fail>
[bench4-route] name=kilo_kernel_small_hk dataset=kilo_kernel_small kernel_lane=<hk|rk|default> kernel_name=<kernel-mainline|kernel-bootstrap> fallback_guard=<...> vm_engine=<...> result_parity=<ok|skip>
```

## Smoke Contract

### `phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh`

検証項目:

1. `bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 1` が終了コード 0
2. 出力に `[bench4] name=kilo_kernel_small_hk` がある
3. `aot_status=ok` がある
4. `c_ms=` `py_ms=` `ny_vm_ms=` `ny_aot_ms=` の各キーがある
5. `ratio_c_vm=` `ratio_c_py=` `ratio_c_aot=` の各キーがある
6. `[bench4-route]` があり `kernel_lane=hk` / `fallback_guard=strict-no-fallback` / `result_parity=ok` を満たす

## Gate Wiring（optional）

APP-PERF-02 は full preset に optional で配線する:

- optional step table:
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`
  - `PERF_GATE_KILO_CROSSLANG_CHECK`
- preset wrapper:
  - `tools/perf/run_phase21_5_perf_gate_bundle.sh full`

## Rules（必須）

- `src/**` の Rust/LLVM 実装は変更しない
- fallback の新規追加は禁止
- 既存 gate の既定負荷を増やさない（full preset の optional 配線のみ）
- 1タスク1目的（kilo kernel のみ）

## Acceptance Commands

```bash
# 1) new 4-way compare
PERF_VM_FORCE_NO_FALLBACK=1 \
tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 1

# 2) contract smoke
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh

# 3) optional gate wiring check
PERF_GATE_KILO_CROSSLANG_CHECK=1 NYASH_LLVM_SKIP_BUILD=1 \
  bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh
```

## Non-goals（APP-PERF-02ではやらない）

- app wallclock 統合（APP-PERF-03 で扱う）
- regression guard の新規しきい値追加
- `bench_compare_c_py_vs_hako.sh` の keys 拡張（必要時は別タスク）
