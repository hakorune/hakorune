# 293x-887 MIMAP-284A Segment Arena Backing Modeled Allocation-Ledger Release Intent Inventory

Status: landed
Date: 2026-05-20

## Decision

Add a scalar/model inventory owner that consumes an accepted modeled
allocation-ledger release-candidate report and records a modeled release-intent
entry.

The release intent is a model fact only. It is the next boundary after the
release-candidate closeout and before any real arena backing release,
pointer-derived lookup, segment-map mutation, atomic bitmap execution, OSVM,
provider activation, host allocator replacement, hooks, or `#[global_allocator]`.

## Scope

- Add one owner:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentInventory
```

- Add one returned report box and one owner-local `ReportFields` record payload.
- Consume:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport
```

- Record a modeled release-intent token and mirror the source allocator model
  fields needed by later release/recycle rows.
- Keep validation L2 daily unless a new backend route shape appears.

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport
  accepted == 1
  release_candidate_present == 1
  modeled_release_candidate_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
release_intent_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled release intent accepted |
| 1 | release-candidate report missing |
| 2 | release-candidate report rejected |
| 3 | invalid release intent token |
| 4 | duplicate release intent token |
| 5 | closed substrate requirement present |

## Stop Lines

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
- No backend `.inc` matcher by app, box, owner, or row name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_intent_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- The release-intent owner records accepted candidate facts into scalar/model
  release-intent state.
- Missing, rejected, invalid-token, duplicate-token, and closed-substrate
  rejects are counted and visible in the report/counters.
- The proof app validates accepted and reject paths.
- The MIR check confirms the `ReportFields` record stays builder-local and does
  not become a runtime record object.
- L3 EXE remains deferred to a future closeout pack unless this row opens a new
  backend route shape.

## Landed Scope

- Added
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentInventory`
  as the scalar/model release-intent owner.
- Added
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReportFields`
  as the owner-local record payload for report construction.
- Added the MIMAP-284A proof app, proof manifest row, module export, memory
  README entry, design SSOT, and guard index entry.
- Kept L3/L4 EXE evidence deferred to the release-intent closeout pack.

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_intent_guard.sh --level L2
```

## Selected Next Row

`MIMAP-285A`:

```text
segment arena backing modeled allocation-ledger release intent diagnostics
```

The next row should observe MIMAP-284A counters and last-intent facts without
recording new release-intent rows or opening real allocator execution.
