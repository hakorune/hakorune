---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for StringIsIntegerFacts authority facade.
---

# MIRBUILDER-STRING-IS-INTEGER-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `StringIsIntegerFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=string_is_integer_facts.authority_facade
input_contract=BackendSafeStringIsIntegerFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/string_is_integer_facts.hako
```

This does not adopt full AST traversal, substring expression materialization,
route selection, backend lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-string-is-integer-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/string_is_integer_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_string_is_integer_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_string_is_integer_facts_hako_adoption_decision_guard.sh
oracle_rows=7
parity_status=green
```

## Adopted Semantics

```text
string_is_integer_acceptance
direct_is_digit_shape
local_range_shape
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
substring_expression_materialization=0
route_selection_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
hako_generation=0
runtime_fallback=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-013
```
