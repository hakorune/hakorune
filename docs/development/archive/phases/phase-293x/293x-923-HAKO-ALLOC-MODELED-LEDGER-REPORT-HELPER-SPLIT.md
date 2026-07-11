# 293x-923 HAKO-ALLOC MODELED LEDGER REPORT CAPSULE SPLIT Modeled Ledger Report Capsule Split

Status: landed
Date: 2026-05-20

## Decision

Split the modeled ledger report and reject capsules out of
`segment_allocation_modeled_ledger_box.hako` into a dedicated helper module
while keeping the direct report/reject methods on the owner box.

## Context

`segment_allocation_modeled_ledger_box.hako` owns the modeled consume and
release state for the segment allocation ledger. Its report/reject capsules had
grown into a distinct seam, so the capsule data was split into a dedicated
helper module while the owner retained the direct report/reject methods.

This row keeps the ledger state in the main box and moves report capsule
construction plus reject aggregation into a dedicated helper box.

## Scope

- Keep `segment_allocation_modeled_ledger_box.hako` as the stable ledger owner.
- Move report/reject capsule data into
  `segment_allocation_modeled_ledger_report_box.hako`.
- Update the hako module export, memory README, and closeout guards to point to
  the helper.
- Keep the same modeled consume and release routes by building the capsules in
  the owner box.

## Non-Goals

- Do not change the modeled ledger acceptance contract.
- Do not open real segment allocation/free, raw pointer residence, arena
  backing, segment-map mutation, atomics, OSVM/page-source calls, worker
  scheduling, provider activation, host allocator replacement, hooks, or
  backend matcher behavior.
- Do not change the current blocker token.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_span_facts_guard.sh
bash tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_ledger_closeout_guard.sh
bash tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_closeout_guard.sh
bash tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
