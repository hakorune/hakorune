---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008C preflight metadata loader and lowering design-stop findings.
Related:
  - docs/development/current/main/phases/phase-296x/296x-473-MIR-FMEM-008C-HANDOFF-ORDER.md
  - docs/development/current/main/phases/phase-296x/296x-472-FMEM-TABLE-004-INCOMPLETE-PROOF-CHECK.md
  - src/llvm_py/builders/function_metadata.py
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/tests/test_fastmem_metadata_loader.py
---

# 296x-474 MIR-FMEM-008C Preflight Metadata

## Decision

Open only the metadata-loader part of `MIR-FMEM-008C-PRE`.

The Rust MIR JSON producer already emits:

```text
field_size
element_size
```

The Python LLVM metadata loader must preserve those fields before any
GEP/load/store lowering can consume verified access plans.

## Implemented

`_load_fastmem_access_plan_metadata()` now normalizes:

```text
field_size
element_size
```

alongside the existing block/instruction/region/table/index/offset/stride
numeric fields.

This is a producer-readiness fix only.

## Preflight Finding

Current Python LLVM `memop.py` lowers only value-style FastMemory MemOps:

```text
AddrOf
LogicalShr
BitAnd
Add
Sub
OwnerEq
```

It does not yet lower:

```text
TableIndex
FieldLoad
FieldStore
```

The verified access plan rows are now loadable by `(block, instruction_index)`,
but the actual lowering row should stop for design consultation before changing
behavior because these seams are still ambiguous:

```text
table operand pointer representation
TableIndex result value representation in vmap
whether pointer_to_element loads should produce a raw LLVM pointer value or a
region-local LayoutRef-like handle
how FieldLoad / FieldStore select LLVM scalar type from field_type
where to record lowering evidence counters without making hako_check producer-
specific
```

## Boundary

Allowed:

```text
preserve all verified access-plan numeric metadata needed by the lowerer
add a Python unit test for metadata normalization
document the design-stop seam
```

Forbidden:

```text
open TableIndex lowering
open FieldLoad / FieldStore lowering
invent a pointer representation in the lowerer
query Type ABI / Provider ABI
call Python-template C bridge
open product activation, hook install, global allocator, or winner claim
```

## Acceptance

```bash
python3 -m unittest src.llvm_py.tests.test_fastmem_metadata_loader
python3 -m py_compile src/llvm_py/builders/function_metadata.py src/llvm_py/instructions/memop.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
Design consult:
  decide the TableIndex result representation and the first safe FieldLoad /
  FieldStore lowering boundary before MIR-FMEM-008C-A.
```
