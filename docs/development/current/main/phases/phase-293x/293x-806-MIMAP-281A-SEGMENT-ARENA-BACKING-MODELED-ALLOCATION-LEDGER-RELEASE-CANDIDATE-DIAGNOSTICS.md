# 293x-806 MIMAP-281A Segment Arena Backing Modeled Allocation-Ledger Release Candidate Diagnostics

Status: queued next
Date: 2026-05-19

## Decision

Observe MIMAP-280A release-candidate inventory counters and last-candidate
facts without recording new release-candidate rows or opening real allocator
execution.

This row resumes after HAKO-ALLOC-REPORT-RECORD-005 closes the
release-candidate `ReportFields` sidecar.

## Context

MIMAP-280A records a model-only release candidate from accepted modeled
allocation-ledger facts. The next row should expose scalar diagnostic summary
facts so the family can be closed out before any real arena backing release,
segment-map mutation, or raw pointer residence opens.

## Scope

- Add a scalar diagnostic owner for MIMAP-280A release-candidate facts.
- Publish inventory / accepted / reject counters.
- Publish missing/rejected ledger, invalid release-candidate token, duplicate
  release-candidate token, and closed-substrate reject category facts.
- Publish last reason, last segment, last arena, and last release-candidate
  token.
- Keep this row L2 daily unless it introduces a new backend route shape.

## Stop Lines

- No new release-candidate rows.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation or release.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
