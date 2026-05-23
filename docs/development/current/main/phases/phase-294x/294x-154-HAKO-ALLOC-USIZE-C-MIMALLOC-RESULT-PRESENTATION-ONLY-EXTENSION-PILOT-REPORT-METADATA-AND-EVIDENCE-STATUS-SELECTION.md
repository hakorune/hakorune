---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative report metadata and evidence-status field group after the comparison count / byte payload row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-153-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-COUNT-AND-BYTE-PAYLOADS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
---

# 294x-154 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Report Metadata And Evidence-Status Selection

## Decision

Select the report metadata and evidence-status fields in
`HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport`
as `HAKO-ALLOC-USIZE-FIELD-GROUP-166`:

- `allocator_id`
- `runner_kind`
- `workload_id`
- `exit_code`
- `evidence_complete`

These fields carry the shared comparison identity and completion/evidence
status for the MIMAP-560A presentation-only extension pilot report. They do
not carry comparison count/byte payloads, deltas, reason vocabulary, or the
later performance/memory conclusion evidence flags.

## Stop Line

This selection does not migrate:

- `allocation_count`, `free_count`, `requested_bytes`, `peak_rss_bytes`,
  `steady_rss_bytes`, `hako_allocation_count`, `hako_requested_bytes`,
  `c_allocation_count`, `c_requested_bytes`, or `c_peak_rss_bytes`, because
  those are the comparison payload row and stay on `HAKO-ALLOC-USIZE-FIELD-GROUP-165`;
- `allocation_count_delta` or `requested_bytes_delta`, because they are delta
  comparison evidence and stay signed until their own row;
- `last_reason`, because it is reason vocabulary;
- `performance_conclusion_made`, `memory_conclusion_made`,
  `repeated_benchmark_executed`, `process_replacement_executed`,
  `hook_installed`, `backend_matcher_added`, `global_allocator_installed`,
  `hidden_discovery_used`, `provider_package_generated`,
  `would_replace_host_allocator`, `would_install_hook`,
  `would_add_backend_matcher`, or `would_run_thread`, because those remain a
  later evidence-flag seam.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next Selection Hint

Unless a newer SSOT overrides this card, `HAKO-ALLOC-USIZE-FIELD-GROUP-167`
should select the MIMAP-560A presentation-only extension pilot performance
conclusion, memory conclusion, repeated-benchmark, and process-replacement
evidence fields.
