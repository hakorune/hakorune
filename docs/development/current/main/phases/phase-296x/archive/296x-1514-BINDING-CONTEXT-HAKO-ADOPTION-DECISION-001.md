# 296x-1514 BINDING-CONTEXT-HAKO-ADOPTION-DECISION-001

Status: closed
Date: 2026-06-20

## Purpose

Decide whether the BindingContext generated artifact should stay permanently
derived, wait for a real mainline route seam, or move toward native `.hako`
adoption after enough regeneration evidence exists.

This is a decision row. It must not overwrite generated artifacts or remove
Rust bootstrap/oracle routes.

## Selected By

```text
296x-1513-BINDING-CONTEXT-DERIVED-ARTIFACT-MAINLINE-SELECTION-001
```

## Scope

Allowed:

```text
BindingContext route/adoption decision
evidence inventory from 1512 and 1513
next-row selection
Rust bootstrap/oracle retention proof
```

Forbidden:

```text
manual edits to generated BindingContext artifact
HakoAdopted source move without a decision
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
VariableContext adoption
MirBuilder-wide adoption claim
```

## Acceptance Draft

```text
binding_context_current_state=DerivedMainline_candidate
selected_next_route={wait_for_route_seam|hako_native_adoption|permanent_derived}
rust_bootstrap_retained=1
rust_oracle_retained=1
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
```

## Closeout

```text
output_contract=rust-lifecycle-binding-context-adoption-decision-v0
binding_context_current_state=DerivedMainline_candidate
selected_next_route=wait_for_route_seam
rust_bootstrap_retained=1
rust_oracle_retained=1
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
summary=ok
```

Evidence:

```text
tools/checks/rust_lifecycle_binding_context_adoption_decision_guard.sh
```

Decision:

```text
selected_next_route=wait_for_route_seam
```

Reason:

```text
BindingContext is a verified generated artifact and a DerivedMainline
candidate, but there is no existing selfhost build-line route seam that
consumes generated family artifacts. HakoAdopted would be premature before
the route seam exists and before regeneration has run through more than one
family.
```

Next:

```text
296x-1515-BINDING-CONTEXT-DERIVED-MAINLINE-ROUTE-SEAM-001
```

## Stop Line

```text
do_not_overwrite_HakoAdopted_source=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_select_VariableContext_or_MirBuilder_wide=1
do_not_add_runtime_fallback=1
```
