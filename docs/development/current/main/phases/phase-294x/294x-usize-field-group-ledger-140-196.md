---
Status: Active
Date: 2026-05-24
Scope: landed `usize` field-group blocker ledger, field-group 140 through current provider-facing rows.
Related:
  - docs/development/current/main/phases/phase-294x/294x-usize-field-group-ledger.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x Usize Field Group Ledger 140-196

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
HAKO-ALLOC-USIZE-FIELD-GROUP-164:
  selection-only row for the next explicit non-negative stored field group.
  Keep decision/report fields, signed sentinels, route/state/status/reason
  vocabulary, comparison payloads, performance/memory conclusions, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope unless the selected group explicitly owns one of those seams.

  Next hint:
    unless a newer SSOT overrides it, select the MIMAP-560A
    `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
    owner-local counters.
```

HAKO-ALLOC-USIZE-FIELD-GROUP-165:
  selection-only row for the next explicit non-negative stored field group.
  Keep decision/report fields, signed sentinels, route/state/status/reason
  vocabulary, comparison payloads, performance/memory conclusions, provider /
  hook / global-allocator rows, worker/TLS, threads, and `#[global_allocator]`
  out of scope unless the selected group explicitly owns one of those seams.

  Next hint:
    unless a newer SSOT overrides it, select the MIMAP-560A
    `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
    comparison count and byte payload fields.

HAKO-ALLOC-USIZE-FIELD-GROUP-166:
  selection-only row for the next explicit non-negative stored field group.
  Keep decision/report fields, signed sentinels, route/state/status/reason
  vocabulary, comparison payload deltas, performance/memory conclusions,
  provider / hook / global-allocator rows, worker/TLS, threads, and
  `#[global_allocator]` out of scope unless the selected group explicitly owns
  one of those seams.

  Next hint:
    unless a newer SSOT overrides it, select the MIMAP-560A
    `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
    report metadata and evidence-status fields.

HAKO-ALLOC-USIZE-FIELD-GROUP-167:
  selection-only row for the next explicit non-negative stored field group.
  Keep decision/report fields, signed sentinels, route/state/status/reason
  vocabulary, comparison payload deltas, provider / hook / global-allocator
  rows, worker/TLS, threads, and `#[global_allocator]` out of scope unless
  the selected group explicitly owns one of those seams.

  Next hint:
    unless a newer SSOT overrides it, select the MIMAP-560A
    `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
    performance conclusion, memory conclusion, repeated-benchmark, and
    process-replacement evidence fields.

HAKO-ALLOC-USIZE-FIELD-GROUP-168:
  selection-only row for the next explicit non-negative stored field group.
  Keep decision/report fields, signed sentinels, route/state/status/reason
  vocabulary, comparison payload deltas, performance/memory conclusions,
  repeated-benchmark / process-replacement rows, worker/TLS, threads, and
  `#[global_allocator]` out of scope unless the selected group explicitly owns
  one of those seams.

  Next hint:
    unless a newer SSOT overrides it, close out the MIMAP-560A
    `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
    presentation-only extension pilot after the later
    hook_installed, backend_matcher_added, global_allocator_installed,
    hidden_discovery_used, provider_package_generated,
    would_replace_host_allocator, would_install_hook,
    would_add_backend_matcher, and would_run_thread evidence flags land.

  Closeout follow-on:
    `MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-001`

Current blocker:

```text
MIMALLOC-COMPARISON-MEMORY-REPORT-001:
  landed by 294x-161. Added a normalized memory comparison report that
  consumes the existing hako EXE memory evidence and explicit C mimalloc
  runner evidence, keeps hako/C workloads separately visible, and leaves
  provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, and winner claims closed.
```

Current blocker:

```text
MIMALLOC-COMPARISON-MEMORY-REPORT-CLOSEOUT-001:
  selected current. Close out the normalized memory report, then choose either
  a same-workload comparison pack or an explicit return to the next `usize`
  field-group row.
```

Current blocker:

```text
MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-001:
  landed by 294x-162. Added a hako-side representative small-block proof app
  that mirrors the explicit C mimalloc runner request sequence, allowing the
  normalized memory report to publish `workload_match=1` and
  `requested_bytes_delta=0` while provider activation, host replacement, hooks,
  global allocator install, worker/TLS, atomics, and winner claims remain
  closed.
```

Current blocker:

```text
MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT-001:
  landed by 294x-163. Closed the same-workload memory report pack after
  `workload_match=1`, `requested_bytes_delta=0`, and positive single-run RSS
  evidence landed on both sides, while winner claims and repeated-run
  statistics remain closed.
```

Current blocker:

```text
MIMALLOC-COMPARISON-RSS-PRESENTATION-001:
  landed by 294x-164. Formatted the existing same-workload single-run RSS
  evidence into `mimalloc-comparison-rss-presentation-v0`, including byte
  fields and MiB display helpers, without adding repeated-run aggregation,
  provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, or winner claims.
```

Current blocker:

```text
MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT-001:
  landed by 294x-165. Closed the presentation-only RSS report after
  `mimalloc-comparison-rss-presentation-v0` stabilized single-run byte and
  MiB display fields while repeated-run aggregation, winner claims, provider
  activation, host replacement, hooks, global allocator install, worker/TLS,
  atomics, and provider package / DLL generation remain closed.
```

Current blocker:

```text
MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-001:
  landed by 294x-166. Added
  `mimalloc-comparison-repeated-run-evidence-v0` over same-workload RSS
  presentation samples, publishing sample count and min/max RSS ranges while
  winner claims, provider activation, host replacement, hooks, global allocator
  install, worker/TLS, atomics, and provider package / DLL generation remain
  closed.
```

Current blocker:

```text
MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT-001:
  landed by 294x-167. Closed the repeated-run RSS evidence row after sample
  count and min/max RSS ranges landed, while winner claims, provider
  activation, host replacement, hooks, global allocator install, worker/TLS,
  atomics, and provider package / DLL generation remain closed.
```

Current blocker:

```text
MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-001:
  landed by 294x-168. Added
  `mimalloc-comparison-summary-no-winner-v0` over repeated-run RSS evidence,
  exposing workload, sample count, RSS ranges, and closed seams without
  claiming a performance or memory-use winner.
```

Current blocker:

```text
MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-CLOSEOUT-001:
  landed by 294x-169. Closed the no-winner comparison summary slice after
  repeated-run RSS evidence and range-only summary formatting landed. Winner
  claims, statistical significance claims, provider activation, host
  replacement, hooks, global allocator install, worker/TLS, atomics, and
  provider package / DLL generation remain closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-169:
  landed by 294x-170. Selected
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport` report mirror
  counters (`fallback_attempt_count`, `source_success_count`,
  `source_failure_count`, `retry_success_count`, `retry_failure_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-170`, while keeping alloc-miss
  status/source/final/id/base/byte mirrors and comparison/provider seams out of
  scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-170:
  landed by 294x-171. Migrated only the selected alloc-miss report mirror
  counters to exact `usize`, while status, reason, bool-like, source/final, id,
  pointer, and byte payload fields remain signed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-171:
  landed by 294x-172. Selected
  `HakoAllocObjectLifecycleFacadePageSourceAttachReport` mirror counters
  (`source_reserved`, `source_committed`, `facade_page_count`,
  `source_reject`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-172`, while keeping status,
  added page id, pointer-like base, byte/page payload mirrors, alloc-miss
  source/final mirrors, and provider/comparison seams out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-172:
  landed by 294x-173. Migrated only the selected page-source attach report
  mirror counters to exact `usize`, while status, added page id, base, bytes,
  block-size, capacity, reserved, alloc-miss source/final mirrors, and provider
  seams remain signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-173:
  landed by 294x-174. Selected
  `HakoAllocObjectLifecycleFacadePageSourceAttachReport` payload fields
  (`bytes`, `block_size`, `capacity`, `reserved`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-174`, while keeping status, added page id,
  pointer-like base, alloc-miss source/final mirrors, and provider/comparison
  seams out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-174:
  landed by 294x-175. Migrated only the selected page-source attach report
  payload fields to exact `usize`, while status, added page id, base,
  alloc-miss source/final mirrors, and provider seams remain signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-175:
  landed by 294x-176. Selected the alloc-miss report source count mirrors
  (`source_reserved`, `source_committed`, `source_reject`,
  `source_facade_page_count`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-176`, while
  source status, added page id, pointer-like base, byte-length mirror, retry
  and final status/reason/id payloads, and provider seams remain signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-176:
  landed by 294x-177. Migrated only the selected alloc-miss source count
  mirrors to exact `usize`, while source status, source added page id,
  pointer-like base, byte-length mirror, retry/final status and reason, and
  page/block id payloads remain signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-177:
  landed by 294x-178. Selected the alloc-miss report source byte-length mirror
  (`source_bytes`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-178`, while source status,
  source added page id, pointer-like base, retry/final status and reason, and
  page/block id payloads remain signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-178:
  landed by 294x-179. Migrated only the selected alloc-miss source byte-length
  mirror to exact `usize`, while source status, source added page id,
  pointer-like base, retry/final status and reason, and page/block id payloads
  remain signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-179:
  landed by 294x-180. Selected `HakoAllocHugeThresholdRouter`
  size observers (`last_padded_size`, `last_good_size`,
  `last_huge_threshold`) as `HAKO-ALLOC-USIZE-FIELD-GROUP-180`, while
  route-kind and pointer observers, aligned-small path observers, huge model /
  release / unreserve / unregister / decommit rows, and provider seams remain
  signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-180:
  landed by 294x-181 as deferred. Direct router observer migration passed the
  narrow VM/MIR huge-threshold route guard, but failed the downstream
  pure-first huge/OSVM comparison EXE path. Keep router size observers selected
  but deferred; select aligned-small padded-size as the next dependency row.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-181:
  landed by 294x-182. Migrated only
  `HakoAllocPageMapAlignedSmallPath.last_padded_size` to exact `usize`, while
  huge-threshold router observers, pointer-shaped fields, alignment observer,
  metadata store payloads, and provider seams remain signed/closed.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-182:
  landed by 294x-184. Migrated the safe huge-threshold router size observers
  `last_padded_size` and `last_huge_threshold` to exact `usize`; kept
  `last_good_size` signed because the huge path can store the
  `SizeClassBox.good_size(...) == -1` sentinel there.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-183:
  landed by 294x-185. Selected `HakoAllocHugePageModel`
  `last_requested_size` and `last_committed_size` as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-184`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-184:
  landed by 294x-186. Migrated only the selected huge page model size observers
  to exact `usize`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-185:
  landed by 294x-187. Selected page-source adapter `last_bytes` observers as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-186`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-186:
  landed by 294x-188. Migrated only the selected page-source adapter byte-length
  observers to exact `usize`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-187:
  landed by 294x-189. Selected provider selection inventory owner-local
  counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-188`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-188:
  landed by 294x-190. Migrated only the selected provider selection inventory
  owner-local counters to exact `usize`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-189:
  landed by 294x-191. Selected provider activation dry-run unsupported behavior
  owner-local counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-190`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-190:
  landed by 294x-192. Migrated only the selected provider activation dry-run
  unsupported behavior owner-local counters to exact `usize`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-191:
  landed by 294x-193. Selected the owner-local
  `HakoAllocProviderActivationInputBundleInventory` counters as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-192`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-192:
  landed by 294x-194. Migrated only the selected provider activation input
  bundle inventory owner-local counters to exact `usize`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-193:
  landed by 294x-195. Selected the owner-local
  `HakoAllocProviderActivationModeledOpenPilot` counters as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-194`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-194:
  landed by 294x-196. Migrated only the selected provider activation
  modeled-open pilot owner-local counters to exact `usize`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-195:
  landed by 294x-197. Selected the owner-local
  `HakoAllocProviderCallCapabilityGateInventory` counters as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-196`.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-196:
  landed by 294x-198. Migrated only the selected provider call capability gate
  inventory owner-local counters to exact `usize`.
```

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

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-203:
  landed by 294x-205. Selected the owner-local
  `HakoAllocProviderCallNoopExecutionSeamPilot` counters (`seam_count`,
  `accepted_count`, `reject_count`, `missing_preflight_reject_count`,
  `rejected_preflight_reject_count`, `not_ready_reject_count`,
  `closed_execution_reject_count`, `closed_host_replacement_reject_count`,
  `closed_hook_reject_count`, and `closed_backend_matcher_reject_count`) as
  `HAKO-ALLOC-USIZE-FIELD-GROUP-204`. Keep `last_reason`, report mirrors,
  no-op/open/executed flags, provider API call flags, bool-like readiness /
  would-execute flags, and provider call / hook / replacement seams separate.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-204:
  landed by 294x-206. Migrated only those selected provider-call no-op
  execution seam owner-local counters to exact `usize`; kept `last_reason`,
  report mirrors, no-op/open/executed flags, provider API call flags,
  bool-like readiness / would-execute flags, provider calls, host replacement,
  hooks, global allocator install, worker/TLS, atomics, provider package / DLL
  generation, and #[global_allocator] out of scope.
```

Current blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-205:
  selected current. Select the next explicit non-negative production field
  group. Do not migrate status/reason vocabularies, bool-like flags, signed
  sentinels, pointer-like payloads, provider calls, host replacement, hooks,
  global allocator install, worker/TLS, atomics, provider package / DLL
  generation, or #[global_allocator].
```
