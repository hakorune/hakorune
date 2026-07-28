---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-007 first retirement slice for Python-template C replacement-front bridge.
Related:
  - docs/development/current/main/phases/phase-296x/296x-443-PYTHON-C-BRIDGE-RETIREMENT-GATE.md
  - docs/development/current/main/phases/phase-296x/296x-445-MIR-FASTMEM-PRODUCER-NEUTRAL-PARITY.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
---

# 296x-446 Python Template C Bridge Retirement First Slice

## Decision

Retire the Python-template C replacement front from normal runtime use by
requiring an explicit baseline flag whenever a benchmark tool asks to generate
or LD_PRELOAD that bridge.

```text
--allow-python-template-c-bridge-baseline
```

This keeps the bridge available for intentional diagnostic comparison while
closing accidental runtime dependency and hidden fallback.

## Boundary

Allowed:

```text
explicit diagnostic baseline run
producer-neutral parity comparison
historical report/check fixtures
optional future MIR-to-C artifact generated from MIR MemOps
```

Rejected:

```text
implicit replacement-front Python-template C generation
hidden fallback from MIR/LLVM producer to Python-template C
using Python-template C as allocator semantic SSOT
product activation through Python-template C
```

## Acceptance

```text
replacement-front mode without --allow-python-template-c-bridge-baseline:
  fail-fast before benchmark build/run

replacement-front mode with --allow-python-template-c-bridge-baseline:
  allowed as explicit baseline only
```

## Next

```text
MIR-FMEM-007B:
  inventory remaining Python-template C bridge references and remove or
  quarantine non-baseline runtime entrypoints.
```
