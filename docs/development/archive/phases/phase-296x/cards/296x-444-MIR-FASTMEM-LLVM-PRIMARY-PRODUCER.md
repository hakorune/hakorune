---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-005 MIR -> LLVM/object primary producer for safe FastMemory MemOp subset.
Related:
  - docs/development/current/main/phases/phase-296x/296x-442-FASTMEM-PRODUCER-TASK-ORDER-REALIGN.md
  - docs/development/current/main/phases/phase-296x/296x-443-PYTHON-C-BRIDGE-RETIREMENT-GATE.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
---

# 296x-444 MIR FastMem LLVM Primary Producer

## Decision

Open the primary LLVM/object producer only for the FastMemory MemOps that have
complete value semantics without page-layout or allocator-owner runtime state:

```text
AddrOf
LogicalShr
BitAnd
Add
Sub
OwnerEq
```

Keep these MemOps closed until dedicated rows add the missing contracts:

```text
TableIndex:
  requires table/layout contract.

FieldLoad / FieldStore:
  require typed layout metadata and offset verification.

CurrentAllocOwnerId:
  requires allocator owner TLS/runtime intrinsic contract.
```

## Required Code Shape

```text
MIR JSON:
  emits fastmem_regions metadata
  emits memop instructions only for LLVM-supported MemOpKind subset
  fail-fast for unsupported MemOpKind

LLVM lowerer:
  lowers memop value subset to llvmlite integer/pointer operations
  no C layer on the primary path
  no Python-template C bridge deletion in this row
```

## Landed

```text
mir_json_memop_transport=1
fastmem_regions_metadata_json=1
llvm_memop_value_subset_lowering=1
llvm_memop_memory_layout_ops_lowering=0
llvm_current_alloc_owner_id_lowering=0
python_template_c_bridge_kept_as_baseline=1
```

## Boundaries

```text
replacement_front_producer=mir_to_llvm_lowering
python_template_c_bridge_kept_as_baseline=1
producer_neutral_parity_pass=0
python_template_c_bridge_retired=0
mir_to_c_required_before_llvm=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Next

```text
MIR-FMEM-006:
  producer-neutral parity against python_template_c_bridge.
```
