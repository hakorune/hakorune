# Hako Alloc Numeric Field Group Ledger

Status: Active
Scope: long-form exact `usize` field-group selection and migration history.
Related:
- `NUMERIC_FIELDS.md`
- `docs/development/current/main/phases/phase-294x/294x-usize-field-group-ledger.md`

This file owns the detailed field-group history that used to make
`NUMERIC_FIELDS.md` too large. Keep `NUMERIC_FIELDS.md` focused on policy,
categories, current inventory, sentinel notes, and migration order.

## Field Group History

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
  until the aligned-small metadata store count migrates; pointer and alignment
  observers stay `i64`, while padded-size observer migration is handled by
  `HAKO-ALLOC-USIZE-FIELD-GROUP-181`.

- `huge_threshold_router_box.hako` / `HakoAllocHugeThresholdRouter` route /
  reject counter fields:
  `small_route_count`, `small_success_count`, `small_reject_count`,
  `huge_route_count`, `huge_reject_count`, `invalid_alignment_count`,
  `invalid_size_count`, `reject_count`.
  `HAKO-ALLOC-USIZE-FIELD-GROUP-057` selects and migrates this group because
  these are non-negative route-local counters. `last_route_kind`,
  `last_result_ptr`, and `last_good_size` stay `i64`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-180` was deferred by `294x-181` after the
  downstream pure-first huge/OSVM comparison EXE path rejected the direct
  router observer migration. The row selected
  `HakoAllocPageMapAlignedSmallPath.last_padded_size` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-181`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-181` migrated
  `HakoAllocPageMapAlignedSmallPath.last_padded_size` to exact `usize`, while
  the router observers, pointer-shaped fields, alignment observer, and metadata
  store payloads stay signed/closed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-182` migrated the safe huge-threshold router
  size observers `last_padded_size` and `last_huge_threshold` to exact
  `usize`. `last_good_size` stays `i64` because huge requests can set it to
  the signed `SizeClassBox.good_size(...) == -1` sentinel.

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
- `HAKO-ALLOC-USIZE-FIELD-GROUP-171` selected the downstream
  `HakoAllocObjectLifecycleFacadePageSourceAttachReport` mirror counters as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-172`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-172` migrated
  `HakoAllocObjectLifecycleFacadePageSourceAttachReport.source_reserved`,
  `source_committed`, `facade_page_count`, and `source_reject` to exact
  `usize`, while status, added-page id, pointer-like base, byte-length, and
  page payload mirrors stay signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-173` selected the remaining non-negative
  `HakoAllocObjectLifecycleFacadePageSourceAttachReport` page-source payloads
  as `HAKO-ALLOC-USIZE-FIELD-GROUP-174`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-174` migrated
  `HakoAllocObjectLifecycleFacadePageSourceAttachReport.bytes`,
  `block_size`, `capacity`, and `reserved` to exact `usize`, while status,
  added-page id, and pointer-like base stay signed.
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
- `HAKO-ALLOC-USIZE-FIELD-GROUP-169` selected the downstream alloc-miss
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport` report mirror
  counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-170`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-170` migrated
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport.fallback_attempt_count`,
  `source_success_count`, `source_failure_count`, `retry_success_count`, and
  `retry_failure_count` to exact `usize`, while status/reason/ok-like,
  source/final, page/block id, pointer-like base, and byte-length mirrors stay
  signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-175` selected the alloc-miss
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport` source count
  mirrors as `HAKO-ALLOC-USIZE-FIELD-GROUP-176`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-176` migrated
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport.source_reserved`,
  `source_committed`, `source_reject`, and `source_facade_page_count` to exact
  `usize`, while source status, added page id, pointer-like base, byte-length
  mirror, retry/final status and reason, and page/block id payloads stay signed.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-177` selected
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport.source_bytes` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-178`.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-178` migrated
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport.source_bytes` to
  exact `usize`, while source status, added page id, pointer-like base,
  retry/final status and reason, and page/block id payloads stay signed.
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
- `HAKO-ALLOC-USIZE-FIELD-GROUP-185` selected the page-source executor adapter
  byte-length observers (`HakoAllocPageSourceUnreserveAdapter.last_bytes`,
  `HakoAllocPageSourceRecommitAdapter.last_bytes`, and
  `HakoAllocPageSourceDecommitAdapter.last_bytes`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-186`. Keep pointer-like `last_base`, status
  `last_rc`, policy behavior, OSVM substrate behavior, and provider / hook /
  replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-186` migrated those three page-source executor
  adapter byte-length observers to exact `usize`, while pointer-like
  `last_base`, status `last_rc`, policy behavior, OSVM substrate behavior, and
  provider / hook / replacement seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-187` selected the owner-local
  `HakoAllocProviderSelectionInventory` counters (`selection_count`,
  `accepted_count`, `reject_count`, `missing_readiness_reject_count`,
  `rejected_readiness_reject_count`, `invalid_readiness_token_reject_count`,
  `invalid_candidate_token_reject_count`, `invalid_provider_kind_reject_count`,
  and `closed_execution_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-188`. Keep `last_reason`, report mirrors,
  tokens/kind vocabularies, bool-like inactive / would-execute flags, and
  provider activation / hook / replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-188` migrated those
  `HakoAllocProviderSelectionInventory` owner-local counters to exact `usize`,
  while `last_reason`, report mirrors, tokens/kind vocabularies, bool-like
  inactive / would-execute flags, and provider activation / hook / replacement
  seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-189` selected the owner-local
  `HakoAllocProviderActivationDryRunUnsupportedBehavior` counters
  (`dry_run_count`, `accepted_count`, `reject_count`,
  `missing_bundle_reject_count`, `rejected_bundle_reject_count`,
  `invalid_request_token_reject_count`, `invalid_mode_reject_count`,
  `unsupported_evidence_reject_count`, and `closed_execution_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-190`. Keep `last_reason`, report mirrors,
  activation tokens/mode payloads, bool-like unsupported / inactive /
  would-execute flags, and provider activation / hook / replacement seams
  separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-190` migrated those
  `HakoAllocProviderActivationDryRunUnsupportedBehavior` owner-local counters
  to exact `usize`, while `last_reason`, report mirrors, activation
  token/mode payloads, bool-like unsupported / inactive / would-execute flags,
  and provider activation / hook / replacement seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-191` selected the owner-local
  `HakoAllocProviderActivationInputBundleInventory` counters (`bundle_count`,
  `accepted_count`, `reject_count`, `missing_outcome_reject_count`,
  `rejected_outcome_reject_count`, `invalid_candidate_reject_count`,
  `invalid_kind_reject_count`, `invalid_request_token_reject_count`,
  `invalid_mode_reject_count`, `unsupported_evidence_reject_count`, and
  `closed_execution_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-192`.
  Keep `last_reason`, report mirrors, activation token/mode payloads,
  bool-like unsupported / inactive / would-execute flags, and provider
  activation / hook / replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-192` migrated those
  `HakoAllocProviderActivationInputBundleInventory` owner-local counters to
  exact `usize`, while `last_reason`, report mirrors, activation token/mode
  payloads, bool-like unsupported / inactive / would-execute flags, and
  provider activation / hook / replacement seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-193` selected the owner-local
  `HakoAllocProviderActivationModeledOpenPilot` counters
  (`modeled_open_count`, `accepted_count`, `reject_count`,
  `missing_dry_run_reject_count`, `rejected_dry_run_reject_count`,
  `invalid_request_token_reject_count`, `invalid_mode_reject_count`,
  `closed_call_reject_count`, `closed_host_replacement_reject_count`,
  `closed_hook_reject_count`, and `closed_backend_matcher_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-194`. Keep `last_reason`, report mirrors,
  activation token/mode payloads, bool-like inactive / modeled-open /
  would-execute flags, and provider call / hook / replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-194` migrated those
  `HakoAllocProviderActivationModeledOpenPilot` owner-local counters to exact
  `usize`, while `last_reason`, report mirrors, activation token/mode
  payloads, bool-like inactive / modeled-open / would-execute flags, and
  provider call / hook / replacement seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-195` selected the owner-local
  `HakoAllocProviderCallCapabilityGateInventory` counters (`inventory_count`,
  `accepted_count`, `reject_count`, `missing_model_reject_count`,
  `inactive_model_reject_count`, `missing_capability_reject_count`,
  `invalid_capability_reject_count`, and `closed_execution_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-196`. Keep `last_reason`, report mirrors,
  capability flags, modeled-open payloads, bool-like inactive / would-execute
  flags, and provider call / hook / replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-196` migrated those
  `HakoAllocProviderCallCapabilityGateInventory` owner-local counters to exact
  `usize`, while `last_reason`, report mirrors, capability flags,
  modeled-open payloads, bool-like inactive / would-execute flags, and provider
  call / hook / replacement seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-197` selected the owner-local
  `HakoAllocProviderCallDryRunUnsupportedBehavior` counters (`dry_run_count`,
  `accepted_count`, `reject_count`, `missing_gate_reject_count`,
  `rejected_gate_reject_count`, and `closed_execution_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-198`. Keep `last_reason`, report mirrors,
  capability flags, dry-run payloads, bool-like inactive / would-execute flags,
  and provider call / hook / replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-198` migrated those
  `HakoAllocProviderCallDryRunUnsupportedBehavior` owner-local counters to
  exact `usize`, while `last_reason`, report mirrors, capability flags,
  dry-run payloads, bool-like inactive / would-execute flags, and provider
  call / hook / replacement seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-199` selected the owner-local
  `HakoAllocProviderCallModeledOpenPilot` counters (`modeled_open_count`,
  `accepted_count`, `reject_count`, `missing_dry_run_reject_count`,
  `rejected_dry_run_reject_count`, `missing_capability_reject_count`,
  `invalid_capability_reject_count`, `unsupported_outcome_reject_count`,
  `closed_call_reject_count`, `closed_host_replacement_reject_count`,
  `closed_hook_reject_count`, and `closed_backend_matcher_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-200`. Keep `last_reason`, report mirrors,
  capability flags, modeled-open payloads, bool-like inactive / would-execute
  flags, and provider call / hook / replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-200` migrated those
  `HakoAllocProviderCallModeledOpenPilot` owner-local counters to exact
  `usize`, while `last_reason`, report mirrors, capability flags,
  modeled-open payloads, bool-like inactive / would-execute flags, and
  provider call / hook / replacement seams remain unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-201` selected the owner-local
  `HakoAllocProviderCallExecutionCapabilityPreflight` counters
  (`preflight_count`, `accepted_count`, `reject_count`,
  `missing_model_reject_count`, `inactive_model_reject_count`,
  `missing_capability_reject_count`, `invalid_capability_reject_count`,
  `closed_execution_reject_count`, `closed_host_replacement_reject_count`,
  `closed_hook_reject_count`, and `closed_backend_matcher_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-202`. Keep `last_reason`, report mirrors,
  capability flags, preflight payloads, bool-like readiness / would-execute
  flags, and provider call / hook / replacement seams separate.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-202` migrated those
  `HakoAllocProviderCallExecutionCapabilityPreflight` owner-local counters to
  exact `usize`, while `last_reason`, report mirrors, capability flags,
  preflight payloads, bool-like readiness / would-execute flags, and provider
  call / hook / replacement seams remain unchanged.
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
- `HAKO-ALLOC-USIZE-FIELD-GROUP-203` selected
  `HakoAllocProviderCallNoopExecutionSeamPilot` owner-local counters
  (`seam_count`, `accepted_count`, `reject_count`,
  `missing_preflight_reject_count`, `rejected_preflight_reject_count`,
  `not_ready_reject_count`, `closed_execution_reject_count`,
  `closed_host_replacement_reject_count`, `closed_hook_reject_count`, and
  `closed_backend_matcher_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-204`, while `last_reason`, report fields,
  no-op/open/executed flags, provider API call flags, bool-like readiness /
  would-execute flags, provider / hook / replacement rows, worker/TLS, and
  threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-204` migrated those ten
  `HakoAllocProviderCallNoopExecutionSeamPilot` owner-local counters to exact
  `usize`, while `last_reason`, report fields, no-op/open/executed flags,
  provider API call flags, bool-like readiness / would-execute flags, provider
  / hook / replacement rows, worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-205` selected
  `HakoAllocProviderCallRealApiExecutionPreflight` owner-local counters
  (`preflight_count`, `accepted_count`, `reject_count`,
  `missing_noop_reject_count`, `rejected_noop_reject_count`,
  `missing_capability_reject_count`, `invalid_capability_reject_count`,
  `already_executed_reject_count`, `closed_execution_reject_count`,
  `closed_host_replacement_reject_count`, `closed_hook_reject_count`, and
  `closed_backend_matcher_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-206`, while `last_reason`, report fields, real
  API preflight payloads, capability flags, provider API call flags,
  bool-like readiness / would-execute flags, provider / hook / replacement
  rows, worker/TLS, and threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-206` migrated those twelve
  `HakoAllocProviderCallRealApiExecutionPreflight` owner-local counters to
  exact `usize`, while `last_reason`, report fields, real API preflight
  payloads, capability flags, provider API call flags, bool-like readiness /
  would-execute flags, provider / hook / replacement rows, worker/TLS, and
  threads stay unchanged.
- `HAKO-ALLOC-USIZE-FIELD-GROUP-207` selected
  `HakoAllocProviderCallRealApiStubExecutionPilot` owner-local counters
  (`execution_count`, `accepted_count`, `reject_count`,
  `missing_preflight_reject_count`, `rejected_preflight_reject_count`,
  `not_ready_reject_count`, `already_executed_reject_count`,
  `closed_execution_reject_count`, `closed_host_replacement_reject_count`,
  `closed_hook_reject_count`, and `closed_backend_matcher_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-208`, while `last_reason`, report fields, stub
  execution payloads, result-code fields, actual provider API call flags,
  bool-like readiness / would-execute flags, provider / hook / replacement
  rows, worker/TLS, and threads stay unchanged.

All other live production numeric stored fields remain `i64` until their own
field-group row records the invariant, stop line, and acceptance gate.

Non-stored exact `usize` facades are tracked separately from stored-field
migration. M187 adds `SizeClassBox` `usize` input facades; those do not change
the stored field count and keep sentinel-returning results signed. M188 extends
that non-stored facade pattern to allocation request sizes and alignments.
