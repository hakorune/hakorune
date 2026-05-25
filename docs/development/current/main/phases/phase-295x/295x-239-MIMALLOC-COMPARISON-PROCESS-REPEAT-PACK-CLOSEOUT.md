---
Status: Landed
Date: 2026-05-25
Scope: close the selected process-repeat evidence pack and stop median-only row growth.
Related:
  - docs/development/current/main/phases/phase-295x/295x-236-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK.md
  - docs/development/current/main/phases/phase-295x/295x-237-MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK.md
  - docs/development/current/main/phases/phase-295x/295x-238-MIMALLOC-COMPARISON-MIXED-SMALL-PROCESS-REPEAT-PACK.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-239 Process-Repeat Pack Closeout

## Blocker

```text
MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT-295X-002
```

## Decision

Close the process-repeat evidence pack for the selected `.hako` mimalloc port
workload families:

```text
representative-reuse-cycle-small-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
```

The evidence is accepted as diagnostic process-repeat evidence only. It keeps
the existing port seams visible, but it is not accepted as a speed winner, RSS
winner, timing parity requirement, body-timing result, provider activation
result, or allocator replacement result.

Further rows that only add more medians under the same runner, schema,
measurement policy, stop-line, and interpretation are parked. Future rows must
change at least one of:

```text
allocator semantic capability
workload family contract
runner/output contract
measurement policy
diagnostic interpretation
phase or carryover boundary
```

## Stop Line

This row does not add new samples, compute speed winners, compute RSS winners,
require timing parity, add body-internal timers, change runtime behavior, make
`empty` the default runtime config, or open provider/DLL/replacement/hook/global
allocator seams.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION-295X-002
```

The next row returns the lane to allocator-facing semantic selection instead of
adding another same-policy median-only benchmark row.
