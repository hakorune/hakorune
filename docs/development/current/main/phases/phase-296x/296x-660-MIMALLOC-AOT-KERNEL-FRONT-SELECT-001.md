---
Status: Decision
Date: 2026-06-14
Scope: non-folded exact-AOT kernel-front selection after startup closeout.
Blocker: HAKO-BOOL-SCALAR-LOWERING-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-659-AOT-STARTUP-CLOSEOUT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/perf/bench_micro_c_vs_aot_lanes.sh
  - tools/perf/bench_micro_aot_asm.sh
  - tools/hako_check/README.md
---

# 296x-660 MIMALLOC-AOT-KERNEL-FRONT-SELECT-001

## Decision

Select `kilo_micro_userbox_flag_toggle` as the next non-folded exact-AOT
kernel front.

```text
selected_front=kilo_micro_userbox_flag_toggle
selected_role=non_folded_exact_kernel_front
kernel_not_constant_folded=1
kernel_instr_share=0.972
startup_share=0.029
c_pair_available=1
exact_front_owner_family=typed_object_inline_bool_scalar_lowering
next_task=BOOL-SCALAR-LOWERING-001
```

The next slice should inspect and reduce the exact typed-object inline-bool
scalar lowering residue. Do not return to startup tuning for this front.

## Sweep

Command shape:

```bash
bash tools/perf/bench_micro_c_vs_aot_lanes.sh <bench> 1 3 50
```

Summary:

```text
name                              kernel  startup  ny_cycles  c_cycles  C/Hako
kilo_micro_array_getset           0.977   0.023    4030064    4017695   1.00
kilo_micro_userbox_flag_toggle    0.972   0.029    6016295    4013517   0.67
kilo_micro_userbox_point_add      0.945   0.056    2011813    2011522   1.00
kilo_micro_array_string_store     0.838   0.164    816514     35020649  42.89
kilo_micro_concat_const_suffix    0.795   0.207    613994     606650    0.99
startup-sentinel family           ~0.013  ~0.996   small      mixed     mixed
```

`ratio_kernel_cycles` is `C / Hako`. A value below 1 means the Hako exact-AOT
kernel is slower than the paired C kernel.

## Why `flag_toggle`

`kilo_micro_userbox_flag_toggle` has the shape needed for the next compiler
optimization owner:

```text
kernel_not_constant_folded=1
startup_noise_low=1
kernel_share_high=1
method_call_count=0
exact_typed_object_route_selected=1
inline_bool_field_route_visible=1
c_comparable_shape_exists=1
```

The source kernel is small and stable:

```hako
box Flag {
  enabled: BoolBox
}

static box Main {
  main() {
    local ops = 2000000
    local flip_at = 1000000
    local f = new Flag()
    f.enabled = true
    local acc = 0
    local i = 0
    loop (i < ops) {
      acc = acc + (f.enabled == true)
      f.enabled = i < flip_at
      i = i + 1
    }
    return acc + (f.enabled == true)
  }
}
```

`hako_check perf-surface` reports a low source-level method-call risk:

```text
method_call_count=0
field_set_count=4
loop_field_set_count=2
hot_path_risk=low
summary=ok
```

The MIR shape still contains exact field read/write work:

```text
mir_instruction_count=42
call_count=0
field_get_count=2
field_set_count=2
phi_count=2
copy_count=16
branch_count=1
summary=ok
```

Route metadata shows `Flag.enabled` is on the exact typed-object inline-bool
route:

```text
semantic_op=TypedObjectExactSlotSetI64
selected_route=hako.typed_object.slot_store_i64
value_class=inline_bool

semantic_op=TypedObjectExactSlotGetI64
selected_route=hako.typed_object.slot_load_i64
value_class=inline_bool
```

## Assembly Clue

Hako exact-AOT `ny_main` has a repeated bool normalization sequence in the hot
loop:

```asm
movzbl %cl,%ecx
and    $0x1,%ecx
add    %rcx,-0x8(%rsp)
cmp    $0xf4240,%rax
setb   %cl
inc    %rax
cmp    $0x1e8480,%rax
jne    <ny_main+0x10>
```

The paired C kernel keeps the loop-carried boolean as a 0/1 scalar and adds it
directly:

```asm
movq -8(%rsp), %rdx
addq %rcx, %rdx
xorl %ecx, %ecx
cmpq $999999, %rax
setle %cl
incq %rax
movq %rdx, -8(%rsp)
cmpq $2000000, %rax
jne .L2
```

This does not prove the whole owner by itself. It is enough to select the next
seam:

```text
candidate_owner=typed_object_inline_bool_scalar_lowering
candidate_residue=redundant_bool_canonicalization_in_hot_loop
```

## Rejected Fronts

Do not select the folded/startup sentinel family:

```text
kilo_micro_userbox_counter_step
kilo_micro_userbox_counter_step_chain
kilo_micro_userbox_point_sum
kilo_micro_substring_only
kilo_micro_substring_views_only
kilo_micro_substring_concat
```

Reason:

```text
startup_share_near_1=1
kernel_front_not_primary=1
```

Do not select equivalence guards:

```text
kilo_micro_array_getset
kilo_micro_userbox_point_add
kilo_micro_concat_const_suffix
```

Reason:

```text
kernel_cycles_already_near_c=1
next_optimization_owner_not_exposed=1
```

Do not select `kilo_micro_array_string_store` from this pass:

```text
hako_kernel_faster_than_c=1
not_a_regression_front=1
```

Do not select failed leaf/string fronts from this pass:

```text
failed_fronts=
  kilo_leaf_array_rmw_add1
  kilo_leaf_array_string_indexof_const
  kilo_leaf_array_string_len
  kilo_leaf_map_get_missing
  kilo_leaf_map_getset_has
  kilo_micro_concat_birth
  kilo_micro_concat_hh_len
  kilo_micro_indexof_line
  kilo_micro_len_substring_views
selection_status=tooling_or_runner_pending
```

Those can become future fronts only after their runner/link/tooling issue is
classified.

## Next Slice

```text
BOOL-SCALAR-LOWERING-001:
  inspect exact typed-object inline-bool scalar lowering
  identify whether bool canonicalization can be avoided after a proven 0/1 scalar
  keep route truth in typed-object exact slot metadata
  do not change source semantics
  do not change MIRBuilder witness unless route evidence is lost
```

Acceptance for the next slice:

```text
front=kilo_micro_userbox_flag_toggle
kernel_not_constant_folded=1
startup_share_low=1
selected_route=hako.typed_object.slot_load_i64/hako.typed_object.slot_store_i64
value_class=inline_bool
bool_scalar_lowering_residue_classified=1
source_semantics_changed=0
mirbuilder_truth_changed=0
startup_lane_reopened=0
summary=ok
```

## Stop Line

```text
do not optimize startup from this front
do not alter benchmark source semantics
do not change product NyRT entry behavior
do not widen Type ABI or plugin ABI for this slice
do not add allocator/provider replacement claims
do not touch MIRBuilder unless route metadata is proven insufficient
```
