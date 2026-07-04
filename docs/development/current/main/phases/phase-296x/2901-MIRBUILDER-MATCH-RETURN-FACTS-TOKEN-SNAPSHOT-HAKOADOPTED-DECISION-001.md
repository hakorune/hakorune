---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for MatchReturnFacts token snapshot reducer.
---

# MIRBUILDER-MATCH-RETURN-FACTS-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `MatchReturnFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=match_return_facts.backend_safe_token_snapshot_reducer
input_contract=BackendSafeMatchReturnTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/match_return_facts.hako
```

This does not adopt strict/release caller policy, `Freeze` construction,
BranchN composition, return lowering, or full AST traversal.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-match-return-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/match_return_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_match_return_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_match_return_facts_token_snapshot_hako_adoption_decision_guard.sh
oracle_rows=7
parity_status=green
```

Required rows:

```text
accept_var_int_returns
accept_int_bool_returns
skip_not_match_expr
reject_scrutinee_unsupported
reject_too_few_arms
reject_non_literal_arm
reject_nonliteral_else
```

## Adopted Semantics

```text
match_expr_detection
scrutinee_token_support
arm_count_minimum
arm_label_literal_support
arm_return_literal_support
else_return_literal_support
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_extract_match_return_facts_ast_owner_adopted=0
strict_release_policy_adopted=0
freeze_construction_adopted=0
reject_logging_handoff_tables_adopted=0
branchn_composition_adopted=0
return_lowering_migrated=0
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
MIRBUILDER-MATCH-RETURN-FACTS-FULL-AST-AND-FREEZE-CONSULTATION-001
```

This backlog owns full AST traversal, caller strict/release policy, `Freeze`
construction, reject logging/handoff tables, and BranchN/return lowering.

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-004
```

Select the next smallest Fact owner. Keep `ExitOnlyBlockRecipe` and
`NoExitBlockRecipe` as thicker follow-up candidates unless the Fact frontier is
explicitly closed.
