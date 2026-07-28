---
Status: Done
Date: 2026-06-06
Scope: Docs-only realignment of the FastMemory producer transition order.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-442 FastMemory Producer Task Order Realign

## Decision

The required producer path is the primary product path:

```text
.hako fastmem / capability source
  -> MIR MemOp / FastMemRegion
  -> verifier
  -> LLVM/object
```

`MIR -> C` is allowed only as an optional backend artifact for debug, diff, or
bootstrap work. It is not required before the primary LLVM/object producer and
must not become allocator semantic truth.

## Task Order

```text
MIR-FMEM-005:
  MIR -> LLVM/object primary producer.

MIR-FMEM-006:
  Producer-neutral parity against the current python_template_c_bridge.

MIR-FMEM-007:
  Retire python_template_c_bridge after producer-neutral parity is proven.

MIR-FMEM-C-ARTIFACT:
  Optional MIR -> C debug/diff/bootstrap artifact producer.
  Not on the required product path.
```

## Boundaries

```text
mir_to_c_required_before_llvm=0
mir_to_c_semantic_ssot=0
python_template_c_semantic_ssot=0
llvm_object_primary_producer_next=1
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Stop Line

Do not use this realignment to open allocator replacement, provider activation,
runtime self-allocation through `hako_alloc`, or C-only semantics. C may remain
only as a generated artifact from MIR/FastMemory lowering.
