---
Status: Complete
Date: 2026-06-24
Scope: Close out MIR instruction JSON schema sync coverage.
---

# MIR-INSTRUCTION-SCHEMA-SYNC-CLOSEOUT-001

## Decision

Do not add a new generator or split `backend_core_ops.rs` in this slice.

The accepted P1 cleanup is already implemented: the MIR JSON schema owns an
explicit instruction `op` enum, and the backend-core contract test keeps that
enum synced with `LLVM_SUPPORTED_JSON_OPS`.

## Evidence

```text
docs/reference/mir/json_v0.schema.json
  definitions.instruction.properties.op.enum

src/mir/contracts/backend_core_ops.rs
  LLVM_SUPPORTED_JSON_OPS
  mir_json_schema_op_enum_matches_backend_opcode_allowlist
  instruction_diet_ledger_counts_match_docs_ssot
```

## Acceptance

```text
doc <-> ledger count sync remains covered
schema op enum <-> LLVM_SUPPORTED_JSON_OPS sync is covered
VariantMake / VariantTag / VariantProject are schema-visible
MemOp is schema-visible
new generator = 0
backend behavior changed = 0
```

## Verification

```text
cargo test -q mir_json_schema_op_enum_matches_backend_opcode_allowlist --lib
cargo test -q instruction_diet_ledger_counts_match_docs_ssot --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result:

```text
All commands above are green.
```

## Parked

```text
MIR-INSTRUCTION-DERIVED-DOC-SCHEMA-P2
  parked until instruction vocabulary churn justifies a generator

MIR-BACKEND-CORE-OPS-OWNER-SPLIT-P3
  parked until backend_core_ops.rs becomes hard to extend
```
