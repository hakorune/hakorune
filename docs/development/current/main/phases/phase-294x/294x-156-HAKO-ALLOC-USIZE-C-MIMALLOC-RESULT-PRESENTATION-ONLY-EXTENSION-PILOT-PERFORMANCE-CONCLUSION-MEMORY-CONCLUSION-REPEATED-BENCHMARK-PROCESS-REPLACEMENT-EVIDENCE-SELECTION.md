---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative evidence field group after the report metadata and evidence-status row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-155-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-REPORT-METADATA-AND-EVIDENCE-STATUS-FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
---

# 294x-156 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Performance Conclusion Memory Conclusion Repeated Benchmark Process Replacement Evidence Selection

## Decision

Select the performance and evidence fields in
`HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
as `HAKO-ALLOC-USIZE-FIELD-GROUP-167`:

- `performance_conclusion_made`
- `memory_conclusion_made`
- `repeated_benchmark_executed`
- `process_replacement_executed`

These fields carry the post-comparison conclusion and repeated-evidence
signals for the MIMAP-560A presentation-only extension pilot. They do not
carry the later hook/provider/global-allocator evidence flags.

## Stop Line

This selection does not migrate:

- `presentation_count`, `accepted_count`, `blocked_count`,
  `missing_pilot_reject_count`, `blocked_pilot_reject_count`,
  `missing_presentation_input_reject_count`, or `closed_stop_line_reject_count`,
  because those are the owner-local counters row and stay on
  `HAKO-ALLOC-USIZE-FIELD-GROUP-164`;
- `allocator_id`, `runner_kind`, `workload_id`, `allocation_count`,
  `free_count`, `requested_bytes`, `peak_rss_bytes`, `steady_rss_bytes`,
  `exit_code`, `evidence_complete`, `hako_allocation_count`,
  `hako_requested_bytes`, `c_allocation_count`, `c_requested_bytes`, or
  `c_peak_rss_bytes`, because those are earlier report payload rows and stay
  on their own field groups;
- `allocation_count_delta` or `requested_bytes_delta`, because they are delta
  comparison evidence and stay signed until their own row;
- `last_reason`, because it is reason vocabulary;
- `hook_installed`, `backend_matcher_added`, `global_allocator_installed`,
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

Unless a newer SSOT overrides this card, `HAKO-ALLOC-USIZE-FIELD-GROUP-168`
should select the MIMAP-560A presentation-only extension pilot later
hook/provider/global-allocator evidence flags.

