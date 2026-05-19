# Hako Alloc Numeric Field Inventory

Status: SSOT
Date: 2026-05-19
Scope: stored numeric fields under `lang/src/hako_alloc/memory/`.
Related:
- `docs/development/current/main/phases/phase-294x/294x-16-HAKO-ALLOC-NUMERIC-FIELD-INVENTORY.md`
- `docs/development/current/main/design/usize-semantic-foundation-ssot.md`

## Decision

Production `hako_alloc` numeric stored fields migrate to exact `usize` only by
documented non-negative field group.

Current production `usize` field group:

- `allocator_facade_box.hako` / `HakoAllocProductionFacade` event counters:
  `alloc_count`, `free_count`, `reject_count`.
- `segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport`
  byte/capacity report fields:
  `source_capacity`, `source_committed_bytes`, `source_uncommitted_bytes`,
  `padded_bytes`, `slot_capacity`, `planned_backing_bytes`,
  `planned_committed_bytes`, `applied_backing_bytes`,
  `applied_committed_bytes`, `remaining_source_bytes`.
  This group was selected by `HAKO-ALLOC-USIZE-FIELD-GROUP-001` and migrated by
  `HAKO-ALLOC-USIZE-FIELD-GROUP-002`.
- `segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReport`
  observer mirror byte fields:
  `last_report_applied_backing_bytes`,
  `last_report_applied_committed_bytes`,
  `last_report_remaining_source_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-004`
  because it only mirrors already-migrated release-candidate byte facts.
  Diagnostic counters, reasons, tokens, ids, and sentinels stay `i64`.
- `segment_arena_backing_modeled_allocation_ledger_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationLedgerReport`
  byte/capacity report fields:
  `source_capacity`, `source_committed_bytes`, `source_uncommitted_bytes`,
  `padded_bytes`, `slot_capacity`, `planned_backing_bytes`,
  `planned_committed_bytes`, `applied_backing_bytes`,
  `applied_committed_bytes`, `remaining_source_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-006`
  because it is the owner-local allocation-ledger report group that feeds the
  already-migrated release-candidate family. Counters, reasons, tokens, ids,
  and sentinels stay `i64`.
- `segment_arena_backing_modeled_allocation_ledger_diagnostic_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnosticReport`
  observer mirror byte fields:
  `last_report_applied_backing_bytes`,
  `last_report_applied_committed_bytes`,
  `last_report_remaining_source_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-008`
  because it only mirrors already-migrated allocation-ledger byte facts.
  Diagnostic counters, reasons, tokens, ids, and sentinels stay `i64`.
- `segment_arena_backing_modeled_allocation_apply_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationApplyReport`
  byte/capacity report fields:
  `source_capacity`, `source_committed_bytes`, `source_uncommitted_bytes`,
  `padded_bytes`, `slot_capacity`, `planned_backing_bytes`,
  `planned_committed_bytes`, `applied_backing_bytes`,
  `applied_committed_bytes`, `remaining_source_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-010`
  because it is the owner-local allocation-apply report group that feeds the
  already-migrated allocation-ledger family. Counters, reasons, tokens, ids,
  and sentinels stay `i64`.
- `segment_arena_backing_modeled_allocation_apply_diagnostic_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReport`
  observer mirror byte fields:
  `last_report_applied_backing_bytes`,
  `last_report_applied_committed_bytes`,
  `last_report_remaining_source_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-012`
  because it only mirrors already-migrated allocation-apply byte facts.
  Diagnostic counters, reasons, tokens, ids, and sentinels stay `i64`.

- `segment_arena_backing_modeled_allocation_plan_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationPlanReport`
  byte/capacity report fields:
  `source_capacity`, `source_committed_bytes`, `source_uncommitted_bytes`,
  `padded_bytes`, `slot_capacity`, `planned_backing_bytes`,
  `planned_committed_bytes`, `remaining_source_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-014`
  because it is the owner-local allocation-plan report group that feeds the
  already-migrated allocation-apply family. Counters, reasons, tokens, ids, and
  sentinels stay `i64`.

- `segment_arena_backing_modeled_allocation_plan_diagnostic_box.hako`
  / `HakoAllocSegmentArenaBackingModeledAllocationPlanDiagnosticReport`
  observer mirror byte fields:
  `last_report_planned_backing_bytes`,
  `last_report_planned_committed_bytes`,
  `last_report_remaining_source_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-016`
  because it only mirrors already-migrated allocation-plan byte facts.
  Diagnostic counters, reasons, tokens, ids, and sentinels stay `i64`.

- `segment_arena_backing_modeled_source_accounting_box.hako`
  / `HakoAllocSegmentArenaBackingModeledSourceAccountingReport`
  byte/capacity report fields:
  `source_capacity`, `source_committed_bytes`, `source_uncommitted_bytes`,
  `slot_capacity`, `padded_bytes`, `accounted_padded_bytes`,
  `available_after_padded_bytes`.
  This group was selected and migrated by `HAKO-ALLOC-USIZE-FIELD-GROUP-018`
  because it is the owner-local source-accounting report group that feeds the
  already-migrated allocation-plan family. Counters, reasons, tokens, ids, and
  sentinels stay `i64`.

- `segment_arena_backing_modeled_source_accounting_diagnostic_box.hako`
  / `HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReport`
  observer mirror byte fields:
  `last_report_source_capacity`,
  `last_report_source_committed_bytes`,
  `last_report_source_uncommitted_bytes`,
  `last_report_accounted_padded_bytes`,
  `last_report_available_after_padded_bytes`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-020` selects this group because it only mirrors
  already-migrated source-accounting byte facts. Diagnostic counters, reasons,
  tokens, ids, and sentinels stay `i64`.

- `segment_arena_backing_modeled_source_bridge_box.hako`
  / `HakoAllocSegmentArenaBackingModeledSourceBridgeReport` byte/capacity
  report fields:
  `source_capacity`, `source_committed_bytes`, `requested_bytes`,
  `padded_bytes`, `slot_capacity`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-022` selects this group because it is the
  owner-local source-bridge report group that feeds the already-migrated
  source-accounting family. Counters, reasons, tokens, ids, alignments, and
  sentinels stay `i64`.

- `segment_arena_backing_modeled_source_bridge_diagnostic_box.hako`
  / `HakoAllocSegmentArenaBackingModeledSourceBridgeDiagnosticReport` observer
  mirror byte fields:
  `last_report_source_capacity`, `last_report_source_committed_bytes`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-024` selects this group because it only mirrors
  already-migrated source-bridge byte facts. Diagnostic counters, reasons,
  tokens, ids, alignments, and sentinels stay `i64`.

- `segment_arena_backing_modeled_arena_slot_box.hako`
  / `HakoAllocSegmentArenaBackingModeledArenaSlotReport` byte/capacity report
  fields:
  `requested_bytes`, `padded_bytes`, `slot_capacity`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-026` selects this group because it is the
  owner-local arena-slot report group that feeds the already-migrated
  source-bridge family. Counters, reasons, tokens, ids, alignments, geometry,
  and sentinels stay `i64`.

- `segment_arena_backing_modeled_residence_arena_binding_box.hako`
  / `HakoAllocSegmentArenaBackingModeledResidenceArenaBindingReport` geometry
  count / page-size report fields:
  `slice_count`, `committed_slices`, `free_slices`, `page_size`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-028` selects and migrates this group because it
  is the owner-local non-negative geometry group that feeds the
  already-migrated arena-slot family. This is intentionally not a byte/capacity
  row.
  Counters, reasons, tokens, ids, alignments, `row_index`, and sentinels stay
  `i64`.

Selected next production `usize` field group:

- none. `HAKO-ALLOC-USIZE-FIELD-GROUP-029` is a closeout row for the residence
  arena-binding geometry count / page-size group, not a new migration group.

All other live production numeric stored fields remain `i64` until their own
field-group row records the invariant, stop line, and acceptance gate.

Non-stored exact `usize` facades are tracked separately from stored-field
migration. M187 adds `SizeClassBox` `usize` input facades; those do not change
the stored field count and keep sentinel-returning results signed. M188 extends
that non-stored facade pattern to allocation request sizes and alignments.

## Categories

- `signed-sentinel`: uses a negative value such as `-1`; do not migrate until
  the state shape is split.
- `signed-delta`: may intentionally move above and below zero.
- `ptr-id`: modeled pointer/handle integer. Keep as `i64` until pointer-shaped
  API parity and failure-handle contracts are explicit.
- `enum`: small result/status vocabulary. Keep as `i64` until the owning row
  defines a narrower representation.
- `index`: non-negative id / slot / bin index.
- `size`: object or block size.
- `capacity`: count of available storage slots or reserved blocks.
- `count`: event, occupancy, stack-top, or statistic count.
- `byte-length`: accumulated or requested bytes.

## Stored Field Inventory

Current stored numeric field count: 220.

Stored `signed-delta` fields are live only in observer delta fields and remain
`i64`.
Stored `signed-sentinel` fields are live only in observer/result fields and
remain `i64`.

Probe-only exact `usize` stored fields live in `usize_field_probe_box.hako`.
They are intentionally excluded from the production migration inventory below.
C205a allocator metadata `record` declarations are also excluded from the live
stored-field count: they describe identity-free metadata shapes, not runtime
state. C205c/C205d store-owner counters are counted because those boxes own
live scalar storage.

The original 294x-16 detailed baseline is retained below. M185 adds the grouped
post-M184 inventory after the baseline so the current owner map remains
readable without losing field names.

| File | Box | Field | Current Type | Category | Migration Note |
| --- | --- | --- | --- | --- | --- |
| `page_box.hako` | `HakoAllocPageModel` | `page_id` | `i64` | `index` | Candidate after id/index call sites use exact non-negative semantics. |
| `page_box.hako` | `HakoAllocPageModel` | `block_size` | `i64` | `size` | Candidate after exact `usize` backend/storage lowering exists. |
| `page_box.hako` | `HakoAllocPageModel` | `capacity` | `i64` | `capacity` | Structural candidate after stats groups and capacity invariant guard. |
| `page_box.hako` | `HakoAllocPageModel` | `reserved` | `i64` | `capacity` | Candidate with `capacity`; keep invariant `reserved <= capacity`. |
| `page_box.hako` | `HakoAllocPageModel` | `used` | `i64` | `count` | Candidate after dynamic range checks cover decrement paths. |
| `page_box.hako` | `HakoAllocPageModel` | `free_top` | `i64` | `count` | Candidate, but preserve stack-top underflow checks first. |
| `page_box.hako` | `HakoAllocPageModel` | `local_free_top` | `i64` | `count` | Candidate with local-free collection row. |
| `page_box.hako` | `HakoAllocPageModel` | `alloc_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_box.hako` | `HakoAllocPageModel` | `local_free_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_box.hako` | `HakoAllocPageModel` | `reject_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_box.hako` | `HakoAllocPageModel` | `peak_used` | `i64` | `count` | Candidate with `used`. |
| `page_box.hako` | `HakoAllocPageModel` | `requested_bytes` | `i64` | `byte-length` | Candidate after checked add/overflow diagnostics are live for byte sums. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `bin` | `i64` | `index` | Candidate after bin vocabulary is exact non-negative. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `page_count` | `i64` | `count` | Candidate with queue length/capacity rows. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `has_direct_page` | `i64` | `count` | Binary presence state split from the old `-1` direct-page sentinel. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `direct_page_index` | `i64` | `index` | Non-negative after 294x-17; migration candidate after queue index contracts. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `add_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `select_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `direct_hit_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `refresh_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `reject_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocHandle` | `page_id` | `i64` | `index` | Candidate after handle id contracts are exact non-negative. |
| `page_heap_box.hako` | `HakoAllocHandle` | `block_id` | `i64` | `index` | Candidate after block-id sentinel returns are split. |
| `page_heap_box.hako` | `HakoAllocHandle` | `requested_size` | `i64` | `size` | Candidate after requested-size callers use exact non-negative semantics. |
| `page_heap_box.hako` | `HakoAllocPage` | `page_id` | `i64` | `index` | Candidate after page id contracts are exact non-negative. |
| `page_heap_box.hako` | `HakoAllocPage` | `block_size` | `i64` | `size` | Candidate with size-class migration. |
| `page_heap_box.hako` | `HakoAllocPage` | `capacity` | `i64` | `capacity` | Candidate, but this prototype may be superseded by `HakoAllocPageModel`. |
| `page_heap_box.hako` | `HakoAllocPage` | `free_top` | `i64` | `count` | Candidate, preserve underflow checks first. |
| `page_heap_box.hako` | `HakoAllocPage` | `alloc_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocPage` | `free_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocPage` | `reuse_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocPage` | `current_used` | `i64` | `count` | Candidate after decrement paths are guarded. |
| `page_heap_box.hako` | `HakoAllocPage` | `peak_used` | `i64` | `count` | Candidate with `current_used`. |
| `page_heap_box.hako` | `HakoAllocPage` | `requested_bytes` | `i64` | `byte-length` | Candidate after checked add/overflow diagnostics are live for byte sums. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `alloc_count` | `usize` | `count` | Migrated in 294x-19e as facade-local monotonic stats. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `free_count` | `usize` | `count` | Migrated in 294x-19e as facade-local monotonic stats. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `reject_count` | `usize` | `count` | Migrated in 294x-19e as facade-local monotonic stats. |

## M185 Grouped Current Inventory

This table is the post-M184 current production inventory grouped by owner. It
excludes `usize_field_probe_box.hako`.

| File | Box | Stored Numeric Fields | Migration Note |
| --- | --- | --- | --- |
| `alloc_fast_path_heap_box.hako` | `HakoAllocFastPathHandle` | `page_id`, `block_id`, `requested_size` | id/index + size fields; keep `i64` until object-return API parity and sentinel-return seams are split. |
| `alloc_fast_path_heap_box.hako` | `HakoAllocFastPathHeap` | `bin`, `block_size`, `page_capacity`, `next_page_id`, `alloc_count`, `release_count`, `fallback_count`, `page_create_count`, `reject_count` | mixed index/size/capacity/count group; migrate only after owner-local exact numeric gate. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `alloc_count`, `free_count`, `reject_count` | already exact `usize` via 294x-19e. |
| `aligned_small_meta_store_box.hako` | `HakoAllocAlignedSmallMetaStore` | `count` | C205c metadata-store counter; migrate with the aligned-small metadata owner, not with record declarations. |
| `huge_page_meta_store_box.hako` | `HakoAllocHugePageMetaStore` | `count`, `live_count` | C205d metadata-store counters; migrate with the huge-page metadata owner, not with record declarations. |
| `huge_page_model_box.hako` | `HakoAllocHugePageModel` | `huge_count`, `live_count`, `allocate_count`, `release_count`, `release_reject_count`, `zero_reject_count`, `commit_reject_count`, `register_fail_count`, `reject_count`, `next_page_id`, `next_ptr`, `last_result_ptr`, `last_page_id`, `last_requested_size`, `last_committed_size`, `last_failure_kind` | huge counters are candidates; ptr/id/status/size observers stay `i64` until huge handle contract is exact. |
| `huge_release_seam_box.hako` | `HakoAllocHugeReleaseSeam` | `release_count`, `unregister_count`, `lookup_miss_count`, `not_huge_count`, `model_reject_count`, `reject_count`, `last_page_id`, `last_requested_size`, `last_committed_size`, `last_failure_kind` | counters are candidates; `last_page_id = -1` is signed-sentinel and stays `i64`. |
| `huge_threshold_router_box.hako` | `HakoAllocHugeThresholdRouter` | `small_route_count`, `small_success_count`, `small_reject_count`, `huge_route_count`, `huge_reject_count`, `invalid_alignment_count`, `invalid_size_count`, `reject_count`, `last_route_kind`, `last_result_ptr`, `last_padded_size`, `last_good_size`, `last_huge_threshold` | count + enum/ptr/size observers; exact migration waits for M187-M188 request-path rows. |
| `osvm_backed_fast_path_heap_box.hako` | `HakoAllocOsVmPageBacking` | `page_id`, `base`, `bytes` | id/ptr-like base/byte-length group; keep `i64` until OSVM pointer-size contract. |
| `osvm_backed_fast_path_heap_box.hako` | `HakoAllocOsVmBackedHandle` | `page_id`, `block_id`, `requested_size` | id/index + size fields; keep `i64` until object-return API parity. |
| `osvm_backed_fast_path_heap_box.hako` | `HakoAllocOsVmBackedFastPathHeap` | `bin`, `block_size`, `page_capacity`, `next_page_id`, `backing_count`, `alloc_count`, `release_count`, `fallback_count`, `page_create_count`, `reject_count`, `reserve_count`, `commit_count`, `decommit_count`, `source_reject_count` | mixed index/size/capacity/count group; migrate after OSVM request-path exactness. |
| `page_box.hako` | `HakoAllocPageModel` | `page_id`, `block_size`, `capacity`, `reserved`, `used`, `free_top`, `local_free_top`, `alloc_count`, `local_free_count`, `local_free_collect_count`, `local_free_collected_blocks`, `reject_count`, `retired`, `retire_count`, `peak_used`, `requested_bytes` | page-local core group; migrate in smaller owner-local rows because decrement/stack-top invariants matter. |
| `page_heap_box.hako` | `HakoAllocHandle` | `page_id`, `block_id`, `requested_size` | legacy prototype handle; keep `i64` until superseded by current page-map owners or object-return parity. |
| `page_heap_box.hako` | `HakoAllocPage` | `page_id`, `block_size`, `capacity`, `free_top`, `alloc_count`, `free_count`, `reuse_count`, `current_used`, `peak_used`, `requested_bytes` | legacy prototype page; migrate only if this owner remains live after page-model migration. |
| `page_map_aligned_small_path_box.hako` | `HakoAllocPageMapAlignedSmallPath` | `meta_count`, `next_ptr`, `alloc_count`, `invalid_alignment_count`, `oversized_count`, `alloc_fail_count`, `register_fail_count`, `reject_count`, `last_result_ptr`, `last_alignment`, `last_padded_size` | counters are candidates; ptr/result/alignment/size observers wait for M188. |
| `page_map_box.hako` | `HakoAllocPageMapEntry` | `ptr`, `page_id`, `block_id`, `live` | ptr/id/index + binary live flag; keep `i64` until pointer/result API shape is exact. |
| `page_map_box.hako` | `HakoAllocPageMap` | `entry_count`, `live_count`, `register_count`, `lookup_count`, `lookup_miss_count`, `unregister_count`, `reject_count` | page-map counters are low-risk candidates after page-map owner gate. |
| `page_map_realloc_alloc_copy_release_box.hako` | `HakoAllocPageMapReallocAllocCopyReleasePath` | `next_ptr`, `success_count`, `copy_count`, `same_class_reject_count`, `alloc_fail_count`, `lookup_miss_count`, `stale_page_count`, `released_block_count`, `reject_count`, `last_result_ptr`, `last_alloc_page_id`, `last_alloc_block_id` | copy/counters are candidates; `last_alloc_* = -1` sentinels and ptr fields stay `i64`. |
| `page_map_realloc_failure_contract_box.hako` | `HakoAllocPageMapReallocFailureContract` | `success_count`, `same_class_success_count`, `move_success_count`, `zero_reject_count`, `oversized_reject_count`, `alloc_fail_count`, `lookup_miss_count`, `stale_page_count`, `released_block_count`, `unexpected_reject_count`, `reject_count`, `last_result_ptr`, `last_failure_kind`, `last_max_block_size` | failure-matrix counters are candidates; ptr/status/size observers wait for API parity. |
| `page_map_realloc_same_class_box.hako` | `HakoAllocPageMapReallocSameClassPath` | `same_class_count`, `grow_reject_count`, `lookup_miss_count`, `stale_page_count`, `released_block_count`, `reject_count`, `last_result_ptr` | counters are candidates; result pointer stays `i64`. |
| `page_map_release_box.hako` | `HakoAllocPageMapReleaseSeam` | `page_count`, `page_register_count`, `release_count`, `unregister_count`, `lookup_miss_count`, `stale_page_count`, `page_release_reject_count`, `reject_count` | counters are candidates after release invariant rows remain green. |
| `page_map_release_invariant_box.hako` | `HakoAllocPageMapReleaseObserver` | `observe_count`, `success_count`, `reject_count`, `live_count_before`, `release_count_before`, `unregister_count_before`, `page_used_before`, `local_free_before`, `last_ptr`, `last_page_id`, `last_block_id`, `last_result`, `last_entry_live_before`, `last_lookup_after`, `last_live_count_delta`, `last_release_count_delta`, `last_unregister_count_delta`, `last_page_used_delta`, `last_local_free_delta` | observer-only; signed sentinel and signed delta fields stay `i64`. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `bin`, `page_count`, `has_direct_page`, `direct_page_index`, `add_count`, `select_count`, `direct_hit_count`, `refresh_count`, `reject_count` | queue counters are candidates; index/flag migration waits for queue contracts. |
| `remote_free_page_integration_box.hako` | `HakoAllocRemoteFreePageInbox` | `head_cell`, `init_status`, `pending_top`, `remote_push_count`, `remote_collect_count`, `retry_count`, `reject_count` | mailbox status/count fields remain `i64` until pointer-atomic lane is exact. |
| `secure_free_list_diagnostics_box.hako` | `HakoAllocSecureFreeListDiagnostics` | `scan_count`, `ok_count`, `fail_count`, `out_of_range_free_block_count`, `duplicate_free_block_count`, `live_block_in_free_list_count`, `free_count_mismatch_count`, `local_free_count_mismatch_count`, `last_ok`, `last_out_of_range_free_block`, `last_duplicate_free_block`, `last_live_block_in_free_list`, `last_free_count_mismatch`, `last_local_free_count_mismatch` | diagnostics counters/flags are candidates, but keep `i64` until secure-list hardening semantics settle. |
| `secure_free_list_policy_box.hako` | `HakoAllocSecureFreeListPolicy` | none | M184 has no stored numeric fields; `-1` and `-2` are non-stored return sentinels. |

## Sentinel Notes

Stored negative sentinel:

- `page_map_release_invariant_box.hako`:
  `last_page_id`, `last_block_id`.
- `page_map_realloc_alloc_copy_release_box.hako`:
  `last_alloc_page_id`, `last_alloc_block_id`.
- `huge_release_seam_box.hako`: `last_page_id`.

Non-stored sentinel seams that must be considered in the next row:

- `HakoAllocPageModel.acquire(...)` returns `-1` on reject.
- `HakoAllocPageQueue.addPage(...)` returns `-1` on reject.
- `HakoAllocPageQueue.directPageId()` returns `-1` when no direct page exists.
- `HakoAllocSecureFreeListPolicy.end_next()` returns `-1`.
- `HakoAllocSecureFreeListPolicy.invalid_next()` returns `-2`.

## Migration Order

1. Keep `signed-sentinel` fields as `i64` or split them first.
2. Migrate low-risk stats `count` fields by owner-local group.
3. Probe `capacity` / stack-top fields with underflow checks.
4. Probe `size` and `byte-length` fields only after checked arithmetic
   diagnostics are stable enough for allocator byte sums.
5. Probe `index` fields after sentinel returns and not-found states are
   explicit.
