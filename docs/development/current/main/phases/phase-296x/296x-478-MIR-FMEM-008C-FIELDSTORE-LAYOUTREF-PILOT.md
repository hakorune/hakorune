---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008C FieldStore from LayoutRef LLVM producer pilot.
Related:
  - docs/development/current/main/phases/phase-296x/296x-475-MIR-FMEM-008C-LAYOUTREF-DECISION.md
  - docs/development/current/main/phases/phase-296x/296x-477-MIR-FMEM-008C-FIELDLOAD-LAYOUTREF-PILOT.md
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/tests/test_fastmem_memop_layoutref.py
---

# 296x-478 MIR-FMEM-008C FieldStore LayoutRef Pilot

## Decision

Open the third behavior slice of `MIR-FMEM-008C`:

```text
FieldStore(base: LayoutRef, field_id, value)
```

This row opens only mutable plain fields. It does not open owner mutation,
atomic/publication fields, local-free-head semantics, or allocator lifecycle
behavior.

## Implemented

`MemOpKind::FieldStore` lowering now:

```text
requires a verified field_store access plan at the current block/instruction
requires base to be present in resolver.fastmem_layout_refs
requires base.layout_id == plan.layout_id
requires mutability=mutable
requires field_class in plain_scalar | plain_pointer
requires an 8-byte usize/u64/i64/pointer-like field type
resolves the store value as an ordinary i64 operand
computes field_addr = layout_ref_ptr + byte_offset
stores the i64 value through the verified field pointer
```

`FieldStore` has no result and must not write a metadata pointer into ordinary
`vmap`.

## Boundary

Allowed:

```text
FieldStore only from a value present in fastmem_layout_refs
verified field_store access plan at the current block/instruction
mutable plain scalar / plain pointer fields
LLVM GEP/store from verifier-provided offset/type/alignment metadata
```

Forbidden:

```text
FieldStore from ordinary vmap base values
FieldStore from layout-mismatched LayoutRef
owner_worker_id / page_claim_only fields
immutable_after_claim fields
local_free_head fields
remote_head / atomic / synchronized publication fields
Type ABI hot lookup
Provider ABI hot dispatch
Python-template C bridge fallback
product activation, hook install, global allocator, winner claim
```

## Acceptance

```bash
python3 -m unittest \
  src.llvm_py.tests.test_fastmem_memop_layoutref \
  src.llvm_py.tests.test_fastmem_metadata_loader
python3 -m py_compile \
  src/llvm_py/instructions/memop.py \
  src/llvm_py/resolver.py \
  src/llvm_py/context/function_lower_context.py \
  src/llvm_py/tests/test_fastmem_memop_layoutref.py \
  src/llvm_py/tests/test_fastmem_metadata_loader.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIR-FMEM-008C report/check closeout:
  add producer-neutral evidence for TableIndex->LayoutRef, FieldLoad, and
  FieldStore coverage before reopening owner-runtime MemOps.
```
