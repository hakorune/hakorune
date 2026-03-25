---
Status: Decision
Decision: accepted
Date: 2026-03-26
Scope: `phase-21_5` / `kilo` perf lane を reopen できるかを current daily evidence で判定し、次の exact adjacent front を narrow に固定する。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P7-PRE-PERF-RUNWAY-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P10-SMALL-PERF-REENTRY-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P11-SMALL-ENTRY-STARTUP-INVENTORY.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
---

# P8: Perf Reopen Judgment

## Purpose

- `W1..W4` close 後に、perf lane を automatic に reopen しない。
- current daily evidence だけで reopen 可否を判定し、next exact front を 1 本に固定する。

## Evidence

### Green evidence

- `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_pure_first_min.sh`
- `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_compat_keep_min.sh`
- `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`

これらは boundary mainline / explicit compat keep の contract が pre-perf runway 後も崩れていないことを示す。

### Reopen-closing evidence

1. `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh method_call_only_small 1 1`
   - current result: `aot_status=ok`
   - current reason: `ok`
2. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_loop_integer_hotspot_contract_vm.sh`
   - green
3. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_strlen_ir_contract_vm.sh`
   - green
4. `bash tools/checks/dev_gate.sh quick`
   - green

`method_call_only` family の boundary acceptance blocker は retired と読む。

## Judgment

- `perf/kilo` lane は **reopen 可能**。
- current decision is `reopen allowed`.
- `llvmlite` / harness は引き続き perf judge の外側に置く。
- `phase21_5` chip8 quick smoke は monitor-only AOT keep のままとする。

## Next Exact Adjacent Front

- `P10-SMALL-PERF-REENTRY-TASK-PACK.md` is now closed.
- current next exact front is `P11-SMALL-ENTRY-STARTUP-INVENTORY.md`.
- medium/full `kilo` widening はこの startup front の後に判断する。

## Non-Goals

- `kilo` / `micro kilo` の retune を始めること
- asm top 追跡を先に reopen すること
- `substring_concat` / `array_getset` の leaf 編集へ戻ること
- `llvmlite` keep lane を perf comparator に戻すこと

## Reopen Condition

`perf/kilo` reopen condition は次で満たされたよ。

1. `bench_compare_c_vs_hako.sh method_call_only_small 1 1` が `aot_status=ok`
2. `phase21_5_perf_loop_integer_hotspot_contract_vm.sh` が green
3. `phase21_5_perf_strlen_ir_contract_vm.sh` が green
4. 上の 3 件が boundary mainline (`.hako -> ny-llvmc(boundary) -> C ABI`) で通る

current stop line:
- reopen は allowed だが、current first lane は `P11` startup inventory に固定する。
