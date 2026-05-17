---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-113A segment allocation modeled local-free scalar lane closeout guard.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-612-MIMAP-113A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-SCALAR-LANE-CLOSEOUT-GUARD.md
---

# Hako Alloc Segment Allocation Modeled Local-Free Scalar Lane Closeout

## Decision

`MIMAP-113A` is a guard-only closeout for the scalar local-free lane.

It locks the owner/proof/guard wiring and inactive stop lines before selecting
the next segment allocation row.

## Closed Rows

| Row | Status | Scope |
| --- | --- | --- |
| `MIMAP-107A` | landed | released-span ledger from release span facts |
| `MIMAP-109A` | landed | local-free candidate ledger from released-span reports |
| `MIMAP-111A` | landed | local-free apply-plan ledger from candidate reports |

## Required Stop Lines

The closeout must keep these seams inactive:

```text
real segment allocation/free execution
free-list mutation
page-state mutation
arena backing allocation
raw pointer residence
segment-map pointer membership or lookup
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
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_scalar_lane_closeout_guard.sh
```

The public guard entrypoint is manifest-backed through
`tools/checks/guard_rows.toml` and delegates to its implementation under
`tools/checks/impl/`.

## Next Row

`MIMAP-113A` selects:

```text
MIMAP-114A post-local-free-scalar-closeout row selection
```
