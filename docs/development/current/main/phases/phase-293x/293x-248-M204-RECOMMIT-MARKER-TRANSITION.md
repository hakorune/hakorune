# 293x-248 M204 Recommit Marker Transition

Status: Complete

## Purpose

M204 lets a successful M202/M203 recommit report transition the M198 decommit
state marker so M200 can classify the page as reusable again.

The transition is generation-counted:

```text
isMarked(page_id) == countMarkedPage(page_id) > countRecommittedPage(page_id)
```

This avoids physical removal from `marked_page_ids` and keeps repeated
decommit/recommit cycles representable without adding a map or mutating heap
page state.

## Decision

Decision: accepted.

Extend:

```text
lang/src/hako_alloc/memory/purge_decommit_state_marker_box.hako
```

Add:

- `recommitted_page_ids`
- `countMarkedPage(page_id)`
- `countRecommittedPage(page_id)`
- `markIfRecommitted(page_id, report)`

`markIfRecommitted(...)` accepts only reports with `recommit_executed != 0` and
rejects widened reports that also claim marker clearing, unreserve, or OS
release execution.

## Stop Lines

- Do not physically remove entries from `marked_page_ids`.
- Do not mutate heap/page/backing state.
- Do not call page-source APIs from the marker owner.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not change allocation, release, realloc, aligned, huge, or purge
  decommit/recommit adapter behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- A page marked decommitted is blocked by M200 before recommit transition.
- A successful recommit report records one recommit transition.
- M200 sees the page as reusable after the transition.
- Duplicate recommit transition is rejected.
- A later decommit mark can open a new marked generation.
- Pure-first EXE proof output matches the transition matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_recommit_marker_transition_guard.sh
```
