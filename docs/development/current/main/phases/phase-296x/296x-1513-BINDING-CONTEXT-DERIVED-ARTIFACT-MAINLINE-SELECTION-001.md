# 296x-1513 BINDING-CONTEXT-DERIVED-ARTIFACT-MAINLINE-SELECTION-001

Status: closed
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

## Closeout

```text
output_contract=rust-lifecycle-binding-context-mainline-selection-v0
binding_context_artifact_state=DerivedMainline_candidate
mainline_selection_scope=BindingContext_only
generated_artifact_manifest_verified=1
rust_bootstrap_retained=1
rust_oracle_retained=1
silent_fallback=0
runtime_try_hako_then_rust_fallback=0
source_selfhost_claim=0
backend_behavior_changed=0
selected_on_mainline=0
summary=ok
```

Evidence:

```text
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
tools/checks/rust_lifecycle_binding_context_mainline_selection_guard.sh
```

Decision:

```text
BindingContext is admitted as a DerivedMainline candidate only.
The generated artifact remains DerivedShadow and is not selected on the
active build line in this row.
```

Reason:

```text
No existing selfhost build-line route seam consumes generated family artifacts
yet. Selecting a candidate manifest first keeps the route explicit without
inventing a runtime try-Hako-then-Rust fallback.
```

Boundary:

```text
The 1513 guard verifies the route manifest, artifact manifest, and
deterministic regeneration only. It does not rerun the 1512 generated-artifact
EXE gate, because backend shape acceptance belongs to the artifact pilot row
and not to route-selection metadata.
```

Next:

```text
296x-1514-BINDING-CONTEXT-HAKO-ADOPTION-DECISION-001
```

## Stop Line

```text
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_make_generated_Hako_edit_authority=1
do_not_select_VariableContext_or_MirBuilder_wide=1
do_not_add_runtime_fallback=1
```
