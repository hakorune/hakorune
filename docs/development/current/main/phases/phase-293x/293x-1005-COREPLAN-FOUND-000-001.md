---
Status: Active
Date: 2026-06-14
Scope: Select the first CorePlan foundation family and add the first guard.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# 293x-1005 COREPLAN-FOUND-000/001

## Decision

`COREPLAN-FOUND-000` selects one family:

```text
selected_family=B1_remaining_compatibility_normalizer_legoization
```

`COREPLAN-FOUND-001` implements the first proof boundary:

```text
selected_family_ssot_exists=1
fixture_or_guard_named=1
release_default_changed=0
planner_required_failfast_preserved=1
```

This is a BoxShape row. It does not add a new accepted shape.

## Why

B1 is the lowest-risk foundation row. It prevents stale normalizer names and
route-specific helper shelves from becoming new truth while keeping the next
real implementation row focused on skeleton plus feature composition.

## Deliverables

```text
docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md
tools/checks/coreplan_compat_normalizer_legoization_guard.sh
src/mir/builder/control_flow/plan/normalizer/README.md
docs/tools/check-scripts-index.md
```

## Acceptance

```text
coreplan_next_family_selected=1
selected_family=B1_remaining_compatibility_normalizer_legoization
selected_family_ssot_exists=1
fixture_or_guard_named=1
release_default_changed=0
planner_required_failfast_preserved=1
boxcount_boxshape_mixed=0
summary=ok
```

## Verification

```bash
bash tools/checks/coreplan_compat_normalizer_legoization_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Line

```text
do not add new accepted loop/source shape in this card
do not add a new loop_*_v0 box in this card
do not route around CorePlan gaps in .hako
do not make TypeAbiCatalog or JoinIR semantic truth
```
