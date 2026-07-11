---
Status: Landed
Date: 2026-07-05
Scope: NoExitBlockRecipe backend-safe token snapshot parity slice.
---

# MIRBUILDER-NO-EXIT-BLOCK-RECIPE-TOKEN-SNAPSHOT-PARITY-001

## Decision

Select `try_build_no_exit_block_recipe` as the next Fact-owner parity pilot and
land its backend-safe token snapshot reducer.

```text
selected_owner=no_exit_block_recipe.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_build_no_exit_block_recipe
input_contract=BackendSafeNoExitBlockTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/no_exit_block_recipe.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- returns a facts-side `NoExitBlockRecipe` DTO
- observes Stmt / IfJoin / LoopV0 contract tokens and no-exit rejection
- adopts only symbolic `IfV2{Join}` shape and `LoopV0(body_contract=ExitAllowed)` tokens
- keeps recursive `RecipeBodies`, `CondBlockView`, count_control_flow, in-arena
  ExitAllowed construction, join-if lowering, and LoopV0 lowering in Rust

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-no-exit-block-recipe-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/no_exit_block_recipe.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_no_exit_block_recipe_parity_gate.sh
oracle_rows=11
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
recursive_recipe_bodies_materialization=0
recipe_tree_materialization=0
cond_block_view_adopted=0
count_control_flow_adopted=0
exit_allowed_in_arena_adopted=0
join_if_lowering_adopted=0
join_payload_construction_adopted=0
join_binding_update_policy_adopted=0
nested_loop_lowering_adopted=0
loop_v0_lowering_adopted=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-NO-EXIT-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
```
