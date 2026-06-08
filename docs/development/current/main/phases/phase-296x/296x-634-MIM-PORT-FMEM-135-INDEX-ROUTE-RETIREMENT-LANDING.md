---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-135.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - src/mir/builder/fastmem.rs
  - src/mir/builder/indexing.rs
  - src/mir/builder/fastmem/ops.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-634 MIM-PORT-FMEM-135 Index Route Retirement Landing

## Purpose

Record that fastmem index access now routes through the shared index helper
path while keeping the fastmem region contract and verified-table evidence
intact.

## Implementation

```text
fastmem index load/store:
  record fastmem index access site
  lower through shared build_index_access_from_values helper

inventory / check:
  index_access_required_verified_table_count
  index_access_required_verified_table_miss_count
  fastmem_index_access_site_count
```

The index route no longer owns a dedicated `Index -> MemOp(TableIndex)` source
lowerer. The shared index helper keeps the source shape while the verified-table
route remains the contract truth.

## Verification

```bash
cargo test -q fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
FastMemory index accesses now use the shared index helper path with verified-
table evidence and the dedicated index route lowerer is retired from the source
path.
```

## Closeout

```text
next: MIRBUILDER-FMEM-012 numeric route retirement
```
