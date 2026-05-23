---
Status: Landed
Date: 2026-05-24
Scope: C mimalloc result presentation-only extension pilot comparison count and byte payload exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-152-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-COUNT-AND-BYTE-PAYLOAD-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
---

# 294x-153 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Count And Byte Payloads

## Decision

Migrate only the selected non-negative comparison count and byte payload
fields in `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport`
to exact `usize` storage:

- `allocation_count`
- `free_count`
- `requested_bytes`
- `peak_rss_bytes`
- `steady_rss_bytes`
- `hako_allocation_count`
- `hako_requested_bytes`
- `c_allocation_count`
- `c_requested_bytes`
- `c_peak_rss_bytes`

The MIMAP-560A presentation-only extension pilot guard now asserts these
fields are exact `usize` in the typed-object plan while the delta payloads,
reason vocabulary, status flags, and comparison conclusion seams remain
signed.

## Stop Line

This row does not migrate:

- `allocation_count_delta` or `requested_bytes_delta`, because they are delta
  comparison evidence and stay signed until their own row;
- `last_reason`, because it is reason vocabulary;
- `allocator_id`, `runner_kind`, `workload_id`, `exit_code`,
  `evidence_complete`, performance conclusions, memory conclusions, repeated
  benchmark execution, process allocator replacement, hooks, backend matcher
  additions, provider package generation, worker/TLS, threads, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
