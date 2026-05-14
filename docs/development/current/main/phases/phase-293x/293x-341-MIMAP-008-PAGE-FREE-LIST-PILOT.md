# 293x-341 MIMAP-008 Page / Free-list Pilot

Status: landed.
Decision: accepted.

## Goal

Fix the first Hakorune-side page/free-list executable pilot before moving to
lifecycle integration.

## Landing shape

`MIMAP-008` adopts the existing `HakoAllocPageModel` as the owner instead of adding
a second near-duplicate model.

The landed proof fixes:

- free-list acquire
- oversize acquire rejection
- local-free release
- double-release rejection
- busy reactivation rejection
- retire-on-empty
- reactivation that drains local-free blocks

## Files

- `lang/src/hako_alloc/memory/page_box.hako`
- `apps/mimalloc-page-free-list-pilot-proof/main.hako`
- `apps/mimalloc-page-free-list-pilot-proof/README.md`
- `docs/development/current/main/design/mimalloc-page-free-list-pilot-ssot.md`
- `tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh`

## Guard

```bash
bash tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh
```

## Explicit non-goals

- no OSVM
- no decommit/recommit substrate
- no global allocator activation
- no hook/provider activation
- no `PackedArray` backend dependency

Next selected row: `MIMAP-009 lifecycle integration pilot`.
