---
Status: Done
Date: 2026-06-06
Scope: docs-only identity boundary for hako_alloc, mimalloc port, and replacement-front producer transition.
Related:
  - docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-440 Hako Alloc Mimalloc Identity Boundary

## Decision

Document the identity boundary before continuing MIR-FMEM implementation:

```text
hako_alloc:
  .hako body/source truth of the mimalloc port
  not a separate allocator family

replacement-front C shim:
  temporary execution bridge for the same mimalloc port
  allowed only while product activation is closed

Python-template C:
  current bridge producer
  not semantic SSOT
  retirement required after MIR producer parity

runtime/bootstrap allocator:
  allocator used by Hakorune compiler/runtime/tooling itself
  separate from hako_alloc product/application allocation
```

## Task Impact

This card does not change the active implementation blocker.

```text
current_blocker_token=MIR-FMEM-004
```

Continue with:

```text
MIR-FMEM-004:
  verifier gates for fastmem escape/layout/safepoint/allocation/ABI boundaries

MIR-FMEM-005:
  MIR -> C backend artifact producer

MIR-FMEM-006:
  MIR -> LLVM/object primary producer

MIR-FMEM-007:
  retire python_template_c_bridge after producer-neutral parity is proven
```

## Stop Line

This docs card does not open:

```text
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
runtime self-allocation through hako_alloc
```

## Files

```text
docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md
docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
docs/development/current/main/workstreams/mimalloc-current.md
docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
```
