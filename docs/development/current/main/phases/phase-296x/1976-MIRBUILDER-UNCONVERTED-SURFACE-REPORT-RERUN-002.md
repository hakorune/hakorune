# 1976 - MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002

## Token

```text
MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002
```

## Purpose

Regenerate the crate-wide unconverted surface report after
`SOURCE-SELFHOST-POST-RERUN-006-NEXT-LANE-SELECTOR-001` selected the report
freshness lane.

This card also tightens the report provenance contract:

```text
source_selfhost_family_guard_manifest_hash:
  full source-selfhost-family-guard-manifest-v0.json file hash

projection_descriptor_ledger_hash:
  projection-policy row subset hash
```

This keeps full manifest freshness separate from projection descriptor ledger
freshness.

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
    rust_lifecycle_mirbuilder_unconverted_surface_report_rerun_002_guard.sh
```

## Acceptance

```text
report_regenerated = 1
source_selfhost_family_guard_manifest_hash_fresh = 1
projection_descriptor_ledger_hash_fresh = 1
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

The report remains a diagnostic inventory. It is now fresh against the full
family guard manifest and the projection descriptor ledger.

## Recommended Next Task

```text
MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-007
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
