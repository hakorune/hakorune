---
Status: Done
Date: 2026-06-06
Scope: Docs-only clarification for Python-template C bridge retirement timing.
Related:
  - docs/development/current/main/phases/phase-296x/296x-442-FASTMEM-PRODUCER-TASK-ORDER-REALIGN.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# 296x-443 Python C Bridge Retirement Gate

## Decision

The Python-template C replacement front is a temporary experiment/measurement
bridge and must not remain a semantic producer. It does not disappear at the
moment `MIR-FMEM-005` first produces LLVM/object code.

Retirement is gated by producer-neutral parity:

```text
MIR-FMEM-005:
  Build the primary MIR -> LLVM/object producer.
  Keep python_template_c_bridge as comparison baseline.

MIR-FMEM-006:
  Prove producer-neutral parity using the same report.kv / hako_check contract.
  Compare MIR-to-LLVM evidence against the current bridge.

MIR-FMEM-007:
  Retire python_template_c_bridge after parity passes.
```

## Why

The bridge still has useful work until parity is proven:

```text
baseline timing / counter evidence
report.kv schema comparison
hako_check contract comparison
diagnostic fallback for producer mismatch investigation
```

Removing it in `MIR-FMEM-005` would make mismatches harder to diagnose. Keeping
it past `MIR-FMEM-006` would keep duplicate allocator semantics longer than
needed.

## Retirement Gate

```text
replacement_front_producer=mir_to_llvm_lowering
producer_neutral_report_schema=1
producer_neutral_parity_pass=1
python_template_c_bridge_runtime_dependency_count=0
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Stop Line

Do not delete the bridge during `MIR-FMEM-005`. Do not keep it as a hidden
fallback after `MIR-FMEM-007`. Optional MIR-to-C debug/diff artifact support is
separate from the Python-template C bridge and may remain if it is generated
from MIR MemOps.
