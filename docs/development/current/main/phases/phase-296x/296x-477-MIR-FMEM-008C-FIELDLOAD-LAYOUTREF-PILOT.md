---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008C FieldLoad from LayoutRef LLVM producer pilot.
Related:
  - docs/development/current/main/phases/phase-296x/296x-475-MIR-FMEM-008C-LAYOUTREF-DECISION.md
  - docs/development/current/main/phases/phase-296x/296x-476-MIR-FMEM-008C-TABLEINDEX-LAYOUTREF-PILOT.md
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/tests/test_fastmem_memop_layoutref.py
---

# 296x-477 MIR-FMEM-008C FieldLoad LayoutRef Pilot

## Decision

Open the second behavior slice of `MIR-FMEM-008C`:

```text
FieldLoad(base: LayoutRef, field_id) -> ordinary scalar value
```

`TableIndex` remains the only producer of backend-private LayoutRefs in this
row. `FieldLoad` is the first allowed LayoutRef consumer.

## Implemented

`MemOpKind::FieldLoad` lowering now:

```text
requires a verified field_load access plan at the current block/instruction
requires base to be present in resolver.fastmem_layout_refs
requires base.layout_id == plan.layout_id
requires field_class in plain_scalar | plain_pointer
requires an 8-byte usize/u64/i64/pointer-like field type
computes field_addr = layout_ref_ptr + byte_offset
loads the field as i64
writes only the loaded scalar to ordinary vmap
```

The implementation keeps raw metadata pointers backend-private and fail-fast
rejects non-LayoutRef bases, layout mismatches, and atomic/publication fields.

## Boundary

Allowed:

```text
FieldLoad only from a value present in fastmem_layout_refs
verified field_load access plan at the current block/instruction
base.layout_id == plan.layout_id
readonly plain scalar / plain pointer fields
LLVM GEP/load from verifier-provided offset/type/alignment metadata
ordinary vmap output only for the loaded scalar value
```

Forbidden:

```text
FieldStore
FieldLoad from ordinary vmap values
FieldLoad from layout-mismatched LayoutRef
remote_head / atomic / synchronized publication fields
owner_worker_id mutation
layout offset recomputation in lowering
Type ABI hot lookup
Provider ABI hot dispatch
Python-template C bridge fallback
product activation, hook install, global allocator, winner claim
```

## Lowering Contract

The LLVM producer may keep raw metadata pointers only in:

```text
resolver.fastmem_layout_refs
```

`FieldLoad` consumes that private map and writes only the loaded scalar result
to ordinary `vmap`.

Lowering must fail fast when a `FieldLoad` site has no verified access plan or
when the base is not a LayoutRef:

```text
[llvm/fastmem:missing-verified-field-load-plan]
[llvm/fastmem:expected-layout-ref]
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
MIR-FMEM-008C FieldStore pilot:
  consume LayoutRef from fastmem_layout_refs and emit verified GEP/store for
  mutable plain fields only. Keep owner fields and atomic/publication fields
  closed until their dedicated owner/AtomicRemoteHead rows.
```
