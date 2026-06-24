---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-134.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - src/mir/builder/fastmem.rs
  - src/mir/builder/fields.rs
  - src/mir/builder/fastmem/tests/memops.rs
  - src/mir/builder/fastmem/tests/branch.rs
  - src/mir/builder/fastmem/tests/region.rs
  - tools/hako_check/fastmem_capability_inventory_common.py
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_capability_inventory_smoke.sh
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-633 MIM-PORT-FMEM-134 Field Route Retirement Landing

## Purpose

Record that fastmem field access now rides the shared `FieldGet` / `FieldSet`
builder path while keeping the fastmem region contract and verified-direct
obligations intact.

## Implementation

```text
fastmem field load:
  record fastmem field access site
  lower through shared FieldGet builder

fastmem field store:
  record fastmem field access site
  lower through shared FieldSet builder

inventory / check:
  field_access_required_verified_direct_count
  field_access_required_verified_direct_miss_count
  fastmem_verified_field_access_count
```

The field route no longer owns a dedicated `MemOp(FieldLoad/FieldStore)` path.
The shared field shell keeps the source shape while the verified-direct route
remains the contract truth.

## Verification

```bash
cargo test -q fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
FastMemory field accesses now use shared field builders with verified-direct
field evidence, and the dedicated field route lowerer is retired from the
source path.
```

## Closeout

```text
next: MIRBUILDER-FMEM-011 index route retirement
```
