---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for StmtOnlyBlockRecipe token snapshot reducer.
---

# MIRBUILDER-STMT-ONLY-BLOCK-RECIPE-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `StmtOnlyBlockRecipe`.

```text
decision=HakoAdoptedScoped
adopted_owner=stmt_only_block_recipe.backend_safe_token_snapshot_reducer
input_contract=BackendSafeStmtOnlyBlockTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/stmt_only_block_recipe.hako
```

This does not adopt full AST traversal, `ScopeBox` flattening, or
`RecipeBodies` materialization.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-stmt-only-block-recipe-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/stmt_only_block_recipe.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_stmt_only_block_recipe_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_stmt_only_block_recipe_token_snapshot_hako_adoption_decision_guard.sh
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

## Adopted Semantics

```text
stmt_only_acceptance
stmt_count
stmt_kind_sequence
empty_block_rejection
non_local_exit_rejection
unsupported_vocab_rejection
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_build_stmt_only_block_recipe_ast_owner_adopted=0
scopebox_flattening_adopted=0
recipe_bodies_materialization=0
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
MIRBUILDER-STMT-ONLY-BLOCK-RECIPE-FULL-AST-TRAVERSAL-CONSULTATION-001
```

This backlog owns full AST traversal, `ScopeBox` flattening, and
`RecipeBodies` materialization. It is not a blocker for the scoped token
snapshot reducer adoption.

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-003
```

Select the next smallest Fact owner. Keep plan-track owners held until another
Fact owner slice lands or the Fact-track frontier is explicitly closed.
