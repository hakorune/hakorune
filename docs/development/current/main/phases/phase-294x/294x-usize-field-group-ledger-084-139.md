---
Status: Active
Date: 2026-05-24
Scope: landed `usize` field-group blocker ledger, comparison return through field-group 139.
Related:
  - docs/development/current/main/phases/phase-294x/294x-usize-field-group-ledger.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x Usize Field Group Ledger 084-139

Post-closeout follow-on:

```text
MIMALLOC-COMPARISON-VSLICE-008:
  landed by 294x-60. Selected hako-side pure-first EXE memory-use evidence as
  the next narrow comparison row.
```

Current blocker:

```text
MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-001:
  landed by 294x-61. Added a hako pure-first EXE memory-use evidence runner
  over already selected comparison `.hako` apps.
```

Current blocker:

```text
MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-002:
  landed by 294x-62. Returned from the comparison evidence lane to
  `HAKO-ALLOC-USIZE-FIELD-GROUP-084` and migrated the live legacy page-heap
  stats counters (`alloc_count`, `free_count`, `reuse_count`) to exact `usize`.
  Provider activation, host replacement, hooks, TLS, atomics, and allocator
  replacement remain parked.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-085:
  landed by 294x-63. Migrated OSVM-backed `backing_count` to exact `usize`
  after preserving the signed `page_id < 0` guard as the id/index seam.
  Page/block identity, backing pointer-like payloads, provider activation,
  host replacement, hooks, TLS, atomics, and allocator replacement remain
  parked.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-086:
  landed by 294x-64. Selected
  `HakoAllocPageMapReleaseSeam.page_count` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-087`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-087:
  landed by 294x-65. Migrated `HakoAllocPageMapReleaseSeam.page_count` to exact
  `usize`, preserving the signed `page_id < 0` guard as the id/index seam.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-088:
  landed by 294x-66. Selected `HakoAllocFastPathHeap.next_page_id` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-089`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-089:
  landed by 294x-67. Migrated `HakoAllocFastPathHeap.next_page_id` to exact
  `usize`, preserving signed handle/page id payloads and the
  `handle.page_id < 0` guard as the explicit id/index seam.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-090:
  landed by 294x-68. Selected `HakoAllocHugePageModel.next_page_id` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-091`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-091:
  landed by 294x-69. Migrated `HakoAllocHugePageModel.next_page_id` to exact
  `usize`; published page id payloads, pointer-like fields, size observers,
  statuses, page-map entries, and OSVM-backed `next_page_id` remain signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-092:
  landed by 294x-70. Selected `HakoAllocPageQueue.bin` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-093`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-093:
  landed by 294x-71. Migrated `HakoAllocPageQueue.bin` to exact `usize`; heap
  bin mirrors, size-class policy return shapes, direct-page flags/indexes,
  page/block identities, sentinels, pointer-like fields, and byte-sum fields
  remain unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-094:
  landed by 294x-72. Selected `HakoAllocFastPathHeap.bin` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-095`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-095:
  landed by 294x-73. Migrated `HakoAllocFastPathHeap.bin` to exact `usize`,
  tightened the heap-local `birth(bin: usize, ...)` surface, and kept
  `HakoAllocOsVmBackedFastPathHeap.bin`, `SizeClassBox.size_to_bin(...)` /
  `size_to_bin_usize(...)` return shapes, page/block identity payloads,
  pointer-like fields, and sentinel-bearing seams unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-096:
  landed by 294x-74. Selected `HakoAllocObjectLifecyclePageQueue`
  count/page-count group as `HAKO-ALLOC-USIZE-FIELD-GROUP-097`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-097:
  landed by 294x-75. Migrated `HakoAllocObjectLifecyclePageQueue.page_count`
  plus the monotonic queue-local count group to exact `usize`, and kept
  `last_selected_index`, `last_selected_page_id`, `last_selected_kind`, and the
  `addPage()` `-1` reject seam signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-098:
  landed by 294x-76. Selected the object-lifecycle facade result source-counter
  owner in `object_lifecycle_facade_result_box.hako` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-099`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-099:
  landed by 294x-77. Migrated
  `HakoAllocObjectLifecycleAllocResult.attempt_count`, `success_count`,
  `failure_count`, `reusable_success_count`, `active_success_count` plus
  `HakoAllocObjectLifecycleReleaseResult.success_count` and `failure_count` to
  exact `usize`, while keeping `last_*`, `last_reason`, `last_ok`,
  alignment/realloc observers, and the downstream stats-snapshot mirror signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-100:
  landed by 294x-78. Selected the downstream
  `object_lifecycle_facade_stats_box.hako` snapshot mirror owner as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-101`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-101:
  landed by 294x-79. Migrated the seven
  `HakoAllocObjectLifecycleFacadeStatsSnapshot` mirror counts to exact `usize`,
  while keeping `last_*`, `last_reason`, `last_ok`, alignment/realloc
  observers, totals helpers, page/block identity payloads, pointer-like
  fields, and unrelated lifecycle observer owners unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-102:
  landed by 294x-80. Selected the owner-local
  `object_lifecycle_facade_page_source_box.hako` counter owner as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-103`, but kept page-source report/status/source
  observer fields, ids, bytes, and page payload mirrors out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-103:
  landed by 294x-81. Migrated
  `HakoAllocObjectLifecycleFacadePageSourceAttach.reserve_count`,
  `commit_count`, `attach_count`, and `reject_count` to exact `usize`, while
  keeping `HakoAllocObjectLifecycleFacadePageSourceAttachReport.status`,
  `source_*`, `added_page_id`, `facade_page_count`, `base`, `bytes`,
  `block_size`, `capacity`, and `reserved` signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-104:
  landed by 294x-82. Selected the owner-local
  `object_lifecycle_facade_page_source_alloc_miss_box.hako` fallback counter
  owner as `HAKO-ALLOC-USIZE-FIELD-GROUP-105`, while keeping the alloc-miss
  report observer seam, its count mirrors, and unrelated OSVM/bin/provider/hook
  rows out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-105:
  landed by 294x-83. Migrated
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.fallback_attempt_count`,
  `source_success_count`, `source_failure_count`, `retry_success_count`, and
  `retry_failure_count` to exact `usize`, while keeping the alloc-miss report
  `status`, `initial_*`, `fallback_attempted`, `source_*`, `retry_*`,
  `final_*`, `source_base`, `source_bytes`, `final_page_id`,
  `final_block_id`, and the report-mirror counts signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-106:
  landed by 294x-84. Selected
  `HakoAllocRecommitFailFastEntry.attempt_count`, `no_recommit_count`,
  `blocked_count`, and `missing_count` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-107`, while keeping the recommit report,
  `last_page_id = -1`, and the closed-execution
  `recommit_execution_count` / `source_execution_count` evidence signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-107:
  landed by 294x-85. Migrated only
  `HakoAllocRecommitFailFastEntry.attempt_count`, `no_recommit_count`,
  `blocked_count`, and `missing_count` to exact `usize`, while keeping the
  recommit report, `last_page_id = -1`, and the closed-execution
  `recommit_execution_count` / `source_execution_count` evidence signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-108:
  landed by 294x-86. Selected
  `HakoAllocPageSourceUnreserveAdapter.call_count`, `success_count`, and
  `reject_count` as `HAKO-ALLOC-USIZE-FIELD-GROUP-109`, while keeping
  `last_base`, `last_bytes`, and `last_rc` signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-109:
  landed by 294x-87. Migrated only
  `HakoAllocPageSourceUnreserveAdapter.call_count`, `success_count`, and
  `reject_count` to exact `usize`, while keeping `last_base`, `last_bytes`,
  and `last_rc` signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-110:
  landed by 294x-88. Selected
  `HakoAllocPageSourceRecommitAdapter.call_count`, `success_count`, and
  `reject_count` as `HAKO-ALLOC-USIZE-FIELD-GROUP-111`, while keeping
  `last_base`, `last_bytes`, and `last_rc` signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-111:
  landed by 294x-89. Migrated only
  `HakoAllocPageSourceRecommitAdapter.call_count`, `success_count`, and
  `reject_count` to exact `usize`, while keeping `last_base`, `last_bytes`,
  and `last_rc` signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-112:
  landed by 294x-91. Selected
  `HakoAllocPageSourceDecommitAdapter.call_count`, `success_count`, and
  `reject_count` as `HAKO-ALLOC-USIZE-FIELD-GROUP-113`, while keeping
  `last_base`, `last_bytes`, and `last_rc` signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-113:
  landed by 294x-92. Migrated only
  `HakoAllocPageSourceDecommitAdapter.call_count`, `success_count`, and
  `reject_count` to exact `usize`, while keeping `last_base`, `last_bytes`,
  and `last_rc` signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-114:
  landed by 294x-93. Selected the decommit-side
  `HakoAllocPurgeDecommitStateMarker` counters (`attempt_count`,
  `marked_count`, `reject_count`, `duplicate_count`,
  `missing_report_count`, `not_decommitted_count`, and
  `release_field_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-115`, while
  keeping marker arrays, `last_page_id`, report fields, and recommit-side
  counters signed/unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-115:
  landed by 294x-94. Migrated only the decommit-side
  `HakoAllocPurgeDecommitStateMarker` counters (`attempt_count`,
  `marked_count`, `reject_count`, `duplicate_count`,
  `missing_report_count`, `not_decommitted_count`, and
  `release_field_reject_count`) to exact `usize`, while keeping marker arrays,
  `last_page_id`, report fields, and recommit-side counters unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-116:
  landed by 294x-95. Selected the recommit-side
  `HakoAllocPurgeDecommitStateMarker` counters (`recommit_attempt_count`,
  `recommitted_count`, `recommit_reject_count`,
  `duplicate_recommit_count`, `missing_recommit_report_count`,
  `not_recommitted_count`, `recommit_widened_reject_count`, and
  `unmarked_recommit_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-117`, while keeping marker arrays,
  `last_page_id`, report fields, and page-source / heap execution state
  unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-117:
  landed by 294x-96. Migrated only the recommit-side
  `HakoAllocPurgeDecommitStateMarker` counters (`recommit_attempt_count`,
  `recommitted_count`, `recommit_reject_count`,
  `duplicate_recommit_count`, `missing_recommit_report_count`,
  `not_recommitted_count`, `recommit_widened_reject_count`, and
  `unmarked_recommit_reject_count`) to exact `usize`, while keeping marker
  arrays, `last_page_id`, report fields, and page-source / heap execution
  state unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-118:
  landed by 294x-97. Selected
  `HakoAllocBoundedDecommitPolicy.attempt_count`, `blocked_count`,
  `decommit_attempt_count`, `decommit_success_count`, and
  `source_reject_count` as `HAKO-ALLOC-USIZE-FIELD-GROUP-119`, while keeping
  `max_decommit_bytes`, report fields, fake proof source counters, page-source
  adapter state, and heap/page execution state unchanged.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-119:
  landed by 294x-98. Migrated only the selected
  `HakoAllocBoundedDecommitPolicy` owner-local counters
  (`attempt_count`, `blocked_count`, `decommit_attempt_count`,
  `decommit_success_count`, and `source_reject_count`) to exact `usize`,
  while keeping `max_decommit_bytes`, report fields, fake proof source
  counters, page-source adapter state, heap/page mutation, OSVM byte/pointer
  payloads, provider / hook / global-allocator rows, TLS, atomics, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-120:
  landed by 294x-99. Selected the
  `HakoAllocHeapReusePriorityPolicy` owner-local counters (`select_count`,
  `active_pick_count`, `recommitted_pick_count`, `retired_pick_count`,
  `fresh_pick_count`, `decommitted_skip_count`, and `missing_skip_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-121`, while keeping decision fields,
  route/page-id sentinels, page lifecycle observer counters, heap/page queues,
  page-source adapters, heap/page mutation, OSVM byte/pointer payloads,
  provider / hook / global-allocator rows, TLS, atomics, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-121:
  landed by 294x-100. Migrated only the selected heap reuse priority
  owner-local counters to exact `usize`, while keeping decision fields,
  `last_route`, `last_page_id`, page lifecycle observer counters, heap/page
  queues, page-source adapters, heap/page mutation, OSVM byte/pointer payloads,
  provider / hook / global-allocator rows, TLS, atomics, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-122:
  landed by 294x-101. Selected the
  `HakoAllocPageLifecycleInvariantObserver` owner-local counters
  (`observe_count`, `missing_count`, `active_count`, `retired_count`,
  `decommitted_count`, and `recommitted_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-123`, while keeping lifecycle report fields,
  `last_page_id`, `last_state`, heap/page queues, page-source adapters,
  heap/page mutation, OSVM byte/pointer payloads, provider / hook /
  global-allocator rows, TLS, atomics, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-123:
  landed by 294x-102. Migrated only the selected page lifecycle observer
  owner-local counters to exact `usize`, while keeping lifecycle report
  fields, `last_page_id`, `last_state`, heap/page queues, page-source
  adapters, heap/page mutation, OSVM byte/pointer payloads, provider / hook /
  global-allocator rows, TLS, atomics, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-124:
  landed by 294x-103. Selected the
  `HakoAllocAbandonedReclaimInventory` owner-local counters
  (`classify_count`, `candidate_count`, `reject_count`,
  `missing_backing_reject_count`, `owner_active_reject_count`,
  `remote_pending_reject_count`, `decommitted_reject_count`,
  `abandoned_live_count`, `abandoned_retired_count`, and
  `purge_forward_candidate_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-125`,
  while keeping decision fields, `last_page_id`, `last_reason`, reclaim
  scheduling/execution, atomics, remote-free draining, page-source calls, OSVM
  byte/pointer payloads, provider / hook / global-allocator rows, TLS, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-125:
  landed by 294x-104. Migrated only the selected abandoned/reclaim inventory
  owner-local counters to exact `usize`, while keeping decision fields,
  `last_page_id`, `last_reason`, reclaim scheduling/execution, atomics,
  remote-free draining, page-source calls, OSVM byte/pointer payloads,
  provider / hook / global-allocator rows, TLS, and `#[global_allocator]` out
  of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-126:
  landed by 294x-105. Selected the
  `HakoAllocAllocatorComparisonCMimallocExplicitRunnerExecutionPilot`
  owner-local counters (`pilot_count`, `accepted_count`, `reject_count`,
  `missing_diagnostic_reject_count`, `rejected_diagnostic_reject_count`,
  `missing_runner_reject_count`, `missing_output_reject_count`,
  `missing_memory_evidence_reject_count`,
  `missing_output_contract_reject_count`, `failed_runner_reject_count`, and
  `invalid_run_count_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-127`,
  while keeping runner payloads, memory/RSS evidence, report mirrors,
  `last_reason`, stop-line flags, provider / hook / global-allocator rows,
  worker/TLS, threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-127:
  landed by 294x-106. Migrated only the selected explicit C mimalloc runner
  execution pilot owner-local counters to exact `usize`, while keeping runner
  payload records, memory/RSS evidence, report fields, reason vocabulary,
  stop-line flags, provider / hook / global-allocator rows, worker/TLS,
  threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-128:
  landed by 294x-107. Selected the
  `HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic`
  owner-local counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_diagnostic_blocked_count`, `rejected_diagnostic_blocked_count`,
  `missing_runner_blocked_count`, `missing_output_blocked_count`,
  `missing_memory_evidence_blocked_count`,
  `missing_output_contract_blocked_count`, `failed_runner_blocked_count`, and
  `invalid_run_count_blocked_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-129`,
  while keeping runner payloads, memory/RSS evidence, report mirrors,
  `last_reason`, stop-line flags, provider / hook / global-allocator rows,
  worker/TLS, threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-129:
  landed by 294x-108. Migrated only the selected explicit C mimalloc runner
  evidence diagnostic owner-local counters to exact `usize`, while keeping
  runner payloads, memory/RSS evidence, report fields, reason vocabulary,
  stop-line flags, provider / hook / global-allocator rows, worker/TLS,
  threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-130:
  landed by 294x-109. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultLedger` owner-local counters
  (`ledger_count`, `accepted_count`, `reject_count`,
  `missing_hako_diagnostic_reject_count`,
  `blocked_hako_diagnostic_reject_count`,
  `missing_c_diagnostic_reject_count`, and
  `blocked_c_diagnostic_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-131`, while keeping comparison payloads,
  signed deltas, report mirrors, `last_reason`, conclusion flags, repeated
  benchmark execution, provider / hook / global-allocator rows, worker/TLS,
  threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-131:
  landed by 294x-110. Migrated only the selected C mimalloc result ledger
  owner-local counters to exact `usize`, while keeping comparison payloads,
  signed deltas, report fields, reason vocabulary, conclusion flags, repeated
  benchmark execution, provider / hook / global-allocator rows, worker/TLS,
  threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-132:
  landed by 294x-111. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic`
  owner-local counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_hako_blocked_count`, `blocked_hako_blocked_count`,
  `missing_c_blocked_count`, and `blocked_c_blocked_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-133`, while keeping comparison payloads,
  signed deltas, report mirrors, `last_reason`, conclusion flags, repeated
  benchmark execution, provider / hook / global-allocator rows, worker/TLS,
  threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-133:
  landed by 294x-112. Migrated only the selected C mimalloc result ledger
  diagnostic owner-local counters to exact `usize`, while keeping comparison
  payloads, signed deltas, report fields, reason vocabulary, conclusion flags,
  repeated benchmark execution, provider / hook / global-allocator rows,
  worker/TLS, threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-134:
  landed by 294x-113. Selected the
  `HakoAllocAllocatorComparisonCMimallocExecutionInventory` owner-local
  counters (`inventory_count`, `accepted_count`, `reject_count`,
  `missing_runner_reject_count`, `missing_workload_reject_count`,
  `missing_hako_metrics_reject_count`,
  `missing_output_contract_reject_count`,
  `missing_memory_usage_contract_reject_count`,
  `missing_evidence_storage_reject_count`, `missing_run_count_reject_count`,
  and `invalid_run_count_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-135`, while keeping run-count payloads, report
  mirrors, `last_reason`, C execution behavior, provider / hook /
  global-allocator rows, worker/TLS, threads, and `#[global_allocator]` out of
  scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-135:
  landed by 294x-114. Migrated only the selected C mimalloc execution
  inventory owner-local counters to exact `usize`, while keeping run-count
  payloads, report fields, reason vocabulary, C execution behavior, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-136:
  landed by 294x-115. Selected the
  `HakoAllocAllocatorComparisonCMimallocExecutionDiagnostic` owner-local
  counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_runner_blocked_count`, `missing_workload_blocked_count`,
  `missing_hako_metrics_blocked_count`,
  `missing_output_contract_blocked_count`,
  `missing_memory_usage_contract_blocked_count`,
  `missing_evidence_storage_blocked_count`, `missing_run_count_blocked_count`,
  and `invalid_run_count_blocked_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-137`, while keeping run-count payloads, report
  mirrors, `last_reason`, C execution behavior, provider / hook /
  global-allocator rows, worker/TLS, threads, and `#[global_allocator]` out of
  scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-137:
  landed by 294x-116. Migrated only the selected C mimalloc execution
  diagnostic owner-local counters to exact `usize`, while keeping run-count
  payloads, report fields, reason vocabulary, C execution behavior, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-138:
  landed by 294x-117. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultSummaryInventory` owner-local
  counters (`summary_count`, `ready_count`, `blocked_count`,
  `missing_ledger_reject_count`, `missing_diagnostic_reject_count`, and
  `blocked_diagnostic_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-139`,
  while keeping comparison payloads, report mirrors, `last_reason`,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-139:
  landed by 294x-118. Migrated only the selected C mimalloc result summary
  inventory owner-local counters to exact `usize`, while keeping comparison
  payloads, report fields, reason vocabulary, performance/memory conclusions,
  repeated benchmark execution, provider / hook / global-allocator rows,
  worker/TLS, threads, and `#[global_allocator]` out of scope.
```
