# 1818 - MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native Hako source seed for the FunctionRegionStackPop leaf
family selected by the second seed pilot target selection.

This card does not run the HakoAdopted decision and does not claim Source
Selfhost.

## Output

```text
native source seed:
  lang/src/compiler/lib/function_region_stack_pop_native_seed.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-function-region-stack-pop-hako-native-source-seed-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_function_region_stack_pop_hako_native_source_seed_guard.sh
```

## Result

```text
native_source_owner_seed_present = 1
generator_overwrite_guard = 1
next_card = MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-ADOPTION-DECISION-001
```

## Acceptance

```text
FunctionRegionStackPop HakoMainline promotion evidence consumed
FunctionRegionStackPop native source seed exists outside lang/generated
module export for native seed exists
native seed preserves prepared region-stack pop mutation frame
generator overwrite guard is explicit
host_env_lookup = 0
manual_family_selection = 0
family_adoption_decision = 0
source_selfhost_claim = 0
generated_artifact_as_edit_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```

## Non-Claims

```text
no HakoAdopted decision
no Source Selfhost claim
no Rust deletion
no runner semantic ownership
```
