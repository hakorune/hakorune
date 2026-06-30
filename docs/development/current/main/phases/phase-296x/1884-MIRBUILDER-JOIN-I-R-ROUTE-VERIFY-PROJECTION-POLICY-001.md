# 1884 - MIRBUILDER-JOIN-I-R-ROUTE-VERIFY-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-JOIN-I-R-ROUTE-VERIFY-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected JoinIRRouteVerify projection-policy cluster.

The selected subcluster is the evidence-quality slice of facts, verifier,
diagnostic, merge-helper, recipe-index, and observability helper surfaces:

```text
owner_edge_confidence = FixtureMapped
stable_deny_reason = UnsupportedDirectShape
shape_signature = shape.join_i_r_route_verify
borrow_axis = NoBorrow
type_transport_axis = Known
verifier_or_oracle_state = Present
```

These surfaces are helper vocabulary under the route verify owner. They do not
open a standalone Hako projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_join_i_r_route_verify_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-join-i-r-route-verify-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_join_i_r_route_verify_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::join_i_r_route_verify
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 53
helper buckets:
  edgecfg_compose
  facts_or_recognizer
  merge_contract_or_logging
  merge_rewriter
  recipe_index
  verify_diagnostic
  verify_observability

markers:
  ControlFlowDetector
  is_supported_bool_expr_with_canon
  detect_break_in_body
  detect_continue_in_body
  FlowboxVia
  emit_flowbox_adopt_tag
  Freeze::
  planner_reject_detail
  is_effect_only_stmt
  should_skip_
  start_index
  end_index
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
