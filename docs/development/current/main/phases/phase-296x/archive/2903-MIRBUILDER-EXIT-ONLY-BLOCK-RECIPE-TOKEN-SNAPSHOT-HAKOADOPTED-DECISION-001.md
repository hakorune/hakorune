---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for ExitOnlyBlockRecipe token snapshot reducer.
---

# MIRBUILDER-EXIT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `ExitOnlyBlockRecipe`.

```text
decision=HakoAdoptedScoped
adopted_owner=exit_only_block_recipe.backend_safe_token_snapshot_reducer
input_contract=BackendSafeExitOnlyBlockTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/exit_only_block_recipe.hako
```

This does not adopt recursive `RecipeBodies` materialization,
`ExitAllowedBlockRecipe`, `NoExitBlockRecipe`, join-if semantics, or full AST
traversal.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-only-block-recipe-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/exit_only_block_recipe.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_exit_only_block_recipe_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_exit_only_block_recipe_token_snapshot_hako_adoption_decision_guard.sh
oracle_rows=8
parity_status=green
```

Required rows:

```text
accept_break
accept_stmt_then_return
accept_if_exit_all
accept_if_exit_if_not_all_paths
accept_effect_only_not_all_paths
reject_empty
reject_then_fallthrough_else_exit
reject_unsupported_if_condition
```

## Adopted Semantics

```text
exit_only_acceptance
block_contract_token
stmt_count
item_kind_sequence
if_mode_sequence
ends_with_exit_on_all_paths
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_build_exit_only_block_recipe_ast_owner_adopted=0
recipe_bodies_materialization=0
recipe_tree_materialization=0
cond_block_view_adopted=0
exit_allowed_block_recipe_adopted=0
no_exit_block_recipe_adopted=0
join_if_semantics_adopted=0
loop_body_recipe_adopted=0
backend_capability_expansion=0
mir_mutation_migrated=0
route_selection_migrated=0
id_allocation_migrated=0
hako_generation=0
runtime_fallback=0
new_backend_route=0
new_abi=0
```

## Backlog

```text
MIRBUILDER-EXIT-ONLY-BLOCK-RECIPE-FULL-AST-AND-RECIPEBODIES-CONSULTATION-001
```

This backlog owns full AST traversal, recursive `RecipeBodies` materialization,
`CondBlockView`, and adjacent `ExitAllowedBlockRecipe` adoption.

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-005
```

Select the next smallest Fact owner. Keep `NoExitBlockRecipe` held until
`ExitAllowedBlockRecipe` or loop-body recipe scope is explicitly selected.
