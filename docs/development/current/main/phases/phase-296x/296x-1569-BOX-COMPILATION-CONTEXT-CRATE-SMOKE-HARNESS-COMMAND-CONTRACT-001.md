# 296x-1569 BOX-COMPILATION-CONTEXT-CRATE-SMOKE-HARNESS-COMMAND-CONTRACT-001

Status: landed
Date: 2026-06-22

## Purpose

Define the minimal crate-smoke harness command contract after the harness
design is fixed.

## Scope

```text
BoxCount: one consultation command contract
owner: MirBuilder crate-smoke harness command contract
input: landed readiness inventory + probe selection + harness owner selection + harness design
output: durable command contract and guard
```

## Observed State

```text
subject=hakorune_mir_builder::context::BoxCompilationContext
current_landed_slice=BoxCompilationContext_ctor_is_empty_only
crate_level_probe_candidate=BoxCompilationContext
selected_next_owner=minimal crate smoke harness command contract
crate_level_probe_opened=0
nightly_rustc_adapter_opened=0
route_selection_opened=0
runtime_fallback_opened=0
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_box_compilation_context_crate_smoke_harness_command_contract.py --check-reference
bash tools/checks/rust_mirbuilder_box_compilation_context_crate_smoke_harness_command_contract_guard.sh
```

## Acceptance

```text
the thin command sequence is fixed in one machine-readable fixture
the probe candidate remains explicit
the harness owner remains explicit
crate-level probe remains unopened
nightly rustc adapter remains unopened
route selection remains unopened
runtime fallback remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```
