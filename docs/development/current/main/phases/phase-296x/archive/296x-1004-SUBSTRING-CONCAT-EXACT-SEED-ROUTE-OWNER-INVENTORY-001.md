# 296x-1004 SUBSTRING-CONCAT-EXACT-SEED-ROUTE-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: owner inventory / exact-seed route shape

## Contract

```text
output_contract=hako-substring-concat-exact-seed-route-owner-inventory-v0
source_evidence=296x-1003,target/substring-concat-exact-seed-owner-1004
row_kind=owner_inventory
implementation_started=0

target_front=kilo_micro_substring_concat
selected_route=substring_concat_loop_ascii
selected_route_owner=function_level_exact_seed
selected_backend_consumer=substring_concat_loop_ascii

c_loop_shape=accumulate_const_len_only
c_loop_body_observable_text_copy=0
c_loop_acc_delta=18
c_loop_iterations=300000
c_return_masks_acc_low_byte=1

aot_loop_shape=stack_byte_copy_rotation_then_const_return
aot_loop_iterations=300000
aot_loop_contains_runtime_helper_call=0
aot_loop_contains_heap_allocation=0
aot_loop_contains_publication=0
aot_final_return_constant=5400016
aot_final_return_hex=0x5265d0

runtime_boundary_owner=0
generic_fastpath_consumer_owner=0
exact_seed_route_retire_selected=0
selected_owner_family=exact_seed_dead_byte_copy_loop
selected_next=SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-DESIGN-001
summary=ok
```

## Purpose

Classify the selected `kilo_micro_substring_concat` route before implementation.

The fresh selection row proved the selected route is not a product runtime
helper and not a generic fastpath consumer. This row compares the C pair and
the selected exact-AOT `ny_main` body to identify the next seam.

## Evidence

Artifacts:

```text
target/substring-concat-exact-seed-owner-1004/c.main.txt
target/substring-concat-exact-seed-owner-1004/aot.ny_main.txt
target/fresh-compiler-owner-selection-1003/substring_concat_microasm.log
```

C pair loop:

```asm
mov    -0x8(%rsp),%rax
add    $0x12,%rax
dec    %rdx
mov    %rax,-0x8(%rsp)
jne    ...
movzbl %al,%eax
ret
```

Reading:

```text
C compiler reduces the string work to acc += 18 in the loop.
The volatile accumulator prevents total loop removal, but the text copy is gone.
```

AOT exact seed loop:

```asm
mov    (%rax),%esi
movzwl 0x4(%rax),%edi
movzbl 0x6(%rax),%r8d
...
mov    %r8b,0x6(%rdx)
mov    %di,0x4(%rdx)
mov    %esi,(%rdx)
movb   $0x78,-0x1(%rsp)
movb   $0x78,-0x10(%rsp)
dec    %rcx
jne    ...
mov    $0x5265d0,%eax
ret
```

Reading:

```text
The exact seed already returns a constant final value, but it still executes
the stack byte-copy / rotation loop. No runtime helper, allocation, or public
StringBox boundary is in the selected loop body.
```

## Decision

The next owner is:

```text
exact_seed_dead_byte_copy_loop
```

The next implementation design should use a guarded closed-form return seam for
this exact seed route, not a new product-runtime helper and not a generic
`StringDeadTextRegionPlan` reachability force.

## Not Selected

```text
runtime_helper_boundary:
  rejected
  reason=ny_main loop has no runtime helper call

generic_fastpath_consumer:
  rejected
  reason=current MIR has exact_seed selected and no current
         string_dead_text_region_plans candidate

exact_seed_retire_or_reprioritize:
  rejected_for_now
  reason=the active selected route is useful; first fix its selected body shape
```

## Stop Line

```text
do not branch by benchmark name
do not branch by source path
do not infer from helper names
do not force StringDeadTextRegionPlan reachability
do not retire substring_concat_loop_ascii as a drive-by
do not change product StringBox storage
do not add a runtime helper
```

## Next

```text
SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-DESIGN-001
```

The design row must pin the exact metadata/route conditions required before
the backend may replace the selected exact-seed loop body with:

```text
ret i64 <final_return_constant>
```

Unknown means no lowering.
