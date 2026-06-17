# 296x-1007 SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-CLOSEOUT-001

Status: Landed
Date: 2026-06-17
Scope: closeout / return to fresh selection

## Contract

```text
output_contract=hako-substring-concat-exact-seed-closed-form-return-closeout-v0
source_evidence=296x-1003..1006
row_kind=closeout

target_front=kilo_micro_substring_concat
selected_route=substring_concat_loop_ascii
selected_owner_family=exact_seed_dead_byte_copy_loop
keeper_shape=stable_length_scalar_via_phi_carry_base

closed_form_return_enabled=1
ny_main_closed_form_return=1
ny_main_loop_removed=1
target_kernel_shape_win=1

ny_kernel_instr_before=4803110
ny_kernel_cycles_before=4806539
ny_kernel_instr_after=3102
ny_kernel_cycles_after=4368
ratio_kernel_instr_after=483.98
ratio_kernel_cycles_after=69.36

product_stringbox_storage_changed=0
runtime_helper_added=0
route_priority_changed=0
exact_seed_retired=0
benchmark_name_branch_count=0
source_name_branch_count=0
helper_name_inference_count=0
raw_mir_rescan_added=0

next_task=FRESH-COMPILER-OWNER-SELECTION-AFTER-SUBSTRING-CONCAT-CLOSEOUT-001
summary=ok
```

## Purpose

Close the selected `kilo_micro_substring_concat` exact-seed owner.

The active `substring_concat_loop_ascii` route now emits a direct constant
return for the measured front:

```asm
ny_main:
  mov $0x5265d0,%eax
  ret
```

## Result

The previous selected owner was not a runtime helper or missing generic
fastpath consumer. It was the exact-seed loop body retaining dead stack byte
copies after the final return value was already known.

That owner is now closed for this front.

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
do not keep extending this front without fresh owner evidence
do not use this closeout to retire exact seed routes globally
do not claim product StringBox storage or runtime helper changes
```

## Next

```text
FRESH-COMPILER-OWNER-SELECTION-AFTER-SUBSTRING-CONCAT-CLOSEOUT-001
```

Return to fresh front / owner selection.
