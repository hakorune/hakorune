# 293x-921 CHECKS-ALLOCATOR-RELEASE-CANDIDATE-GUARD-HELPER-SPLIT Allocator Release Candidate Guard Helper Split

Status: landed
Date: 2026-05-20

## Decision

Split the MIMAP-280A release-candidate guard into a thin wrapper plus a
dedicated helper section library while preserving the same L2 guard behavior.

## Context

The release-candidate guard was still carrying the document checks, forbidden
pattern checks, VM assertions, and MIR JSON assertions inline. That shape was
no longer aligned with the other thin-wrapper guard splits already landed in
the current lane.

This row keeps the same acceptance contract while moving the reusable section
logic into a dedicated helper file.

## Scope

- Keep `tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh`
  as the stable entrypoint.
- Move the section logic into `tools/checks/lib/allocator_release_candidate_sections.sh`.
- Keep the root proof-app manifest include-aware and keep the row file check
  pointed at the included arena-backing manifest.
- Preserve the existing L2-only release-candidate behavior.

## Non-Goals

- Do not change the proof-app command.
- Do not change the validation profile.
- Do not open L3/L4 evidence for MIMAP-280A.
- Do not change the release-candidate owner semantics.
- Do not broaden the unsupported seams.

## Required Evidence

```text
bash -n tools/checks/lib/allocator_release_candidate_sections.sh
bash -n tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
git diff --check
```
