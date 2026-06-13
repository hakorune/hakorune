---
Status: SSOT
Date: 2026-06-14
Scope: COREPLAN-FOUND-000/001 selection and first guard for compatibility
normalizer lego-ization.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - src/mir/builder/control_flow/plan/REGISTRY.md
  - src/mir/builder/control_flow/plan/normalizer/README.md
  - src/mir/builder/control_flow/plan/features/README.md
---

# CorePlan Compatibility Normalizer Lego-ization

## Decision

`COREPLAN-FOUND-000` selects:

```text
selected_family=B1_remaining_compatibility_normalizer_legoization
```

`COREPLAN-FOUND-001` fixes the first contract for that family:

```text
selected_family_ssot_exists=1
fixture_or_guard_named=1
release_default_changed=0
planner_required_failfast_preserved=1
```

This row is BoxShape-only. It does not add a new accepted loop/source shape.

## Why This Family First

B1 is the safest CorePlan foundation row because it reduces future pattern
growth before adding more vocabulary. The active problem is not that one more
loop form is missing. The active problem is that stale compatibility-normalizer
names can invite another route-specific branch instead of a skeleton plus
feature composition.

## Contract

```text
coreplan_selected_family=B1_remaining_compatibility_normalizer_legoization
boxcount_boxshape_mixed=0
legacy_normalizer_new_route_branch_allowed=0
normalizer_stale_route_file_count=0
normalizer_to_feature_pipeline_rule_documented=1
joinir_regression_gate_named=1
selfhost_gate_named=1
```

## Boundary

Normalizer modules may:

```text
consume facts/canon/recipe inputs
lower verified condition/value/body fragments
provide semantic compatibility aliases during migration
```

Normalizer modules must not:

```text
own a new accepted source shape
re-parse AST to decide route acceptance
grow route-specific one-shape branches
become the current execution truth for a new CorePlan family
```

Reusable behavior goes to:

```text
src/mir/builder/control_flow/plan/skeletons/
src/mir/builder/control_flow/plan/features/
src/mir/builder/control_flow/plan/parts/
src/mir/builder/control_flow/plan/recipe_tree/
```

Route-specific compatibility folders may remain only with a retire/promote row
in:

```text
src/mir/builder/control_flow/plan/REGISTRY.md
```

## First Guard

The first guard is:

```bash
bash tools/checks/coreplan_compat_normalizer_legoization_guard.sh
```

It is a structural drift guard. It verifies:

```text
COREPLAN-FOUND-000 selected B1
COREPLAN-FOUND-001 names this SSOT and guard
normalizer README does not list retired route files as active modules
features README keeps plan/<kind>/normalizer as thin adapters
REGISTRY keeps remaining legacy normalizer work as planned lego-ization
retired route files are not reintroduced
```

## Required Gates Before Any Code Shape Change

Any future implementation row that actually changes a route must name a focused
fixture and keep the existing integration gates green:

```bash
./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh
```

Selfhost remains opt-in:

```bash
SMOKES_ENABLE_SELFHOST=1 \
  ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh
```

## Stop Line

```text
do not add a new loop_*_v0 box in this row
do not add a new accepted source shape in this row
do not modify .hako to work around a CorePlan gap
do not make JoinIR the semantic truth owner
do not hide planner_required ambiguity behind Ok(None)
```
