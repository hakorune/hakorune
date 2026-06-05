---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-004 verifier gates for FastMemory MemOp region contracts.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-441 MIR FastMem Verifier Gates

## Decision

Land the first code-side verifier gates for FastMemory MIR MemOps:

```text
FastMemRegion side-table metadata:
  region id must match table index
  contract id must be non-empty
  metadata emitted_memop_count must match executable MemOp count

MemOp instruction:
  region must exist
  kind must be in the V0 MemOpKind allowlist
  dst / operand arity / effect mask must match the MemOpKind contract

No-escape:
  MemOp-produced values may feed other MemOps
  MemOp-produced values must not escape into ordinary MIR consumers
```

## Boundaries

This row does not open JSON, VM, LLVM, C backend, or product replacement
support for MemOps.

```text
lowering_opened=0
json_support_opened=0
vm_support_opened=0
llvm_support_opened=0
c_artifact_support_opened=0
product_activation=0
```

## Next

```text
MIR-FMEM-005:
  MIR -> C backend artifact producer
  C is generated from MIR MemOps and remains an optional artifact producer
```

## Verification

```text
cargo test -q fastmem --lib
cargo check -q
```
