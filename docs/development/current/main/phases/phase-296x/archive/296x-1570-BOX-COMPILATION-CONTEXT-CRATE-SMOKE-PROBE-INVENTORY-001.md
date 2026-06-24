# 296x-1570 BOX-COMPILATION-CONTEXT-CRATE-SMOKE-PROBE-INVENTORY-001

Status: landed
Date: 2026-06-22

## Purpose

Inventory the representative BoxCompilationContext crate-smoke probe surface
after the command contract is fixed.

## Scope

```text
BoxCount: one consultation inventory
owner: MirBuilder crate-smoke probe inventory
input: landed readiness inventory + probe selection + harness owner selection + harness design + command contract
output: durable probe inventory and guard
```

## Observed State

```text
subject=hakorune_mir_builder::context::BoxCompilationContext
current_landed_slice=BoxCompilationContext_ctor_is_empty_only
crate_level_probe_candidate=BoxCompilationContext
selected_next_owner=representative crate smoke probe inventory
crate_level_probe_opened=0
nightly_rustc_adapter_opened=0
route_selection_opened=0
runtime_fallback_opened=0
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_box_compilation_context_crate_smoke_probe_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_box_compilation_context_crate_smoke_probe_inventory_guard.sh
```

## Acceptance

```text
the representative probe surface is fixed in one machine-readable fixture
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
