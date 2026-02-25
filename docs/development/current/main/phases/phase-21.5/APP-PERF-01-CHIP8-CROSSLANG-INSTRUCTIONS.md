# APP-PERF-01 指示書 — chip8 kernel crosslang baseline（VM/LLVM-AOT/C/Python）

Status: Draft for implementation  
Scope: Phase 21.5 perf lane（real-app style baseline）

## Context

目的は「実アプリ寄り workload を 4 系統で比較する導線」を最小差分で固定すること。

- Nyash/Hakorune: VM と LLVM-AOT
- 参照: C と Python
- 対象: `chip8` 由来の deterministic kernel（I/O なし、固定ループ）

このタスクは **1ブロッカー=1受理形** として、`chip8_kernel_small` のみを追加する。

## Deliverables（6 files）

1. `benchmarks/bench_chip8_kernel_small.hako`（新規）
2. `benchmarks/c/bench_chip8_kernel_small.c`（新規）
3. `benchmarks/python/bench_chip8_kernel_small.py`（新規）
4. `tools/perf/bench_compare_c_py_vs_hako.sh`（新規）
5. `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh`（新規）
6. `benchmarks/README.md`（更新）

## Benchmark Logic Contract（3言語同一）

```
n = 400000
i = 0
pc = 0
op = 7
acc = 1
sum = 0
MOD = 1000000007

loop while i < n:
  op = (op * 73 + 19) % 65536
  branch = op % 16

  if branch < 4:
    acc = (acc + (op % 251) + pc) % MOD
  elif branch < 8:
    acc = (acc + (op % 127) + 3) % MOD
  elif branch < 12:
    acc = (acc + (branch * branch) + (pc % 17)) % MOD
  else:
    acc = (acc + (op % 97) + (pc % 13) + 11) % MOD

  pc = (pc + 2 + (op % 3)) % 4096
  sum = (sum + acc + pc + branch) % MOD
  i = i + 1

result = (sum + acc + pc) % MOD
```

- Hako: `return result`
- Python: `print(result)`
- C: `return (int)(result & 0xFF)`（既存 perf ルールに合わせる）

## New Script Contract

### `tools/perf/bench_compare_c_py_vs_hako.sh`

Usage:

```bash
tools/perf/bench_compare_c_py_vs_hako.sh <bench_key> [warmup] [repeat]
```

想定キー（APP-PERF-01時点）:

- `chip8_kernel_small`

挙動:

1. C/Python をそれぞれ median 計測（`warmup`/`repeat`）
2. `PERF_AOT=1 bash tools/perf/bench_compare_c_vs_hako.sh <key> ...` を呼ぶ
3. VM行/AOT行をパースして 1 行 summary を出力

出力契約（1行）:

```text
[bench4] name=chip8_kernel_small c_ms=<n> py_ms=<n> ny_vm_ms=<n> ny_aot_ms=<n> ratio_c_vm=<r> ratio_c_py=<r> ratio_c_aot=<r> aot_status=<ok|skip|fail>
```

## Smoke Contract

### `phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh`

検証項目:

1. `bench_compare_c_py_vs_hako.sh chip8_kernel_small 1 1` が終了コード 0
2. 出力に `[bench4] name=chip8_kernel_small` がある
3. `aot_status=ok` がある
4. `c_ms=` `py_ms=` `ny_vm_ms=` `ny_aot_ms=` の各キーがある

## README Update

`benchmarks/README.md` に追記:

1. case table に `chip8_kernel_small`
2. 実行例:

```bash
tools/perf/bench_compare_c_py_vs_hako.sh chip8_kernel_small 1 1
```

3. gate 実行例:

```bash
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh
```

## Rules（必須）

- `src/**` の Rust/LLVM 実装は変更しない
- fallback の新規追加は禁止
- 既存 gate の既定負荷を増やさない（optional 配線は今回不要）
- 1タスク1目的（chip8 kernel のみ）

## Acceptance Commands

```bash
# 1) new 4-way compare
tools/perf/bench_compare_c_py_vs_hako.sh chip8_kernel_small 1 1

# 2) contract smoke
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh

# 3) existing quick perf gate regression check
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh
```

## Non-goals（APP-PERF-01ではやらない）

- `kilo` crosslang baseline（APP-PERF-02 で扱う）
- `run_progressive_ladder_21_5.sh` への組み込み
- regression guard への新規しきい値追加
