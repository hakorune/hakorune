# 1949 - MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001

## Token

```text
MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001
```

## Purpose

Regenerate the crate-wide unconverted surface report after projection descriptor
closeout changed the Source Selfhost family guard manifest / descriptor ledger.

This card refreshes report provenance only. It does not choose a family, shape,
or blocker axis.

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
    rust_lifecycle_mirbuilder_unconverted_surface_report_rerun_guard.sh
```

## Acceptance

```text
report_regenerated = 1
projection_descriptor_ledger_hash_fresh = 1
native_owner_seed_capability_survey_hash_fresh = 1
scan_unit = rust_function_or_method
join_unit = semantic_owner_edge
scanned_surface_count = 1584
classified_once_count = 1584
missing_projection_policy_count = 1384
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

The report remains a diagnostic inventory. It is now fresh against the current
descriptor ledger and can be consumed by the next native owner seed capability
rerun.

## Recommended Next Task

```text
MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-002
```

Re-evaluate native owner seed capability after the source-surface report
freshness repair.

## Non-Claims

```text
no family selection
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
