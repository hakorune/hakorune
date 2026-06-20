# 296x-1482 RUSTC-SEMIR-LIFECYCLE-FACTS-ADAPTER-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Select the first external rustc semantic adapter probe for producing
`RustLifecycleFacts-v0`.

This row must not use raw rustc debug dumps as the stable schema and must not
let the adapter choose Hako representation policy.

## Selected By

```text
296x-1481-RUST-TO-HAKO-LIFECYCLE-PARITY-GATE-001
```

## Scope

```text
candidate_subject:
  BindingContext or VariableContext focused slice

adapter_output:
  RustLifecycleFacts-v0 only

forbidden_output:
  HakoLifecyclePlan-v0
  .hako source
  backend lowering
  representation policy choices
```

## Acceptance

```text
probe_subject_selected=1
stable_fact_schema_boundary_documented=1
adapter_policy_owner=0
raw_rustc_dump_as_schema=0
implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Selection

```text
selected_subject=BindingContext
selected_next_task=RUSTC-SEMIR-BINDING-CONTEXT-LIFECYCLE-FACTS-ADAPTER-PROBE-001
selected_reason=BindingContext has the smallest focused lifecycle fact shape:
BTreeMap field, shared-read methods, unique-write mutation methods,
ImmediateValue BindingId, TrivialMemory Drop, and no returned borrow boundary.
VariableContext is parked because it includes returned shared/mutable map,
snapshot/restore, and carrier consumers.
implementation_started=0
```

## Closeout

```text
probe_subject_selected=1
stable_fact_schema_boundary_documented=1
adapter_policy_owner=0
raw_rustc_dump_as_schema=0
implementation_started=0
backend_behavior_changed=0
```

## Stop Line

```text
do_not_start_rustc_integration_without_subject_selection=1
do_not_emit_HakoLifecyclePlan_from_adapter=1
do_not_emit_hako_source_from_adapter=1
do_not_parse_raw_pretty_mir_as_stable_schema=1
```
