---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for ExitAllowedBlockRecipe token snapshot reducer.
---

# MIRBUILDER-EXIT-ALLOWED-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `ExitAllowedBlockRecipe`.

```text
decision=HakoAdoptedScoped
adopted_owner=exit_allowed_block_recipe.backend_safe_token_snapshot_reducer
input_contract=BackendSafeExitAllowedBlockTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/exit_allowed_block_recipe.hako
```

This does not adopt `try_build_exit_allowed_block_in_arena`, recursive
`RecipeBodies` materialization, `NoExitBlockRecipe`, join-if semantics, or
LoopV0 lowering.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-allowed-block-recipe-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/exit_allowed_block_recipe.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_exit_allowed_block_recipe_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_exit_allowed_block_recipe_token_snapshot_hako_adoption_decision_guard.sh
oracle_rows=10
parity_status=green
```

## Adopted Semantics

```text
exit_allowed_acceptance
block_contract_token
stmt_count
item_kind_sequence
if_mode_sequence
loop_body_contract_token_snapshot
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_build_exit_allowed_block_recipe_ast_owner_adopted=0
try_build_exit_allowed_block_in_arena_adopted=0
recipe_bodies_materialization=0
recipe_tree_materialization=0
cond_block_view_adopted=0
no_exit_block_recipe_adopted=0
join_if_semantics_adopted=0
loop_v0_lowering_adopted=0
backend_capability_expansion=0
mir_mutation_migrated=0
route_selection_migrated=0
id_allocation_migrated=0
hako_generation=0
runtime_fallback=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-006
```

Select the next smallest Fact owner. `NoExitBlockRecipe` can now be considered,
but join-if and nested-loop lowering claims must remain separate.
