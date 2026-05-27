---
Status: Current
Date: 2026-05-27
Scope: decide whether row 47 diagnostic evidence can enter optimization.
Blocker: HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-47-HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC.md
---

# 296x-48 Hako Mimalloc Post Diagnostic Decision

## Purpose

Decide whether the row 47 diagnostic evidence is strong enough to enter the
first keeper optimization row.

## Required Input

```text
output_contract=hako-mimalloc-owner-narrow-diagnostic-v0
gap_owner=<one primary owner>
diagnostic_kind=<selected diagnostic>
next_optimization_allowed=0|1
winner_claim=0
```

## Decision Rules

```text
if next_optimization_allowed=1 and gap_owner in compiler_lowering,allocator_algorithm:
  select HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001

otherwise:
  select another diagnostic or taxonomy refresh row
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
