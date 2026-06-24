---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-AOT-KERNEL-FRONT-SELECT-002
Scope: Select the next boot-amortized non-folded exact-AOT kernel front after
  the bool-scalar lowering keeper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-660-MIMALLOC-AOT-KERNEL-FRONT-SELECT-001.md
  - docs/development/current/main/phases/phase-296x/296x-661-BOOL-SCALAR-LOWERING-001.md
  - docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/perf/bench_micro_c_vs_aot_lanes.sh
---

# MIMALLOC-AOT-KERNEL-FRONT-SELECT-002

## Purpose

Pick the next exact-AOT optimization front with measurement evidence before
touching code. Because Hakorune process startup is currently too large for
small microbench totals, this row uses the boot-amortized kernel lane as the
primary selection surface.

```text
row_kind=selection
implementation_started=0
perf_first_required=1
source_read_before_perf=0
boot_amortized_kernel_lane_primary=1
process_total_lane_diagnostic_only=1
```

This row resumes optimization after the compiler-foundation checkpoint. The
previous exact front, `kilo_micro_userbox_flag_toggle`, landed its keeper in
`BOOL-SCALAR-LOWERING-001`.

## Measurement Fix

Some runtime/string exact kernels need symbols from `libnyash_kernel.a`. Pulling
that archive can also pull the product `main`, which collided with the resident
kernel runner's own `main`.

This row fixes the measurement harness only:

```text
measurement_harness_linkfix=1
product_runtime_changed=0
optimizer_changed=0
source_semantics_changed=0
```

The resident runner now keeps its `main` first and allows a duplicate archive
`main` while linking measurement-only binaries.

## Evidence Summary

Command shape:

```bash
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_lanes.sh <bench> 1 5 200
PERF_MICROASM_RUNNER_MODE=direct KEEP_PERF_MICROASM_ARTIFACTS=1 \
  bash tools/perf/bench_micro_aot_asm.sh kilo_micro_indexof_line ny_main 3
```

Logs:

```text
target/perf_front_select_002/lanes_20260615_021633.log
target/perf_front_select_002/lanes_extra_continue_20260615_021803.log
target/perf_front_select_002/lanes_string_after_linkfix_20260615_021937.log
```

Boot-amortized resident kernel lane:

```text
kilo_micro_array_getset:
  ratio_kernel_cycles=1.00
  classification=equivalence_guard

kilo_micro_userbox_point_add:
  ratio_kernel_cycles=1.03
  classification=equivalence_guard

kilo_micro_concat_const_suffix:
  ratio_kernel_cycles=0.99
  classification=equivalence_guard

kilo_micro_userbox_flag_toggle:
  ratio_kernel_cycles=1.76
  classification=previous_keeper_still_green

kilo_micro_array_string_store:
  ratio_kernel_cycles=41.98
  classification=hako_faster_or_c_pair_not_next_owner

kilo_micro_indexof_line:
  ratio_total_cycles=0.19
  ratio_kernel_cycles=9.45
  classification=process_total_vs_resident_kernel_route_split

substring/counter tiny family:
  ny_kernel_cycles=~2k
  classification=tiny_or_startup_sentinel_not_kernel_front
```

Read:

```text
meaningful_hako_slower_exact_kernel_front_found=0
process_total_route_split_visible=1
boot_amortized_kernel_lane_worked=1
```

`kilo_micro_indexof_line` is the important diagnostic: process-total says Hako
is slower, while resident `ny_main` kernel says Hako is faster. That is not a
kernel-lowering owner. It is a product-route/body-timing question.

## Measurement Policy

The product boot path is not the optimization target for this row. Boot cost
must be measured, but it must not choose the kernel owner.

```text
boot_optimization_lane_reopened=0
product_nyrt_entry_changed=0
boot_amortization_required=1
kernel_inner_runs_min=50
kernel_inner_runs_preferred=200
process_total_lane_role=diagnostic
resident_kernel_lane_role=primary
```

Use the existing micro-lanes runner. Its resident kernel lane repeatedly calls
`ny_main` / `bench_main` in-process, which makes boot a small fixed setup cost
instead of the measured body.

Suggested command shape:

```bash
bash tools/perf/bench_micro_c_vs_aot_lanes.sh <bench> 1 5 200
bash tools/perf/bench_micro_aot_asm.sh <bench> ny_main 3
```

If a candidate only looks bad in process-total but the resident kernel lane is
equivalent to C, reject it as a boot/loader diagnostic, not as a kernel owner.

## Selection Rules

The selected front must satisfy:

```text
kernel_not_constant_folded=1
boot_amortized_kernel_gap_visible=1
kernel_delta_visible=1
c_pair_available=1
runner_status=ok
owner_family_single_enough=1
```

Reject these without new evidence:

```text
counter_step_chain:
  role=startup_sentinel
  exact_kernel_target=0

boot/startup_lane:
  reopened=0
  product_nyrt_entry_changed=0
  role=diagnostic_only

equivalence_guards:
  do not select fronts already kernel-equivalent to C unless new residue appears
```

## Required First Step

Re-measure candidate fronts before reading or editing implementation code.

Suggested command shape:

```bash
bash tools/perf/bench_micro_c_vs_aot_lanes.sh <bench> 1 5 200
bash tools/perf/bench_micro_aot_asm.sh <bench> ny_main 3
```

If the front is noisy or startup-dominated in process-total, read the resident
kernel lane first. Do not patch around measurement noise.

## Candidate Families

Start from the post-`BOOL-SCALAR-LOWERING-001` exact-front set:

```text
array_getset:
  likely equivalence guard unless new residue appears

userbox_point_add:
  likely equivalence guard unless new residue appears

array_string_store:
  candidate if kernel share remains high and owner is not startup/IO noise

concat_const_suffix:
  likely equivalence guard unless new residue appears

typed_object_or_field_family:
  candidate if non-folded field route residue is visible
```

The selected owner must be stated before implementation:

```text
selected_front=<bench>
selected_owner_family=<single owner>
selected_hot_transition=<asm or perf block>
primary_lane=resident_kernel
kernel_inner_runs=<N>
rejected_fronts=<list with reason>
next_task=<one implementation row>
```

## Stop Line

```text
do not edit source before perf evidence
do not reopen startup from counter_step_chain
do not claim product runtime startup speedup
do not select an owner from process-total startup noise
do not change .hako source semantics
do not change MIRBuilder truth
do not change Type ABI / plugin ABI
do not start Arc retirement or object substrate replacement
```

## Acceptance

```text
mimalloc_aot_kernel_front_select_002_active=1
previous_keeper=BOOL-SCALAR-LOWERING-001
perf_evidence_required_before_code=1
boot_amortized_kernel_lane_primary=1
process_total_lane_diagnostic_only=1
meaningful_hako_slower_exact_kernel_front_found=0
measurement_harness_linkfix=1
process_total_route_split_visible=1
implementation_started=0
next_task=MIMALLOC-BODY-TIMING-FRONT-SELECT-001
summary=ok
```
