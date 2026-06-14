---
Status: Active
Date: 2026-06-15
Task: PARAM-DIRECT-CONSUMER-FORWARDING-IMPLEMENTATION-001
Scope: Implement the narrow MIRBuilder param direct-consumer forwarding keeper
  selected by the refreshed object-lifecycle body-timing evidence.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-667-PARAM-DIRECT-CONSUMER-FORWARDING-GUARD-SURFACE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/checks/k2_wide_phase296x_param_direct_consumer_forwarding_guard_surface_guard.sh
---

# PARAM-DIRECT-CONSUMER-FORWARDING-IMPLEMENTATION-001

## Purpose

Implement the selected keeper:

```text
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
```

The implementation must only target the param-origin expression copies that are
proven to feed direct consumers in the current object-lifecycle MIR.

## Baseline Guard

```bash
bash tools/checks/k2_wide_phase296x_param_direct_consumer_forwarding_guard_surface_guard.sh
```

Pinned baseline:

```text
param_candidate_copy_count=7
safe_forward_total_count=7
safe_forward_field_get_count=2
safe_forward_field_set_count=2
safe_forward_compare_count=3
unsafe_forward_count=0
```

## Implementation Rules

Allowed:

```text
remove param-origin expression materialization copies for direct consumers
preserve value identity and consumer semantics
keep forwarding local to the selected MIRBuilder owner
```

Forbidden:

```text
broad LocalSSA copy coalescing
same-block field_get reuse retry
historical field_get expression chain policy reuse
source-level .hako rewrite
allocator provider activation
hook or global allocator installation
product NyRT startup change
```

## Acceptance

The implementation must provide a before/after report that shows:

```text
before_param_candidate_copy_count=7
after_param_candidate_copy_count < before_param_candidate_copy_count
after_unsafe_forward_count=0
selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding
semantic_smoke_ok=1
body_timing_remeasured=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=pending
```
