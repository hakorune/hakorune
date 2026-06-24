---
Status: Landed
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

## Evidence

Implemented:

```text
tools/allocator/hako_mimalloc_post_diagnostic_decision.py
```

The decision report emits:

```text
output_contract=hako-mimalloc-post-diagnostic-decision-v0
decision=refresh_gap_taxonomy_after_hygiene
selected_next_row=HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001
optimization_started=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Because row 47 selected `measurement_hygiene_refresh` and
`next_optimization_allowed=0`, this row must not enter keeper optimization yet.

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001
```

The next row should run taxonomy again over the hygiene measurement evidence.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_diagnostic_decision_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
