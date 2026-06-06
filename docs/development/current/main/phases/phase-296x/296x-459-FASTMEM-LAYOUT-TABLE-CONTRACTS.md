---
Status: Landed
Date: 2026-06-06
Row: MIR-FMEM-008B
Scope: concrete FastMemory layout/table contract resolution before LLVM GEP/load/store.
Related:
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-457-VERIFIED-MEM-ACCESS-PLAN.md
---

# 296x-459 FastMemory Layout/Table Contracts

## Purpose

Move `FastMemAccessPlan` rows from symbolic-only metadata toward verified
memory-profile access plans.

This row does not open LLVM GEP/load/store lowering. It prepares the verified
truth lowering will later consume.

## Decision

```text
FastMemLayoutContractV0:
  PageMetaLayoutV0

FastMemTableContractV0:
  page_table

Verifier/contract resolver:
  owns canonical field ids
  owns alias normalization
  owns offset/type/alignment/mutability/field-class facts
  owns table element representation/stride/alignment/bounds policy

Lowering:
  remains closed
```

## Selected

```text
PageMetaLayoutV0 field contract
owner_id compatibility alias to owner_worker_id
page_table contract shell
FastMemAccessPlan verified/rejected transition
JSON continues to emit canonical resolved facts
focused Rust unit tests
```

## Deferred

```text
LLVM GEP/load/store emission
CurrentAllocOwnerId / OwnerEq
AtomicRemoteHead behavior
TLS backing transfer
owner slot reuse
Python-template C bridge retirement
product activation
hook install
global allocator claim
winner claim
```

## Acceptance

```bash
cargo test -q fastmem_layout_contract --lib
cargo test -q fastmem_access_plan --lib
cargo test -q fastmem --lib
bash tools/checks/current_state_pointer_guard.sh
```

Expected contract shape:

```text
FieldLoad owner_id:
  verified as owner_worker_id

FieldStore local_free_head:
  verified

FieldStore remote_head:
  rejected as atomic-field-plain-store

TableIndex page_table:
  contract facts are resolved, but lowering remains closed until bounds policy
  is strong enough for LLVM GEP.
```

## Landed

```text
src/mir/fastmem_layout_contract.rs:
  PageMetaLayoutV0 field contract
  page_table contract shell
  owner_id -> owner_worker_id compatibility alias
  remote_head plain FieldStore rejection

src/mir/fastmem_access_plan.rs:
  field access plans become verified through the contract resolver
  page_table access plans carry resolved shell facts but stay rejected as
  table-length-unresolved

Lowering:
  still closed
```

Proof:

```bash
cargo test -q fastmem_layout_contract --lib
cargo test -q fastmem_access_plan --lib
cargo test -q fastmem --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Next

Before LLVM GEP/load/store lowering opens, decide the `TableIndex` bounds policy:

```text
Option A:
  keep page_table length unresolved and lower only field-only verified rows

Option B:
  introduce an explicit PageTableLengthV0 contract

Option C:
  require a MIR bounds proof row before TableIndex becomes lowerable
```

## Stop Line

Stop and ask for design review before lowering if any of these become true:

```text
LLVM lowerer recomputes layout offsets
field aliases leak into verified JSON
table representation is guessed from names
remote_head can be written through plain FieldStore
TableIndex is marked lowerable without explicit bounds policy
```
