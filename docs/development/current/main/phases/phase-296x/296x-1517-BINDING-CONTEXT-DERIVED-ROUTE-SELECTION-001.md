# 296x-1517 BINDING-CONTEXT-DERIVED-ROUTE-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Use the selfhost family-artifact route seam SSOT to decide whether the
BindingContext derived artifact can be selected as `derived_hako` for the
focused family route.

## Selected By

```text
296x-1516-SELFHOST-FAMILY-ARTIFACT-ROUTE-SEAM-SSOT-001
```

## Scope

Allowed:

```text
BindingContext route manifest update
BindingContext-only guard
explicit selected route or not-selected reason
Rust bootstrap/oracle retention proof
```

Forbidden:

```text
native Hako adoption
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
VariableContext or MirBuilder-wide selection
manual edits to generated BindingContext artifact
```

## Acceptance Draft

```text
family_id=hakorune_mir_builder::binding_context
selected_route={derived_hako|not_selected_with_reason}
route_seam_ssot_verified=1
artifact_manifest_verified=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
```

## Stop Line

```text
do_not_select_more_than_BindingContext=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_add_runtime_fallback=1
```
