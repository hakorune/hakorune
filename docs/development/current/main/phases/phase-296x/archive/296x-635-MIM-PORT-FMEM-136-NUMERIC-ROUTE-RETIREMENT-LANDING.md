---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-136.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - src/mir/builder/fastmem.rs
  - src/mir/builder/ops/mod.rs
  - src/mir/builder/fastmem/tests/memops.rs
  - src/mir/builder/fastmem/tests/region.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-635 MIM-PORT-FMEM-136 Numeric Route Retirement Landing

## Purpose

Record that fastmem numeric binary operations now use the shared binary helper
path while keeping the numeric route evidence and ordinary BinOp shape intact.

## Implementation

```text
fastmem numeric binary ops:
  evaluate operands in fastmem context
  lower through shared build_binary_op_from_values helper

inventory / check:
  fastmem_numeric_verified_direct_count
  fastmem_numeric_required_route_miss_count
  fastmem_dedicated_binary_op_lowering_count=0
```

The numeric route no longer owns a dedicated `BinaryOp -> MemOp(Add/Sub/Shr/BitAnd)`
source lowerer. The shared binary helper keeps the source shape while the
verified-direct numeric route remains the contract truth.

## Verification

```bash
cargo test -q fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
FastMemory numeric binary operations now use the shared binary helper path
with verified-direct numeric evidence, and the dedicated numeric route lowerer
is retired from the source path.
```

## Closeout

```text
next: MIRBUILDER-FMEM-013 intrinsic registry cleanup
```
