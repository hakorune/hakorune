---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: MIMAP-281A modeled allocation-ledger release-candidate diagnostics.
Related:
  - docs/development/current/main/phases/phase-293x/293x-805-MIMAP-280A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-INVENTORY.md
  - docs/development/current/main/phases/phase-293x/293x-806-MIMAP-281A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-DIAGNOSTICS.md
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release-Candidate Diagnostics SSOT

## Decision

MIMAP-281A adds an observer-only diagnostic owner for MIMAP-280A release-candidate
inventory facts.

The diagnostic owner may read:

- release-candidate inventory counters;
- the last accepted release-candidate report;
- reject category counters for missing/rejected ledger, invalid candidate
  token, duplicate candidate token, and closed substrate.

The diagnostic owner must not record new release-candidate rows or mutate the
inventory owner.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box.hako
```

The owner publishes:

- `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnostic`
- `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReport`
- `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields`

`ReportFields` is an owner-local record payload. The returned diagnostic report
remains a box until cross-function record return/pass and backend lowering are
opened by a separate row.

## Validation

MIMAP-281A is a scalar diagnostic row:

```text
validation_profile = scalar-mir
exe = deferred-to-closeout
```

L3/L4 evidence is deferred to the release-candidate closeout pack.

## Stop Lines

- No new release-candidate rows.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation or release.
- No real segment-map mutation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Usize Follow-Up

The release-candidate family still uses `i64` for report values because it mixes
reason codes, sentinels, identifiers, and byte/capacity quantities in one
returned report box.

The next safe exact-`usize` work is a separate field-group row after the
release-candidate closeout:

```text
MIMAP-282A:
  release-candidate closeout pack

HAKO-ALLOC-USIZE-FIELD-GROUP-001:
  select allocator byte/capacity field-group pilot

HAKO-ALLOC-USIZE-FIELD-GROUP-002:
  migrate one owner-local byte/capacity field group only
```

Only non-negative byte/capacity fields are candidates. Reason codes, status
flags, token/id fields, and `-1` sentinel-bearing indexes remain `i64`.
