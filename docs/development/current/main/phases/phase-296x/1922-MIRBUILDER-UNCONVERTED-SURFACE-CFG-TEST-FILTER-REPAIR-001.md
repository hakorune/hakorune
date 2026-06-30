# 1922 - MIRBUILDER-UNCONVERTED-SURFACE-CFG-TEST-FILTER-REPAIR-001

## Token

```text
MIRBUILDER-UNCONVERTED-SURFACE-CFG-TEST-FILTER-REPAIR-001
```

## Purpose

Repair the crate-wide unconverted surface report so `#[cfg(test)]` Rust
methods are classified as `TestOnlySurface` before projection-policy cluster
selection.

The priority resolver selected `RecipeTreeMatcherCluster`, but the selected
known-transport candidates were test-only `VerifiedRecipeBlock::*_port_contains`
helpers. Test helpers must stay visible for diagnostics, but they must not
drive Hako projection policy work or Source Selfhost owner selection.

## Output

```text
updated tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_unconverted_surface_report.py

updated fixtures:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-unconverted-surface-report-v0.json
    mirbuilder-unconverted-surface-next-owner-resolution-v0.json
    mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json
    mirbuilder-projection-policy-cluster-priority-resolution-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_unconverted_surface_cfg_test_filter_repair_guard.sh
```

## Contract

```text
cfg_test_surface:
  true for functions with a directly preceding #[cfg(test)] attribute

classification:
  TestOnlySurface

next_owner_kind:
  None
```

## Decision

```text
kind = RepairUnconvertedSurfaceCfgTestFilter

next_card =
  MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-PROJECTION-POLICY-001
```

## Acceptance

```text
return_port_contains = TestOnlySurface
break_port_contains = TestOnlySurface
continue_port_contains = TestOnlySurface
RecipeTreeMatcher test helpers do not select projection policy
missing_projection_policy_count = 1384
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no projection policy selected by this repair
no Hako emitted
no HakoAdopted decision
no native source seed materialization
no Source Selfhost claim
```
