# 1867 - MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
```

## Purpose

Resolve the first policy boundary for
`carrier_merge::lower_assignment_stmt`, which is the delegated semantic helper
behind the loop-cond continue-only assignment statement arm.

## Decision

```text
selected_policy = MutationFrameContractRequired
projection_surface_selected = 0
decision = SelectMutationFrameContract
```

The helper is not a syntax-only assignment wrapper. It reseals
`builder.variable_ctx.variable_map` from `current_bindings`, delegates RHS /
assignment lowering, and then updates `carrier_updates`, `current_bindings`,
and `builder.variable_ctx.variable_map` depending on the returned binding.

That mutation frame must be fixed before a HakoShadow projector or native
source seed can be selected.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-merge-assignment-statement-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_merge_assignment_statement_projection_policy_guard.sh
```

## Acceptance

```text
delegated_from_assignment_ast_arm = 1
source_report_classification = MissingProjectionPolicy
selected_policy = MutationFrameContractRequired
projection_surface_selected = 0
mutation_frame_contract_required = 1
selected_next_card = MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001
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
1. MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001
   Fix the precise state mutation frame consumed by the carrier-merge
   assignment helper.
```

## Non-Claims

```text
no Hako generation
no HakoShadow projector selection
no HakoAdopted decision
no native source seed
no Source Selfhost claim
no route repair
```
