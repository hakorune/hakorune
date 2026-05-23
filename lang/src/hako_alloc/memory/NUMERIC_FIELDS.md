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

- `segment_arena_backing_requirement_matrix_box.hako`
  / `HakoAllocSegmentArenaBackingRequirementMatrixReport` geometry count /
  page-size report fields:
  `slice_count`, `committed_slices`, `free_slices`, `page_size`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-030` selects and migrates this group because it
  is the owner-local non-negative geometry group that feeds the
  already-migrated residence arena-binding family. This is intentionally not a
  byte/capacity row. Counters, reasons, ids, alignments, requirement flags, and
  blocker counts stay `i64`.

- `segment_arena_backing_readiness_inventory_box.hako`
  / `HakoAllocSegmentArenaBackingReadinessInventoryReport` geometry count /
  page-size report fields:
  `slice_count`, `committed_slices`, `free_slices`, `page_size`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-032` selects and migrates this group because it
  is the owner-local non-negative geometry group that feeds the
  already-migrated requirement-matrix family. This is intentionally not a
  byte/capacity row. Counters, reasons, ids, alignments, flags, and sentinels
  stay `i64`.

- `segment_map_accepted_readiness_modeled_consume_ledger_box.hako`
  / `HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport`
  block/count report fields:
  `old_page_used`, `page_capacity`, `request_blocks`, `new_page_used`,
  `remaining_blocks`, `ledger_count_after`, `ledger_live_count_after`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-035` selects and migrates this group because it
  owns non-negative modeled consume/ledger block counts downstream of guarded
  readiness. Reasons, diagnostic kinds, ids, indexes, tokens, block-start
  sentinels, and owner counters stay `i64`.

- `segment_map_accepted_readiness_modeled_consume_ledger_box.hako`
  / `HakoAllocSegmentMapModeledConsumeLedgerReleaseReport` release-side
  block/count report fields:
  `live_before`, `live_after`, `ledger_count_after`,
  `ledger_live_count_after`, `released_blocks`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-037` selects and migrates this group because it
  owns non-negative modeled consume-ledger release counts after the
  accepted-readiness consume-ledger block/count family was closed out. Reasons,
  ids, indexes, tokens, block-start/end sentinels, and owner counters stay
  `i64`.

- `segment_allocation_modeled_local_free_reuse_ledger_box.hako`
  / `HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReport` reuse/page
  count report fields:
  `page_used_before_reuse`, `page_used_after_reuse`,
  `page_local_free_before_reuse`, `page_local_free_after_reuse`,
  `collect_count_after_reuse`, `ledger_count_after`,
  `ledger_live_count_after`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-040` selects and migrates this group because it
  owns non-negative modeled local-free reuse/page/ledger counts downstream of the
  consume-ledger release chain. Reasons, indexes, tokens, segment/page ids,
  reused block ids, flags, and owner counters stay `i64`.

- `segment_allocation_modeled_local_free_reuse_ledger_box.hako`
  / `HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport`
  release-apply count report fields:
  `release_apply_count_after`, `release_apply_reject_count_after`,
  `ledger_live_count_after`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-043` selects and migrates this group because it owns
  non-negative modeled local-free reuse release-apply/ledger counts downstream
  of the already-migrated local-free reuse ledger count family. Reasons,
  indexes, tokens, segment/page ids, reused block ids, flags, and owner
  counters stay `i64`.

- `segment_allocation_modeled_local_free_reuse_ledger_box.hako`
  / `HakoAllocSegmentAllocationModeledLocalFreeReuseLedger` release-apply
  primary counter fields:
  `release_apply_attempt_count`, `release_apply_count`,
  `release_apply_reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-045` selects and migrates this group because these
  non-negative owner-local counters feed the already-migrated release-apply
  report count fields. Per-reason counters, reasons, indexes, tokens,
  segment/page ids, reused block ids, flags, and sentinels stay `i64`.

- `segment_allocation_modeled_local_free_reuse_ledger_box.hako`
  / `HakoAllocSegmentAllocationModeledLocalFreeReuseLedger` release-apply
  shape/lookup reject counter fields:
  `release_apply_upstream_reject_count`,
  `release_apply_invalid_shape_reject_count`,
  `release_apply_duplicate_reject_count`,
  `release_apply_missing_reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-047` selects and migrates this group because these
  non-negative per-reason counters are the first narrow reject-counter group
  after the primary release-apply counters were closed out. Execution/capability
  reject counters, reasons, indexes, tokens, segment/page ids, reused block ids,
  flags, and sentinels stay `i64`.

Selected next production `usize` field group:

- `segment_allocation_modeled_local_free_reuse_ledger_box.hako`
  / `HakoAllocSegmentAllocationModeledLocalFreeReuseLedger` release-apply
  execution/capability reject counter fields:
  `release_apply_execution_reject_count`,
  `release_apply_raw_pointer_reject_count`,
  `release_apply_segment_map_reject_count`,
  `release_apply_arena_reject_count`,
  `release_apply_atomic_bitmap_reject_count`,
  `release_apply_osvm_reject_count`,
  `release_apply_thread_reject_count`,
  `release_apply_provider_reject_count`,
  `release_apply_backend_matcher_reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-049` selects and migrates this group because these
  non-negative per-reason counters are the remaining release-apply reject
  counters after the shape/lookup group was closed out. Reasons, indexes,
  tokens, segment/page ids, reused block ids, flags, and sentinels stay `i64`.

- `page_map_box.hako` / `HakoAllocPageMap` owner-local counter fields:
  `entry_count`, `live_count`, `register_count`, `lookup_count`,
  `lookup_miss_count`, `unregister_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-051` selects and migrates this group because
  these are non-negative page-map owner counters with no signed sentinels.
  `HakoAllocPageMapEntry` pointer/id fields and live flag stay `i64`.

- `page_map_release_box.hako` / `HakoAllocPageMapReleaseSeam` release event /
  reject counter fields:
  `page_register_count`, `release_count`, `unregister_count`,
  `lookup_miss_count`, `stale_page_count`, `page_release_reject_count`,
  `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-052` selects and migrates this group because
  these are non-negative release-seam counters downstream of the page-map
  counter group. `page_count` stays `i64` because it is compared with
  signed `page_id` values in this owner.

- `page_map_realloc_same_class_box.hako` /
  `HakoAllocPageMapReallocSameClassPath` same-class/no-move event / reject
  counter fields:
  `same_class_count`, `grow_reject_count`, `lookup_miss_count`,
  `stale_page_count`, `released_block_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-053` selects and migrates this group because
  these are non-negative path-local counters downstream of the page-map and
  release-seam counter groups. `last_result_ptr` stays `i64` because it is a
  pointer-shaped result observer.

- `page_map_realloc_alloc_copy_release_box.hako` /
  `HakoAllocPageMapReallocAllocCopyReleasePath` fallback event / reject
  counter fields:
  `success_count`, `copy_count`, `same_class_reject_count`,
  `alloc_fail_count`, `lookup_miss_count`, `stale_page_count`,
  `released_block_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-054` selects and migrates this group because
  these are non-negative fallback-local counters downstream of the page-map,
  release-seam, and same-class/no-move counter groups. `next_ptr`,
  `last_result_ptr`, and the `last_alloc_* = -1` sentinel fields stay `i64`.

- `page_map_realloc_failure_contract_box.hako` /
  `HakoAllocPageMapReallocFailureContract` failure-matrix event / reject
  counter fields:
  `success_count`, `same_class_success_count`, `move_success_count`,
  `zero_reject_count`, `oversized_reject_count`, `alloc_fail_count`,
  `lookup_miss_count`, `stale_page_count`, `released_block_count`,
  `unexpected_reject_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-055` selects and migrates this group because
  these are non-negative failure-contract counters over already migrated
  M174/M175 path counters. `last_result_ptr`, `last_failure_kind`, and
  `last_max_block_size` stay `i64`.

- `page_map_aligned_small_path_box.hako` /
  `HakoAllocPageMapAlignedSmallPath` aligned-small path event / reject counter
  fields:
  `alloc_count`, `invalid_alignment_count`, `oversized_count`,
  `alloc_fail_count`, `register_fail_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-056` selects and migrates this group because
  these are non-negative M178 path-local counters. `meta_count` stays `i64`
  until the aligned-small metadata store count migrates; pointer, alignment,
  and padded-size observers stay `i64`.

- `huge_threshold_router_box.hako` / `HakoAllocHugeThresholdRouter` route /
  reject counter fields:
  `small_route_count`, `small_success_count`, `small_reject_count`,
  `huge_route_count`, `huge_reject_count`, `invalid_alignment_count`,
  `invalid_size_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-057` selects and migrates this group because
  these are non-negative route-local counters. `last_route_kind`,
  `last_result_ptr`, `last_padded_size`, `last_good_size`, and
  `last_huge_threshold` stay `i64`.

- `page_queue_box.hako` / `HakoAllocPageQueue` stats counter fields:
  `add_count`, `select_count`, `direct_hit_count`, `refresh_count`,
  `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-058` selects and migrates this group because
  these are non-negative queue-local counters. `bin`, `page_count`,
  `has_direct_page`, and `direct_page_index` stay `i64`.

- `page_queue_box.hako` / `HakoAllocPageQueue.page_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-078` selects and migrates this owner-local
  queue length after the direct-page cache presence/index split. `bin`,
  `has_direct_page`, and `direct_page_index` stay `i64`.

- `page_queue_box.hako` / `HakoAllocPageQueue.direct_page_index`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-079` selects and migrates this non-negative
  direct-page cache index after `has_direct_page` became the explicit presence
  flag. `bin` and `has_direct_page` stay `i64`.

- `page_heap_box.hako` / `HakoAllocPage` legacy stats counter fields:
  `alloc_count`, `free_count`, `reuse_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-084` selects and migrates this group because
  these are live monotonic stats counters still exercised by legacy heap
  apps/facade checks, without signed sentinels or identity semantics.
  `page_id`, `block_size`, `capacity`, `free_top`, `current_used`,
  `peak_used`, and `requested_bytes` stay `i64`.

- `page_box.hako` / `HakoAllocPageModel` local page counter fields:
  `alloc_count`, `local_free_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-059` selects and migrates this group because
  these are monotonic, non-negative page-local counters. Identity,
  size/capacity, stack-top, live-count, local-free collection, lifecycle, and
  byte-length fields stay `i64`.

- `page_box.hako` / `HakoAllocPageModel` local-free collection counter fields:
  `local_free_collect_count`, `local_free_collected_blocks`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-060` selects and migrates this group because
  these are monotonic, non-negative page-local collection counters. Stack-top,
  live-count, lifecycle, and byte-length fields stay `i64`.

- `page_box.hako` / `HakoAllocPageModel` lifecycle event/reject counter fields:
  `retire_count`, `decommit_count`, `recommit_count`, `reuse_count`,
  `lifecycle_reject_count`, `reactivate_count`, `reactivate_reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-061` selects and migrates this group because
  these are monotonic, non-negative lifecycle event/reject counters. The
  `retired` / `decommitted` state flags, stack-top/live-count, identity,
  size/capacity, and byte-length fields stay `i64`.

- `page_box.hako` / `HakoAllocPageModel` stack-top and occupancy fields:
  `used`, `free_top`, `local_free_top`, `peak_used`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-073` selects and migrates this group after
  proof-only stack-top decrement/increment and `ArrayBox.get/set` index probes
  landed. Identity, block size, capacity, reserved count, lifecycle flags, and
  byte-length fields stay `i64`.

- `page_box.hako` / `HakoAllocPageModel` capacity fields:
  `capacity`, `reserved`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-075` selects and migrates this group after the
  proof-only capacity-bound row fixed signed-index guards and loop bounds
  against exact `usize` capacity. Identity, block size, lifecycle flags, and
  byte-length fields stay `i64`.

- `page_box.hako` / `HakoAllocPageModel` size/byte fields:
  `block_size`, `requested_bytes`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-077` selects and migrates this group after the
  proof-only request-size/block-size comparison and byte-sum row fixed the
  owner-local request path. Identity and lifecycle state flags stay `i64`.

- `aligned_small_meta_store_box.hako` / `HakoAllocAlignedSmallMetaStore` and
  `page_map_aligned_small_path_box.hako` / `HakoAllocPageMapAlignedSmallPath`
  metadata count fields:
  `count`, `meta_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-062` selects and migrates this group because
  `count` is the non-negative C205c metadata-store owner count and
  `meta_count` is only the M178 path-local mirror of that owner count.
  Pointer, alignment, and padded-size observers stay `i64`.

- `osvm_backed_fast_path_heap_box.hako` /
  `HakoAllocOsVmBackedFastPathHeap.backing_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-085` selects and migrates this owner-local
  backing-array length after the signed `page_id < 0` guard remained the
  explicit id/index seam. `bin`, `next_page_id`, backing `page_id` / `base`,
  and handle page/block ids stay `i64`.

- `page_map_release_box.hako` / `HakoAllocPageMapReleaseSeam.page_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-086` selects this field for
  `HAKO-ALLOC-USIZE-FIELD-GROUP-087` because it is the release seam page-array
  length. `HAKO-ALLOC-USIZE-FIELD-GROUP-087` migrates it to exact `usize` while
  keeping the signed `page_id < 0` guard as the id/index seam before comparing
  against exact `usize` `page_count`.

Selected next production `usize` field group:

- `HAKO-ALLOC-USIZE-FIELD-GROUP-096` selected the
  `HakoAllocObjectLifecyclePageQueue` count/page-count group as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-097`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-097` migrated
  `HakoAllocObjectLifecyclePageQueue.page_count`, `add_count`,
  `request_count`, `select_count`, `reuse_select_count`,
  `active_select_count`, `decommitted_skip_count`, `retired_skip_count`,
  `unavailable_skip_count`, `miss_count`, and `reject_count` to exact
  `usize`, while `last_selected_index`, `last_selected_page_id`,
  `last_selected_kind`, and the `addPage()` `-1` reject seam stay signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-098` selected the facade source-owner monotonic
  alloc/release counters in `object_lifecycle_facade_result_box.hako` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-099`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-099` migrated
  `HakoAllocObjectLifecycleAllocResult.attempt_count`, `success_count`,
  `failure_count`, `reusable_success_count`, `active_success_count` plus
  `HakoAllocObjectLifecycleReleaseResult.success_count` and `failure_count` to
  exact `usize`, while `last_*`, `last_reason`, `last_ok`,
  alignment/realloc observers, and the downstream stats snapshot mirror remain
  signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-100` selected the downstream
  `object_lifecycle_facade_stats_box.hako` mirror owner as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-101`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-101` migrated the seven
  `HakoAllocObjectLifecycleFacadeStatsSnapshot` mirror counts to exact `usize`,
  while `last_*`, `last_reason`, `last_ok`, alignment/realloc observers,
  totals helpers, and unrelated lifecycle observer owners remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-102` selected the owner-local
  `HakoAllocObjectLifecycleFacadePageSourceAttach` counter owner in
  `object_lifecycle_facade_page_source_box.hako` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-103`, while explicitly deferring the attached
  page-source report/status/source observer fields, ids, bytes, and page
  payload mirrors.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-103` migrated
  `HakoAllocObjectLifecycleFacadePageSourceAttach.reserve_count`,
  `commit_count`, `attach_count`, and `reject_count` to exact `usize`, while
  `HakoAllocObjectLifecycleFacadePageSourceAttachReport.status`, `source_*`,
  `added_page_id`, `facade_page_count`, `base`, `bytes`, `block_size`,
  `capacity`, and `reserved` remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-104` selected the owner-local
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback` counter owner in
  `object_lifecycle_facade_page_source_alloc_miss_box.hako` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-105`, while explicitly deferring the signed
  alloc-miss report observer seam and its count mirrors.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-105` migrated
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.fallback_attempt_count`,
  `source_success_count`, `source_failure_count`, `retry_success_count`, and
  `retry_failure_count` to exact `usize`, while the alloc-miss report
  `status`, `initial_*`, `fallback_attempted`, `source_*`, `retry_*`,
  `final_*`, `source_base`, `source_bytes`, `final_page_id`,
  `final_block_id`, and report-mirror counts remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-106` is now the next selection-only
  row. It selected the owner-local `HakoAllocRecommitFailFastEntry`
  classification/report counters (`attempt_count`, `no_recommit_count`,
  `blocked_count`, `missing_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-107`. Keep the recommit report,
  `last_page_id`, closed-execution evidence counters, page-source attach report
  seam, alloc-miss report/count-mirror seam, and unrelated lifecycle / OSVM /
  bin / provider / hook rows separate from this migration.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-107` migrated those four
  `HakoAllocRecommitFailFastEntry` owner-local counters to exact `usize`, while
  recommit report fields, `last_page_id = -1`, and the
  `recommit_execution_count` / `source_execution_count` closed-execution
  evidence counters remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-108` selected the owner-local
  `HakoAllocPageSourceUnreserveAdapter` outcome counters (`call_count`,
  `success_count`, `reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-109`, while
  `last_base`, `last_bytes`, and `last_rc` remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-109` migrated those three
  `HakoAllocPageSourceUnreserveAdapter` owner-local counters to exact `usize`,
  while `last_base`, `last_bytes`, and `last_rc` remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-110` selected the owner-local
  `HakoAllocPageSourceRecommitAdapter` outcome counters (`call_count`,
  `success_count`, `reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-111`, while
  `last_base`, `last_bytes`, and `last_rc` remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-111` migrated those three
  `HakoAllocPageSourceRecommitAdapter` owner-local counters to exact `usize`,
  while `last_base`, `last_bytes`, and `last_rc` remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-112` selected the owner-local
  `HakoAllocPageSourceDecommitAdapter` outcome counters (`call_count`,
  `success_count`, `reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-113`, while
  `last_base`, `last_bytes`, and `last_rc` remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-113` migrated those three
  `HakoAllocPageSourceDecommitAdapter` owner-local counters to exact `usize`,
  while `last_base`, `last_bytes`, and `last_rc` remain signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-114` selected the decommit-side
  `HakoAllocPurgeDecommitStateMarker` counters (`attempt_count`,
  `marked_count`, `reject_count`, `duplicate_count`,
  `missing_report_count`, `not_decommitted_count`, and
  `release_field_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-115`, while
  marker arrays, `last_page_id`, report fields, and recommit-side counters
  stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-115` migrated those seven decommit-side
  `HakoAllocPurgeDecommitStateMarker` counters to exact `usize`, while marker
  arrays, `last_page_id`, report fields, and recommit-side counters stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-116` selected the recommit-side
  `HakoAllocPurgeDecommitStateMarker` counters (`recommit_attempt_count`,
  `recommitted_count`, `recommit_reject_count`, `duplicate_recommit_count`,
  `missing_recommit_report_count`, `not_recommitted_count`,
  `recommit_widened_reject_count`, and `unmarked_recommit_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-117`, while marker arrays, `last_page_id`,
  report fields, and page-source / heap execution state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-117` migrated those eight recommit-side
  `HakoAllocPurgeDecommitStateMarker` counters to exact `usize`, while marker
  arrays, `last_page_id`, report fields, and page-source / heap execution
  state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-118` selected
  `HakoAllocBoundedDecommitPolicy` counters (`attempt_count`, `blocked_count`,
  `decommit_attempt_count`, `decommit_success_count`, `source_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-119`, while `max_decommit_bytes`, report
  fields, fake proof source counters, page-source adapter state, and heap/page
  execution state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-119` migrated those five
  `HakoAllocBoundedDecommitPolicy` owner-local counters to exact `usize`,
  while `max_decommit_bytes`, report fields, fake proof source counters,
  page-source adapter state, and heap/page execution state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-120` selected
  `HakoAllocHeapReusePriorityPolicy` counters (`select_count`,
  `active_pick_count`, `recommitted_pick_count`, `retired_pick_count`,
  `fresh_pick_count`, `decommitted_skip_count`, `missing_skip_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-121`, while decision fields, route/page-id
  sentinels, page lifecycle observer counters, heap/page queues, page-source
  adapters, and heap/page execution state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-121` migrated those seven
  `HakoAllocHeapReusePriorityPolicy` owner-local counters to exact `usize`,
  while decision fields, `last_route`, `last_page_id`, page lifecycle observer
  counters, heap/page queues, page-source adapters, and heap/page execution
  state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-122` selected
  `HakoAllocPageLifecycleInvariantObserver` counters (`observe_count`,
  `missing_count`, `active_count`, `retired_count`, `decommitted_count`,
  `recommitted_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-123`, while lifecycle
  report fields, `last_page_id`, `last_state`, heap/page queues, page-source
  adapters, and heap/page execution state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-123` migrated those six
  `HakoAllocPageLifecycleInvariantObserver` owner-local counters to exact
  `usize`, while lifecycle report fields, `last_page_id`, `last_state`,
  heap/page queues, page-source adapters, and heap/page execution state stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-124` selected
  `HakoAllocAbandonedReclaimInventory` counters (`classify_count`,
  `candidate_count`, `reject_count`, `missing_backing_reject_count`,
  `owner_active_reject_count`, `remote_pending_reject_count`,
  `decommitted_reject_count`, `abandoned_live_count`,
  `abandoned_retired_count`, `purge_forward_candidate_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-125`, while decision fields, `last_page_id`,
  `last_reason`, reclaim execution, atomics, remote-free draining,
  page-source calls, and OSVM execution state stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-125` migrated those ten
  `HakoAllocAbandonedReclaimInventory` owner-local counters to exact `usize`,
  while decision fields, `last_page_id`, `last_reason`, reclaim execution,
  atomics, remote-free draining, page-source calls, and OSVM execution state
  stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-126` selected
  `HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot`
  owner-local counters (`pilot_count`, `accepted_count`, `reject_count`,
  `missing_diagnostic_reject_count`, `rejected_diagnostic_reject_count`,
  `missing_runner_reject_count`, `missing_output_reject_count`,
  `missing_memory_evidence_reject_count`,
  `missing_output_contract_reject_count`, `failed_runner_reject_count`,
  `invalid_run_count_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-127`,
  while runner payload records, report fields, `last_reason`, stop-line flags,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-127` migrated those eleven
  `HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot`
  owner-local counters to exact `usize`, while runner payload records, report
  fields, `last_reason`, stop-line flags, provider / hook / global-allocator
  rows, worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-128` selected
  `HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic`
  owner-local counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_diagnostic_blocked_count`, `rejected_diagnostic_blocked_count`,
  `missing_runner_blocked_count`, `missing_output_blocked_count`,
  `missing_memory_evidence_blocked_count`,
  `missing_output_contract_blocked_count`, `failed_runner_blocked_count`,
  `invalid_run_count_blocked_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-129`,
  while runner payloads, report fields, `last_reason`, stop-line flags,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-129` migrated those eleven
  `HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic`
  owner-local counters to exact `usize`, while runner payloads, report fields,
  `last_reason`, stop-line flags, provider / hook / global-allocator rows,
  worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-130` selected
  `HakoAllocAllocatorComparisonCMimallocResultLedger` owner-local counters
  (`ledger_count`, `accepted_count`, `reject_count`,
  `missing_hako_diagnostic_reject_count`,
  `blocked_hako_diagnostic_reject_count`,
  `missing_c_diagnostic_reject_count`, `blocked_c_diagnostic_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-131`, while comparison payloads, signed deltas,
  report fields, `last_reason`, conclusion flags, repeated benchmark
  execution, provider / hook / global-allocator rows, worker/TLS, and threads
  stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-131` migrated those seven
  `HakoAllocAllocatorComparisonCMimallocResultLedger` owner-local counters to
  exact `usize`, while comparison payloads, signed deltas, report fields,
  `last_reason`, conclusion flags, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-132` selected
  `HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic` owner-local
  counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_hako_blocked_count`, `blocked_hako_blocked_count`,
  `missing_c_blocked_count`, `blocked_c_blocked_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-133`, while comparison payloads, signed deltas,
  report fields, `last_reason`, conclusion flags, repeated benchmark
  execution, provider / hook / global-allocator rows, worker/TLS, and threads
  stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-133` migrated those seven
  `HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic` owner-local
  counters to exact `usize`, while comparison payloads, signed deltas, report
  fields, `last_reason`, conclusion flags, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-134` selected
  `HakoAllocAllocatorComparisonCMimallocExecutionInventory` owner-local
  counters (`inventory_count`, `accepted_count`, `reject_count`,
  `missing_runner_reject_count`, `missing_workload_reject_count`,
  `missing_hako_metrics_reject_count`,
  `missing_output_contract_reject_count`,
  `missing_memory_usage_contract_reject_count`,
  `missing_evidence_storage_reject_count`, `missing_run_count_reject_count`,
  `invalid_run_count_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-135`,
  while run-count payloads, report fields, `last_reason`, C execution
  behavior, provider / hook / global-allocator rows, worker/TLS, and threads
  stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-135` migrated those eleven
  `HakoAllocAllocatorComparisonCMimallocExecutionInventory` owner-local
  counters to exact `usize`, while run-count payloads, report fields,
  `last_reason`, C execution behavior, provider / hook / global-allocator
  rows, worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-136` selected
  `HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic` owner-local
  counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_runner_blocked_count`, `missing_workload_blocked_count`,
  `missing_hako_metrics_blocked_count`,
  `missing_output_contract_blocked_count`,
  `missing_memory_usage_contract_blocked_count`,
  `missing_evidence_storage_blocked_count`, `missing_run_count_blocked_count`,
  `invalid_run_count_blocked_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-137`,
  while run-count payloads, report fields, `last_reason`, C execution
  behavior, provider / hook / global-allocator rows, worker/TLS, and threads
  stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-137` migrated those eleven
  `HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic` owner-local
  counters to exact `usize`, while run-count payloads, report fields,
  `last_reason`, C execution behavior, provider / hook / global-allocator
  rows, worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-138` selected
  `HakoAllocAllocatorComparisonCMimallocResultSummaryInventory` owner-local
  counters (`summary_count`, `ready_count`, `blocked_count`,
  `missing_ledger_reject_count`, `missing_diagnostic_reject_count`,
  `blocked_diagnostic_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-139`,
  while comparison payloads, report fields, `last_reason`,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-139` migrated those six
  `HakoAllocAllocatorComparisonCMimallocResultSummaryInventory` owner-local
  counters to exact `usize`, while comparison payloads, report fields,
  `last_reason`, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-140` selected
  `HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic` owner-local
  counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_summary_blocked_count`, `blocked_summary_blocked_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-141`, while comparison payloads, report fields,
  `last_reason`, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-141` migrated those five
  `HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic` owner-local
  counters to exact `usize`, while comparison payloads, report fields,
  `last_reason`, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-142` selected
  `HakoAllocAllocatorComparisonCMimallocResultReportingInventory` owner-local
  counters (`reporting_count`, `ready_count`, `blocked_count`,
  `missing_summary_diagnostic_reject_count`,
  `blocked_summary_diagnostic_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-143`, while comparison payloads, report fields,
  `last_reason`, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-143` migrated those five
  `HakoAllocAllocatorComparisonCMimallocResultReportingInventory` owner-local
  counters to exact `usize`, while comparison payloads, report fields,
  `last_reason`, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-144` selected
  `HakoAllocAllocatorComparisonCMimallocResultReportingDiagnostic` owner-local
  counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_reporting_blocked_count`, `blocked_reporting_blocked_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-145`, while comparison payloads, report fields,
  `last_reason`, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-145` migrated those five
  `HakoAllocAllocatorComparisonCMimallocResultReportingDiagnostic` owner-local
  counters to exact `usize`, while comparison payloads, report fields,
  `last_reason`, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, and threads stay
  unchanged.

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

Current stored numeric field count: 267.

Stored `signed-delta` fields are live only in observer delta fields and remain
`i64`.
Stored `signed-sentinel` fields are live only in observer/result fields and
remain `i64`.

Probe-only exact `usize` stored fields live in `usize_field_probe_box.hako`.
They are intentionally excluded from the production migration inventory below.
The probe covers capacity, occupancy, byte-length accumulation, and the first
stack-top decrement/increment shape with explicit underflow/overflow rejects.
It also covers exact `usize` stack-top values used as `ArrayBox.get/set`
indexes before production page stack fields migrate.
`294x-44` extends the probe to signed-index guards against exact `usize`
capacity bounds and `loop(i < capacity)` bound checks.
`294x-46` extends the probe to request-size comparison against exact `usize`
block size and accepted-request byte-sum accumulation before production
page-model size/byte fields migrate.
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
| `page_box.hako` | `HakoAllocPageModel` | `block_size` | `usize` | `size` | Exact page block-size via `HAKO-ALLOC-USIZE-FIELD-GROUP-077`. |
| `page_box.hako` | `HakoAllocPageModel` | `capacity` | `usize` | `capacity` | Exact page capacity via `HAKO-ALLOC-USIZE-FIELD-GROUP-075`. |
| `page_box.hako` | `HakoAllocPageModel` | `reserved` | `usize` | `capacity` | Exact reserved block bound via `HAKO-ALLOC-USIZE-FIELD-GROUP-075`. |
| `page_box.hako` | `HakoAllocPageModel` | `used` | `usize` | `count` | Exact page occupancy count via `HAKO-ALLOC-USIZE-FIELD-GROUP-073`. |
| `page_box.hako` | `HakoAllocPageModel` | `free_top` | `usize` | `count` | Exact page free-stack top via `HAKO-ALLOC-USIZE-FIELD-GROUP-073`. |
| `page_box.hako` | `HakoAllocPageModel` | `local_free_top` | `usize` | `count` | Exact page local-free stack top via `HAKO-ALLOC-USIZE-FIELD-GROUP-073`. |
| `page_box.hako` | `HakoAllocPageModel` | `alloc_count` | `usize` | `count` | Exact page-local counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-059`. |
| `page_box.hako` | `HakoAllocPageModel` | `local_free_count` | `usize` | `count` | Exact page-local counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-059`. |
| `page_box.hako` | `HakoAllocPageModel` | `reject_count` | `usize` | `count` | Exact page-local counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-059`. |
| `page_box.hako` | `HakoAllocPageModel` | `peak_used` | `usize` | `count` | Exact page peak occupancy mirror via `HAKO-ALLOC-USIZE-FIELD-GROUP-073`. |
| `page_box.hako` | `HakoAllocPageModel` | `requested_bytes` | `usize` | `byte-length` | Exact page requested-byte sum via `HAKO-ALLOC-USIZE-FIELD-GROUP-077`. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `bin` | `i64` | `index` | Candidate after bin vocabulary is exact non-negative. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `page_count` | `usize` | `count` | Exact queue page count via `HAKO-ALLOC-USIZE-FIELD-GROUP-078`. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `has_direct_page` | `i64` | `count` | Binary presence state split from the old `-1` direct-page sentinel. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `direct_page_index` | `usize` | `index` | Exact direct-page cache index via `HAKO-ALLOC-USIZE-FIELD-GROUP-079`. |
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
| `page_heap_box.hako` | `HakoAllocPage` | `alloc_count` | `usize` | `count` | Exact legacy page stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-084`. |
| `page_heap_box.hako` | `HakoAllocPage` | `free_count` | `usize` | `count` | Exact legacy page stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-084`. |
| `page_heap_box.hako` | `HakoAllocPage` | `reuse_count` | `usize` | `count` | Exact legacy page stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-084`. |
| `page_heap_box.hako` | `HakoAllocPage` | `current_used` | `i64` | `count` | Candidate after decrement paths are guarded. |
| `page_heap_box.hako` | `HakoAllocPage` | `peak_used` | `i64` | `count` | Candidate with `current_used`. |
| `page_heap_box.hako` | `HakoAllocPage` | `requested_bytes` | `i64` | `byte-length` | Candidate after checked add/overflow diagnostics are live for byte sums. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `alloc_count` | `usize` | `count` | Migrated in 294x-19e as facade-local monotonic stats. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `free_count` | `usize` | `count` | Migrated in 294x-19e as facade-local monotonic stats. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `reject_count` | `usize` | `count` | Migrated in 294x-19e as facade-local monotonic stats. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `page_count` | `usize` | `count` | Exact object-lifecycle queue page-count storage via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `add_count` | `usize` | `count` | Exact object-lifecycle queue add counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `request_count` | `usize` | `count` | Exact object-lifecycle queue request counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `select_count` | `usize` | `count` | Exact object-lifecycle queue successful-select counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `reuse_select_count` | `usize` | `count` | Exact object-lifecycle queue reuse-select counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `active_select_count` | `usize` | `count` | Exact object-lifecycle queue active-select counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `decommitted_skip_count` | `usize` | `count` | Exact object-lifecycle queue decommitted-skip counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `retired_skip_count` | `usize` | `count` | Exact object-lifecycle queue retired-skip counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `unavailable_skip_count` | `usize` | `count` | Exact object-lifecycle queue unavailable-skip counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `miss_count` | `usize` | `count` | Exact object-lifecycle queue miss counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `reject_count` | `usize` | `count` | Exact object-lifecycle queue reject counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `last_selected_index` | `i64` | `signed-sentinel` | `-1` marks the no-selection seam; keep signed until the selected-index contract is split from the reject vocabulary. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `last_selected_page_id` | `i64` | `signed-sentinel` | `-1` marks the no-selection seam; selected page-id publication stays signed until page-id seams are split. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `last_selected_kind` | `i64` | `enum` | Small selection-kind vocabulary stays signed until the enum lane gets a dedicated representation row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `last_page_id` | `i64` | `signed-sentinel` | `-1` marks the no-selection seam; alloc page-id publication stays signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `last_block_id` | `i64` | `signed-sentinel` | `-1` marks the no-selection seam; alloc block-id publication stays signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `last_reason` | `i64` | `enum` | Facade alloc failure vocabulary stays signed until its own representation row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `last_ok` | `i64` | `enum` | Facade alloc success flag stays signed until bool/flag storage gets a dedicated row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `attempt_count` | `usize` | `count` | Exact facade alloc attempt counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `success_count` | `usize` | `count` | Exact facade alloc success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `failure_count` | `usize` | `count` | Exact facade alloc failure counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `reusable_success_count` | `usize` | `count` | Exact facade alloc reusable-success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `active_success_count` | `usize` | `count` | Exact facade alloc active-success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReleaseResult` | `last_page_id` | `i64` | `signed-sentinel` | `-1` marks the no-selection seam; release page-id publication stays signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReleaseResult` | `last_block_id` | `i64` | `signed-sentinel` | `-1` marks the no-selection seam; release block-id publication stays signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReleaseResult` | `last_reason` | `i64` | `enum` | Facade release failure vocabulary stays signed until its own representation row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReleaseResult` | `last_ok` | `i64` | `enum` | Facade release success flag stays signed until bool/flag storage gets a dedicated row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReleaseResult` | `success_count` | `usize` | `count` | Exact facade release success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReleaseResult` | `failure_count` | `usize` | `count` | Exact facade release failure counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAlignmentResult` | `last_requested` | `i64` | `signed-sentinel` | `-1` marks unsupported/unset alignment requests; keep signed until alignment observers split from the reject seam. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAlignmentResult` | `last_normalized` | `i64` | `signed-sentinel` | `-1` marks unsupported/unset normalized alignment; keep signed until alignment observers split from the reject seam. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAlignmentResult` | `last_reason` | `i64` | `enum` | Facade alignment reason vocabulary stays signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAlignmentResult` | `last_supported` | `i64` | `enum` | Facade alignment supported flag stays signed until bool/flag storage gets a dedicated row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_page_id` | `i64` | `signed-sentinel` | `-1` marks missing source page-id; realloc page-id observers stay signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_block_id` | `i64` | `signed-sentinel` | `-1` marks missing source block-id; realloc block-id observers stay signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_new_page_id` | `i64` | `signed-sentinel` | `-1` marks missing destination page-id; move-result observers stay signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_new_block_id` | `i64` | `signed-sentinel` | `-1` marks missing destination block-id; move-result observers stay signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_requested_size` | `i64` | `size` | Requested-size observer stays signed until realloc request-size storage gets its own row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_reason` | `i64` | `enum` | Facade realloc reason vocabulary stays signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_ok` | `i64` | `enum` | Facade realloc success flag stays signed until bool/flag storage gets a dedicated row. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `status` | `i64` | `enum` | Page-source attach status vocabulary stays signed while the owner-local counter row remains separate from report observers. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `source_reserved` | `i64` | `count` | Page-source reserve mirror stays signed until the report observer seam gets its own row. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `source_committed` | `i64` | `count` | Page-source commit mirror stays signed until the report observer seam gets its own row. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `added_page_id` | `i64` | `signed-sentinel` | `-1` marks attach failure; added page-id publication stays signed. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `facade_page_count` | `i64` | `count` | Downstream facade page-count mirror stays signed until the page-source report observer seam gets its own row. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `source_reject` | `i64` | `count` | Page-source reject mirror stays signed until the report observer seam gets its own row. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `base` | `i64` | `ptr-id` | Reserved OSVM base payload stays signed until the pointer/id seam gets its own row. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `bytes` | `i64` | `byte-length` | Reserved/committed byte-length observer stays signed until the page-source observer seam gets its own row. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `block_size` | `i64` | `size` | Attached page block-size payload stays signed until page payload migration is explicit. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `capacity` | `i64` | `capacity` | Attached page capacity payload stays signed until page payload migration is explicit. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `reserved` | `i64` | `capacity` | Attached page reserved-block payload stays signed until page payload migration is explicit. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttach` | `reserve_count` | `usize` | `count` | Exact owner-local page-source reserve counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-103`. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttach` | `commit_count` | `usize` | `count` | Exact owner-local page-source commit counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-103`. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttach` | `attach_count` | `usize` | `count` | Exact owner-local page attach counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-103`. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttach` | `reject_count` | `usize` | `count` | Exact owner-local page-source reject counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-103`. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `alloc_attempt_count` | `usize` | `count` | Exact downstream mirror of the facade alloc attempt counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `alloc_success_count` | `usize` | `count` | Exact downstream mirror of the facade alloc success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `alloc_failure_count` | `usize` | `count` | Exact downstream mirror of the facade alloc failure counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `alloc_reusable_success_count` | `usize` | `count` | Exact downstream mirror of the facade alloc reusable-success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `alloc_active_success_count` | `usize` | `count` | Exact downstream mirror of the facade alloc active-success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `release_success_count` | `usize` | `count` | Exact downstream mirror of the facade release success counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `release_failure_count` | `usize` | `count` | Exact downstream mirror of the facade release failure counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`. |

## M185 Grouped Current Inventory

This table is the post-M184 current production inventory grouped by owner. It
excludes `usize_field_probe_box.hako`.

| File | Box | Stored Numeric Fields | Migration Note |
| --- | --- | --- | --- |
| `alloc_fast_path_heap_box.hako` | `HakoAllocFastPathHandle` | `page_id`, `block_id`, `requested_size` | `requested_size` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-081`; page/block id fields stay `i64` until id/index contracts are split. |
| `abandoned_reclaim_inventory_box.hako` | `HakoAllocAbandonedReclaimInventory` | `classify_count`, `candidate_count`, `reject_count`, `missing_backing_reject_count`, `owner_active_reject_count`, `remote_pending_reject_count`, `decommitted_reject_count`, `abandoned_live_count`, `abandoned_retired_count`, `purge_forward_candidate_count`, `last_page_id`, `last_reason` | inventory counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-125`; `last_page_id = -1` and reason vocabulary stay signed. |
| `allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot` | `pilot_count`, `accepted_count`, `reject_count`, `missing_diagnostic_reject_count`, `rejected_diagnostic_reject_count`, `missing_runner_reject_count`, `missing_output_reject_count`, `missing_memory_evidence_reject_count`, `missing_output_contract_reject_count`, `failed_runner_reject_count`, `invalid_run_count_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-127`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box.hako` | `HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic` | `diagnostic_count`, `ready_count`, `blocked_count`, `missing_diagnostic_blocked_count`, `rejected_diagnostic_blocked_count`, `missing_runner_blocked_count`, `missing_output_blocked_count`, `missing_memory_evidence_blocked_count`, `missing_output_contract_blocked_count`, `failed_runner_blocked_count`, `invalid_run_count_blocked_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-129`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_result_ledger_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultLedger` | `ledger_count`, `accepted_count`, `reject_count`, `missing_hako_diagnostic_reject_count`, `blocked_hako_diagnostic_reject_count`, `missing_c_diagnostic_reject_count`, `blocked_c_diagnostic_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-131`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_result_ledger_diagnostic_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic` | `diagnostic_count`, `ready_count`, `blocked_count`, `missing_hako_blocked_count`, `blocked_hako_blocked_count`, `missing_c_blocked_count`, `blocked_c_blocked_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-133`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_result_summary_inventory_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultSummaryInventory` | `summary_count`, `ready_count`, `blocked_count`, `missing_ledger_reject_count`, `missing_diagnostic_reject_count`, `blocked_diagnostic_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-139`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_result_summary_diagnostic_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic` | `diagnostic_count`, `ready_count`, `blocked_count`, `missing_summary_blocked_count`, `blocked_summary_blocked_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-141`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_result_reporting_inventory_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultReportingInventory` | `reporting_count`, `ready_count`, `blocked_count`, `missing_summary_diagnostic_reject_count`, `blocked_summary_diagnostic_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-143`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_result_reporting_diagnostic_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultReportingDiagnostic` | `diagnostic_count`, `ready_count`, `blocked_count`, `missing_reporting_blocked_count`, `blocked_reporting_blocked_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-145`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_result_presentation_only_conclusion_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyConclusionPilot` | `presentation_count`, `accepted_count`, `blocked_count`, `missing_pilot_reject_count`, `blocked_pilot_reject_count`, `missing_presentation_input_reject_count`, `closed_stop_line_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-147`; `last_reason` stays signed reason vocabulary and report mirrors / comparison payloads remain separate. |
| `allocator_comparison_c_mimalloc_result_presentation_follow_on_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationFollowOnPilot` | `follow_on_count`, `accepted_count`, `blocked_count`, `missing_pilot_reject_count`, `blocked_pilot_reject_count`, `missing_follow_on_input_reject_count`, `closed_stop_line_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-149`; `last_reason` stays signed reason vocabulary and report mirrors / comparison payloads remain separate. |
| `allocator_comparison_c_mimalloc_result_presentation_extension_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionPilot` | `extension_count`, `accepted_count`, `blocked_count`, `missing_pilot_reject_count`, `blocked_pilot_reject_count`, `missing_extension_input_reject_count`, `closed_stop_line_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-151`; `last_reason` stays signed reason vocabulary and report mirrors / comparison payloads remain separate. |
| `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilot` | `follow_on_count`, `accepted_count`, `blocked_count`, `missing_pilot_reject_count`, `blocked_pilot_reject_count`, `missing_follow_on_input_reject_count`, `closed_stop_line_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-153`; `last_reason` stays signed reason vocabulary and report mirrors / comparison payloads remain separate. |
| `allocator_comparison_c_mimalloc_execution_inventory_box.hako` | `HakoAllocAllocatorComparisonCMimallocExecutionInventory` | `inventory_count`, `accepted_count`, `reject_count`, `missing_runner_reject_count`, `missing_workload_reject_count`, `missing_hako_metrics_reject_count`, `missing_output_contract_reject_count`, `missing_memory_usage_contract_reject_count`, `missing_evidence_storage_reject_count`, `missing_run_count_reject_count`, `invalid_run_count_reject_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-135`; `last_reason` stays signed reason vocabulary. |
| `allocator_comparison_c_mimalloc_execution_diagnostic_box.hako` | `HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic` | `diagnostic_count`, `ready_count`, `blocked_count`, `missing_runner_blocked_count`, `missing_workload_blocked_count`, `missing_hako_metrics_blocked_count`, `missing_output_contract_blocked_count`, `missing_memory_usage_contract_blocked_count`, `missing_evidence_storage_blocked_count`, `missing_run_count_blocked_count`, `invalid_run_count_blocked_count`, `last_reason` | owner-local counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-137`; `last_reason` stays signed reason vocabulary. |
| `alloc_fast_path_heap_box.hako` | `HakoAllocFastPathHeap` | `bin`, `block_size`, `page_capacity`, `next_page_id`, `alloc_count`, `release_count`, `fallback_count`, `page_create_count`, `reject_count` | event/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-067`; `block_size` and `page_capacity` are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-080`; `next_page_id` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-089`; `bin` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-095`. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `alloc_count`, `free_count`, `reject_count` | already exact `usize` via 294x-19e. |
| `aligned_small_meta_store_box.hako` | `HakoAllocAlignedSmallMetaStore` | `count` | exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-062`; C205c metadata-store counter migrated with the aligned-small metadata owner. |
| `huge_page_meta_store_box.hako` | `HakoAllocHugePageMetaStore` | `count`, `live_count` | exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-063`; C205d metadata-store counters migrated with the huge-page metadata owner, not with record declarations. |
| `huge_page_model_box.hako` | `HakoAllocHugePageModel` | `huge_count`, `live_count`, `allocate_count`, `release_count`, `release_reject_count`, `zero_reject_count`, `commit_reject_count`, `register_fail_count`, `reject_count`, `next_page_id`, `next_ptr`, `last_result_ptr`, `last_page_id`, `last_requested_size`, `last_committed_size`, `last_failure_kind` | `huge_count` and `live_count` are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-064`; event/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-065`; `next_page_id` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-091`; ptr/id/status/size observers stay `i64` until huge handle contract is exact. |
| `huge_release_seam_box.hako` | `HakoAllocHugeReleaseSeam` | `release_count`, `unregister_count`, `lookup_miss_count`, `not_huge_count`, `model_reject_count`, `reject_count`, `last_page_id`, `last_requested_size`, `last_committed_size`, `last_failure_kind` | counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-066`; `last_page_id = -1` is signed-sentinel and stays `i64`. |
| `huge_threshold_router_box.hako` | `HakoAllocHugeThresholdRouter` | `small_route_count`, `small_success_count`, `small_reject_count`, `huge_route_count`, `huge_reject_count`, `invalid_alignment_count`, `invalid_size_count`, `reject_count`, `last_route_kind`, `last_result_ptr`, `last_padded_size`, `last_good_size`, `last_huge_threshold` | route/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-057`; enum/ptr/size observers stay `i64`. |
| `heap_reuse_priority_box.hako` | `HakoAllocHeapReusePriorityPolicy` | `select_count`, `active_pick_count`, `recommitted_pick_count`, `retired_pick_count`, `fresh_pick_count`, `decommitted_skip_count`, `missing_skip_count`, `last_route`, `last_page_id` | pick/skip counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-121`; route vocabulary and `last_page_id = -1` stay signed. |
| `page_lifecycle_invariant_box.hako` | `HakoAllocPageLifecycleInvariantObserver` | `observe_count`, `missing_count`, `active_count`, `retired_count`, `decommitted_count`, `recommitted_count`, `last_page_id`, `last_state` | observer state counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-123`; `last_page_id = -1` and state vocabulary stay signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAllocResult` | `last_page_id`, `last_block_id`, `last_reason`, `last_ok`, `attempt_count`, `success_count`, `failure_count`, `reusable_success_count`, `active_success_count` | alloc attempt/success/failure counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`; page/block id sentinels, reason vocabulary, and ok flag stay signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReleaseResult` | `last_page_id`, `last_block_id`, `last_reason`, `last_ok`, `success_count`, `failure_count` | release success/failure counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-099`; page/block id sentinels, reason vocabulary, and ok flag stay signed. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleAlignmentResult` | `last_requested`, `last_normalized`, `last_reason`, `last_supported` | alignment observers remain signed until the alignment request/normalized seam gets a dedicated row. |
| `object_lifecycle_facade_result_box.hako` | `HakoAllocObjectLifecycleReallocResult` | `last_page_id`, `last_block_id`, `last_new_page_id`, `last_new_block_id`, `last_requested_size`, `last_reason`, `last_ok` | realloc observers remain signed until realloc ids/requested-size and success/failure vocabularies get their own rows. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttachReport` | `status`, `source_reserved`, `source_committed`, `added_page_id`, `facade_page_count`, `source_reject`, `base`, `bytes`, `block_size`, `capacity`, `reserved` | page-source attach report observer stays signed after `HAKO-ALLOC-USIZE-FIELD-GROUP-103`; status/source mirrors, page id, pointer-like base, byte-length, and page payload mirrors remain deferred. |
| `object_lifecycle_facade_page_source_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAttach` | `reserve_count`, `commit_count`, `attach_count`, `reject_count` | exact owner-local counters via `HAKO-ALLOC-USIZE-FIELD-GROUP-103`; report/status/source/page payload observers remain separate. |
| `object_lifecycle_facade_page_source_alloc_miss_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport` | `status`, `initial_ok`, `initial_reason`, `fallback_attempted`, `source_status`, `source_reserved`, `source_committed`, `source_reject`, `source_added_page_id`, `source_facade_page_count`, `source_base`, `source_bytes`, `retry_ok`, `retry_reason`, `final_ok`, `final_reason`, `final_page_id`, `final_block_id`, `fallback_attempt_count`, `source_success_count`, `source_failure_count`, `retry_success_count`, `retry_failure_count` | alloc-miss report observer stays signed after `HAKO-ALLOC-USIZE-FIELD-GROUP-105`; initial/source/retry/final mirrors, page/block ids, pointer-like base, byte-length, and count mirrors remain deferred while the owner/report split stays strict. |
| `object_lifecycle_facade_page_source_alloc_miss_box.hako` | `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback` | `fallback_attempt_count`, `source_success_count`, `source_failure_count`, `retry_success_count`, `retry_failure_count` | exact owner-local alloc-miss fallback counters via `HAKO-ALLOC-USIZE-FIELD-GROUP-105`; signed report/status/source/final mirrors remain separate. |
| `object_lifecycle_facade_stats_box.hako` | `HakoAllocObjectLifecycleFacadeStatsSnapshot` | `alloc_attempt_count`, `alloc_success_count`, `alloc_failure_count`, `alloc_reusable_success_count`, `alloc_active_success_count`, `release_success_count`, `release_failure_count` | exact downstream mirror owner via `HAKO-ALLOC-USIZE-FIELD-GROUP-101`; totals helpers stay derived and alignment/realloc/lifecycle observer seams remain separate. |
| `object_lifecycle_page_queue_box.hako` | `HakoAllocObjectLifecyclePageQueue` | `page_count`, `add_count`, `request_count`, `select_count`, `reuse_select_count`, `active_select_count`, `decommitted_skip_count`, `retired_skip_count`, `unavailable_skip_count`, `miss_count`, `reject_count`, `last_selected_index`, `last_selected_page_id`, `last_selected_kind` | page-count plus monotonic queue counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-097`; selected index/page id `-1` seams and the selection-kind vocabulary stay signed. |
| `osvm_backed_fast_path_heap_box.hako` | `HakoAllocOsVmPageBacking` | `page_id`, `base`, `bytes` | `bytes` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-083`; page id and ptr-like base stay `i64` until OSVM pointer/id contracts split. |
| `osvm_backed_fast_path_heap_box.hako` | `HakoAllocOsVmBackedHandle` | `page_id`, `block_id`, `requested_size` | `requested_size` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-082`; page/block id fields stay `i64` until id/index contracts are split. |
| `osvm_backed_fast_path_heap_box.hako` | `HakoAllocOsVmBackedFastPathHeap` | `bin`, `block_size`, `page_capacity`, `next_page_id`, `backing_count`, `alloc_count`, `release_count`, `fallback_count`, `page_create_count`, `reject_count`, `reserve_count`, `commit_count`, `decommit_count`, `source_reject_count` | event/source counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-068`; `block_size` / `page_capacity` are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-083`; `backing_count` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-085`; bin and next page id stay `i64`. |
| `page_box.hako` | `HakoAllocPageModel` | `page_id`, `block_size`, `capacity`, `reserved`, `used`, `free_top`, `local_free_top`, `alloc_count`, `local_free_count`, `local_free_collect_count`, `local_free_collected_blocks`, `reject_count`, `retired`, `decommitted`, `retire_count`, `decommit_count`, `recommit_count`, `reuse_count`, `lifecycle_reject_count`, `reactivate_count`, `reactivate_reject_count`, `peak_used`, `requested_bytes` | page-local alloc/local-free/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-059`; local-free collection counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-060`; lifecycle event/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-061`; stack-top/occupancy fields are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-073`; capacity fields are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-075`; block-size and requested-byte fields are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-077`; identity and lifecycle state flags stay `i64`. |
| `page_heap_box.hako` | `HakoAllocHandle` | `page_id`, `block_id`, `requested_size` | legacy prototype handle; keep `i64` until superseded by current page-map owners or object-return parity. |
| `page_heap_box.hako` | `HakoAllocPage` | `page_id`, `block_size`, `capacity`, `free_top`, `alloc_count`, `free_count`, `reuse_count`, `current_used`, `peak_used`, `requested_bytes` | legacy prototype page; stats counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-084`; identity, size/capacity, occupancy, and byte fields stay signed until their own rows. |
| `page_map_aligned_small_path_box.hako` | `HakoAllocPageMapAlignedSmallPath` | `meta_count`, `next_ptr`, `alloc_count`, `invalid_alignment_count`, `oversized_count`, `alloc_fail_count`, `register_fail_count`, `reject_count`, `last_result_ptr`, `last_alignment`, `last_padded_size` | aligned-small path event/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-056`; `meta_count` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-062` as the C205c store count mirror; ptr/result/alignment/size observers stay `i64`. |
| `page_map_box.hako` | `HakoAllocPageMapEntry` | `ptr`, `page_id`, `block_id`, `live` | ptr/id/index + binary live flag; keep `i64` until pointer/result API shape is exact. |
| `page_map_box.hako` | `HakoAllocPageMap` | `entry_count`, `live_count`, `register_count`, `lookup_count`, `lookup_miss_count`, `unregister_count`, `reject_count` | already exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-051`; entry pointer/id fields remain `i64`. |
| `page_map_realloc_alloc_copy_release_box.hako` | `HakoAllocPageMapReallocAllocCopyReleasePath` | `next_ptr`, `success_count`, `copy_count`, `same_class_reject_count`, `alloc_fail_count`, `lookup_miss_count`, `stale_page_count`, `released_block_count`, `reject_count`, `last_result_ptr`, `last_alloc_page_id`, `last_alloc_block_id` | fallback event/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-054`; `next_ptr`, result ptr, and `last_alloc_* = -1` sentinels stay `i64`. |
| `page_map_realloc_failure_contract_box.hako` | `HakoAllocPageMapReallocFailureContract` | `success_count`, `same_class_success_count`, `move_success_count`, `zero_reject_count`, `oversized_reject_count`, `alloc_fail_count`, `lookup_miss_count`, `stale_page_count`, `released_block_count`, `unexpected_reject_count`, `reject_count`, `last_result_ptr`, `last_failure_kind`, `last_max_block_size` | failure-matrix counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-055`; ptr/status/size observers stay `i64`. |
| `page_map_realloc_same_class_box.hako` | `HakoAllocPageMapReallocSameClassPath` | `same_class_count`, `grow_reject_count`, `lookup_miss_count`, `stale_page_count`, `released_block_count`, `reject_count`, `last_result_ptr` | same-class/no-move counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-053`; result pointer stays `i64`. |
| `page_map_release_box.hako` | `HakoAllocPageMapReleaseSeam` | `page_count`, `page_register_count`, `release_count`, `unregister_count`, `lookup_miss_count`, `stale_page_count`, `page_release_reject_count`, `reject_count` | release event/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-052`; `page_count` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-087`; page-map entry ids and flags remain `i64`. |
| `page_map_release_invariant_box.hako` | `HakoAllocPageMapReleaseObserver` | `observe_count`, `success_count`, `reject_count`, `live_count_before`, `release_count_before`, `unregister_count_before`, `page_used_before`, `local_free_before`, `last_ptr`, `last_page_id`, `last_block_id`, `last_result`, `last_entry_live_before`, `last_lookup_after`, `last_live_count_delta`, `last_release_count_delta`, `last_unregister_count_delta`, `last_page_used_delta`, `last_local_free_delta` | observer counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-070`; before-snapshots, signed sentinels, statuses, and signed delta fields stay `i64`. |
| `purge_page_source_unreserve_adapter_box.hako` | `HakoAllocPageSourceUnreserveAdapter` | `call_count`, `success_count`, `reject_count`, `last_base`, `last_bytes`, `last_rc` | call/success/reject counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-109`; pointer-like base, byte-length payload, and rc/status stay signed. |
| `purge_recommit_failfast_box.hako` | `HakoAllocRecommitFailFastEntry` | `attempt_count`, `no_recommit_count`, `blocked_count`, `missing_count`, `recommit_execution_count`, `source_execution_count`, `last_page_id` | attempt/no-recommit/blocked/missing counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-107`; closed-execution evidence counters and `last_page_id = -1` stay signed. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `bin`, `page_count`, `has_direct_page`, `direct_page_index`, `add_count`, `select_count`, `direct_hit_count`, `refresh_count`, `reject_count` | queue stats counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-058`; `page_count` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-078`; `direct_page_index` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-079`; `bin` is exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-093`; presence flag stays `i64`. |
| `remote_free_page_integration_box.hako` | `HakoAllocRemoteFreePageInbox` | `head_cell`, `init_status`, `pending_top`, `remote_push_count`, `remote_collect_count`, `retry_count`, `reject_count` | mailbox status/count fields remain `i64` until pointer-atomic lane is exact. |
| `secure_free_list_diagnostics_box.hako` | `HakoAllocSecureFreeListDiagnostics` | `scan_count`, `ok_count`, `fail_count`, `out_of_range_free_block_count`, `duplicate_free_block_count`, `live_block_in_free_list_count`, `free_count_mismatch_count`, `local_free_count_mismatch_count`, `last_ok`, `last_out_of_range_free_block`, `last_duplicate_free_block`, `last_live_block_in_free_list`, `last_free_count_mismatch`, `last_local_free_count_mismatch` | diagnostics counters are exact `usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-069`; `last_*` observation flags stay `i64` until bool / flag semantics are split. |
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
3. Probe `capacity` / stack-top fields with underflow checks. `294x-41`
   extends the proof-only probe for this; `294x-42` extends the same probe to
   exact `usize` stack-top values used as `ArrayBox.get/set` indexes. `294x-43`
   migrates the production page stack-top/occupancy owner-local group.
   `294x-44` probes exact `usize` capacity bounds with current-lane signed
   indexes before production capacity/reserved migration. `294x-45` migrates
   the production page capacity/reserved owner-local group.
4. Probe `size` and `byte-length` fields. `294x-46` extends the proof-only
   probe to exact `usize` block-size comparison and accepted-request byte-sum
   accumulation before production page-model size/byte fields migrate. `294x-47`
   migrates the production page-model size/byte owner-local group.
5. Probe `index` fields after sentinel returns and not-found states are
   explicit.
