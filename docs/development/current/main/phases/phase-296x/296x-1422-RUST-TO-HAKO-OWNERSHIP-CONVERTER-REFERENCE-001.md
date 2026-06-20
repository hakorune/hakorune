# 296x-1422 RUST-TO-HAKO-OWNERSHIP-CONVERTER-REFERENCE-001

Status: closed
Date: 2026-06-20

## Purpose

Document and task the precise meaning of an ownership-aware Rust-to-Hako
converter.

## Selected By

```text
296x-1421-POST-JOIN-ID-VOCABULARY-DECISION-OWNER-SELECTION-001
```

## Scope

```text
reference=docs/development/current/main/design/rust-to-hako-ownership-converter-reference.md
ssot=docs/development/current/main/design/rust-lifecycle-projection-ssot.md
emitter_contract=docs/development/current/main/design/rust-to-hako-lifecycle-emitter-contract.md
```

Decision:

```text
converter_role=verified_plan_renderer
converter_policy_owner=0
rust_adapter_policy_owner=0
hako_lifecycle_resolver_policy_owner=1
verifier_required_before_lifecycle_emission=1
skeleton_route_lifecycle_claim=0
```

## Task Decomposition

```text
RustSubsetModule-v0:
  structure / skeleton input only

RustLifecycleFacts-v0:
  rustc-derived lifecycle sidecar

HakoLifecyclePlan-v0:
  Hako-owned ownership / borrow / move / Drop projection

VerifierResult:
  required positive evidence before lifecycle-aware emission

converter / emitter:
  render verified plans only
```

## Acceptance

```text
ownership_converter_reference_written=1
rust_lifecycle_projection_ssot_points_to_reference=1
converter_policy_owner=0
emitter_policy_owner=0
verified_plan_required=1
implementation_started=0
backend_behavior_changed=0
generated_program_claim=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
ownership_converter_reference_written=1
rust_lifecycle_projection_ssot_points_to_reference=1
converter_policy_owner=0
emitter_policy_owner=0
verified_plan_required=1
implementation_started=0
backend_behavior_changed=0
generated_program_claim=0
```

Next:

```text
296x-1423-POST-OWNERSHIP-CONVERTER-REFERENCE-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_add_Rust_lifetime_syntax=1
do_not_modify_converter_core=1
do_not_start_rustc_adapter_probe=1
do_not_claim_whole_crate_ownership_conversion=1
do_not_resume_trim_helper_in_this_row=1
```

