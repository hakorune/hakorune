# 296x-1513 BINDING-CONTEXT-DERIVED-ARTIFACT-MAINLINE-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Decide whether and how the generated BindingContext artifact from 296x-1512
may be selected on a focused selfhost mainline route.

This row is a route-selection / integration boundary. It must keep Rust
bootstrap and Rust oracle routes explicit.

## Selected By

```text
296x-1512-BINDING-CONTEXT-DERIVED-HAKO-ARTIFACT-PILOT-001
```

## Scope

Allowed:

```text
focused BindingContext route selection inventory
generated artifact route manifest / route label
explicit rust_bootstrap / rust_oracle retention proof
silent fallback guard
Stage1/Stage2 acceptance definition
```

Forbidden:

```text
HakoAdopted native source decision
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
VariableContext promotion
crate-wide MirBuilder claim
loop / PHI / lowering conversion
manual edits to generated Hako artifact
```

## Acceptance Draft

```text
binding_context_artifact_state=DerivedMainline_candidate
mainline_selection_scope=BindingContext_only
generated_artifact_manifest_verified=1
rust_bootstrap_retained=1
rust_oracle_retained=1
silent_fallback=0
runtime_try_hako_then_rust_fallback=0
source_selfhost_claim=0
backend_behavior_changed=0
```

## Stop Line

```text
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_make_generated_Hako_edit_authority=1
do_not_select_VariableContext_or_MirBuilder_wide=1
do_not_add_runtime_fallback=1
```
