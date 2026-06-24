# 296x-1515 BINDING-CONTEXT-DERIVED-MAINLINE-ROUTE-SEAM-001

Status: closed
Date: 2026-06-20

## Purpose

Design and implement the smallest explicit selfhost build-line seam that can
select the BindingContext derived artifact without deleting Rust bootstrap,
without making generated `.hako` edit authority, and without runtime
try-Hako-then-Rust fallback.

## Selected By

```text
296x-1514-BINDING-CONTEXT-HAKO-ADOPTION-DECISION-001
```

## Scope

Allowed:

```text
focused BindingContext route seam inventory
explicit route label / manifest reader if needed
build-line selection proof for BindingContext only
Rust bootstrap/oracle retention proof
silent fallback guard
```

Forbidden:

```text
HakoAdopted native source move
Rust bootstrap removal
runtime fallback from Hako to Rust
VariableContext or MirBuilder-wide selection
loop / PHI / lowering conversion
manual edits to generated BindingContext artifact
```

## Acceptance Draft

```text
binding_context_route_seam_defined=1
mainline_selection_scope=BindingContext_only
selected_route={derived_hako|not_selected_with_reason}
rust_bootstrap_retained=1
rust_oracle_retained=1
runtime_try_hako_then_rust_fallback=0
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
```

## Closeout

```text
output_contract=rust-lifecycle-binding-context-route-seam-v0
binding_context_route_seam_defined=1
mainline_selection_scope=BindingContext_only
selected_route=not_selected_with_reason
not_selected_reason=no_selfhost_family_artifact_route_seam
rust_bootstrap_retained=1
rust_oracle_retained=1
runtime_try_hako_then_rust_fallback=0
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
summary=ok
```

Evidence:

```text
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
tools/checks/rust_lifecycle_binding_context_route_seam_guard.sh
```

Decision:

```text
selected_route=not_selected_with_reason
not_selected_reason=no_selfhost_family_artifact_route_seam
```

Reason:

```text
The repository has route vocabulary and a BindingContext family route
manifest, but no existing selfhost build-line seam consumes generated family
artifacts. This row records the explicit non-selection instead of building a
general route system inside a BindingContext-only task.
```

Next:

```text
296x-1516-SELFHOST-FAMILY-ARTIFACT-ROUTE-SEAM-SSOT-001
```

## Stop Line

```text
do_not_build_a_general_route_system_in_this_row=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_select_VariableContext_or_MirBuilder_wide=1
do_not_add_runtime_fallback=1
```
