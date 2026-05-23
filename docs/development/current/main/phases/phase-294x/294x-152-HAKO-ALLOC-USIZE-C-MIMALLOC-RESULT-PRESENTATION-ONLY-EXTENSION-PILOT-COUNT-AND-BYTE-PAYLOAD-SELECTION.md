---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative comparison count and byte report payload field group after the MIMAP-560A presentation-only extension pilot counters.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-151-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-ONLY-EXTENSION-PILOT-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh
---

# 294x-152 Hako Alloc Usize C Mimalloc Result Presentation-Only Extension Pilot Count And Byte Payload Selection

## Decision

Select the non-negative comparison count and byte payload fields in
`HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport`
as `HAKO-ALLOC-USIZE-FIELD-GROUP-165`:

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

These fields count the MIMAP-560A presentation-only extension pilot
comparison evidence payload itself. They do not carry delta payloads, signed
sentinels, reason vocabulary, status flags, conclusions, or provider / host
allocator state.

## Stop Line

This selection does not migrate:

- `allocation_count_delta` or `requested_bytes_delta`, because they are delta
  comparison evidence and stay signed until their own row;
- `last_reason`, because it is reason vocabulary;
- report decision fields, status flags, performance/memory conclusions,
  repeated benchmark execution, process allocator replacement, hooks, backend
  matcher additions, provider package generation, worker/TLS, threads, or
  `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next Selection Hint

Unless a newer SSOT overrides this card, `HAKO-ALLOC-USIZE-FIELD-GROUP-165`
should select the MIMAP-560A presentation-only extension pilot report count
and byte payload fields.
