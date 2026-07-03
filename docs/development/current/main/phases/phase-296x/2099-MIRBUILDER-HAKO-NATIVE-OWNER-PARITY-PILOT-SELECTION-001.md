# 2099 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-001

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-001
```

## Purpose

Select the first small hand-authored `.hako` native owner pilot after the
Rust-to-Hako converter role pivot.

This card does not claim Source Selfhost and does not make a Hako adoption
decision. It selects a pilot target whose correctness will be proven later by
Rust-oracle JSON fixtures and a parity gate.

## Selection Policy

Target selection may use human/AI judgment. The candidate inventory is an
advisory ranking only.

Correctness must be proven by parity, not by the ranking.

```text
candidate_ranking_is_advisory = 1
manual_target_selection_allowed = 1
correctness_proof_is_parity_gate = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Candidate Score Rules

Positive signals:

```text
pure_input_to_output_owner
small_rust_surface_100_to_300_lines
existing_rust_tests
json_serializable_boundary
low_dependency_surface
no_builder_context_mutation
```

Negative signals:

```text
mutates_mir_metadata_or_context
depends_on_builder_lowerer_or_backend
borrow_lifetime_projection_policy_surface
result_option_carrier_inference_surface
phi_or_localssa_surface
shape_specific_exact_seed_matcher
no_existing_tests
```

## Ranked Candidates

```text
0. return_prelude_policy
   file: src/mir/policies/return_prelude_policy.rs
   role: parity harness smoke only
   reason: tiny pure policy, but too small to validate the owner migration model
   selected_as_owner_pilot: no

1. storage_class_classifier
   file: src/mir/storage_class.rs
   selected_as_owner_pilot: yes
   reason: MirType -> StorageClass is pure, small, has existing Rust tests,
           and maps cleanly to JSON input/output pairs
   scope: classifier only; refresh_function_storage_class_facts stays Rust

2. static_scalar_facts
   file: src/mir/builder/static_scalar_facts.rs
   selected_as_owner_pilot: no
   reason: small facts owner with tests, but under builder path

3. effect_capability_plan
   file: src/mir/effect_capability_plan.rs
   selected_as_owner_pilot: no
   reason: good JSON shape and tests, but depends on rune profile expansion

4. generic_method_route_facts
   file: src/mir/generic_method_route_facts.rs
   selected_as_owner_pilot: no
   reason: has tests, but carrier/key-route semantics can widen too quickly
```

## Selected Pilot

```text
selected_owner:
  storage_class_classifier

selected_rust_surface:
  src/mir/storage_class.rs::infer_storage_class equivalent

excluded_rust_surface:
  refresh_module_storage_class_facts
  refresh_function_storage_class_facts
  metadata.value_storage_classes mutation

selected_next_card:
  MIRBUILDER-STORAGE-CLASS-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Next Sequence

```text
1. MIRBUILDER-STORAGE-CLASS-CLASSIFIER-RUST-ORACLE-FIXTURE-001
   Dump stable JSON input/output pairs for MirType -> StorageClass.

2. MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
   Hand-write the `.hako` implementation.

3. MIRBUILDER-STORAGE-CLASS-CLASSIFIER-PARITY-GATE-001
   Run `.hako` against oracle JSON and diff normalized output.

4. MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-ADOPTION-DECISION-001
   Allowed only after parity is green.
```

## Fixture Boundary

Oracle rows should be plain JSON pairs:

```json
{
  "input": {
    "mir_type": "Integer"
  },
  "expected": {
    "storage_class": "inline_i64"
  }
}
```

Non-deterministic IDs, MIR metadata mutation, and function/block traversal are
out of scope for the first pilot.

## Non-Claims

```text
source_selfhost_claim = 0
source_selfhost_complete = 0
hako_adopted_decision = 0
native_seed_materialization = 0
hako_generation = 0
generated_artifact_as_native_edit_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```
