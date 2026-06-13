---
Status: Active
Date: 2026-06-13
Scope: MIMALLOC-USERBOX-COUNTER-HEAVY-001A owner reclassification.
Blocker: HAKO-MIMALLOC-USERBOX-COUNTER-HEAVY-OPTIMIZATION-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-657-MIMALLOC-USERBOX-COUNTER-HEAVY-001-BASELINE-REFRESH.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/perf/bench_micro_aot_asm.sh
  - tools/perf/bench_micro_c_vs_aot_lanes.sh
---

# 296x-658 MIMALLOC-USERBOX-COUNTER-HEAVY-001A Owner Reclassification

## Decision

Reclassify `kilo_micro_userbox_counter_step_chain` from a kernel
`step_chain` dispatch/boxing optimization target into a startup/entry floor
sentinel.

The 296x-657 interpretation was a hypothesis taken before route/assembly
inspection. The updated evidence shows:

```text
counter_step_chain_kernel_dispatch_owner=0
counter_step_chain_startup_entry_floor_sentinel=1
selected_kernel_is_constant_folded=1
nyrt_entry_startup_dominates_total=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

Do not use `Counter.step_chain/0` or `Counter.step/0` as the next
MIRBuilder/lowering optimization owner unless a later measurement front keeps a
real non-folded kernel and names a new hot owner.

## AOT Kernel Assembly

Command:

```bash
PERF_MICROASM_RUNNER_MODE=direct KEEP_PERF_MICROASM_ARTIFACTS=1 \
  bash tools/perf/bench_micro_aot_asm.sh \
    kilo_micro_userbox_counter_step_chain ny_main 1
```

Observed `ny_main`:

```asm
000000000040d6b0 <ny_main>:
  40d6b0: b8 ab 41 20 05  mov $0x52041ab,%eax
  40d6b5: c3              ret
```

Interpretation:

```text
aot_step_chain_dispatch_in_kernel=0
aot_field_access_in_kernel=0
aot_boxing_in_kernel=0
aot_kernel_shape=constant_return
```

## C Kernel Assembly

The C comparison front also folds the loop result to an immediate. Its `main`
still prints the result, but the counter loop itself is gone:

```asm
0000000000001060 <main>:
  sub $0x8,%rsp
  mov $0x52041ab,%edx
  lea fmt,%rsi
  mov $0x2,%edi
  xor %eax,%eax
  call __printf_chk@plt
  xor %eax,%eax
  add $0x8,%rsp
  ret
```

Interpretation:

```text
c_step_chain_dispatch_in_kernel=0
c_kernel_shape=constant_result_plus_print
```

The AOT and C kernels are both too small to justify a `step_chain`
dispatch/boxing optimization. The observed total delta is not proof of a
remaining userbox method-call kernel.

## Lane Counters

Command:

```bash
bash tools/perf/bench_micro_c_vs_aot_lanes.sh \
  kilo_micro_userbox_counter_step_chain 1 5 50
```

Result:

```text
c_total_instr=129068
c_total_cycles=226643
c_total_ms=3
c_kernel_instr=3362
c_kernel_cycles=5067
c_kernel_ms=0.060
ny_total_instr=472617
ny_total_cycles=866291
ny_total_ms=4
ny_startup_instr=470914
ny_startup_cycles=850820
ny_startup_ms=4
ny_kernel_instr=6193
ny_kernel_cycles=10510
ny_kernel_ms=0.080
ratio_total_instr=0.27
ratio_total_cycles=0.26
ratio_total_ms=0.75
ratio_kernel_instr=0.54
ratio_kernel_cycles=0.48
ratio_kernel_ms=0.75
```

Interpretation:

```text
ny_total_instr_approx_ny_startup_instr=1
ny_total_cycles_mostly_startup_entry=1
kernel_lane_too_small_for_keeper=1
```

The kernel lane is small enough that instruction/cycle ratios are not a stable
keeper signal. The assembly shape is the stronger owner evidence.

## Adjacent Front Check

Quick adjacent-front lane checks:

```text
kilo_micro_userbox_counter_step:
  startup dominated; same owner family as counter_step_chain

kilo_micro_userbox_point_sum:
  startup dominated; same owner family as counter_step_chain

kilo_micro_userbox_point_add:
  real kernel remains
  c_kernel_cycles=2010550
  ny_kernel_cycles=2011692
  ratio_kernel_cycles=1.00
  ny_kernel_ipc=3.98
  c_kernel_ipc=5.97
```

`point_add` is a useful kernel-equivalence guard, but the current evidence does
not show a kernel-cycle regression to fix. It should not replace
`counter_step_chain` as the next exact-front optimization owner without a new
hot-block owner.

## Tooling Note

`kilo_leaf_map_getset_has` is not yet classified from this pass because its
lane runner hit a link conflict:

```text
multiple definition of `main'
```

This is a tooling/front compatibility issue, not a kernel owner decision. If
that front becomes a candidate, fix or isolate the runner shape first.

## Next

Choose one of these lanes before editing compiler/runtime code:

```text
Option A: startup/entry floor
  Use counter_step_chain/counter_step/point_sum as startup sentinels.
  Owner family: NyRT entry, startup, result/output, loader/libc floor.

Option B: new non-folded exact kernel front
  Search for a front whose AOT and C kernels both remain non-trivial and whose
  kernel cycles/instructions show a real Hako regression.

Option C: point_add guard only
  Keep point_add as a kernel equivalence guard.
  Do not optimize it unless annotate/objdump names a Hako-only hot block.
```

Recommended next row:

```text
MIMALLOC-USERBOX-KERNEL-FRONT-SELECT-001:
  scan candidate exact fronts with lanes + asm before choosing the next
  kernel optimization owner.
```

If total microbench cost remains the priority instead of kernel lowering,
return to the startup/entry floor lane. Do not continue from
`Counter.step_chain/0` as a dispatch/boxing owner.

## Stop Line

```text
do not add MIRBuilder witnesses for counter_step_chain
do not optimize Counter.step_chain dispatch from this evidence
do not claim allocator replacement
do not reopen provider activation
do not use total ratio alone as kernel keeper evidence
do not silently switch to another front without lanes + asm
```
