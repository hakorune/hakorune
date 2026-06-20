# 296x-1500 RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-IMPLEMENTATION-001

Status: closed
Date: 2026-06-20

## Purpose

Add the standalone rustc semantic adapter tool skeleton and diagnostic-only
preflight guard.

This row must not extract lifecycle facts.

## Selected By

```text
296x-1499-POST-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-DESIGN-OWNER-SELECTION-001
```

## Scope

```text
tool=tools/rust_lifecycle/rustc_semir_adapter/
command=cargo run --manifest-path tools/rust_lifecycle/rustc_semir_adapter/Cargo.toml -- --preflight
guard=tools/checks/rustc_semir_adapter_tool_preflight_guard.sh
```

Allowed:

```text
standalone adapter tool Cargo.toml
diagnostic-only main.rs
preflight guard
root/product no-rustc-private check
```

Forbidden:

```text
RustLifecycleAdapterFacts generation
HakoLifecyclePlan-v0 output
.hako source output
product Cargo.toml rustc_private dependency
backend behavior change
```

## Acceptance

```text
adapter_tool_preflight_green=1
standalone_tool_manifest_exists=1
root_Cargo_rustc_private_dependency=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_tool_preflight_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

```text
tool_manifest=tools/rust_lifecycle/rustc_semir_adapter/Cargo.toml
tool_main=tools/rust_lifecycle/rustc_semir_adapter/src/main.rs
guard=tools/checks/rustc_semir_adapter_tool_preflight_guard.sh
```

The tool supports only:

```bash
cargo run --manifest-path tools/rust_lifecycle/rustc_semir_adapter/Cargo.toml -- --preflight
```

The preflight reports rustc version diagnostics and confirms no facts, Hako
plan, `.hako`, or backend behavior are produced.

## Closeout

```text
adapter_tool_preflight_green=1
standalone_tool_manifest_exists=1
root_Cargo_rustc_private_dependency=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Next:

```text
POST-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_generate_facts=1
do_not_emit_hako_plan=1
do_not_emit_hako_source=1
do_not_modify_product_Cargo_for_rustc_private=1
do_not_change_backend=1
```
