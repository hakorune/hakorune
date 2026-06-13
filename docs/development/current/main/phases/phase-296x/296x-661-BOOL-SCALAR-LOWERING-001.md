---
Status: Landed
Date: 2026-06-14
Scope: exact inline-bool scalar lowering for the selected flag-toggle front.
Blocker: HAKO-MIMALLOC-AOT-KERNEL-FRONT-SELECT-296X-002
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-660-MIMALLOC-AOT-KERNEL-FRONT-SELECT-001.md
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_flag_toggle_micro.inc
  - tools/build_hako_llvmc_ffi.sh
  - tools/perf/bench_micro_c_vs_aot_lanes.sh
  - tools/perf/bench_micro_aot_asm.sh
---

# 296x-661 BOOL-SCALAR-LOWERING-001

## Decision

The selected `kilo_micro_userbox_flag_toggle` front is fixed by carrying the
proven inline-bool field as a 0/1 SSA scalar in the exact micro route emitter.

```text
front=kilo_micro_userbox_flag_toggle
owner=typed_object_inline_bool_scalar_lowering
implemented_owner_file=lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_flag_toggle_micro.inc
source_semantics_changed=0
mirbuilder_truth_changed=0
startup_lane_reopened=0
summary=ok
```

## Change

Before this row, the exact flag-toggle emitter wrote the synthetic BoolBox
field through an `i64` alloca and re-normalized the loaded value each iteration:

```llvm
%enabled.cur = load i64, ptr %enabled
%enabled.cur.bool = icmp ne i64 %enabled.cur, 0
%enabled.cur.i64 = zext i1 %enabled.cur.bool to i64
%acc.next = add i64 %acc.cur, %enabled.cur.i64
```

That lowered to the hot-loop sequence:

```asm
movzbl %cl,%ecx
and    $0x1,%ecx
add    %rcx,-0x8(%rsp)
```

The route metadata already proves `Flag.enabled` is an inline-bool exact slot.
The emitter now carries that proven 0/1 value as loop SSA:

```llvm
%enabled.cur = phi i64 [1, %entry], [%enabled.next, %loop.body]
%acc.next = add i64 %acc.cur, %enabled.cur
%enabled.next.bool = icmp slt i64 %i.cur, 1000000
%enabled.next = zext i1 %enabled.next.bool to i64
```

The volatile `acc` alloca remains the benchmark anchor.

## Before / After

Before, from `296x-660`:

```text
ny_kernel_instr=16006198
ny_kernel_cycles=6016295
c_kernel_instr=18002599
c_kernel_cycles=4013517
ratio_kernel_instr=1.12
ratio_kernel_cycles=0.67
```

After rebuilding `libhako_llvmc_ffi`:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/perf/bench_micro_c_vs_aot_lanes.sh \
  kilo_micro_userbox_flag_toggle 1 3 50
```

Result:

```text
ny_kernel_instr=14006194
ny_kernel_cycles=2284127
c_kernel_instr=18002599
c_kernel_cycles=4011119
ratio_kernel_instr=1.29
ratio_kernel_cycles=1.76
ny_kernel_ipc=6.13
aot_status=ok
```

Read:

```text
bool_canonicalization_removed_from_hot_loop=1
kernel_cycles_improved=1
kernel_now_faster_than_c_pair=1
```

## Assembly Evidence

Command:

```bash
PERF_MICROASM_RUNNER_MODE=direct \
KEEP_PERF_MICROASM_ARTIFACTS=1 \
  bash tools/perf/bench_micro_aot_asm.sh \
    kilo_micro_userbox_flag_toggle ny_main 3
```

New hot loop:

```asm
movq   $0x0,-0x8(%rsp)
mov    $0x1,%eax
xor    %ecx,%ecx
add    %rax,-0x8(%rsp)
xor    %eax,%eax
cmp    $0xf4240,%rcx
setb   %al
inc    %rcx
cmp    $0x1e8480,%rcx
jne    <ny_main+0x10>
mov    -0x8(%rsp),%rax
ret
```

The prior `movzbl %cl,%ecx` and `and $0x1,%ecx` pair is gone from the loop.

## Guard Vocabulary

```text
front=kilo_micro_userbox_flag_toggle
selected_route=hako.typed_object.slot_load_i64/hako.typed_object.slot_store_i64
value_class=inline_bool
bool_scalar_lowering_residue_classified=1
bool_scalar_lowering_residue_fixed=1
bool_canonicalization_removed_from_hot_loop=1
source_semantics_changed=0
mirbuilder_truth_changed=0
startup_lane_reopened=0
product_nyrt_entry_changed=0
summary=ok
```

## Stop Line

```text
do not generalize this into a source-level bool semantics change
do not move bool truth into MIRBuilder
do not reopen startup from this front
do not claim product default startup speedup
do not widen Type ABI or plugin ABI for this slice
```

## Next

Return to front selection for the next non-folded exact-AOT owner.

Potential next selection rules:

```text
kernel_not_constant_folded=1
startup_share_low=1
kernel_delta_visible=1
c_pair_available=1
runner_status=ok
```
