# 296x-1490 RUSTC-SEMIR-BINDING-CONTEXT-LIFECYCLE-FACTS-EXTRACTION-PILOT-001

Status: closed
Date: 2026-06-20

## Purpose

Produce the first real adapter output toward `RustLifecycleFacts-v0` for
`BindingContext`.

The pilot must keep the adapter target-neutral.

## Selected By

```text
296x-1489-POST-RUSTC-SEMIR-BINDING-CONTEXT-TOOLCHAIN-PREFLIGHT-OWNER-SELECTION-001
```

## Scope

```text
subject=hakorune_mir_builder::binding_context::BindingContext
reference_fixture=binding-context-adapter-facts-v0.json
output_kind=RustLifecycleAdapterFacts
```

Allowed:

```text
focused adapter output for BindingContext only
schema comparison against existing fixture
diagnostic report
```

Forbidden:

```text
HakoLifecyclePlan-v0 output
.hako source output
Hako representation policy choices
VariableContext facts
backend behavior change
```

## Acceptance

```text
binding_context_facts_extraction_green=1
output_kind=RustLifecycleAdapterFacts
target_neutral_adapter=1
hako_policy_owner=0
variable_context_facts_generated=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_binding_context_facts_extraction_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

```text
extractor=tools/rust_lifecycle/extract_binding_context_facts.py
guard=tools/checks/rustc_semir_binding_context_facts_extraction_guard.sh
source=crates/hakorune_mir_builder/src/binding_context.rs
reference_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json
```

The extractor reads the selected Rust source slice and derives the existing
target-neutral `RustLifecycleAdapterFacts` shape for `BindingContext`.

It verifies:

```text
BindingContext.binding_map is BTreeMap<String, BindingId>
BTreeMap implies deterministic_order_required=true
no impl Drop for BindingContext -> TrivialMemory
&self methods -> SharedRead / CallOnly
&mut self methods -> UniqueWrite / CallOnly
insert(name: String, binding_id: BindingId) -> ConsumeArgument
lookup/remove returning Option<BindingId> -> ImmediateValue / TrivialMemory
```

## Closeout

```text
binding_context_facts_extraction_green=1
output_kind=RustLifecycleAdapterFacts
target_neutral_adapter=1
hako_policy_owner=0
variable_context_facts_generated=0
backend_behavior_changed=0
```

Next:

```text
POST-RUSTC-SEMIR-BINDING-CONTEXT-FACTS-EXTRACTION-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_emit_hako_plan=1
do_not_emit_hako_source=1
do_not_choose_Hako_representation=1
do_not_widen_to_VariableContext=1
```
