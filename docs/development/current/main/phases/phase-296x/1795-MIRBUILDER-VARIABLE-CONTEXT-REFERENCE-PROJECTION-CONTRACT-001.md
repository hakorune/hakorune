---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Fix the VariableContext Rust reference to Hako projection contract.
Related:
  - docs/development/current/main/phases/phase-296x/1794-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-reference-projection-contract-v0.json
  - tools/checks/rust_lifecycle_variable_context_reference_projection_contract_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001

## Goal

Record the design consultation result as a machine-checkable projection
contract. Rust-to-Hako conversion for this family is semantic 1:1 through a
verified projection, not syntax 1:1 through Rust reference spelling.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Contract

```text
projection_model:
  SemanticOneToOneVerifiedProjection

syntax_one_to_one_required:
  0

variable_map:
  rust_return = &BTreeMap<String, ValueId>
  hako_projection = OwnedReadSnapshotProjection
  raw_alias_selected = false

variable_map_mut:
  rust_return = &mut BTreeMap<String, ValueId>
  hako_projection = ExplicitMutationApiOnly
  raw_mutable_alias_selected = false
  mut_lease_selected = false
```

## Native API Boundary

```text
current_native_api:
  lookup
  contains
  len
  is_empty
  snapshot
  restore
  replace_owned_map
  insert
  remove

future_native_api_candidates:
  entries_snapshot
  snapshot_owned
  restore_owned
```

The future names are not claimed as implemented by this card. They are
contract candidates for a later naming cleanup or consumer-driven projection
row.

## Acceptance

```text
selected_rust_surfaces_classified = 1
replacement_policies_explicit = 1
native_hako_api_has_no_raw_borrow_alias = 1
alias_isolation_guarded = 1
mutation_frame_guarded = 1
restore_replace_not_merge = 1
deterministic_iteration_preserved = 1
emitter_policy_free = 1
syntax_one_to_one_claim = 0
full_variable_context_claim = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
manual_family_selection = 0
```

## Non-Claims

```text
Rust lifetime syntax in Hako = 0
raw variable_map alias = 0
raw variable_map_mut alias = 0
MutLease = 0
entries_snapshot implementation = 0
Source Selfhost = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-reference-projection-contract-v0
projection_model=SemanticOneToOneVerifiedProjection
syntax_one_to_one_required=0
variable_map_projection=OwnedReadSnapshotProjection
variable_map_mut_projection=ExplicitMutationApiOnly
raw_variable_map_alias_selected=0
raw_variable_map_mut_alias_selected=0
mut_lease_selected=0
full_variable_context_claim=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
summary=ok
```
