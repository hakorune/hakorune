# 296x-1488 RUSTC-SEMIR-BINDING-CONTEXT-TOOLCHAIN-PREFLIGHT-001

Status: closed
Date: 2026-06-20

## Purpose

Add the smallest toolchain preflight for a future BindingContext rustc semantic
adapter.

This row checks the external adapter entry boundary only. It must not extract
real lifecycle facts from rustc yet.

## Selected By

```text
296x-1487-POST-RUSTC-SEMIR-BINDING-CONTEXT-HARNESS-PROBE-OWNER-SELECTION-001
```

## Scope

```text
subject=BindingContext
goal=toolchain / adapter entry availability
output=diagnostic preflight report
```

Allowed:

```text
toolchain presence check
adapter crate / command discovery
diagnostic-only report
```

Forbidden:

```text
raw rustc pretty dump as stable schema
RustLifecycleFacts-v0 generation
HakoLifecyclePlan-v0 output
.hako source output
backend behavior change
```

## Acceptance

```text
toolchain_preflight_green=1
adapter_entry_identified=1
raw_rustc_dump_as_schema=0
lifecycle_facts_generated=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
toolchain_preflight_green=1
adapter_entry_identified=1
raw_rustc_dump_as_schema=0
lifecycle_facts_generated=0
backend_behavior_changed=0
```

Evidence:

```bash
bash tools/checks/rustc_semir_binding_context_toolchain_preflight_guard.sh
```

Guard output:

```text
output_contract=rustc-semir-binding-context-toolchain-preflight-v0
toolchain_preflight_green=1
adapter_entry_identified=1
cargo_available=1
rustc_available=1
lifecycle_facts_generated=0
backend_behavior_changed=0
summary=ok
```

Next:

```text
296x-1489-POST-RUSTC-SEMIR-BINDING-CONTEXT-TOOLCHAIN-PREFLIGHT-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_extract_lifecycle_facts_in_preflight=1
do_not_parse_rustc_pretty_dump_as_schema=1
do_not_emit_hako_plan=1
do_not_change_backend_behavior=1
```
