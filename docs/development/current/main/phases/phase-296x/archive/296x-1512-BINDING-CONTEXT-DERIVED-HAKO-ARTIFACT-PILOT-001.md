# 296x-1512 BINDING-CONTEXT-DERIVED-HAKO-ARTIFACT-PILOT-001

Status: closed
Date: 2026-06-20

## Purpose

Build the first behavioral Rust-derived Hako artifact pilot for the
BindingContext family.

This row uses the checked HIR, THIR, and MIR facts plus existing lifecycle
fixtures to produce a verified BindingContext artifact. It does not select the
artifact on the selfhost mainline.

## Selected By

```text
296x-1511-DERIVED-TO-NATIVE-HAKO-ARTIFACT-MODEL-SSOT-001
```

## Blocked By

```text
none
```

Resolved prerequisite:

```text
BindingContext targets remove(name) and clear_for_function_entry().
Those map to OrderedMapBox.remove and OrderedMapBox.clear in the generated
artifact. 296x-1512A adds and smoke-guards those collection operations as
library-owned behavior, so the converter/emitter no longer needs to hide this
gap.
```

## Scope

Allowed:

```text
BindingContext family only
Rust facts input reuse
HakoLifecyclePlan projection
HakoBehaviorRecipe for BindingContext methods
CombinedVerifier for selected methods
deterministic generated Hako artifact
artifact provenance manifest
generated Hako parse / MIR / EXE-AOT gate
Rust oracle behavior parity
```

Forbidden:

```text
mainline route selection
HakoAdopted native source decision
VariableContext promotion
crate-wide MirBuilder claim
loop / PHI / lowering conversion
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
manual edits to generated Hako artifact
```

## Target Methods

```text
new/default
is_empty
len
contains
lookup
insert
remove
clear_for_function_entry
```

## Acceptance

```text
family_id=BindingContext
rust_facts_input=verified
hako_lifecycle_plan=verified
hako_behavior_recipe=verified
selected_body_count=all_non_test_methods
unmapped_thir_nodes=0
unmapped_mir_side_effects=0
unresolved_call_targets=0
unclassified_drop_obligations=0
generated_hako_checked_in=1
artifact_manifest_checked_in=1
generated_hako_manual_edit=0
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
rust_oracle_behavior_parity=green
mainline_selected=0
rust_bootstrap_retained=1
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_binding_context_derived_artifact_guard.sh
bash tools/checks/rustc_semir_adapter_binding_context_mir_lifecycle_facts_guard.sh
bash apps/lib/collections/smoke_ordered_map.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Result

```text
family_id=BindingContext
rust_facts_input=verified
hako_lifecycle_plan=verified
hako_behavior_recipe=verified
selected_body_count=all_non_test_methods
unmapped_thir_nodes=0
unmapped_mir_side_effects=0
unresolved_call_targets=0
unclassified_drop_obligations=0
generated_hako_checked_in=1
artifact_manifest_checked_in=1
generated_hako_manual_edit=0
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
rust_oracle_behavior_parity=green
mainline_selected=0
rust_bootstrap_retained=1
backend_behavior_changed=0
summary=ok
```

Artifacts:

```text
lang/generated/rust_derived/hakorune_mir_builder/binding_context.hako
lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json
docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-behavior-recipe-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-derived-artifact-verifier-result-v0.json
tools/rust_lifecycle/generate_binding_context_artifact.py
tools/checks/rust_lifecycle_binding_context_derived_artifact_guard.sh
```

Boundary notes:

```text
Generated artifact state is DerivedShadow.
mainline_selected=0.
Rust bootstrap/oracle route remains retained.
Bool-returning Rust operations use i64_bool_v0 transport in this pilot because
the active pure-first global helper ABI expects scalar i64 returns.
BindingContext key-order behavior is verified by the deterministic_order fact
and the OrderedMapBox smoke from 296x-1512A; the BindingContext generated EXE
smoke avoids nested key_at because that nested field route is not a stable
1512 surface.
```

Executed:

```bash
bash tools/checks/rust_lifecycle_binding_context_derived_artifact_guard.sh
bash tools/checks/rustc_semir_adapter_binding_context_mir_lifecycle_facts_guard.sh
bash apps/lib/collections/smoke_ordered_map.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_select_generated_artifact_on_mainline_in_this_row=1
do_not_edit_generated_Hako_by_hand=1
do_not_emit_executable_tier_TODO_or_null_fallback=1
do_not_hide_Rust_API_to_Hako_API_mapping_inside_emitter=1
do_not_remove_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
```
