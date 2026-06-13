---
Status: Active
Date: 2026-06-14
Scope: Switch current work from exact-front optimization back to compiler
foundation construction.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# 293x-1004 COMPILER-FOUNDATION-SELECTION-001

## Decision

Pause exact-front optimization and build compiler foundation first.

```text
compiler_foundation_lane_active=1
optimization_lane_paused=1
optimization_resume_front_selection=MIMALLOC-AOT-KERNEL-FRONT-SELECT-002
current_foundation_taskboard=docs/development/current/main/workstreams/compiler-foundation-current.md
```

## Why

The exact-front lane has reached a point where further wins are local front
selection and owner-specific optimization. The compiler foundation still has
cross-cutting owners that affect plugin integration, Type ABI / BoxDescriptor
projection, Box lifecycle, CorePlan expressivity, selfhost, and later Arc
retirement.

The clean compiler priority is:

```text
1. Make callable truth one layer:
   BoxCallableRegistry receives provider data from type_registry and PluginLoader.

2. Keep descriptor/query surfaces thin:
   TypeAbiCatalog / TypeAbiPack remain read-only projection and tooling surfaces.

3. Strengthen CorePlan / FlowBox:
   choose one expressivity family at a time and keep planner_required fail-fast.

4. Return to exact-front optimization later:
   after the compiler foundation lane reaches a closeout or explicit pause.
```

## First Owner

The first owner is `BoxCallableRegistry`, not Type ABI.

```text
BoxCallableRegistry:
  canonical callable truth

TypeAbiCatalog:
  read-only projection / tooling query index

PluginLoader:
  plugin callable and lifecycle input provider

type_registry:
  builtin/internal slot input provider
```

This follows:

```text
docs/development/current/main/design/box-callable-registry-ssot.md
docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md
```

## Second Owner

The second owner is CorePlan / JoinIR expressivity.

The next CorePlan family must be selected explicitly before implementation.
Candidate families are listed in:

```text
docs/development/current/main/workstreams/compiler-foundation-current.md
```

## Acceptance

```text
compiler_foundation_lane_active=1
optimization_lane_paused=1
current_state_points_to_compiler_foundation=1
compiler_foundation_taskboard_exists=1
boxcallable_first_owner=1
coreplan_second_owner=1
boxcount_boxshape_mixed=0
summary=ok
```

## Stop Line

```text
do not resume exact-front optimization inside this card
do not make TypeAbiCatalog callable truth
do not change TypeBox ABI v2
do not mix BoxCallable registry cleanup with CorePlan acceptance expansion
do not add .hako workaround for compiler expressivity blockers
```

## Verification

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```
