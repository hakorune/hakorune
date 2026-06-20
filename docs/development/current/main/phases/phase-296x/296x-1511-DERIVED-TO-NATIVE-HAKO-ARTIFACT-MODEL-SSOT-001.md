# 296x-1511 DERIVED-TO-NATIVE-HAKO-ARTIFACT-MODEL-SSOT-001

Status: closed
Date: 2026-06-20

## Purpose

Pivot the Rust-to-Hako MirBuilder migration model from direct family authority
promotion to the Derived-to-Native Hako Artifact Model.

This is a design/task-sequencing row. It must not start BindingContext
projection, Hako emission, generated artifact selection, or build-line
substitution.

## Selected By

```text
296x-1510-RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-MIR-LIFECYCLE-FACTS-001
```

## Scope

Allowed:

```text
document Derived-to-Native Hako Artifact Model SSOT
define generated Hako as execution artifact, not semantic/edit authority
define Rust source as editable reference/oracle during Derived phases
define verifier/parity as acceptance authority
define HakoAdopted as native Hako edit/semantic authority
define BindingContext pilot sequence
preserve Rust bootstrap/oracle/compat wording
update current pointers
```

Forbidden:

```text
BindingContext HakoLifecyclePlan projection
HakoBehaviorRecipe implementation
generated Hako output
artifact manifest output
build-line mainline selection
crate-wide MirBuilder claim
Rust bootstrap removal
backend behavior change
```

## Acceptance

```text
selected_model=derived_to_native_hako_artifact
generated_artifact_is_semantic_authority=0
rust_edit_authority_retained_during_derived_phase=1
hako_native_adoption_gate_required=1
direct_authority_promotion_started=0
binding_context_projection_started=0
hako_artifact_emitted=0
rust_bootstrap_retained=1
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Decision

```text
selected_model=derived_to_native_hako_artifact
ssot=docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
```

The next implementation row is:

```text
296x-1512-BINDING-CONTEXT-DERIVED-HAKO-ARTIFACT-PILOT-001
```

## Closeout

```text
selected_model=derived_to_native_hako_artifact
generated_artifact_is_semantic_authority=0
rust_edit_authority_retained_during_derived_phase=1
hako_native_adoption_gate_required=1
direct_authority_promotion_started=0
binding_context_projection_started=0
hako_artifact_emitted=0
rust_bootstrap_retained=1
backend_behavior_changed=0
summary=ok
```

## Stop Line

```text
do_not_start_BindingContext_projection_in_this_row=1
do_not_emit_generated_Hako_in_this_row=1
do_not_call_generated_artifact_semantic_authority=1
do_not_remove_Rust_bootstrap=1
do_not_claim_Source_Selfhost_from_generated_artifact=1
```
