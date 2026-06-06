---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008D-A CurrentAllocOwnerId LLVM producer lowering.
Related:
  - docs/development/current/main/phases/phase-296x/296x-482-MIR-FMEM-008D-PRE-OWNER-RUNTIME-PREFLIGHT.md
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/tests/test_fastmem_memop_layoutref.py
---

# 296x-483 MIR-FMEM-008D-A CurrentAllocOwnerId Lowering

## Decision

Open only the observation lowering for:

```text
MemOpKind::CurrentAllocOwnerId
```

The Python LLVM producer emits a call to the producer-local helper:

```text
hako_fastmem_current_alloc_owner_id() -> i64
```

The result is an ordinary scalar value in `vmap`, not a LayoutRef and not a raw
metadata pointer.

## Implemented

```text
current_alloc_owner_id:
  arity = 0
  dst required
  declares/calls hako_fastmem_current_alloc_owner_id
  writes returned i64 to ordinary vmap
```

This is a producer intrinsic/helper boundary. It is not a Type ABI lookup and
not a Provider ABI dispatch.

## Still Closed

```text
OwnerEq report/check promotion
owner-runtime complete profile
TLS backing transfer
owner slot reuse as active owner
same-owner local_free routing
remote-owner AtomicRemoteHead routing
thread-exit lifecycle mutation
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
MIR-FMEM-008D-B:
  promote OwnerEq equality lowering with explicit tests and keep routing policy
  closed.
```
