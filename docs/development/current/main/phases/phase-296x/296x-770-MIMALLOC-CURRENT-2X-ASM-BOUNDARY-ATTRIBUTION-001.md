---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001
Scope: Attribute the current reproducible ~2x object-lifecycle body gap with
  direct-exact perf/asm evidence before reopening implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-769-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-FOR-CURRENT-2X-GAP-001.md
  - docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001

## Purpose

296x-769 found mixed runtime/object/generated-runtime seams under the current
~2x body gap. This row uses the canonical direct-exact perf/asm tool to decide
whether a fresh high-confidence owner exists.

This is an attribution row, not an implementation row.

## Measurement

```text
output_contract=hako-mimalloc-current-2x-asm-boundary-attribution-v0
source_evidence=296x-769,296x-709
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
perf_tool=tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh
hako_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
runs=20
in_process_operation_repeat=65536
requested_bytes=2179334144
body_elapsed_ns=54000000
direct_exact_env_contract=mimalloc-direct-exact-env-v0
worker_front_mismatch_guard=1
top_symbol=nyash_array_length_h
top_symbol_percent=70.71
top_symbol_is_target=1
target_symbol=nyash_array_length_h
nonzero_annotate_line_count=175
annotate_nonzero_instruction_count=7
annotate_total_local_percent=100.00
top_instruction_percent=64.27
top_instruction_mnemonic=and
top_instruction_asm=and    $0xfffffffffffffff2,%rax
hot_instruction_0_context=428a16:arithmetic_compare:none:xor    %r15d,%r15d|428a19:other:none:mov    $0xfffffffffffffff0,%rax|428a20:store_like:none:lock xadd %rax,0x8(%r14)|428a26:arithmetic_compare:none:and    $0xfffffffffffffff2,%rax|428a2a:arithmetic_compare:none:cmp    $0x12,%rax|428a2e:branch:none:je     428be0 <nyash_array_length_h+0x2c0>|428a34:arithmetic_compare:none:test   %r15,%r15
hot_instruction_1_percent=34.94
hot_instruction_1_mnemonic=jne
hot_instruction_1_asm=jne    4289d8 <nyash_array_length_h+0xb8>
hot_instruction_1_context=4289e9:branch:none:jmp    4289f7 <nyash_array_length_h+0xd7>|4289eb:memory:none:lea    0x10(%rax),%rcx|4289ef:store_like:none:lock cmpxchg %rcx,0x8(%r14)|4289f5:branch:none:jne    4289d8 <nyash_array_length_h+0xb8>|4289f7:memory:0x20:page_id:cmp    0x20(%r14),%rbx|4289fb:branch:none:jae    428a16 <nyash_array_length_h+0xf6>|4289fd:memory:none:mov    0x18(%r14),%rcx
artifact_dir=/tmp/hakorune_current_2x_helper_asm.YSD05A/asm.out.artifacts.d
perf_report=/tmp/hakorune_current_2x_helper_asm.YSD05A/asm.out.artifacts.d/perf-report.txt
perf_annotate=/tmp/hakorune_current_2x_helper_asm.YSD05A/asm.out.artifacts.d/perf-annotate.txt
```

## Classification

The current top symbol is still `nyash_array_length_h`:

```text
top_symbol=nyash_array_length_h
top_symbol_percent=70.71
```

However, the narrow helper-local row already landed:

```text
array_length_helper_uses_borrowed_ready=1
helper_local_fastpath_already_applied=1
```

Current helper annotation is dominated by atomic Arc / handle-carrier
operations:

```text
lock xadd %rax,0x8(%r14)
lock cmpxchg %rcx,0x8(%r14)
```

This matches the previous post-helper refresh owner:

```text
remaining_owner=handle_registry_typed_handle_boundary
```

## Result

```text
output_contract=hako-mimalloc-current-2x-asm-boundary-attribution-v0
source_evidence=296x-769,296x-709
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
runs=20
in_process_operation_repeat=65536
body_elapsed_ns=54000000
top_symbol=nyash_array_length_h
top_symbol_percent=70.71
target_symbol=nyash_array_length_h
top_symbol_is_target=1
array_length_helper_uses_borrowed_ready=1
helper_local_fastpath_already_applied=1
helper_local_fastpath_remaining=0
remaining_owner=handle_registry_typed_handle_boundary
remaining_owner_confidence=high
selected_owner=handle_registry_typed_handle_boundary
selected_owner_confidence=high
implementation_allowed=0
design_required=1
closed_world_route_required=1
object_substrate_required=1
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
source_hako_changed=0
mirbuilder_object_management_enabled=0
benchmark_name_special_case=0
helper_name_special_case=0
winner_claim=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001
summary=ok
```

## Decision

Do not add another helper-name patch. The current hot symbol is precise, but
the remaining owner is no longer a local `Array.length` helper issue. It is the
handle registry / typed handle boundary around the generic runtime object
carrier.

The next row is a design row. It must decide whether a clean exact-AOT path can
avoid this boundary through RoutePlan / ObjectStoragePlan / closed-world handle
proof, without moving Box management into MIRBuilder and without changing
product defaults.

## Stop Line

```text
do not change nyash_array_length_h again without a new proof
do not add helper-name or benchmark-name compiler branches
do not replace ArrayBox.length with raw layout lowering from this row
do not move Box/Object management into MIRBuilder
do not change runtime object representation in this row
do not change source .hako, product defaults, provider activation,
  replacement, hooks, or global allocator
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001:
  decide the clean boundary for avoiding the handle registry / typed handle
  cost in exact-AOT closed-world routes, likely through RoutePlan /
  ObjectStoragePlan proof, and keep implementation closed until that design
  selects one narrow keeper seam
```
