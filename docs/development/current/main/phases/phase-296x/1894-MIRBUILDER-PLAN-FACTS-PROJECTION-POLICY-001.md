# 1894 - MIRBUILDER-PLAN-FACTS-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-PLAN-FACTS-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected PlanFacts projection-policy cluster.

The selected surfaces are facts predicates, matchers, and a debug accept log
helper:

```text
exit_only_block_ends_with_exit_on_all_paths(arena, block) -> bool
is_supported_value_expr_for_generic_loop(ast) -> bool
is_pure_value_expr_for_generic_loop(ast) -> bool
is_supported_bool_expr_for_generic_loop(ast) -> bool
detect_nested_loop(body) -> bool
match_index_of_bound(condition, idx_var) -> bool
scan_nested_loop_body(body, profile, allow_extended) -> bool
log_accept(box_name, accept_tag)
```

These helpers classify existing plan facts or emit gated diagnostics. They do
not own a standalone Hako projection surface. They remain under the PlanFacts
parent owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_plan_facts_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-plan-facts-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_plan_facts_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::plan_facts
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 8

roles:
  generic_loop_expr_fact_predicate = 3
  exit_only_terminality_fact = 1
  nested_loop_presence_fact = 1
  scan_bound_matcher_fact = 1
  nested_loop_body_profile_fact = 1
  debug_accept_log_helper = 1

markers:
  generic_loop 専用 expr 判定 helpers (SSOT)
  is_supported_value_expr_for_generic_loop
  is_pure_value_expr_for_generic_loop
  is_supported_bool_expr_for_generic_loop
  exit_only_block_ends_with_exit_on_all_paths
  detect_nested_loop
  match_index_of_bound
  Nested-loop body profile (analysis-only, no AST rewrite)
  Emit structured accept log
```

## Acceptance

```text
policy = KeepParentOwner
projection_surface_selected = 0
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
no standalone Hako projection surface
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
