---
Status: Landed
Decision: accepted
Date: 2026-06-28
Scope: Adopt the BoxCompilationContext family as the next narrow HakoAdopted
  candidate after native source owner materialization.
Related:
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
---

# MIRBUILDER-CONTEXT-HAKO-ADOPTION-DECISION-001

## Summary

Adopt the `context` route-manifest entry as the next narrow `HakoAdopted`
candidate now that the native `BoxCompilationContext` source owner exists.
This is intentionally narrow. It does not claim Source Selfhost, does not
remove Rust bootstrap, and does not widen the route beyond the
`BoxCompilationContext` family.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Authority

Selected route evidence:

```text
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
tools/checks/rust_lifecycle_box_compilation_context_derived_route_selection_guard.sh
tools/checks/rust_mirbuilder_box_compilation_context_native_guard.sh
```

Native source owner evidence:

```text
apps/lib/hakorune_mir_builder/box_compilation_context.hako
tools/checks/rust_mirbuilder_box_compilation_context_native_guard.sh
```

## Acceptance

```text
box_compilation_context_current_state = DerivedMainline
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
output_contract=rust-lifecycle-box-compilation-context-adoption-decision-v0
box_compilation_context_current_state=DerivedMainline
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
