# 3340 - SOURCE-SELFHOST-POST-RERUN004-CURRENT-REENTRY-INVENTORY-001

## Token

```text
SOURCE-SELFHOST-POST-RERUN004-CURRENT-REENTRY-INVENTORY-001
```

## Purpose

Record the current reentry decision after the runtime-adjacent ProgramJSON
shadow guard: `SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007` remains valid
historical local selector evidence, but it was already consumed by
`MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004`.

This card prevents replaying the stale-report freshness repair as the current
next task. It does not rerun report generation and does not select a route,
family, owner, or Source Selfhost claim.

## Input Authority

```text
current prerequisite:
  MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-REFRESH-001

consumed historical selector:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007

completed freshness repair:
  MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004

recommended current next:
  SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002
```

## Acceptance

```text
basis_007_valid_local_mechanical_selector = 1
basis_007_consumed_by_rerun_004 = 1
rerun_004_report_regenerated = 1
rerun_004_result = KeepStopped
rerun_004_recommended_next =
  SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002

machine_derived_route_repair_replay = 0
```

## Result

```text
selected_next_card =
  SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002

next_after_selected =
  MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V4
```

`SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002` remains the existing
checkpoint card. It uses the fresh report to select the blocker class by
evidence quality and explicit selector rule, not by raw counts.

## Non-Claims

```text
manual_family_selection = 0
route_membership_alone_as_proof = 0
coverage_percentage_as_proof = 0
source_selfhost_claim = 0
hako_adopted_decision = 0
native_seed_materialization = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-post-rerun004-current-reentry-inventory-v0.json

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_post_rerun004_current_reentry_inventory_guard.sh
```
