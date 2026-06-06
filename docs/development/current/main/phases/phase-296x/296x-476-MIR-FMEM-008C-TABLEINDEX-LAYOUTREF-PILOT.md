---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008C TableIndex to LayoutRef LLVM producer pilot.
Related:
  - docs/development/current/main/phases/phase-296x/296x-475-MIR-FMEM-008C-LAYOUTREF-DECISION.md
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/resolver.py
  - src/llvm_py/context/function_lower_context.py
  - src/llvm_py/tests/test_fastmem_memop_layoutref.py
---

# 296x-476 MIR-FMEM-008C TableIndex LayoutRef Pilot

## Decision

Open the first behavior slice of `MIR-FMEM-008C`:

```text
TableIndex -> LayoutRef
```

Do not open `FieldLoad` or `FieldStore` yet.

## Implemented

The Python LLVM producer now has a function-local FastMemory LayoutRef map:

```text
resolver.fastmem_layout_refs[ValueId]
```

The map is backed by `FunctionLowerContext.fastmem_layout_refs`, so raw
metadata pointers remain function-local and do not enter ordinary `vmap`.

`MemOpKind::TableIndex` lowering now:

```text
requires a verified table_index access plan at the current block/instruction
requires all complete table proof bits
requires element_repr=pointer_to_element
resolves table operand as an address-like i64
computes slot_addr = table_addr + index * element_stride
loads the element address from the pointer slot
stores the resulting LLVM pointer only in fastmem_layout_refs[dst]
```

If a LayoutRef value is later requested as an ordinary scalar operand, lowering
fails fast:

```text
[llvm/fastmem:layout-ref-as-ordinary-value]
```

## Boundary

Allowed:

```text
TableIndex lowerable only from complete verified plans
backend-private raw pointer storage in fastmem_layout_refs
ordinary vmap remains pointer-free for metadata pointers
```

Forbidden:

```text
FieldLoad
FieldStore
inline_element table representation
null-policy checks
LayoutRef Copy / Phi
CurrentAllocOwnerId / OwnerEq changes
AtomicRemoteHead
Python-template C bridge fallback
product activation, hook install, global allocator, winner claim
```

## Acceptance

```bash
python3 -m unittest src.llvm_py.tests.test_fastmem_memop_layoutref
python3 -m unittest src.llvm_py.tests.test_fastmem_metadata_loader
python3 -m py_compile \
  src/llvm_py/instructions/memop.py \
  src/llvm_py/resolver.py \
  src/llvm_py/context/function_lower_context.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIR-FMEM-008C FieldLoad pilot:
  consume LayoutRef from fastmem_layout_refs and emit verified GEP/load for
  readonly scalar/plain-pointer fields only.
```
