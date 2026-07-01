# SOURCE-SELFHOST-POST-RERUN-006-NEXT-LANE-SELECTOR-001

Status: Landed
Date: 2026-07-01
Kind: machine-checkable lane selector
Output contract: `rust-lifecycle-source-selfhost-post-rerun-006-next-lane-selector-v0`

## Purpose

Select the next Source Selfhost progress lane after
`SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-002` without choosing a family,
shape, or axis by hand.

The selector consumes:

- `SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-002`
- `MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-006`
- the current crate-wide unconverted surface report
- the current Source Selfhost family guard manifest

## Decision

```text
decision:
  SelectUnconvertedSurfaceReportRerun

reason_token:
  UnconvertedSurfaceReportFreshnessCheckRequiredAfterBasis002

selected_next_card:
  MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002
```

The selector does not pick a family, shape, axis, native seed, or route repair
lane directly. It routes through the source-surface report freshness lane so
the report tool can re-check the projection descriptor ledger before any new
owner selection.

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
native_seed_materialization = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Guard

```text
tools/checks/rust_lifecycle_source_selfhost_post_rerun_006_next_lane_selector_guard.sh
```

The guard verifies that the selector chooses only
`MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002`, and that Source Selfhost
remains stopped at `SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001`.
