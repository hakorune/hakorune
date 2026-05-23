---
Status: Landed
Date: 2026-05-24
Scope: MIMAP-560A presentation-only extension pilot later hook/provider/global-allocator evidence exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-158-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-LATER-HOOK-PROVIDER-GLOBAL-ALLOCATOR-EVIDENCE-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
---

# 294x-159 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Later Hook Provider Global Allocator Evidence Fields

## Decision

Migrate only the selected later hook/provider/global-allocator evidence flags
in `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
to exact `usize` storage:

- `hook_installed`
- `backend_matcher_added`
- `global_allocator_installed`
- `hidden_discovery_used`
- `provider_package_generated`
- `would_replace_host_allocator`
- `would_install_hook`
- `would_add_backend_matcher`
- `would_run_thread`

The MIMAP-560A presentation-only extension pilot guard now asserts these
fields are exact `usize` in the typed-object plan while the earlier report
metadata, comparison payloads, delta payloads, reason vocabulary, status
flags, and conclusion counters remain on their own seams.

## Stop Line

This row does not migrate:

- `presentation_count`, `accepted_count`, `blocked_count`,
  `missing_pilot_reject_count`, `blocked_pilot_reject_count`,
  `missing_presentation_input_reject_count`, or `closed_stop_line_reject_count`,
  because they are the owner-local counter row and already have their own
  `usize` lane;
- `allocator_id`, `runner_kind`, `workload_id`, `allocation_count`,
  `free_count`, `requested_bytes`, `peak_rss_bytes`, `steady_rss_bytes`,
  `exit_code`, `evidence_complete`, `hako_allocation_count`,
  `hako_requested_bytes`, `c_allocation_count`, `c_requested_bytes`, or
  `c_peak_rss_bytes`, because they are earlier report payload rows with their
  own `usize` lanes;
- `allocation_count_delta` or `requested_bytes_delta`, because they are delta
  comparison evidence and stay signed until their own row;
- `last_reason`, because it is reason vocabulary;
- `presentation_only_extension_present`, `comparison_ready_present`,
  `pilot_present`, `accepted_pilot_present`, `blocked_pilot_present`,
  `accepted_input_pack_present`, `blocked_input_pack_present`,
  `hako_alloc_report_contract_present`, `c_mimalloc_runner_contract_present`,
  `shared_workload_id_present`, `memory_evidence_fields_present`,
  `comparison_preconditions_present`, or the earlier performance / memory /
  repeated-benchmark / process-replacement evidence counters.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next Hint

Unless a newer SSOT overrides this card, the MIMAP-560A presentation-only
extension pilot should close out after this evidence seam lands.
