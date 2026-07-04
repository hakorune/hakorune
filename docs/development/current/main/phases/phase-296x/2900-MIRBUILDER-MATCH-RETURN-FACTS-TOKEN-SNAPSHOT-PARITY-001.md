---
Status: Landed
Date: 2026-07-05
Scope: MatchReturnFacts backend-safe token snapshot parity slice.
---

# MIRBUILDER-MATCH-RETURN-FACTS-TOKEN-SNAPSHOT-PARITY-001

## Decision

Select `try_extract_match_return_facts` as the next small Fact-owner parity
pilot and land its backend-safe token snapshot reducer.

```text
selected_owner=match_return_facts.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_extract_match_return_facts
input_contract=BackendSafeMatchReturnTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/match_return_facts.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- returns a non-leaf fact DTO (`MatchReturnFacts`)
- owns accept/reject reason summary for the return-only match subset
- does not compose BranchN plans
- does not lower returns
- does not mutate MIR or allocate IDs

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-match-return-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/match_return_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_match_return_facts_parity_gate.sh
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

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
strict_release_policy_adopted=0
freeze_construction_adopted=0
branchn_composition_adopted=0
return_lowering_migration=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-MATCH-RETURN-FACTS-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001
```

Adopt only the backend-safe token snapshot reducer if the decision guard keeps
the same non-claims.
