# 2036 - MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001

## Token

```text
MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001
```

## Purpose

Declare ID scalar error semantics and deterministic order obligations before
behavior recipe effect coverage.

## Result

```text
error_semantics_count = 6
deterministic_order_count = 3
runtime_fallback_count = 0
diagnostic_prefix_required_count = 3

decision:
  ErrorAndDeterministicOrderBasisDefined

selected_next_card:
  MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001
```

## Boundary

Invalid or missing IDs deny without runtime fallback. Mutation-frame order is
source-surface order and verifier-visible. This card does not materialize
behavior recipes, verifier fixtures, or native seeds.

## Non-Claims

```text
behavior_recipe_materialization = 0
verifier_result_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
```
