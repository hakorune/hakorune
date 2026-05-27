---
Status: Landed
Date: 2026-05-27
Scope: refresh gap taxonomy after measurement hygiene evidence.
Blocker: HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-48-HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION.md
---

# 296x-49 Hako Mimalloc Gap Taxonomy Refresh

## Purpose

Run the taxonomy adapter again over the measurement hygiene evidence instead of
starting optimization from noisy scout evidence.

## Required Input

```text
output_contract=hako-mimalloc-post-diagnostic-decision-v0
decision=refresh_gap_taxonomy_after_hygiene
selected_next_row=HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001
optimization_started=0
winner_claim=0
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.

## Evidence

This row reuses:

```text
tools/allocator/hako_mimalloc_gap_taxonomy_adapter.py
tools/allocator/mimalloc_repeated_measurement_runner.py
```

The guard runs a hygiene measurement:

```text
sample_count=5
warmup_count=1
operation_repeat=128
```

and refreshes taxonomy as:

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
sample_count=5
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001
```

The next row should decide whether refreshed taxonomy is stable enough for the
first keeper optimization or needs another diagnostic.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_gap_taxonomy_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
