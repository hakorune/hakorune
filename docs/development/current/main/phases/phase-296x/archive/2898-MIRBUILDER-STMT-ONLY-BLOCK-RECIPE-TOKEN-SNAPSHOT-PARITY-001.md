---
Status: Landed
Date: 2026-07-05
Scope: StmtOnlyBlockRecipe backend-safe token snapshot parity slice.
---

# MIRBUILDER-STMT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-PARITY-001

## Decision

Select `try_build_stmt_only_block_recipe` as the next small Fact-owner parity
pilot and land its backend-safe token snapshot reducer.

```text
selected_owner=stmt_only_block_recipe.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_build_stmt_only_block_recipe
input_contract=BackendSafeStmtOnlyBlockTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/stmt_only_block_recipe.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- returns a facts-side `StmtOnlyBlockRecipe` DTO
- validates statement vocabulary and non-local-exit rejection
- keeps `ScopeBox` flattening caller-owned
- does not mutate MIR
- does not allocate IDs
- does not route, lower, or materialize `RecipeBodies` in `.hako`

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-stmt-only-block-recipe-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/stmt_only_block_recipe.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_stmt_only_block_recipe_parity_gate.sh
oracle_rows=7
parity_status=green
```

Required rows:

```text
accept_local_print
accept_if_no_exit
accept_loop_no_exit
reject_empty
reject_break
reject_if_break
reject_scopebox_not_flattened
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
scopebox_flattening_adopted=0
recipe_bodies_materialization=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-STMT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
```

Adopt only the backend-safe token snapshot reducer if the decision guard keeps
the same non-claims.
