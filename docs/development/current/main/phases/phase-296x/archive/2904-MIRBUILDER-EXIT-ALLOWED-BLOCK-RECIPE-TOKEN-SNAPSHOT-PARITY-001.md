---
Status: Landed
Date: 2026-07-05
Scope: ExitAllowedBlockRecipe backend-safe token snapshot parity slice.
---

# MIRBUILDER-EXIT-ALLOWED-BLOCK-RECIPE-TOKEN-SNAPSHOT-PARITY-001

## Decision

Select `try_build_exit_allowed_block_recipe` as the next Fact-owner parity
pilot and land its backend-safe token snapshot reducer.

```text
selected_owner=exit_allowed_block_recipe.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_build_exit_allowed_block_recipe
input_contract=BackendSafeExitAllowedBlockTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/exit_allowed_block_recipe.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- returns a facts-side `ExitAllowedBlockRecipe` DTO
- observes Stmt / Exit / IfV2 item sequence and exit-mode summary
- adopts only the symbolic `LoopV0(body_contract=ExitAllowed)` token
- keeps recursive `RecipeBodies`, LoopV0 lowering, and `CondBlockView` in Rust
- does not adopt `NoExitBlockRecipe` or join-if semantics
- does not lower, route, mutate MIR, or allocate IDs

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-allowed-block-recipe-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/exit_allowed_block_recipe.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_exit_allowed_block_recipe_parity_gate.sh
oracle_rows=10
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
loop_body_contract_token_snapshot=1
loop_v0_lowering_adopted=0
no_exit_block_recipe_adopted=0
join_if_semantics_adopted=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-EXIT-ALLOWED-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
```
