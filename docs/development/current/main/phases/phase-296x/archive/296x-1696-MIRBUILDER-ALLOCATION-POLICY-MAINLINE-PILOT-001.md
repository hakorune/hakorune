# 296x-1696 MIRBUILDER-ALLOCATION-POLICY-MAINLINE-PILOT-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-ALLOCATION-POLICY-MAINLINE-PILOT-001

## Purpose

Promote exactly one prepared-state family route to `DerivedMainline`:

```text
hakorune_mir_builder::next_value_id_prepared_state_kernel
```

This is a family-scoped mainline pilot. It does not claim full `MirBuilder`
conversion, source selfhost, Hako adoption, a new backend route, or runtime
fallback.

## Selection Authority

```text
MirBuilderAllocationPolicyMainlineSelectionPlanV1
  -> DerivedMainlineRouteSelectionV1
  -> pre-execution family route resolver
```

The route key is a stable slot:

```text
hakorune_mir_builder.allocation_policy.next_value_id.prepared_state.v1
```

It is not the callee symbol `MirBuilderAllocationPolicyApi.next_value_id/4`.

## Route Profiles

```text
selfhost_mainline:
  route = derived_hako

rust_bootstrap:
  route = rust_bootstrap

platform_bringup:
  route = rust_bootstrap
```

The routes are selected before execution. If the derived artifact is missing,
stale, or mismatched, the resolver fails closed; it does not retry through Rust.

## Artifact Transition

```text
state:
  DerivedShadow -> DerivedMainline

claims:
  prepared_state_policy_kernel = 1
  mainline_selected = 1
  rust_bootstrap_retained = 1
  full_mirbuilder_object_method = 0
  hako_adopted = 0
  native_hako_edit_authority = 0
  source_selfhost_claim = 0
  runtime_fallback = 0
```

Generated `.hako` bytes remain unchanged.

## Route Closure

```text
SameArtifactHako:
  MirBuilderAllocationPolicyApi.next_value_id/4
  FunctionValueIdCounterStateApi.next/1
  ReservedValueIdMembershipViewApi.has/2
  CoreContextApi.next_value/1

AllowedHostSubstrate:
  ValueIdOrderedMapBox

ForbiddenRustSemanticDependency:
  none
```

## Frontier Result

After this route selection is available, the minimal execution path analyzer
advances to:

```text
callsite:
  Minimal MirBuilder execution path next live edge selection

reason:
  UnsupportedDirectShape

detail:
  NextMinimalExecutionPathEdgeRequired

next slice:
  MIRBUILDER-MINIMAL-EXECUTION-PATH-FRONTIER-REFRESH-001
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_allocation_policy_mainline_pilot_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 -m py_compile \
  tools/rust_lifecycle/mirbuilder_allocation_policy_mainline_selection.py \
  tools/rust_lifecycle/mirbuilder_next_value_id_prepared_state_kernel_artifacts.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/current_state_pointer_guard.sh
```
