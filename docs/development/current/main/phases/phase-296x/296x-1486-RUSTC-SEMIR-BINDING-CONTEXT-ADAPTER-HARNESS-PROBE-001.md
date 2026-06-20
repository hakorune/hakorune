# 296x-1486 RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-HARNESS-PROBE-001

Status: open
Date: 2026-06-20

## Purpose

Add the first minimal harness probe for BindingContext lifecycle facts.

The probe must produce or validate `RustLifecycleFacts-v0` only.

## Selected By

```text
296x-1485-RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-HARNESS-DESIGN-001
```

## Scope

```text
design=docs/development/current/main/design/rustc-semir-binding-context-adapter-harness-design.md
subject=hakorune_mir_builder::binding_context::BindingContext
reference_fixture=binding-context-adapter-facts-v0.json
```

Allowed:

```text
small harness/probe
fixture validation against RustLifecycleFacts-v0 shape
diagnostic report
```

Forbidden:

```text
HakoLifecyclePlan-v0 output
.hako source output
OrderedMapBox policy choice in adapter
raw rustc pretty dump as schema
backend behavior change
```

## Acceptance

```text
harness_probe_green=1
output_kind=RustLifecycleAdapterFacts
subject=BindingContext
adapter_policy_owner=0
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
do_not_emit_hako_plan=1
do_not_emit_hako_source=1
do_not_choose_Hako_representation=1
do_not_change_backend_behavior=1
```
