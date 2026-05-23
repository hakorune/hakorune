---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative evidence field group after the performance / repeated-evidence row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-157-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-PERFORMANCE-CONCLUSION-MEMORY-CONCLUSION-REPEATED-BENCHMARK-PROCESS-REPLACEMENT-EVIDENCE-FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
---

# 294x-158 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Later Hook Provider Global Allocator Evidence Selection

## Decision

Select the later hook/provider/global-allocator evidence flags in
`HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
as `HAKO-ALLOC-USIZE-FIELD-GROUP-168`:

- `hook_installed`
- `backend_matcher_added`
- `global_allocator_installed`
- `hidden_discovery_used`
- `provider_package_generated`
- `would_replace_host_allocator`
- `would_install_hook`
- `would_add_backend_matcher`
- `would_run_thread`

These fields carry the post-comparison evidence seam for the MIMAP-560A
presentation-only extension pilot. They do not change the earlier decision /
report fields, comparison payloads, or conclusion counters.

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
- `performance_conclusion_made`, `memory_conclusion_made`,
  `repeated_benchmark_executed`, or `process_replacement_executed`, because
  those remain on `HAKO-ALLOC-USIZE-FIELD-GROUP-167`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next Hint

Unless a newer SSOT overrides this card, the MIMAP-560A presentation-only
extension pilot should close out after this evidence seam lands.
