# phase21_5 / perf / chip8

Chip8 crosslang baseline family.

## Contains

- `phase21_5_perf_chip8_kernel_crosslang_contract.sh`
- `phase21_5_perf_chip8_kernel_crosslang_contract_vm.sh` (compat wrapper only)

## Contract

- The smoke keeps the chip8 kernel compare path pinned.
- The benchmark driver remains `tools/perf/bench_compare_c_py_vs_hako.sh`.
- While `phase-29ck` pre-perf runway is still active, this smoke is monitor-only for the AOT lane:
  - `aot_status=ok|skip`
  - it does not reopen perf by itself.

## Shared Helpers

- `../../../../../lib/test_runner.sh`
- `../../../../../lib/perf_crosslang_contract.sh`
