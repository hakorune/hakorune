---
Status: Landed
Date: 2026-05-28
Scope: classify the Hako/C object-lifecycle body timing gap before reopening optimization.
Blocker: OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-174-OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER.md
  - tools/allocator/hako_mimalloc_object_lifecycle_body_timing_gap_taxonomy.py
---

# 296x-175 Object Lifecycle Body Timing Gap Taxonomy

## Purpose

Classify the joined Hako/C body timing gap into one owner before reopening any
optimization. This row consumes row174 pair evidence and keeps winner,
replacement, hook, global allocator, and provider activation claims closed.

## Required Output

```text
output_contract=hako-mimalloc-object-lifecycle-body-timing-gap-taxonomy-v0
gap_owner=compiler_lowering|hako_runtime_baseline|measurement_harness
gap_confidence=low|medium|high
evidence_quality=...
next_diagnostic=...
next_optimization_allowed=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
Large Hako/C body_elapsed_ns gaps point first at compiler/runtime lowering
surface, not source-level allocator keeper work. The next row should select a
MIR/body owner diagnostic before any code optimization.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_body_timing_gap_taxonomy_guard.sh
```
