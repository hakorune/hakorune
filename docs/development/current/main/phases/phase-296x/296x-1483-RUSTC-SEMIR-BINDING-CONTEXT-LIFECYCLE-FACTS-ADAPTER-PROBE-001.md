# 296x-1483 RUSTC-SEMIR-BINDING-CONTEXT-LIFECYCLE-FACTS-ADAPTER-PROBE-001

Status: open
Date: 2026-06-20

## Purpose

Probe the first external rustc semantic adapter shape for producing
`RustLifecycleFacts-v0` over `BindingContext`.

This row must keep the adapter target-neutral. It must not emit
`HakoLifecyclePlan-v0`, `.hako`, or backend behavior.

## Selected By

```text
296x-1482-RUSTC-SEMIR-LIFECYCLE-FACTS-ADAPTER-PROBE-001
```

## Scope

```text
subject=hakorune_mir_builder::binding_context::BindingContext
expected_output=RustLifecycleFacts-v0
reference_fixture=binding-context-adapter-facts-v0.json
```

Allowed:

```text
adapter probe fixture or harness
schema validation against existing BindingContext adapter facts
diagnostic-only report
```

Forbidden:

```text
HakoLifecyclePlan-v0 emission
.hako source emission
raw rustc pretty dump as stable schema
Hako representation policy choice in adapter
backend behavior change
```

## Acceptance

```text
binding_context_adapter_probe_green=1
output_kind=RustLifecycleAdapterFacts
target_neutral_adapter=1
hako_policy_owner=0
raw_rustc_dump_as_schema=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_choose_OrderedMapBox_in_adapter=1
do_not_emit_lifecycle_plan=1
do_not_emit_hako_source=1
do_not_integrate_backend=1
```
