---
Status: Landed
Date: 2026-05-27
Scope: decide the next action from refreshed gap taxonomy evidence.
Blocker: HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-49-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH.md
---

# 296x-50 Hako Mimalloc Refreshed Taxonomy Decision

## Purpose

Decide whether refreshed taxonomy can enter a first keeper optimization, or
whether the measurement/owner evidence still requires another diagnostic.

## Required Input

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
sample_count=5
gap_owner=<one primary owner>
evidence_quality=stable|noisy
gap_confidence=low|medium|high
next_diagnostic
winner_claim=0
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.

## Evidence

Implemented:

```text
tools/allocator/hako_mimalloc_refreshed_taxonomy_decision.py
```

The decision report emits:

```text
output_contract=hako-mimalloc-refreshed-taxonomy-decision-v0
decision=enter_first_keeper_optimization|continue_owner_diagnostic
selected_next_row=<row token>
next_optimization_allowed=0|1
optimization_started=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Only stable, non-low-confidence `compiler_lowering` or `allocator_algorithm`
taxonomy may select the first keeper optimization row. Low-confidence taxonomy
must select owner confidence refresh instead.

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-OWNER-CONFIDENCE-REFRESH-296X-001
```

The actual refreshed taxonomy was stable but low-confidence
`hako_runtime_baseline`, so optimization remains closed.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_refreshed_taxonomy_decision_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
