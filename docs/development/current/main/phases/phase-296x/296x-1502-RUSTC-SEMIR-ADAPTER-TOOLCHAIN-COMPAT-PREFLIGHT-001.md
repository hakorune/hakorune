# 296x-1502 RUSTC-SEMIR-ADAPTER-TOOLCHAIN-COMPAT-PREFLIGHT-001

Status: open
Date: 2026-06-20

## Purpose

Add a diagnostic-only toolchain compatibility preflight for the standalone
rustc semantic adapter tool.

This row must not extract facts.

## Selected By

```text
296x-1501-POST-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-OWNER-SELECTION-001
```

## Scope

```text
tool=tools/rust_lifecycle/rustc_semir_adapter/
command=--toolchain-preflight
guard=tools/checks/rustc_semir_adapter_toolchain_compat_guard.sh
```

Allowed:

```text
rustc version/channel/sysroot diagnostics
rustc_private readiness classification
clear fail-fast report for unsupported toolchain
```

Forbidden:

```text
RustLifecycleAdapterFacts generation
HIR/THIR/MIR extraction
HakoLifecyclePlan-v0 output
.hako source output
source-shape fallback
backend behavior change
```

## Acceptance

```text
toolchain_compat_preflight_green=1
rustc_version_reported=1
rustc_channel_classified=1
rustc_private_readiness_reported=1
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_toolchain_compat_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_generate_facts=1
do_not_extract_HIR_THIR_MIR=1
do_not_fallback_to_source_shape=1
do_not_change_backend=1
```
