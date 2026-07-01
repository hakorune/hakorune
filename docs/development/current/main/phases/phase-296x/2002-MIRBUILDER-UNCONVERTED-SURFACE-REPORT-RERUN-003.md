# 2002 - MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003

## Token

```text
MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003
```

## Purpose

Regenerate the crate-wide unconverted surface report after BASIS-003 selected
the source-surface report rerun lane.

This rerun refreshes the report against both:

```text
projection_descriptor_ledger_hash
native_owner_adoption_ledger_hash
```

The second hash is required because `direct_state_plan_refresh`,
`record_packed_layout_refresh`, and `typed_object_plan_refresh` were adopted
after `MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002`.

## Acceptance

```text
report_regenerated = 1
projection_descriptor_ledger_hash_fresh = 1
native_owner_adoption_ledger_hash_fresh = 1
native_owner_adoption_delta_count = 3
scan_unit = rust_function_or_method
join_unit = semantic_owner_edge
scanned_surface_count = 1584
classified_once_count = 1584
missing_projection_policy_count = 1384
borrow_policy_needed_count = 112
unmapped_count = 0
decision = KeepStopped
reason_token = AmbiguousUnconvertedSurfaceCandidates
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```

## Result

```text
decision:
  KeepStopped

reason_token:
  AmbiguousUnconvertedSurfaceCandidates

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

The report remains a diagnostic inventory. It is now fresh against the latest
projection descriptor ledger and native-owner adoption ledger.

## Recommended Next Task

```text
SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001
```

Compute the current native owner map and next blocker class. This checkpoint is
not a Source Selfhost claim.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-unconverted-surface-report-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_unconverted_surface_report.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_unconverted_surface_report_rerun_003_guard.sh
```

## Non-Claims

```text
no family selection
no blocker-class selection by count
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
