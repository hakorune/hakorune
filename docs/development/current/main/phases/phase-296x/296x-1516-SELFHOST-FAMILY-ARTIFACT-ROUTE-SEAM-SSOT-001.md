# 296x-1516 SELFHOST-FAMILY-ARTIFACT-ROUTE-SEAM-SSOT-001

Status: open
Date: 2026-06-20

## Purpose

Define the minimal selfhost family-artifact route seam before any generated
family artifact is selected on the build line.

This row should specify how a checked-in generated artifact can be selected,
how Rust bootstrap/oracle routes remain explicit, and how silent fallback is
rejected.

## Selected By

```text
296x-1515-BINDING-CONTEXT-DERIVED-MAINLINE-ROUTE-SEAM-001
```

## Scope

Allowed:

```text
route seam SSOT
route labels and manifest fields
selection/readiness conditions
guard output contract
next implementation row selection
```

Forbidden:

```text
selecting BindingContext on the active build line
moving generated Hako to native Hako source
Rust bootstrap removal
runtime fallback from Hako to Rust
VariableContext or MirBuilder-wide selection
```

## Acceptance Draft

```text
selfhost_family_artifact_route_seam_ssot=1
allowed_routes={derived_hako,native_hako,rust_bootstrap,rust_compat,host_substrate,unsupported}
selection_requires_manifest=1
selection_requires_guard=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
implementation_started=0
backend_behavior_changed=0
```

## Stop Line

```text
do_not_select_any_generated_artifact_in_this_row=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_add_runtime_fallback=1
```
