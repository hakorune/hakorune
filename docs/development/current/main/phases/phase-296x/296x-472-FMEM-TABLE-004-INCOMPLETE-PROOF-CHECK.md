---
Status: Active
Date: 2026-06-06
Scope: FMEM-TABLE-004 report/check rejection for incomplete FastMemory table access proofs.
Related:
  - docs/development/current/main/phases/phase-296x/296x-471-FMEM-TABLE-003B-OVERFLOW-PROOF.md
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_capability_inventory_common.py
  - tools/hako_check/tests/fastmem_capability_inventory/bad_table_access_proof_inventory.kv
---

# 296x-472 FMEM-TABLE-004 Incomplete Proof Check

## Decision

Add explicit `fastmem-check` fail-fast fields for incomplete FastMemory
`TableIndex` access proofs.

New fields:

```text
fastmem_table_access_proof_incomplete_count
fastmem_table_overflow_proof_missing_count
```

Both fields are zero by default and fail `fastmem-check` when nonzero.

## Boundary

Allowed:

```text
add report/check inventory fields
add a bad fixture proving incomplete table proof rejection
keep existing layout/table producer pilot gates
```

Forbidden:

```text
open LLVM lowering
change MIR proof construction
query Type ABI / Provider ABI
open product activation, hook install, global allocator, or winner claim
```

## Acceptance

```bash
python3 -m py_compile \
  tools/hako_check/replacement_front_report.py \
  tools/hako_check/fastmem_capability_inventory_common.py \
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_check.py

bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIR-FMEM-008C:
  open LLVM lowering only for complete VerifiedTableAccess rows
```
