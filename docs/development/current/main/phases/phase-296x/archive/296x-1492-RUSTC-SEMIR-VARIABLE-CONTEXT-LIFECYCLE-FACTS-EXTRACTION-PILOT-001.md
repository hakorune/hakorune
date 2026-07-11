# 296x-1492 RUSTC-SEMIR-VARIABLE-CONTEXT-LIFECYCLE-FACTS-EXTRACTION-PILOT-001

Status: closed
Date: 2026-06-20

## Purpose

Produce focused source-derived adapter output toward `RustLifecycleFacts-v0`
for `VariableContext`.

The pilot must keep the adapter target-neutral and must not turn returned
borrow or snapshot/restore facts into Hako representation choices.

## Selected By

```text
296x-1491-POST-RUSTC-SEMIR-BINDING-CONTEXT-FACTS-EXTRACTION-OWNER-SELECTION-001
```

## Scope

```text
subject=hakorune_mir_builder::variable_context::VariableContext
reference_fixture=variable-context-adapter-facts-v0.json
output_kind=RustLifecycleAdapterFacts
```

Allowed:

```text
focused adapter output for VariableContext only
schema comparison against existing fixture
diagnostic report
shared helper extraction from BindingContext source extractor if needed
```

Forbidden:

```text
HakoLifecyclePlan-v0 output
.hako source output
Hako representation policy choices
BindingContext behavior change
backend behavior change
```

## Acceptance

```text
variable_context_facts_extraction_green=1
output_kind=RustLifecycleAdapterFacts
target_neutral_adapter=1
hako_policy_owner=0
binding_context_extraction_still_green=1
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_variable_context_facts_extraction_guard.sh
bash tools/checks/rustc_semir_binding_context_facts_extraction_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

```text
extractor=tools/rust_lifecycle/extract_variable_context_facts.py
shared_helper=tools/rust_lifecycle/context_fact_extraction.py
guard=tools/checks/rustc_semir_variable_context_facts_extraction_guard.sh
source=crates/hakorune_mir_builder/src/variable_context.rs
reference_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-facts-v0.json
```

The extractor reads the selected Rust source slice and derives the existing
target-neutral `RustLifecycleAdapterFacts` shape for `VariableContext`.

It verifies:

```text
VariableContext.variable_map is BTreeMap<String, ValueId>
BTreeMap implies deterministic_order_required=true
no impl Drop for VariableContext -> TrivialMemory
&self methods -> SharedRead / CallOnly
&mut self methods -> UniqueWrite / CallOnly
variable_map() returns owner-carrying shared reference metadata
variable_map_mut() returns owner-carrying unique reference metadata
snapshot() returns owned deterministic map metadata
restore(snapshot) consumes an owned deterministic map metadata
```

## Closeout

```text
variable_context_facts_extraction_green=1
output_kind=RustLifecycleAdapterFacts
target_neutral_adapter=1
hako_policy_owner=0
binding_context_extraction_still_green=1
backend_behavior_changed=0
```

Next:

```text
POST-RUSTC-SEMIR-VARIABLE-CONTEXT-FACTS-EXTRACTION-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_emit_hako_plan=1
do_not_emit_hako_source=1
do_not_choose_Hako_representation=1
do_not_change_BindingContext_extraction_behavior=1
```
