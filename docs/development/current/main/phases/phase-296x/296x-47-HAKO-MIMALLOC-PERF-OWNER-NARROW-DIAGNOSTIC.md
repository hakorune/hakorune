---
Status: Current
Date: 2026-05-27
Scope: execute the selected owner-specific diagnostic from row 46.
Blocker: HAKO-MIMALLOC-PERF-OWNER-NARROW-DIAGNOSTIC-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-46-HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION.md
---

# 296x-47 Hako Mimalloc Owner Narrow Diagnostic

## Purpose

Run only the diagnostic selected by row 46. This row should not optimize or
broaden the measurement contract unless the selected diagnostic is explicitly
`measurement_hygiene_refresh`.

## Required Input

```text
output_contract=hako-mimalloc-conditional-diagnostic-selection-v0
selected_diagnostic
measurement_hygiene_required=0|1
next_optimization_allowed=0|1
body_elapsed_ns_primary=0
winner_claim=0
```

## Required Output Shape

```text
front=<exact workload/front>
gap_owner=<one primary owner>
diagnostic_kind=<selected diagnostic>
body_elapsed_ns_secondary=0|1
build_compile_excluded=1 when measurement_hygiene_refresh
sample_count=5|7 only when measurement_hygiene_refresh
next_optimization_allowed=0|1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Stop Line

Do not optimize in this row. If the diagnostic proves the owner is
`compiler_lowering` or `allocator_algorithm`, the next row may select the first
keeper optimization.
