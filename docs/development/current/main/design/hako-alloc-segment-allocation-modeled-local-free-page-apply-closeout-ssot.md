---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-117A segment allocation modeled local-free page-apply closeout guard.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-616-MIMAP-117A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-PAGE-APPLY-CLOSEOUT-GUARD.md
---

# Hako Alloc Segment Allocation Modeled Local-Free Page-Apply Closeout

## Decision

`MIMAP-117A` is a guard-only closeout for the page-model local-free apply seam.

It locks the owner/proof/guard wiring and inactive stop lines before selecting
the next segment allocation row.

## Closed Rows

| Row | Status | Scope |
| --- | --- | --- |
| `MIMAP-107A` | landed | released-span ledger from release span facts |
| `MIMAP-109A` | landed | local-free candidate ledger from released-span reports |
| `MIMAP-111A` | landed | local-free apply-plan ledger from candidate reports |
| `MIMAP-113A` | landed | scalar local-free lane closeout |
| `MIMAP-115A` | landed | explicit page-model apply through `releaseLocal` |

## Required Stop Lines

The closeout must keep these seams inactive:

```text
direct page array mutation
raw pointer residence
segment-map pointer membership or lookup
arena backing allocation
atomic bitmap claim/unclaim
page-source call
OSVM execution, unreserve, or release
real thread scheduling
worker spawning
source-level concurrency semantics
provider activation
hooks
host allocator replacement
backend app/name matcher
```

## Guard

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_apply_closeout_guard.sh
```

The public guard entrypoint is manifest-backed through
`tools/checks/guard_rows.toml` and delegates to its implementation under
`tools/checks/impl/`.

## Next Row

`MIMAP-117A` selects:

```text
MIMAP-118A post-local-free-page-apply-closeout row selection
```
