---
Status: Landed
Date: 2026-06-06
Scope: MIR-FMEM-008D-B OwnerEq LLVM producer lowering.
Related:
  - docs/development/current/main/phases/phase-296x/296x-482-MIR-FMEM-008D-PRE-OWNER-RUNTIME-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-483-MIR-FMEM-008D-A-CURRENT-ALLOC-OWNER-ID-LOWERING.md
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/tests/test_fastmem_memop_layoutref.py
---

# 296x-484 MIR-FMEM-008D-B OwnerEq Lowering

## Decision

Promote existing `owner_eq` lowering as the second owner-runtime slice:

```text
OwnerEq(a, b) -> icmp eq
```

This row only proves equality lowering. It does not choose an allocator route.

## Implemented

```text
owner_eq:
  arity = 2
  operands are resolved as ordinary i64-like owner-id scalars
  result is ordinary i1 bool in vmap
  LayoutRef operands are rejected by existing ordinary operand guard
```

## Still Closed

```text
same-owner local_free routing
remote-owner AtomicRemoteHead routing
TLS backing transfer
owner slot reuse as active owner
owner lifecycle transition
product activation / hook / global allocator / winner claim
```

## Acceptance

```bash
python3 -m unittest src.llvm_py.tests.test_fastmem_memop_layoutref
python3 -m py_compile src/llvm_py/instructions/memop.py src/llvm_py/tests/test_fastmem_memop_layoutref.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIR-FMEM-008D-C:
  add owner-runtime report/check closeout and require positive lowered counts
  for CurrentAllocOwnerId and OwnerEq when the owner-runtime producer profile
  is complete.
```
