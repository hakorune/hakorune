# 1868 - MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001

## Token

```text
MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001
```

## Purpose

Fix the mutation-frame contract for `carrier_merge::lower_assignment_stmt`
before any HakoShadow projector or native source seed is selected.

## Contract

```text
state_inputs:
  current_bindings
  carrier_phis
  builder.variable_ctx.variable_map

state_outputs:
  current_bindings
  carrier_updates
  builder.variable_ctx.variable_map

read_only:
  carrier_phis
```

Mutation order:

```text
1. reseal builder.variable_ctx.variable_map from current_bindings
2. delegate assignment lowering to loop_body_lowering::lower_assignment_stmt
3. if no binding is returned, return effects only
4. if binding targets a carrier phi, update carrier_updates
5. if binding targets a carrier phi or existing current binding, update current_bindings
6. always publish returned binding to builder.variable_ctx.variable_map
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_merge_assignment_statement_mutation_frame_contract_guard.sh
```

## Acceptance

```text
input_projection_policy_consumed = 1
mutation_frame_contract_ready = 1
mutation_order_verified = 1
carrier_phis_read_only = 1
state_outputs = current_bindings, carrier_updates, builder.variable_ctx.variable_map
selected_next_card = MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001
manual_family_selection = 0
hako_generation = 0
hako_shadow_projector_selected = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Recommended Next Tasks

```text
1. MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001
   Build a HakoShadow parity target for the contracted carrier-merge
   assignment mutation frame.
```

## Non-Claims

```text
no Hako generation
no HakoShadow projector selected yet
no HakoAdopted decision
no native source seed
no Source Selfhost claim
no route repair
```
