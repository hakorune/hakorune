---
Status: Landed
Date: 2026-05-24
Scope: MIMAP-560A presentation-only extension pilot report metadata and evidence-status exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-154-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-REPORT-METADATA-AND-EVIDENCE-STATUS-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
---

# 294x-155 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Report Metadata And Evidence-Status Fields

## Decision

Migrate only the selected report metadata and evidence-status fields in
`HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport`
to exact `usize` storage:

- `allocator_id`
- `runner_kind`
- `workload_id`
- `exit_code`
- `evidence_complete`

The MIMAP-560A presentation-only extension pilot guard now asserts these
fields are exact `usize` in the typed-object plan while the comparison count
and byte payload fields, delta payloads, reason vocabulary, status flags, and
later performance/memory conclusion evidence seams remain signed.

## Stop Line

This row does not migrate:

- `allocation_count`, `free_count`, `requested_bytes`, `peak_rss_bytes`,
  `steady_rss_bytes`, `hako_allocation_count`, `hako_requested_bytes`,
  `c_allocation_count`, `c_requested_bytes`, or `c_peak_rss_bytes`, because
  they are the comparison count/byte payload row and already have their own
  `usize` lane;
- `allocation_count_delta` or `requested_bytes_delta`, because they are delta
  comparison evidence and stay signed until their own row;
- `last_reason`, because it is reason vocabulary;
- `presentation_only_extension_present`, `comparison_ready_present`,
  `pilot_present`, `accepted_pilot_present`, `blocked_pilot_present`,
  `accepted_input_pack_present`, `blocked_input_pack_present`,
  `hako_alloc_report_contract_present`, `c_mimalloc_runner_contract_present`,
  `shared_workload_id_present`, `memory_evidence_fields_present`,
  `comparison_preconditions_present`, performance/memory conclusions,
  repeated-benchmark execution, process replacement execution, hooks,
  backend matcher additions, provider package generation, worker/TLS, threads,
  or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
