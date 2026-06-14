---
Status: Landed
Date: 2026-06-15
Task: PARAM-DIRECT-CONSUMER-FORWARDING-GUARD-SURFACE-001
Scope: Define the guard surface for param direct-consumer forwarding before
  MIRBuilder implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-666-PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/allocator/hako_mimalloc_param_direct_consumer_forwarding_candidate_probe.py
  - tools/checks/k2_wide_phase296x_param_direct_consumer_forwarding_guard_surface_guard.sh
---

# PARAM-DIRECT-CONSUMER-FORWARDING-GUARD-SURFACE-001

## Purpose

`PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001` selected
`mir_builder_param_direct_consumer_value_forwarding`. Before implementation,
this row pins the guard surface so the next code slice cannot become broad
LocalSSA coalescing or historical field-get forwarding.

```text
row_kind=guard_surface
implementation_started=0
optimization_open=0
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
```

## Input Evidence

```text
param_candidate_copy_count=7
safe_forward_total_count=7
safe_forward_field_get_count=2
safe_forward_field_set_count=2
safe_forward_compare_count=3
unsafe_forward_count=0
selected_owner_confidence=medium
```

## Guard Intent

The implementation row should be allowed to remove only the current
param-origin expression copy chains that feed direct consumer families.

Guard must pin:

```text
before_param_candidate_copy_count=7
before_safe_forward_total_count=7
before_unsafe_forward_count=0
after_param_candidate_copy_count <= before_param_candidate_copy_count
after_unsafe_forward_count=0
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
```

Guard must reject:

```text
local_ssa_broad_copy_coalescing
same_block_field_get_reuse
field_get_expression_value_copy_chain_without_current_param_proof
source_level_hako_rewrite
allocator_provider_activation
hook_or_global_allocator_installation
```

## Required First Step

Create a small guard script or proof app that runs the current candidate probe
and asserts the selected owner and baseline counts.

Guard:

```text
tools/checks/k2_wide_phase296x_param_direct_consumer_forwarding_guard_surface_guard.sh
```

## Evidence

```bash
bash tools/checks/k2_wide_phase296x_param_direct_consumer_forwarding_guard_surface_guard.sh
```

Result:

```text
[param-forward-guard] ok
```

The guard pins:

```text
param_candidate_copy_count=7
safe_forward_total_count=7
safe_forward_field_get_count=2
safe_forward_field_set_count=2
safe_forward_compare_count=3
unsafe_forward_count=0
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
```

## Selected Next Row

```text
next_task=PARAM-DIRECT-CONSUMER-FORWARDING-IMPLEMENTATION-001
next_card=docs/development/current/main/phases/phase-296x/296x-668-PARAM-DIRECT-CONSUMER-FORWARDING-IMPLEMENTATION-001.md
implementation_open=1
```

## Acceptance

```text
param_direct_consumer_forwarding_guard_surface_001_landed=1
guard_surface_defined=1
guard_script_added=1
guard_script_passed=1
param_candidate_copy_count=7
safe_forward_total_count=7
unsafe_forward_count=0
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
next_task=PARAM-DIRECT-CONSUMER-FORWARDING-IMPLEMENTATION-001
implementation_started=0
optimization_open=0
summary=ok
```
