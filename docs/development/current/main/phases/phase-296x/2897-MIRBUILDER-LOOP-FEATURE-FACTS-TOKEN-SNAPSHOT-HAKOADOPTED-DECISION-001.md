---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for loop_feature_facts token snapshot reducer.
---

# MIRBUILDER-LOOP-FEATURE-FACTS-TOKEN-SNAPSHOT-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `loop_feature_facts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_feature_facts.backend_safe_token_snapshot_reducer
input_contract=BackendSafeLoopBodySnapshotTokenV1
native_edit_authority=lang/src/compiler/lib/loop_feature_facts.hako
```

This does not adopt the full Rust AST walker.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-feature-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_feature_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_feature_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_feature_facts_token_snapshot_hako_adoption_decision_guard.sh
oracle_rows=3
parity_status=green
```

The required rows are:

```text
if_break_continue_return
if_hidden_nested_loop
if_hidden_nested_loop_break_ignored
```

The third row fixes the Rust contract that exits inside a nested loop are not
outer `exit_usage`, while `nested_loop` remains true.

## Adopted Semantics

```text
exit_usage
nested_loop
derived_exit_map
value_join_none
cleanup_none
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_extract_loop_feature_facts_ast_owner_adopted=0
ast_body_snapshot_traversal_adopted=0
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
MIRBUILDER-AST-BODY-SNAPSHOT-TRAVERSAL-BACKEND-CAPABILITY-CONSULTATION-001
```

This backlog owns MapBox/ArrayBox AST body traversal and any backend capability
discussion. It is not a blocker for the scoped token snapshot reducer adoption.

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-002
```

Select the next smallest Fact owner. Keep `build_plan_with_facts_ctx` and
`try_build_outcome` held until another Fact owner slice lands or the Fact-track
frontier is explicitly closed.
