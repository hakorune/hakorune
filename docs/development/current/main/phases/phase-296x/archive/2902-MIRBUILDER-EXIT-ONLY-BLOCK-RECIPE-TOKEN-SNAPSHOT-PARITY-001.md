---
Status: Landed
Date: 2026-07-05
Scope: ExitOnlyBlockRecipe backend-safe token snapshot parity slice.
---

# MIRBUILDER-EXIT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-PARITY-001

## Decision

Select `try_build_exit_only_block_recipe` as the next Fact-owner parity pilot
and land its backend-safe token snapshot reducer.

```text
selected_owner=exit_only_block_recipe.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_build_exit_only_block_recipe
input_contract=BackendSafeExitOnlyBlockTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/exit_only_block_recipe.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- returns a facts-side `ExitOnlyBlockRecipe` DTO
- observes exit item sequence and all-path exit summary
- keeps recursive `RecipeBodies` materialization in Rust
- does not adopt `ExitAllowedBlockRecipe`
- does not adopt `NoExitBlockRecipe` or join-if semantics
- does not lower, route, mutate MIR, or allocate IDs

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-exit-only-block-recipe-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/exit_only_block_recipe.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_exit_only_block_recipe_parity_gate.sh
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

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
recursive_recipe_bodies_materialization=0
recipe_tree_materialization=0
cond_block_view_adopted=0
exit_allowed_block_recipe_adopted=0
no_exit_block_recipe_adopted=0
join_if_semantics_adopted=0
loop_body_recipe_adopted=0
condition_canon_policy_adopted=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-EXIT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
```

Adopt only the backend-safe token snapshot reducer if the decision guard keeps
the same non-claims.
