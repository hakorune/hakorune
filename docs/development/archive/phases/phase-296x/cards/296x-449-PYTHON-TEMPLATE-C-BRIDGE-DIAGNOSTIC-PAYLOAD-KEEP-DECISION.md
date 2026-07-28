---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-007D diagnostic payload keep/archive decision for the retired Python-template C bridge.
Related:
  - docs/development/current/main/phases/phase-296x/296x-446-PYTHON-TEMPLATE-C-BRIDGE-RETIREMENT-FIRST-SLICE.md
  - docs/development/current/main/phases/phase-296x/296x-447-PYTHON-TEMPLATE-C-BRIDGE-QUARANTINE-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-448-PYTHON-TEMPLATE-C-BRIDGE-DIAGNOSTIC-IMPORT-GUARD.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
---

# 296x-449 Python Template C Bridge Diagnostic Payload Keep Decision

## Decision

Keep the remaining Python-template C diagnostic payloads for now.

Do not archive or delete fixed-slot / bins payloads in this slice. The bridge is
retired from normal runtime use, but it still serves as an explicit diagnostic
baseline until the MIR-to-LLVM replacement-front producer covers allocator
layout/table/owner runtime behavior with equivalent report.kv evidence.

## Reason

Current MIR-to-LLVM FastMemory support is intentionally narrow:

```text
open:
  value-only FastMemory MemOps

still closed:
  layout/table MemOps
  allocator owner TLS runtime MemOps
  replacement-front malloc/free execution equivalent
  AtomicRemoteHead lowering
```

Deleting the diagnostic payloads now would remove useful baseline evidence
before the replacement producer can cover the same surfaces.

Fixed-slot payloads also stay because they are still used by:

```text
tools/allocator/python_template_c_bridge_retirement_smoke.sh
tools/allocator/hakmem_fixture_ldpreload_compare.py
```

as explicit diagnostic-baseline proof. They are no longer normal runtime
entrypoints because MIR-FMEM-007/007B/007C already require the baseline flag,
guard build helpers, and block direct payload imports from normal tools.

## Keep

```text
tools/allocator/replacement_front_templates.py
tools/allocator/replacement_front_shim_templates.py
tools/allocator/replacement_front_bins_templates.py
tools/allocator/replacement_front_shim_report_source.py
tools/allocator/replacement_front_bins_report_source.py
tools/allocator/replacement_front_smoke_templates.py
tools/allocator/replacement_front_smokes.py
```

## Archive/Delete Gate

The remaining payloads can be archived or deleted only after all are true:

```text
replacement_front_producer=mir_to_llvm_lowering
replacement_front_backend_artifact=object|exe
replacement_front_mir_memop_enabled=1
replacement_front_mir_fastmem_region_enabled=1
layout_table_memops_lowered=1
allocator_owner_runtime_memops_lowered=1
producer_neutral_report_contract_pass=1
python_template_c_bridge_runtime_dependency_count=0
```

## Rejected

```text
delete fixed-slot bridge payloads now:
  rejected; still used by explicit retirement / hakmem diagnostic baselines

delete bins/page payloads now:
  rejected; still provide product-shaped bridge baseline vocabulary until
  layout/table MemOps have replacement-front execution evidence

move payloads into normal tool paths:
  rejected; they remain diagnostic-only and guarded
```

## Next

```text
MIM-FMEM-018:
  return to allocator owner lifecycle work: thread-exit flush, abandoned owner
  mark/reclaim, and generation bump state machine.
```
