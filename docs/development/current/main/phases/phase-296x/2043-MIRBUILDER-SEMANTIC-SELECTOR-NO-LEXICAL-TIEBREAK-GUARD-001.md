# 2043 - MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001

## Token

```text
MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001
```

## Purpose

Define and run the selector guardrail needed before the ID scalar derivable
owner discriminator resolution.

This card forbids semantic owner selection by owner name, lexical order,
fixture name, manifest order, and first eligible row unless the selector is
guarded by an explicit exactly-one candidate check.

## Active Enforcement Scope

```text
tools/rust_lifecycle/mirbuilder_id_scalar_source_plan_and_recipe_derivability_resolution_003.py
tools/rust_lifecycle/mirbuilder_id_scalar_typed_evidence_index_policy.py
tools/rust_lifecycle/mirbuilder_id_scalar_operation_vocabulary_authority_split.py
```

## Result

```text
active_file_count = 3
forbidden_active_finding_count = 0
exactly_one_guarded_selection_count = 1
historical_finding_count = 9

decision:
  GuardDefined

reason_token:
  SemanticSelectorNoLexicalTiebreakGuardDefined

selected_next_card:
  MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001
```

## Historical Findings

Historical selector findings are recorded in the fixture and are not new
authority for the active ID scalar lane. If these old selectors are reused,
open:

```text
MIRBUILDER-HISTORICAL-SEED-SELECTOR-QUARANTINE-001
```

## Acceptance

```text
forbidden_active_finding_count = 0
historical_finding_count = 9
first_eligible_selection_requires_exactly_one_guard = 1
historical_findings_are_not_new_authority = 1

manual_owner_selection = 0
owner_name_as_proof = 0
lexical_order_as_proof = 0
fixture_name_as_proof = 0
manifest_order_as_proof = 0
first_eligible_without_exactly_one_guard = 0
```

## Non-Claims

```text
source_plan_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Guard

```text
tools/checks/rust_lifecycle_semantic_selector_no_lexical_tiebreak_guard.sh
```
