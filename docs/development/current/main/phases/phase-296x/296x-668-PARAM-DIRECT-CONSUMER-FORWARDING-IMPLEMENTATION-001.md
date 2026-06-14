---
Status: Landed
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

The narrow LocalSSA/consumer-side implementation attempt was rejected before
commit. The probe stayed at seven param candidates after the attempted
`FieldBase / FieldSetValue / CompareOperand` forwarding, which means the true
owner is not the direct consumer LocalSSA finalization seam.

Rejected local attempt evidence:

```text
before_param_candidate_copy_count=7
after_param_candidate_copy_count=7
after_expression_materialization_copy_count=10
after_unsafe_forward_count=0
rejected_attempt=local_ssa_param_direct_consumer_forwarding
rejected_reason=target_param_candidate_count_unchanged
selected_next_owner_refresh=param_alias_local_binding_copy_chain
next_task=PARAM-ALIAS-COPY-OWNER-REFRESH-001
keeper_installed=0
semantic_smoke_ok=0
body_timing_remeasured=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The next row must identify the actual source of the `%param -> copy -> copy`
chains. The observed MIR shows copies already exist before the direct
FieldGet/FieldSet/Compare consumers:

```text
block_593:
  %51 = copy %3
  %50 = copy %51
  field_set box=%50 field=last_page_id

block_597:
  %92 = copy %15
  %91 = copy %92
  field_get box=%91 field=last_selected_page_id

block_600:
  %147 = copy %120
  %150 = copy %147
  compare lhs=%153 ...
```

This points at param alias / local binding chain ownership rather than the
direct consumer LocalSSA seam.

Final acceptance:

```text
param_direct_consumer_forwarding_implementation_001_landed=1
keeper_installed=0
implementation_reverted=1
target_param_candidate_count_unchanged=1
after_unsafe_forward_count=0
next_task=PARAM-ALIAS-COPY-OWNER-REFRESH-001
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
