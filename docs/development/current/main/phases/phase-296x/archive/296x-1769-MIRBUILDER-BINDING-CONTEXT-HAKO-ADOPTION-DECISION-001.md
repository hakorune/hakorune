---
Status: Landed
Date: 2026-06-28
Card: MIRBUILDER-BINDING-CONTEXT-HAKO-ADOPTION-DECISION-001
---

# MIRBUILDER-BINDING-CONTEXT-HAKO-ADOPTION-DECISION-001

## Summary

Adopt the BindingContext family as the next narrow `HakoAdopted` candidate.
The route manifest is already selected on mainline, the native `.hako`
source owner exists, and the generator overwrite guard is green. This card
records the adoption decision without widening the route to VariableContext
or MirBuilder-wide source selfhost.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Authority

Selected route evidence:

```text
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
tools/checks/rust_lifecycle_binding_context_derived_route_selection_guard.sh
tools/checks/rust_lifecycle_binding_context_route_seam_guard.sh
```

Native source owner evidence:

```text
apps/lib/hakorune_mir_builder/binding_context.hako
tools/checks/rust_lifecycle_binding_context_adoption_decision_guard.sh
tools/checks/rust_mirbuilder_binding_context_native_guard.sh
```

## Acceptance

```text
binding_context_current_state = DerivedMainline
selected_next_route = native_hako_source_owner
native_hako_source_owner_present = 1
generator_overwrite_guard = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1
generated_artifact_manual_edit = 0
source_selfhost_claim = 0
backend_behavior_changed = 0
runtime_fallback = 0
decision = Adopt
```

## Non-Claims

```text
Source Selfhost = 0
Rust bootstrap removal = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
VariableContext promotion = 0
MirBuilder-wide route selection = 0
```

## Closeout

```text
output_contract=rust-lifecycle-binding-context-adoption-decision-v0
binding_context_current_state=DerivedMainline
selected_next_route=native_hako_source_owner
native_hako_source_owner_present=1
generator_overwrite_guard=1
rust_bootstrap_retained=1
rust_oracle_retained=1
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
runtime_fallback=0
decision=Adopt
summary=ok
```
