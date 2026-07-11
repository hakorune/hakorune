# 296x-1494 RUSTC-SEMIR-EXTRACTED-FACTS-VERIFIER-PARITY-001

Status: closed
Date: 2026-06-20

## Purpose

Verify that source-derived adapter facts for `BindingContext` and
`VariableContext` are consumable by the existing lifecycle verifier fixture
path.

This row must not add Hako policy, `.hako` emission, backend behavior, or
wider context extraction.

## Selected By

```text
296x-1493-POST-RUSTC-SEMIR-VARIABLE-CONTEXT-FACTS-EXTRACTION-OWNER-SELECTION-001
```

## Scope

```text
subjects=BindingContext,VariableContext
input=extractor-produced RustLifecycleAdapterFacts JSON
consumer=tools/rust_lifecycle/verify_lifecycle_fixture.py
```

Allowed:

```text
temporary generated facts files
comparison with checked-in fixtures
verifier invocation over generated facts
diagnostic report
```

Forbidden:

```text
new HakoLifecyclePlan-v0 output
.hako source output
new verifier semantics
rustc-internal adapter work
backend behavior change
```

## Acceptance

```text
extracted_facts_verifier_parity_green=1
binding_context_generated_facts_verified=1
variable_context_generated_facts_verified=1
checked_in_fixtures_unchanged=1
hako_policy_owner=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_extracted_facts_verifier_parity_guard.sh
bash tools/checks/rustc_semir_binding_context_facts_extraction_guard.sh
bash tools/checks/rustc_semir_variable_context_facts_extraction_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

```text
guard=tools/checks/rustc_semir_extracted_facts_verifier_parity_guard.sh
verifier=tools/rust_lifecycle/verify_lifecycle_fixture.py
override_inputs=--binding-context-facts,--variable-context-facts
```

The guard generates temporary `RustLifecycleAdapterFacts` JSON for
`BindingContext` and `VariableContext`, then feeds those generated files into
the existing lifecycle verifier fixture path.

The checked-in fixtures remain unchanged.

## Closeout

```text
extracted_facts_verifier_parity_green=1
binding_context_generated_facts_verified=1
variable_context_generated_facts_verified=1
checked_in_fixtures_unchanged=1
hako_policy_owner=0
backend_behavior_changed=0
```

Next:

```text
POST-RUSTC-SEMIR-EXTRACTED-FACTS-VERIFIER-PARITY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_change_HakoLifecyclePlan_fixtures=1
do_not_emit_hako_source=1
do_not_add_verifier_semantics=1
do_not_start_rustc_internal_adapter=1
do_not_change_backend=1
```
