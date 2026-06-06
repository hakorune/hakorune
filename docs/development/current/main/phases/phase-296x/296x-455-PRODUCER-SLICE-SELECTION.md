---
Status: Done
Date: 2026-06-06
Scope: fix MIR-FMEM-008A producer-slice selection in report/check evidence before opening lowering behavior.
Blocker: MIR-FMEM-008A
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - tools/hako_check/replacement_front_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-455 Producer Slice Selection

## Purpose

`MIR-FMEM-008A` selects the smallest MIR-to-LLVM replacement-front producer
slice after the diagnostic lifecycle bridge rows landed. This row changes
report/check contracts only. It does not open new LLVM lowering behavior.

## Decision

Next producer body slice:

```text
replacement_front_next_producer_slice=layout_table_producer_pilot
replacement_front_selected_memop_family=layout_table
replacement_front_selected_memop_kinds=TableIndex,FieldLoad,FieldStore
```

Deferred slice:

```text
replacement_front_deferred_memop_family=owner_runtime
replacement_front_deferred_memop_kinds=CurrentAllocOwnerId,OwnerEq
```

The selection is intentionally before behavior:

```text
replacement_front_selection_behavior_change=0
replacement_front_selection_product_activation=0
replacement_front_selection_bridge_retirement_allowed=0
```

## Report / Check Surface

`replacement-front-report`, `fastmem-capability-inventory`,
`fastmem-check`, and `fastmem-producer-parity` now carry the selection fields.
`fastmem-check` fails if `replacement_front_producer_slice_selection_v0=1`
selects owner-runtime first, changes behavior, opens product activation, or
allows bridge retirement.

This makes the next step machine-readable:

```text
MIR-FMEM-008B:
  implement layout/table producer pilot only

MIR-FMEM-008C:
  implement allocator owner runtime MemOps later
```

## Stop Line

Still closed:

```text
LLVM lowering behavior change
CurrentAllocOwnerId / OwnerEq lowering
AtomicRemoteHead lowering
TLS backing transfer
owner slot reuse as active owner
Python-template C diagnostic baseline retirement
product activation
hook install
global allocator claim
winner claim
```

## Acceptance

```bash
python3 -m py_compile \
  tools/hako_check/replacement_front_report.py \
  tools/hako_check/fastmem_capability_inventory_common.py \
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_check.py \
  tools/hako_check/fastmem_producer_parity.py

bash tools/hako_check/replacement_front_report_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_producer_parity_smoke.sh
```

Next row:

```text
MIR-FMEM-008B layout/table producer pilot
```
