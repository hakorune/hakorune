---
Status: Landed
Date: 2026-06-28
Card: MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001
---

# MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001

## Summary

Select the composed-execution closure artifact as the explicit
`derived_hako` mainline route for the minimal MirBuilder path. This is a
route-selection / integration card, not a new semantic projector and not a
full minimal-path mainline claim.

The readiness resolver already sealed the generated Hako executable closure.
This card consumes that readiness result plus the selected route manifest and
records the route seam as `DerivedMainline`.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Selected By

```text
MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-RESOLVER-001
```

## Scope

Allowed:

```text
minimal-path mainline route manifest update
mainline pilot guard
explicit selected route evidence
Rust bootstrap/oracle retention proof
route seam SSOT verification
```

Forbidden:

```text
HakoAdopted native source decision
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
Source Selfhost claim
full minimal-path mainline claim
manual edits to generated Hako artifact
MirBuilder-wide route selection
```

## Acceptance Draft

```text
family_id=hakorune_mir_builder::minimal_path_composed_execution_closure
selected_route=derived_hako
route_state=DerivedMainline
selected_on_mainline=1
mainline_selection_scope=MinimalMirBuilderComposedExecutionClosure_prepared_state_only
route_manifest_verified=1
artifact_manifest_verified=1
rust_bootstrap_retained=1
rust_oracle_retained=1
runtime_try_hako_then_rust_fallback=0
source_selfhost_claim=0
backend_behavior_changed=0
full_minimal_path_mainline_selected=0
```

## Closeout

```text
output_contract=rust-lifecycle-mirbuilder-minimal-path-mainline-pilot-v0
family_id=hakorune_mir_builder::minimal_path_composed_execution_closure
selected_route=derived_hako
route_state=DerivedMainline
selected_on_mainline=1
mainline_selection_scope=MinimalMirBuilderComposedExecutionClosure_prepared_state_only
route_manifest_verified=1
artifact_manifest_verified=1
rust_bootstrap_retained=1
rust_oracle_retained=1
runtime_try_hako_then_rust_fallback=0
source_selfhost_claim=0
backend_behavior_changed=0
full_minimal_path_mainline_selected=0
summary=ok
```

Evidence:

```text
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-pilot-v0.json
tools/checks/rust_lifecycle_mirbuilder_minimal_path_mainline_pilot_guard.sh
```

Boundary:

```text
This selects only the minimal-path composed execution closure route in the
generated family route manifest. It does not claim Source Selfhost, does not
move the family to HakoAdopted, and does not remove Rust bootstrap/oracle
routes.
```

## Stop Line

```text
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_make_generated_Hako_edit_authority=1
do_not_select_full_minimal_path_mainline=1
do_not_add_runtime_fallback=1
```
