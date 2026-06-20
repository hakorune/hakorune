# 296x-1515 BINDING-CONTEXT-DERIVED-MAINLINE-ROUTE-SEAM-001

Status: open
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

## Stop Line

```text
do_not_build_a_general_route_system_in_this_row=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_select_VariableContext_or_MirBuilder_wide=1
do_not_add_runtime_fallback=1
```
