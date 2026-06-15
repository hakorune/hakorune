---
Status: Active
Date: 2026-06-15
Task: MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001
Scope: Design the next direct runtime boundary probe after body timer scaling.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-704-MIMALLOC-BODY-TIMER-ALIGNMENT-PROBE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001

## Purpose

296x-704 should only select this row if scaled body timing shows Hako timer
resolution is no longer the dominant uncertainty but the body gap remains. This
row is a probe-design row, not an implementation row.

## Candidate Probe Families

```text
generated runtime helper boundary:
  measure env.now / body timer and exact-AOT helper entry overhead separately

host handle / object boundary:
  measure HostHandle lookup / Arc carrier / ObjectHandle seam without changing
  object representation

BoxCallable / RoutePlan boundary:
  measure whether the active front hits dynamic route lookup or already has a
  closed-world route shape
```

## Stop Line

```text
do not implement closed-world direct lowering
do not change runtime object representation
do not change product NyRT default
do not patch tracked source .hako
do not add benchmark/helper-name special cases
```

## Acceptance

```text
source_evidence=296x-704
implementation_started=0
summary=pending
```
