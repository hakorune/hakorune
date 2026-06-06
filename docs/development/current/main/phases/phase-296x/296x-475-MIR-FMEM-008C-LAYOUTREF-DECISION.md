---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008C TableIndex result representation decision.
Related:
  - docs/development/current/main/phases/phase-296x/296x-474-MIR-FMEM-008C-PREFLIGHT-METADATA.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/resolver.py
---

# 296x-475 MIR-FMEM-008C LayoutRef Decision

## Decision

Use a `LayoutRef` truth boundary for FastMemory `TableIndex` results.

```text
Verifier / MIR / report:
  TableIndex result = LayoutRef token

LLVM producer internal:
  fastmem_layout_refs[ValueId] = raw LLVM pointer

ordinary vmap:
  raw metadata pointer is forbidden
```

This keeps raw metadata pointers out of ordinary values while letting LLVM use
real pointer values as a backend-private implementation detail.

## V0 Lowering Shape

`TableIndex` is the only behavior opened by this row.

V0 consumes complete verified access plans only:

```text
verified=true
kind=table_index
status=verified
table_length_resolved=1
bounds_proof_valid=1
stride_resolved=1
field_offset_resolved=1
overflow_proof_valid=1
alignment_valid=1
element_layout_verified=1
```

The first table representation is:

```text
element_repr=pointer_to_element
```

For the Python LLVM producer v0, the table operand is resolved as an exact
address-like i64 value. The lowerer computes the pointer-slot address from the
verified stride:

```text
slot_addr = table_addr + index * element_stride
slot_ptr = inttoptr(slot_addr, i64*)
element_addr = load slot_ptr
layout_ref_ptr = inttoptr(element_addr, i8*)
fastmem_layout_refs[result] = LayoutRef(layout_ref_ptr, layout_id, table_id, region)
```

This is a backend-private pointer representation. It is not written to
ordinary `vmap`.

## Guard

If a LayoutRef value is requested as an ordinary scalar, lowering must fail
fast:

```text
LayoutRef used as ordinary value -> [llvm/fastmem:layout-ref-as-ordinary-value]
```

Allowed consumers are future memory-profile ops:

```text
FieldLoad
FieldStore
```

Still forbidden:

```text
Return
Call arg
Box field store
Array store
Provider ABI crossing
Type ABI hot lookup
ordinary arithmetic / compare / debug-print-as-address
```

## Deferred

Do not open in this row:

```text
FieldLoad
FieldStore
inline_element table representation
null-policy checks
LayoutRef Phi
LayoutRef Copy
CurrentAllocOwnerId / OwnerEq
AtomicRemoteHead
product activation, hook install, global allocator, winner claim
```

## Acceptance

```bash
python3 -m unittest src.llvm_py.tests.test_fastmem_memop_layoutref
python3 -m py_compile src/llvm_py/instructions/memop.py src/llvm_py/resolver.py src/llvm_py/context/function_lower_context.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIR-FMEM-008C FieldLoad pilot:
  consume LayoutRef from fastmem_layout_refs and emit verified GEP/load for
  readonly scalar/plain-pointer fields only.
```
