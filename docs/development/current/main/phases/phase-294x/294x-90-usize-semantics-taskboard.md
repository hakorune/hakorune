---
Status: Active
Date: 2026-05-12
Scope: taskboard for exact `usize` / pointer-sized unsigned semantics.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
---

# 294x-90 Usize Semantics Taskboard

## Rule

One row should add one durable semantic slice. Do not combine metadata
preservation, runtime behavior, backend lowering, and hako_alloc migration in
one commit unless a row explicitly says it is docs-only.

VM rows are semantic reference execution rows, not product-owner rows. They may
consume MIR-owned facts/contracts, but VM-only behavior is not completion for
hako_alloc or mimalloc migration.

## Quick Current Truth

- `294x-10f` landed the VM reference exact numeric value representation.
- Production `hako_alloc` facade-local stats are exact `usize`; remaining
  page/heap/queue/handle fields stay `i64` except for explicitly migrated
  page-map, release-seam, and realloc same-class/no-move counter groups.
- Mimalloc `.hako` algorithm rows may continue, but they must not claim
  production `usize` field migration yet.
- Native exact numeric typed-object slot representation exists in
  `nyash_kernel`.
- Exact numeric signed/unsigned field helper lanes exist in `nyash_kernel`.
- Python LLVM and the pure-first C shim consume exact typed-object field ABI for
  exact-storage plans.
- Python LLVM consumes exact add/sub/mul, compare, and logical-shift route
  facts; div/mod/bitwise/wrapping stay later vocabulary.
- Further production `hako_alloc` migration remains field-group gated.
- Mimalloc `.hako` work should now target a comparison-quality vertical slice,
  not a full allocator-wide port. Required fields and paths should be selected
  by the workload/report slice below; broad field migration, provider
  activation, DLL packaging, and host allocator replacement remain parked.

## Next Implementation Queue

| Order | Row | Status | Implementation Boundary |
| --- | --- | --- | --- |
| 0 | `293x-185` | Complete | Allocate a replacement ptr, model copy count, and release the old ptr only after success. |
| 1 | `293x-186` | Complete | Realloc negative matrix and failure contract, no extra API expansion. |
| 2 | `293x-187` | Complete | Alignment normalization, power-of-two validation, and padded-size policy only. |
| 3 | `293x-188` | Complete | Alignment metadata now attaches to normal page-map-backed small allocations. |
| 4 | `M179-M184` | Next | Huge-page and secure-list rows, one responsibility per row. |
| 5 | `M185-M190` | Planned | Remaining `usize` migration and object-return/failure-handle API parity. |

Roadmap correction: `M186 exact usize facade stats` is already complete as
`294x-19e`. Do not schedule duplicate facade migration; use `M185+` for
remaining field groups and allocator API parity only.

## Mimalloc Comparison Vertical Slice Queue

This queue overrides the tempting "finish every remaining allocator field"
interpretation. The short-term goal is a measurable `.hako` / `hako_alloc`
slice that can be compared against the existing C mimalloc runner evidence,
not a full native mimalloc-compatible allocator.

| Order | Slice | Validation | Boundary |
| --- | --- | --- | --- |
| V0 | Select the comparison workload pack | docs + manifest/static guard | Selected by `294x-53`. Use a small fixed-size alloc/free/reuse workload, a mixed small-size workload, realloc same-class/grow fallback, aligned-small, and huge/OSVM-backed allocation. Do not add provider activation or host replacement. |
| V1 | Close only comparison-required `usize` fields | field-group L2, L3 only when first-pattern requires it | Started by `294x-54` for the OSVM-backed byte-length seam. Migrate request size, block size, capacity, queue count/index, and report counters only when the workload consumes them. Keep ids, pointer payloads, sentinels, and status flags signed until their own contracts are needed. |
| V2 | Hako alloc small-path comparison slice | VM + MIR + route preflight; representative EXE closeout | Started by `294x-55` / `MIMALLOC-COMPARISON-VSLICE-003` as a model-only schema pilot. Compose existing size-class, page model, page queue, page-map release, and local reuse paths into one stable output schema. No remote-free stress, TLS, abandoned heap, or atomic bitmap expansion. |
| V3 | Realloc/aligned comparison slice | same as V2 | Started by `294x-57` / `MIMALLOC-COMPARISON-VSLICE-005` as a model-only schema pilot. Reuse M174-M178 behavior and produce requested bytes, copied bytes, live handles, failure reason, and alignment metadata evidence. No new API surface unless the report schema requires it. |
| V4 | Huge/OSVM comparison slice | MIR + route preflight + representative pure-first EXE | Started by `294x-58` / `MIMALLOC-COMPARISON-VSLICE-006` as an OSVM-backed schema pilot. Reuse M179-M181 and existing OSVM page-source composition for huge requests, reporting reserve/commit/decommit evidence without widening page-source ownership. |
| V5 | C mimalloc vs `.hako` report closeout | representative L3 / allocator-wide only at closeout | Landed by `294x-59` / `MIMALLOC-COMPARISON-VSLICE-007`. Aligns the selected V2/V3/V4 `.hako` output schema with the existing C mimalloc explicit runner planning surface: requested bytes, committed/live bytes or handles, operation counts, failure reasons, and RSS/memory-use evidence where available. |

Defer beyond this queue:

- full size-class table parity;
- true worker/TLS and remote-free stress;
- abandoned heap reclamation;
- atomic bitmap execution;
- provider/DLL/global allocator integration;
- complete replacement of all remaining `i64` allocator fields.

Rule: if a row does not help V0-V5 produce comparable evidence, it should be
parked or batched into a later native-allocator phase.

## Phase Closeout Target

Close phase 294x after the comparison vertical slice has enough exact `usize`
storage to produce stable `.hako` / `hako_alloc` reports and compare them with
the C mimalloc runner evidence.

Do not keep extending this phase to drain:

- report mirrors / `ReportFields` payload mirrors;
- bool/status/reason vocabulary fields;
- signed sentinel-bearing ids, indexes, and deltas;
- broad page/heap/queue/handle state outside the comparison slice;
- provider/DLL packaging, hook installation, host/global allocator replacement,
  worker/TLS, true threads, remote-free stress, or abandoned-heap stress.

Next field-group rows should therefore prefer owner-local monotonic counters
that the comparison slice already reads. If the next candidate is only a mirror
or a broad identity/payload field, park it and move to closeout planning.

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

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-140:
  landed by 294x-119. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic` owner-local
  counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_summary_blocked_count`, and `blocked_summary_blocked_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-141`, while keeping comparison payloads, report
  mirrors, `last_reason`, performance/memory conclusions, repeated benchmark
  execution, provider / hook / global-allocator rows, worker/TLS, threads, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-141:
  landed by 294x-120. Migrated only the selected C mimalloc result summary
  diagnostic owner-local counters to exact `usize`, while keeping comparison
  payloads, report fields, reason vocabulary, performance/memory conclusions,
  repeated benchmark execution, provider / hook / global-allocator rows,
  worker/TLS, threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-142:
  landed by 294x-121. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultReportingInventory` owner-local
  counters (`reporting_count`, `ready_count`, `blocked_count`,
  `missing_summary_diagnostic_reject_count`, and
  `blocked_summary_diagnostic_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-143`, while keeping comparison payloads, report
  mirrors, `last_reason`, performance/memory conclusions, repeated benchmark
  execution, provider / hook / global-allocator rows, worker/TLS, threads, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-143:
  landed by 294x-122. Migrated only the selected C mimalloc result reporting
  inventory owner-local counters to exact `usize`, while keeping comparison
  payloads, report fields, reason vocabulary, performance/memory conclusions,
  repeated benchmark execution, provider / hook / global-allocator rows,
  worker/TLS, threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-144:
  landed by 294x-123. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultReportingDiagnostic` owner-local
  counters (`diagnostic_count`, `ready_count`, `blocked_count`,
  `missing_reporting_blocked_count`, and
  `blocked_reporting_blocked_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-145`, while keeping comparison payloads, report
  mirrors, `last_reason`, performance/memory conclusions, repeated benchmark
  execution, provider / hook / global-allocator rows, worker/TLS, threads, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-145:
  landed by 294x-124. Migrated only the selected C mimalloc result reporting
  diagnostic owner-local counters to exact `usize`, while keeping comparison
  payloads, report fields, reason vocabulary, performance/memory conclusions,
  repeated benchmark execution, provider / hook / global-allocator rows,
  worker/TLS, threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-146:
  landed by 294x-125. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyConclusionPilot`
  owner-local counters (`presentation_count`, `accepted_count`,
  `blocked_count`, `missing_pilot_reject_count`,
  `blocked_pilot_reject_count`, `missing_presentation_input_reject_count`,
  and `closed_stop_line_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-147`, while keeping comparison payloads,
  report mirrors, `last_reason`, performance/memory conclusions, repeated
  benchmark execution, provider / hook / global-allocator rows, worker/TLS,
  threads, and `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-147:
  landed by 294x-126. Migrated only the selected C mimalloc result
  presentation-only conclusion pilot owner-local counters to exact `usize`,
  while keeping comparison payloads, report fields, reason vocabulary,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-148:
  landed by 294x-127. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultPresentationFollowOnPilot`
  owner-local counters (`follow_on_count`, `accepted_count`, `blocked_count`,
  `missing_pilot_reject_count`, `blocked_pilot_reject_count`,
  `missing_follow_on_input_reject_count`, and
  `closed_stop_line_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-149`,
  while keeping comparison payloads, report mirrors, `last_reason`,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-149:
  landed by 294x-128. Migrated only the selected C mimalloc result
  presentation follow-on pilot owner-local counters to exact `usize`, while
  keeping comparison payloads, report fields, reason vocabulary,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-150:
  landed by 294x-129. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionPilot`
  owner-local counters (`extension_count`, `accepted_count`, `blocked_count`,
  `missing_pilot_reject_count`, `blocked_pilot_reject_count`,
  `missing_extension_input_reject_count`, and
  `closed_stop_line_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-151`,
  while keeping comparison payloads, report mirrors, `last_reason`,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-151:
  landed by 294x-130. Migrated only the selected C mimalloc result
  presentation extension pilot owner-local counters to exact `usize`, while
  keeping comparison payloads, report fields, reason vocabulary,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-152:
  landed by 294x-131. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnPilot`
  owner-local counters (`follow_on_count`, `accepted_count`, `blocked_count`,
  `missing_pilot_reject_count`, `blocked_pilot_reject_count`,
  `missing_follow_on_input_reject_count`, and
  `closed_stop_line_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-153`,
  while keeping comparison payloads, report mirrors, `last_reason`,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-153:
  landed by 294x-132. Migrated only the selected C mimalloc result
  presentation extension follow-on pilot owner-local counters to exact
  `usize`, while keeping comparison payloads, report fields, reason vocabulary,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-154:
  landed by 294x-133. Selected the
  `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionPilot`
  owner-local counters (`follow_on_extension_count`, `accepted_count`,
  `blocked_count`, `missing_pilot_reject_count`,
  `blocked_pilot_reject_count`,
  `missing_follow_on_extension_input_reject_count`, and
  `closed_stop_line_reject_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-155`,
  while keeping comparison payloads, report mirrors, `last_reason`,
  performance/memory conclusions, repeated benchmark execution, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-155:
  landed by 294x-134. Migrated only the selected C mimalloc result
  presentation extension follow-on extension pilot owner-local counters to
  exact `usize`, while keeping comparison payloads, report fields, reason
  vocabulary, performance/memory conclusions, repeated benchmark execution,
  provider / hook / global-allocator rows, worker/TLS, threads, and
  `#[global_allocator]` out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-161:
  selection-only row for the next explicit non-negative stored field group.
  Keep decision/report fields, signed sentinels, route/state/status/reason
  vocabulary, comparison payloads, performance/memory conclusions, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope unless the selected group explicitly owns one of those seams.

  Next hint:
    unless a newer SSOT overrides it, select the MIMAP-528A
    `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnPilot`
    owner-local counters.
```

## Ladder

| Row | Status | Scope | Done When |
| --- | --- | --- | --- |
| `294x-00` | Complete | phase lock and full visible task inventory | SSOT, README, taskboard, current pointers are in place |
| `294x-01` | Complete | target-width and numeric-kind SSOT in code | target pointer width owner exists; `usize` no longer depends on ad hoc host assumptions |
| `294x-02` | Complete | parser metadata preservation | method, static method, and `birth` params keep declared type metadata; return annotations are preserved where accepted |
| `294x-03` | Complete | AST JSON / Program(JSON) numeric metadata | declared param/return type text round-trips through JSON metadata without changing runtime semantics |
| `294x-04` | Complete | MIR exact numeric type model | signedness/width/pointer-width are represented as side-car MIR metadata distinct from `MirType::Integer` |
| `294x-05` | Complete | exact numeric constants and conversions | constants and dynamic integer conversions range-check into exact numeric metadata |
| `294x-06` | Complete | verifier negative/range fail-fast | statically known exact numeric field writes reject negative and out-of-range values under the MIR verifier |
| `294x-06b` | Complete | dynamic numeric field write guard | runtime-range-sensitive exact numeric fields reject unchecked dynamic values until runtime-check lowering exists |
| `294x-06c` | Complete | runtime-check contract metadata | dynamic exact numeric field writes can be verifier-accepted only with a matching `DynamicIntegerRange` contract |
| `294x-06d` | Complete | VM dynamic range-check execution | the VM interpreter executes existing `DynamicIntegerRange` contracts at `FieldSet` sites and rejects bad dynamic values before mutation |
| `294x-06e` | Complete | dynamic range-check contract refresh | real MIR `FieldSet` producers receive `DynamicIntegerRange` contracts after optimization and before verification |
| `294x-06f` | Complete | backend runtime-check contract fail-fast | unsupported non-VM backend routes reject modules that still carry exact numeric runtime-check contracts |
| `294x-07` | Complete | overflow and checked arithmetic policy | exact numeric add/sub/mul policy is checked/fail-fast; wrapping stays explicit future vocabulary |
| `294x-08` | Complete | unsigned compare and logical shift | exact numeric compare and logical right-shift policy no longer borrow signed i64 semantics |
| `294x-09` | Complete | PHI/Select numeric unification policy | exact numeric facts merge conservatively and fail fast on exact/dynamic or exact/exact mismatches |
| `294x-09a` | Complete | VM reference-executor boundary | VM is a semantic reference executor, not the product/mainline backend owner |
| `294x-09b` | Complete | exact numeric value facts v0 | field reads, copies, and conservative control merges publish MIR-owned exact numeric value facts before VM reference execution |
| `294x-09c` | Complete | exact numeric signature facts v0 | declared params seed MIR-owned exact numeric value facts and declared returns publish function-level exact numeric facts |
| `294x-09d` | Complete | exact numeric add route facts v0 | exact `+` routes are MIR-owned facts before VM reference execution consumes them |
| `294x-09e` | Complete | dev gate quick profile split | daily quick stays slim while allocator-wide owns the full allocator/mimalloc/provider proof ladder |
| `294x-09f` | Complete | quick first-row cargo filter grouping | quick first-row guards group related cargo filters without changing semantic coverage |
| `294x-10` | Complete | VM reference exact `usize` Add route v0 | VM reference execution consumes MIR-owned exact numeric Add route facts without making VM-only behavior a completion criterion |
| `294x-10b` | Complete | VM reference checked arithmetic routes | VM reference execution consumes MIR-owned exact numeric Add/Sub/Mul route facts without VM-owned inference |
| `294x-10c` | Complete | VM reference exact compare routes | VM reference execution consumes MIR-owned exact numeric compare route facts without VM-owned inference |
| `294x-10d` | Complete | VM exact ops module split | exact numeric VM reference execution is split by operation family before more rows land |
| `294x-10e` | Complete | VM reference exact logical shr routes | VM reference execution consumes MIR-owned exact unsigned logical right-shift route facts |
| `294x-10f` | Complete | VM exact numeric runtime value | VM reference exact numeric arithmetic/shift results stay tagged instead of collapsing back to `Integer(i64)` |
| `294x-11` | Complete | literal suffix and const-eval row | `0usize` / exact numeric consts are accepted only with range checks and preserved as MIR exact const facts |
| `294x-12` | Complete | typed-object exact numeric storage | typed-object plans distinguish exact numeric storage names such as `usize` from legacy `i64` while runtime values stay on the integer lane |
| `294x-13` | Complete | backend capability and fail-fast | unsupported non-VM backends reject exact numeric storage/op routes before emission; native lowering remains a later row |
| `294x-14a` | Complete | byte-length usize facade aliases | RawBuf and OSVM byte-length facades expose `usize` names over the non-negative current-lane i64 subset |
| `294x-14` | Complete | low-level capability usize variants | Buf/RawArray/bounds/initialized-range helpers expose provisional `usize` aliases over the non-negative current-lane i64 subset; RawBuf stays byte-buffer only and OSVM byte-length aliases remain from 294x-14a |
| `294x-15` | Complete | raw-layout pointer-sized field row | `usize`/`isize` raw fields are accepted with target layout rules while source syntax/backend execution remain out of scope |
| `294x-16` | Complete | hako_alloc numeric field inventory | every numeric stored field is classified as signed sentinel, signed delta, count, size, capacity, index, or byte length |
| `294x-17` | Complete | sentinel split plan | direct-page stored `-1` sentinel is split into explicit presence state before any `usize` migration |
| `294x-18` | Complete | hako_alloc non-negative field migration probe | capacity/count/byte-length candidates migrate in a proof app while production fields stay signed/current-lane |
| `294x-19` | Blocked | hako_alloc production facade migration | waits for exact typed-object storage plus backend consumption of the exact field ABI |
| `294x-19a` | Complete | native exact numeric typed-object slots | kernel typed-object storage records exact slot kinds and legacy i64 helpers do not mutate exact numeric slots |
| `294x-19b` | Complete | exact numeric field get/set ABI | runtime helpers read/write exact signed/unsigned slots with range/overflow contracts |
| `294x-19c` | Complete | exact field ABI backend consumption | Python LLVM carries typed-object plans, registers exact layouts, creates exact typed-object handles, and lowers exact field get/set helpers |
| `294x-19d` | Complete | exact op backend subset | Python LLVM lowers exact add/sub/mul, compare, and logical-shift route facts with checked traps |
| `294x-19e` | Complete | hako_alloc production facade stats migration | facade-local event counters migrate to exact `usize`; page/heap/queue/handle fields remain `i64` |
| `294x-20` | Complete | mimalloc row resume gate | M167+ mimalloc implementation resumes with clear `usize` support boundaries while page/heap/queue state remains `i64` |

## Required Feature Checklist

### Spec

- [x] Define exact `usize` range owner by target pointer width.
- [x] Define overflow behavior.
- [x] Define logical shift behavior.
- [x] Define unsigned comparison behavior.
- [x] Define conversion from dynamic `Integer(i64)`.
- [x] Define unsupported backend fail-fast tags.
- [x] Define when `i64` remains preferred.

### Parser / AST / JSON

- [x] Preserve method parameter type annotations.
- [x] Preserve static method parameter type annotations.
- [x] Preserve `birth` parameter type annotations.
- [x] Preserve return type annotations or reject them consistently.
- [x] Round-trip declared numeric metadata through AST JSON / Program(JSON).
- [ ] Keep Rust and `.hako` parser fronts aligned.
  - Rust parser supports: literal suffixes (`0usize`), parameter type annotations,
    return type annotations, field type annotations with exact numeric types.
  - Stage-B `.hako` parser (`lang/src/compiler/parser/`) does not yet support:
    literal suffixes, parameter type annotations, return type annotations, or
    field type annotations with exact numeric types.
  - Next row: add literal suffix scanning to Stage-B number scanner, then
    parameter/return type annotation parsing. Separate commit per feature.

### MIR / Analysis

- [x] Add exact numeric MIR type representation.
- [x] Preserve signedness and width.
- [x] Preserve pointer-width target metadata owner.
- [x] Add exact numeric constants or constant metadata.
- [x] Add conversion/cast vocabulary.
- [x] Add PHI/Select unification rules.
- [x] Publish exact numeric value facts for field reads, copies, and control merges.
- [x] Publish route facts for numeric params and returns.
- [x] Publish exact numeric op route facts for first arithmetic producers.
- [x] Add checked exact numeric add/sub/mul policy helpers.
- [x] Add exact numeric compare and logical right-shift policy helpers.

### Runtime / VM

- [x] Add exact `usize` runtime representation or equivalent tagged numeric value.
- [x] Define VM as semantic reference executor, not product/mainline owner.
- [x] Execute existing `DynamicIntegerRange` contracts in the VM interpreter.
- [x] Attach `DynamicIntegerRange` contracts for real exact numeric field-write
  producers after MIR shape is stable.
- [x] Range-check literal construction before exact numeric const facts are published.
- [ ] Range-check construction beyond exact numeric field-write contracts and typed literals.
- [x] Implement checked add/sub/mul in live VM exact numeric op routes.
- [ ] Implement div/mod with zero checks.
- [ ] Implement bitwise ops.
- [x] Implement logical right shift in live VM exact numeric op routes.
- [x] Implement unsigned compare in live VM exact numeric op routes.
- [x] Define display/debug formatting.
- [x] Emit stable diagnostics for overflow/range/shift failures in VM reference routes.

### Verifier / Guards

- [x] Reject negative statically known field assignment to `usize`.
- [x] Reject `-1` sentinel field assignment to `usize` when statically known.
- [x] Reject unchecked dynamic field assignment when the exact numeric range
  does not cover all dynamic `Integer(i64)` values.
- [x] Publish `DynamicIntegerRange` runtime-check contract metadata for exact
  numeric field writes.
- [x] Execute `DynamicIntegerRange` contracts in the VM interpreter before
  field mutation.
- [x] Keep verifier and contract refresh on one shared exact numeric field-write
  facts owner.
- [x] Reject unsupported backend lowering.
- [x] Guard against silent fallback to `Integer(i64)` for exact numeric
  runtime-check contracts.
- [ ] Keep strict/dev checks before broad production acceptance.

### Storage / Backend

- [x] Add typed-object exact numeric storage names to layout plans.
- [x] Fail fast on unsupported backend routes before exact numeric typed-object
  storage or op-route facts silently use legacy `Integer(i64)` lowering.
- [x] Add backend/runtime native `usize` slots.
- [x] Add field get/set ABI for exact numeric slots.
- [x] Add backend lowering/capability-gate consumption for exact numeric field
  get/set ABI.
- [x] Lower Python LLVM exact add/sub/mul, unsigned compare, and logical-shift
  route facts.
- [ ] Add exact numeric div/mod/bitwise/wrapping backend vocabulary if needed.
- [ ] Decide WASM target behavior.
- [ ] Keep C ABI size_t mapping explicit.
- [x] Accept raw layout pointer-sized fields only through target-resolved
  layout rules.

### Low-Level Capability Surface

- [x] RawBuf byte-length `usize` allocation/reallocation facades over the
  non-negative current-lane i64 subset.
- [x] RawBuf length/capacity `usize` variants stay out of scope because
  RawBuf intentionally owns no len/cap policy.
- [x] RawArray length/capacity/index `usize` variants.
- [x] OSVM page size and byte-length `usize` facades over the non-negative
  current-lane i64 subset.
- [x] Bounds checks over `usize`.
- [ ] Atomic or TLS `usize` rows only if needed by allocator proofs.
- [ ] Existing `*_i64` helpers remain until call sites migrate.

### Hako Alloc / Mimalloc

- [x] Inventory every numeric hako_alloc stored field.
- [x] Split the direct-page stored sentinel and keep not-found return sentinels
  signed until their API shape changes.
- [x] Probe capacity/count/byte-length `usize` fields in an isolated hako_alloc
  proof app before production migration.
- [x] Probe stack-top `usize` decrement/increment paths with explicit
  underflow/overflow rejects in the isolated hako_alloc proof app.
- [x] Probe exact `usize` stack-top values as `ArrayBox.get/set` indexes in the
  isolated hako_alloc proof app.
- [x] Migrate production page-model stack-top/occupancy fields after the
  proof-only stack-top and ArrayBox-index probes.
- [x] Probe exact `usize` capacity bounds with current-lane signed loop/index
  values before production page capacity migration.
- [x] Migrate production page-model capacity/reserved fields after the
  proof-only capacity-bound probe.
- [x] Probe exact `usize` request-size / block-size comparison and
  accepted-request byte-sum accumulation before production page-model
  size/byte fields migrate.
- [x] Migrate production page-model `block_size` / `requested_bytes` fields
  after the proof-only request byte-sum probe.
- [x] Mark production `usize` field migration blocked on non-VM exact numeric
  storage, exact field ABI, and backend ABI consumption.
- [x] Update first production proof apps for the facade stats field group.
- [x] Migrate the first production non-negative field group after exact field
  ABI backend consumption and needed exact op backend subset are green.
- [ ] Migrate remaining production non-negative fields only by explicit
  field-group rows.
  - Current migrated candidate: `HakoAllocPageMap` counter fields (`entry_count`,
    `live_count`, `register_count`, `lookup_count`, `lookup_miss_count`,
    `unregister_count`, `reject_count`). All non-negative counts, no sentinels,
    owner-local to one box. Low-risk per NUMERIC_FIELDS.md.
  - Row: `294x-21-HAKO-ALLOC-USIZE-PAGE-MAP-COUNTERS.md`.
  - Proof: existing page-map proof app verifies behavior-preserving counters.
  - Guard: existing page-map guard checks exact `usize` typed-object storage.
  - Stop line: do not migrate page-map entry pointer/id fields in this group.
  - Follow-on migrated group: `HakoAllocPageMapReleaseSeam` event/reject
    counters (`page_register_count`, `release_count`, `unregister_count`,
    `lookup_miss_count`, `stale_page_count`, `page_release_reject_count`,
    `reject_count`) in
    `294x-22-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-COUNTERS.md`.
  - Stop line: keep `page_count` signed until the page-id/page-count
    comparison contract is split.
  - Follow-on migrated group: `HakoAllocPageMapReallocSameClassPath`
    event/reject counters (`same_class_count`, `grow_reject_count`,
    `lookup_miss_count`, `stale_page_count`, `released_block_count`,
    `reject_count`) in
    `294x-23-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-SAME-CLASS-COUNTERS.md`.
  - Stop line: keep `last_result_ptr` signed/pointer-shaped until pointer
    result handles are migrated by their own row.
  - Follow-on migrated group: `HakoAllocPageMapReallocAllocCopyReleasePath`
    fallback event/reject counters (`success_count`, `copy_count`,
    `same_class_reject_count`, `alloc_fail_count`, `lookup_miss_count`,
    `stale_page_count`, `released_block_count`, `reject_count`) in
    `294x-24-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-ALLOC-COPY-RELEASE-COUNTERS.md`.
  - Stop line: keep `next_ptr`, `last_result_ptr`, and `last_alloc_*`
    signed/pointer-shaped or sentinel-bearing until their own rows.
  - Follow-on migrated group: `HakoAllocPageMapReallocFailureContract`
    failure-matrix event/reject counters (`success_count`,
    `same_class_success_count`, `move_success_count`, `zero_reject_count`,
    `oversized_reject_count`, `alloc_fail_count`, `lookup_miss_count`,
    `stale_page_count`, `released_block_count`, `unexpected_reject_count`,
    `reject_count`) in
    `294x-25-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-FAILURE-CONTRACT-COUNTERS.md`.
  - Stop line: keep `last_result_ptr`, `last_failure_kind`, and
    `last_max_block_size` as signed/pointer/status/size observers.
  - Follow-on migrated group: `HakoAllocPageMapAlignedSmallPath`
    event/reject counters (`alloc_count`, `invalid_alignment_count`,
    `oversized_count`, `alloc_fail_count`, `register_fail_count`,
    `reject_count`) in
    `294x-26-HAKO-ALLOC-USIZE-ALIGNED-SMALL-PATH-COUNTERS.md`.
  - Stop line: keep `meta_count`, `next_ptr`, `last_result_ptr`,
    `last_alignment`, and `last_padded_size` signed until metadata-store,
    pointer, alignment, and size observer contracts are split.
  - Follow-on migrated group: `HakoAllocHugeThresholdRouter` route/reject
    counters (`small_route_count`, `small_success_count`,
    `small_reject_count`, `huge_route_count`, `huge_reject_count`,
    `invalid_alignment_count`, `invalid_size_count`, `reject_count`) in
    `294x-27-HAKO-ALLOC-USIZE-HUGE-THRESHOLD-ROUTER-COUNTERS.md`.
  - Stop line: keep route-kind, pointer, size, and threshold observer fields
    signed until their own exactness contracts are split.
  - Follow-on migrated group: `HakoAllocPageQueue` stats counters
    (`add_count`, `select_count`, `direct_hit_count`, `refresh_count`,
    `reject_count`) in `294x-28-HAKO-ALLOC-USIZE-PAGE-QUEUE-COUNTERS.md`.
  - Stop line: keep `bin`, `page_count`, `has_direct_page`, and
    `direct_page_index` signed until queue index/presence contracts are split.
  - Follow-on migrated group: `HakoAllocPageQueue.page_count` in
    `294x-48-HAKO-ALLOC-USIZE-PAGE-QUEUE-PAGE-COUNT.md`.
  - Stop line: keep `bin`, `has_direct_page`, and `direct_page_index` signed.
  - Follow-on migrated group: `HakoAllocPageQueue.direct_page_index` in
    `294x-49-HAKO-ALLOC-USIZE-PAGE-QUEUE-DIRECT-INDEX.md`.
  - Follow-on selected group: `HakoAllocPageQueue.bin` in
    `294x-70-HAKO-ALLOC-USIZE-PAGE-QUEUE-BIN-SELECTION.md`.
  - Follow-on migrated group: `HakoAllocPageQueue.bin` in
    `294x-71-HAKO-ALLOC-USIZE-PAGE-QUEUE-BIN.md`.
  - Stop line: keep `has_direct_page` signed; heap-level bin mirrors and
    size-class return shapes remain separate rows.
  - Follow-on migrated group: `HakoAllocPageModel` local page counters
    (`alloc_count`, `local_free_count`, `reject_count`) in
    `294x-29-HAKO-ALLOC-USIZE-PAGE-MODEL-LOCAL-COUNTERS.md`.
  - Stop line: keep page identity, size/capacity, stack-top, live-count,
    collection, lifecycle, and byte-length fields signed until their own
    contracts are split.
  - Follow-on migrated group: `HakoAllocPageModel` local-free collection
    counters (`local_free_collect_count`, `local_free_collected_blocks`) in
    `294x-30-HAKO-ALLOC-USIZE-PAGE-MODEL-COLLECTION-COUNTERS.md`.
  - Stop line: keep stack-top, live-count, lifecycle, and byte-length fields
    signed until their own contracts are split.
  - Follow-on migrated group: `HakoAllocPageModel` lifecycle event/reject
    counters (`retire_count`, `decommit_count`, `recommit_count`,
    `reuse_count`, `lifecycle_reject_count`, `reactivate_count`,
    `reactivate_reject_count`) in
    `294x-31-HAKO-ALLOC-USIZE-PAGE-MODEL-LIFECYCLE-COUNTERS.md`.
  - Stop line: keep `retired` / `decommitted` lifecycle state flags,
    stack-top/live-count, identity, size/capacity, and byte-length fields
    signed until their own contracts are split.
  - Follow-on migrated group: `HakoAllocAlignedSmallMetaStore.count` and
    `HakoAllocPageMapAlignedSmallPath.meta_count` in
    `294x-32-HAKO-ALLOC-USIZE-ALIGNED-SMALL-META-COUNT.md`.
  - Stop line: keep aligned-small pointer, alignment, and padded-size
    observers signed until their own contracts are split.
  - Follow-on migrated group: `HakoAllocHugePageMetaStore` metadata counters
    (`count`, `live_count`) in
    `294x-33-HAKO-ALLOC-USIZE-HUGE-META-STORE-COUNTERS.md`.
  - Stop line: keep huge-page pointer, id, requested-size, committed-size, and
    live-flag payload / observer fields signed until their own contracts are
    split.
  - Follow-on migrated group: `HakoAllocHugePageModel` metadata mirrors
    (`huge_count`, `live_count`) in
    `294x-34-HAKO-ALLOC-USIZE-HUGE-MODEL-META-MIRRORS.md`.
  - Stop line: keep huge-model event/reject counters, pointer/id/size/status
    observers, and facade report fields signed until their own rows.
  - Follow-on migrated group: `HakoAllocHugePageModel` event/reject counters
    (`allocate_count`, `release_count`, `release_reject_count`,
    `zero_reject_count`, `commit_reject_count`, `register_fail_count`,
    `reject_count`) in
    `294x-35-HAKO-ALLOC-USIZE-HUGE-MODEL-EVENT-COUNTERS.md`.
  - Follow-on selected group: `HakoAllocHugePageModel.next_page_id` in
    `294x-68-HAKO-ALLOC-USIZE-HUGE-MODEL-NEXT-PAGE-ID-SELECTION.md`.
  - Follow-on migrated group: `HakoAllocHugePageModel.next_page_id` in
    `294x-69-HAKO-ALLOC-USIZE-HUGE-MODEL-NEXT-PAGE-ID.md`.
  - Stop line: keep huge-model pointer/id/size/status observers and facade
    report fields signed until their own rows.
  - Follow-on migrated group: `HakoAllocHugeReleaseSeam` event/reject counters
    (`release_count`, `unregister_count`, `lookup_miss_count`,
    `not_huge_count`, `model_reject_count`, `reject_count`) in
    `294x-36-HAKO-ALLOC-USIZE-HUGE-RELEASE-SEAM-COUNTERS.md`.
  - Stop line: keep huge release seam sentinel/status observer fields signed
    until their own rows.
  - Follow-on migrated group: `HakoAllocFastPathHeap` event/reject counters
    (`alloc_count`, `release_count`, `fallback_count`, `page_create_count`,
    `reject_count`) in
    `294x-37-HAKO-ALLOC-USIZE-FAST-PATH-HEAP-COUNTERS.md`.
  - Follow-on migrated group: `HakoAllocFastPathHeap` size/capacity metadata
    (`block_size`, `page_capacity`) in
    `294x-50-HAKO-ALLOC-USIZE-FAST-PATH-HEAP-SIZE-CAPACITY.md`.
  - Stop line: keep fast-path route/index metadata and handle id/size fields
    signed until their own rows.
  - Follow-on migrated group: `HakoAllocFastPathHandle.requested_size` in
    `294x-51-HAKO-ALLOC-USIZE-FAST-PATH-HANDLE-REQUESTED-SIZE.md`.
  - Stop line: keep fast-path handle page/block id fields signed until
    id/index contracts are split.
  - Follow-on migrated group: `HakoAllocOsVmBackedFastPathHeap` event/source
    counters (`alloc_count`, `release_count`, `fallback_count`,
    `page_create_count`, `reject_count`, `reserve_count`, `commit_count`,
    `decommit_count`, `source_reject_count`) in
    `294x-38-HAKO-ALLOC-USIZE-OSVM-BACKED-FAST-PATH-COUNTERS.md`.
  - Stop line: keep OSVM-backed route/index/size/capacity metadata,
    `backing_count`, backing payloads, and handle payloads signed until their
    own rows.
  - Follow-on migrated group: `HakoAllocOsVmBackedHandle.requested_size` in
    `294x-52-HAKO-ALLOC-USIZE-OSVM-BACKED-HANDLE-REQUESTED-SIZE.md`.
  - Stop line: keep OSVM-backed page/block id fields, backing payloads,
    size/capacity metadata, and OSVM byte-length seams signed until their own
    rows.
  - Follow-on migrated group: `HakoAllocOsVmBackedFastPathHeap` size/capacity
    metadata (`block_size`, `page_capacity`) plus
    `HakoAllocOsVmPageBacking.bytes` and the page-source policy byte-length
    params in
    `294x-54-HAKO-ALLOC-USIZE-OSVM-BACKED-BYTE-LENGTH-SEAM.md`.
  - Follow-on migrated group: `HakoAllocOsVmBackedFastPathHeap.backing_count`
    in
    `294x-63-HAKO-ALLOC-USIZE-OSVM-BACKING-COUNT-ID-SEAM.md`.
  - Stop line: keep OSVM-backed `bin`, `next_page_id`, backing `page_id` /
    `base`, and handle page/block ids signed.
  - Follow-on migrated group: `HakoAllocPageMapReleaseSeam.page_count` in
    `294x-65-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-PAGE-COUNT.md`.
  - Stop line: keep page-map entry ids, block ids, pointer-like fields, and
    binary flags signed until their own rows.
  - Follow-on selected group: `HakoAllocFastPathHeap.next_page_id` in
    `294x-66-HAKO-ALLOC-USIZE-FAST-PATH-NEXT-PAGE-ID-SELECTION.md`.
  - Follow-on migrated group: `HakoAllocFastPathHeap.next_page_id` in
    `294x-67-HAKO-ALLOC-USIZE-FAST-PATH-NEXT-PAGE-ID.md`.
  - Stop line: keep fast-path `bin`, handle page/block ids, and OSVM-backed
    `next_page_id` signed until their own rows.
  - Follow-on migrated group: `HakoAllocSecureFreeListDiagnostics` diagnostic
    counters (`scan_count`, `ok_count`, `fail_count`,
    `out_of_range_free_block_count`, `duplicate_free_block_count`,
    `live_block_in_free_list_count`, `free_count_mismatch_count`,
    `local_free_count_mismatch_count`) in
    `294x-39-HAKO-ALLOC-USIZE-SECURE-LIST-DIAGNOSTIC-COUNTERS.md`.
  - Stop line: keep secure-list `last_*` observation flags signed until bool /
    flag semantics are split.
  - Follow-on migrated group: `HakoAllocPageMapReleaseObserver` observer
    counters (`observe_count`, `success_count`, `reject_count`) in
    `294x-40-HAKO-ALLOC-USIZE-PAGE-MAP-RELEASE-OBSERVER-COUNTERS.md`.
  - Stop line: keep release observer before-snapshots, sentinels, statuses, and
    signed deltas as `i64`.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` stack-top fields
    (`free_top`, `local_free_top`) and reject counters
    (`free_top_underflow_reject_count`, `local_free_overflow_reject_count`,
    `local_free_underflow_reject_count`) in
    `294x-41-HAKO-ALLOC-USIZE-STACK-TOP-PROBE.md`.
  - Stop line: production page stack-top, live-count, capacity, byte-length,
    and remote-free mailbox fields remain signed until their owner-local rows.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` exact `usize` stack-top
    values used as `ArrayBox.get/set` indexes in
    `294x-42-HAKO-ALLOC-USIZE-STACK-ARRAY-INDEX-PROBE.md`.
  - Stop line: production page stack fields still do not migrate in this row.
  - Follow-on migrated group: `HakoAllocPageModel` stack-top and occupancy
    fields (`used`, `free_top`, `local_free_top`, `peak_used`) in
    `294x-43-HAKO-ALLOC-USIZE-PAGE-MODEL-STACK-OCCUPANCY.md`.
  - Stop line: keep page identity, block size, capacity, reserved count,
    lifecycle state flags, byte-length fields, queue indexes, and remote-free
    mailbox fields signed until their own owner-local rows.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` exact `usize` capacity
    bound checks with signed loop/index values in
    `294x-44-HAKO-ALLOC-USIZE-CAPACITY-BOUND-PROBE.md`.
  - Stop line: production page capacity/reserved fields still do not migrate
    in this row.
  - Follow-on migrated group: `HakoAllocPageModel` capacity fields
    (`capacity`, `reserved`) in
    `294x-45-HAKO-ALLOC-USIZE-PAGE-MODEL-CAPACITY.md`.
  - Stop line: keep page identity, block size, lifecycle state flags,
    byte-length fields, queue indexes, and remote-free mailbox fields signed.
  - Follow-on probe row: `HakoAllocUsizeFieldProbe` request-size /
    block-size compare and accepted-request byte-sum accumulation in
    `294x-46-HAKO-ALLOC-USIZE-REQUEST-BYTE-SUM-PROBE.md`.
  - Stop line: production page `block_size` and `requested_bytes` still do not
    migrate in this row.
  - Follow-on migrated group: `HakoAllocPageModel` size/byte fields
    (`block_size`, `requested_bytes`) in
    `294x-47-HAKO-ALLOC-USIZE-PAGE-MODEL-SIZE-BYTES.md`.
  - Stop line: keep page identity, lifecycle state flags, queue indexes, and
    remote-free mailbox fields signed.
- [ ] Keep allocator-provider activation out of scope.
- [x] Resume M167+ mimalloc algorithm rows only after the resume gate.
- [x] Land M168 OSVM page-source composition without new native leaves.
- [x] Land M169 local-free collection and retire observation.
- [x] Land M170 remote-free integration through existing pointer atomics only.
- [x] Land M171 page-map model owner.
- [x] Land M172 page-map-backed release seam before scheduling realloc/aligned/page-map/huge-page rows.
- [x] Land M173 pre-realloc release invariant freeze before the realloc body.
- [x] Land M174 realloc same-class/no-move path before alloc-copy-release fallback.
- [x] Land M175 realloc alloc-copy-release fallback before the negative matrix.
- [x] Land M176 realloc negative matrix / failure contract before aligned allocation work.
- [x] Land M177 alignment policy object before aligned execution.
- [x] Land M178 aligned allocation small path before huge routing.

## Open Design Questions

- Decision: VM exact `usize` uses a tagged exact numeric payload shared by all
  exact integer widths.
- Should plain typed arithmetic always checked-fail-fast, or should release
  rows later opt into wrapping with explicit intrinsics?
- Does Program(JSON v0) carry param/return metadata directly, or does phase
  294x introduce a side table to avoid broad schema churn?
- Is the first accepted target 64-bit only, with 32-bit targets fail-fast, or
  should both widths be modeled from the start?
- Which hako_alloc fields can migrate before low-level helper APIs grow
  `usize` variants?
