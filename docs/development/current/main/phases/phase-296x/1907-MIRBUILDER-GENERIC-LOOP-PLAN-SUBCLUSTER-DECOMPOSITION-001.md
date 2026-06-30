# 1907 - MIRBUILDER-GENERIC-LOOP-PLAN-SUBCLUSTER-DECOMPOSITION-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-PLAN-SUBCLUSTER-DECOMPOSITION-001
```

## Purpose

Decompose the selected `GenericLoopPlanCluster` before any projection policy is
selected.

The priority resolver selected:

```text
projection_policy::UnsupportedDirectShape::shape.generic_loop_plan::FixtureMapped::GenericLoopPlanCluster
```

The cluster has `candidate_count = 66`, but its source modules cover multiple
roles: expression matchers, step validation, body-check extractors, shape
detectors, shape-detector utilities, and statement classifiers. This card
therefore creates a path-role decomposition and selects the narrow expression
matcher subcluster first.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_plan_subcluster_decomposition.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-plan-subcluster-decomposition-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_plan_subcluster_decomposition_guard.sh
```

## Subclusters

```text
BodyCheckExprMatchers:
  body_check/expr_matchers/*

BodyCheckStepValidation:
  body_check/step_validation.rs

BodyCheckExtractors:
  body_check_extractors.rs
  facts/extract/collection.rs

BodyCheckShapeDetectors:
  body_check_shape_detectors/*
  except body_check_shape_detectors/utils.rs

BodyCheckShapeDetectorUtils:
  body_check_shape_detectors/utils.rs

StatementClassifierPredicates:
  facts/stmt_classifier/*
```

## Decision

```text
kind = SelectSubclusterProjectionPolicy
selected_subcluster = BodyCheckExprMatchers

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001
```

## Acceptance

```text
input_candidate_count = 66
source_module_count = 17
scanned_function_count = 88
subcluster_count = 6
whole_cluster_projection_policy = 0
whole_cluster_keep_parent_owner = 0
candidate_count_as_proof = 0
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
no whole GenericLoopPlan projection policy
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
