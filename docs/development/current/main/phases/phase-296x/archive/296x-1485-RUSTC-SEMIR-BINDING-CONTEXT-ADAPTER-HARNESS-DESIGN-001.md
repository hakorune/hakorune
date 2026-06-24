# 296x-1485 RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-HARNESS-DESIGN-001

Status: closed
Date: 2026-06-20

## Purpose

Design the smallest external rustc semantic adapter harness boundary for
producing `RustLifecycleFacts-v0` over `BindingContext`.

This row must not implement the adapter.

## Selected By

```text
296x-1484-POST-RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-PROBE-OWNER-SELECTION-001
```

## Design Questions

```text
input:
  one selected Rust source item / module slice

adapter output:
  RustLifecycleFacts-v0 JSON only

stable schema:
  repo-owned JSON vocabulary

forbidden source:
  raw rustc pretty MIR / THIR debug dump as schema

forbidden output:
  HakoLifecyclePlan-v0
  .hako source
  backend lowering
  Hako representation policy choices
```

## Acceptance

```text
harness_boundary_documented=1
adapter_output_contract=RustLifecycleFacts-v0
toolchain_boundary_documented=1
raw_rustc_dump_as_schema=0
adapter_policy_owner=0
implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Design

```text
design=docs/development/current/main/design/rustc-semir-binding-context-adapter-harness-design.md
```

## Closeout

```text
harness_boundary_documented=1
adapter_output_contract=RustLifecycleFacts-v0
toolchain_boundary_documented=1
raw_rustc_dump_as_schema=0
adapter_policy_owner=0
implementation_started=0
backend_behavior_changed=0

selected_next_task=RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-HARNESS-PROBE-001
```

## Stop Line

```text
do_not_call_rustc_in_this_design_row=1
do_not_emit_hako_plan_from_adapter=1
do_not_emit_hako_source_from_adapter=1
do_not_select_Hako_representation_in_adapter=1
```
