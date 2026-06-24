# 296x-1006 SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-17
Scope: implementation / measurement

## Contract

```text
output_contract=hako-substring-concat-exact-seed-closed-form-return-implementation-v0
source_evidence=296x-1005
row_kind=implementation

target_front=kilo_micro_substring_concat
selected_route=substring_concat_loop_ascii
selected_keeper_shape=stable_length_scalar_via_phi_carry_base

changed_file=lang/c-abi/shims/hako_llvmc_ffi_string_metadata_fn_readers.inc
closed_form_emitter_reused=1
new_runtime_helper_added=0
product_stringbox_storage_changed=0
route_priority_changed=0
exact_seed_retired=0
benchmark_name_branch_count=0
source_name_branch_count=0
helper_name_inference_count=0
raw_mir_rescan_added=0

ny_main_closed_form_return=1
ny_main_loop_removed=1
ny_main_return_constant=5400016
ny_main_return_hex=0x5265d0

ny_kernel_instr_before=4803110
ny_kernel_cycles_before=4806539
ny_kernel_instr_after=3102
ny_kernel_cycles_after=4368
target_kernel_shape_win=1

next_task=SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-CLOSEOUT-001
summary=ok
```

## Change

`hako_llvmc_string_corridor_has_stable_length_scalar_base_fn(...)` now accepts
the exact metadata relation selected in 296x-1005:

```text
phi_carry_base(loop_payload_root=35, window_contract=stop_at_merge)
  on relation owner key 20

stable_length_scalar(base_value=20, window_contract=preserve_plan_window)
  on the same relation owner key
```

This is a metadata relation bridge only. It does not inspect raw MIR
instructions and does not branch by benchmark/source/helper name.

## Result

Before:

```asm
ny_main:
  stack byte-copy / rotation loop
  dec %rcx
  jne ...
  mov $0x5265d0,%eax
  ret
```

After:

```asm
ny_main:
  mov $0x5265d0,%eax
  ret
```

## Measurement

Command:

```bash
bash tools/perf/build_perf_release.sh
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat ny_main 1
bash tools/perf/bench_micro_c_vs_aot_lanes.sh kilo_micro_substring_concat 1 3 100
```

Observed:

```text
ny_kernel_instr_before=4803110
ny_kernel_cycles_before=4806539
ny_kernel_instr_after=3102
ny_kernel_cycles_after=4368
ratio_kernel_instr_after=483.98
ratio_kernel_cycles_after=69.36
```

This is a target kernel shape win. It is not a product runtime claim and not a
new generic fastpath consumer claim.

## Verification

```bash
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_route_contract.sh
bash tools/perf/build_perf_release.sh
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat ny_main 1
bash tools/perf/bench_micro_c_vs_aot_lanes.sh kilo_micro_substring_concat 1 3 100
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Line

```text
do not generalize this bridge beyond phi_carry_base -> stable_length_scalar
do not retire exact seeds in this row
do not force StringDeadTextRegionPlan reachability
do not infer by benchmark/source/helper name
```

## Next

```text
SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-CLOSEOUT-001
```
