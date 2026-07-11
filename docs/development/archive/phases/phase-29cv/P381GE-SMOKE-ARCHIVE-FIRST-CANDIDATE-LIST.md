# P381GE Smoke Archive First Candidate List

Date: 2026-05-06
Scope: T6 per-script delete-candidate classification after P381GC/P381GD.

## Decision

The first deletion wave is limited to 45 v2 archive scripts with:

- fixed report `class` in `orphan_candidate`
- `suite_hit_count = 0`
- full-path reference count `0` across repo docs/tools/src/lang/root pointers
- basename reference count `0` across the same search scope, excluding the
  script itself

No legacy `tools/smokes` root script and no `tools/archive/manual-smokes` script
enters this wave.

This card is classification only. Deletion must be a separate commit.

## Bucket Results

| Bucket | Scripts | Report orphans | First-wave candidates | Reading |
| --- | ---: | ---: | ---: | --- |
| `tools/smokes/v2/profiles/archive` | 81 | 25 | 4 | only zero-ref vm_hako_caps block scripts qualify |
| `tools/smokes/v2/profiles/integration/archive` | 13 | 0 | 0 | suite protected |
| `tools/smokes/v2/profiles/integration/apps/archive` | 225 | 217 | 41 | zero-ref subset qualifies |
| legacy `tools/smokes` outside `v2` | 14 | n/a | 0 | 4 zero-ref raw entries need owner policy first |
| `tools/archive/manual-smokes` | 35 | n/a | 0 | no zero-ref raw entries |

## First-Wave Candidate Paths

`tools/smokes/v2/profiles/archive`:

- `tools/smokes/v2/profiles/archive/vm_hako_caps/app1/app1_stack_overflow_after_open_block_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_clear_block_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_delete_block_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_keys_block_vm.sh`

`tools/smokes/v2/profiles/integration/apps/archive`:

- `tools/smokes/v2/profiles/integration/apps/archive/phase100_mutable_accumulator_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase100_string_accumulator_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase100_string_accumulator_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase118_pattern3_if_sum_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase143_p2_loop_true_if_bc_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase143_p2_loop_true_if_bc_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase143_p2_loop_true_if_cb_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase143_p2_loop_true_if_cb_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase145_p2_compound_expr_binop_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase145_p2_compound_expr_binop_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase145_p2_compound_expr_double_intrinsic_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase145_p2_compound_expr_double_intrinsic_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase252_p0_this_methodcall_break_cond_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase252_p0_this_methodcall_break_cond_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase275_p0_eq_number_only_llvm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase275_p0_eq_number_only_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase275_p0_plus_number_only_llvm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase275_p0_plus_number_only_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase284_p1_return_in_loop_llvm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase284_p1_return_in_loop_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern1_frag_poc.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern2_break_no_update_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern2_frag_poc.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern4_frag_poc.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern5_return_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern8_plan_poc_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern9_frag_poc.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern6_matchscan_contract_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern6_matchscan_ok_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern6_reverse_contract_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern6_reverse_ok_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern6_scan_with_init_ok_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern7_splitscan_nearmiss_ok_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern7_splitscan_ok_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29y_stage1_emit_mir_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29y_stage1_hako_run_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase92_pattern2_baseline.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/string_cp_mode_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/vm_hako_caps_boxcall_args_gt1_block_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/vm_hako_caps_compare_ge_block_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/vm_hako_caps_open_handle_phi_block_vm.sh`

## Held Buckets

Legacy `tools/smokes` outside `v2` has four raw zero-ref entries:

- `tools/smokes/archive/smoke_async_spawn.sh`
- `tools/smokes/curated_phi_invariants.sh`
- `tools/smokes/parity_quick.sh`
- `tools/smokes/unified_members.sh`

They are held out of the first deletion wave because they are not covered by
the suite-aware v2 inventory report and need a root-tool lifecycle owner card.

`tools/archive/manual-smokes` has no zero-ref raw entries in this pass.

## Evidence Commands

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=t6_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/integration/archive \
  SMOKE_INVENTORY_LABEL=t6_integration_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/integration/apps/archive \
  SMOKE_INVENTORY_LABEL=t6_integration_apps_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh
```

Reference filter:

```bash
rg -nF -- "$path" docs tools src lang Makefile README.md CURRENT_TASK.md
rg -nF -- "$(basename "$path")" docs tools src lang Makefile README.md CURRENT_TASK.md
```

The committed list uses only candidates where both counts are zero after
excluding the script itself.

## Next

Delete only the 45 listed v2 archive scripts, then rerun:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
