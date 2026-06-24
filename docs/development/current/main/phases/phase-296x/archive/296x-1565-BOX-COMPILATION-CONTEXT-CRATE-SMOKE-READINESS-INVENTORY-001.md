# 296x-1565 BOX-COMPILATION-CONTEXT-CRATE-SMOKE-READINESS-INVENTORY-001

Status: landed
Date: 2026-06-22

## Purpose

Keep the remaining easy-tier smoke readiness in a machine-readable inventory
before any crate-level probe is opened.

## Scope

```text
BoxCount: one consultation inventory
owner: MirBuilder crate-smoke readiness inventory
input: landed BoxCompilationContext slice plus current easy-tier backlog
output: durable readiness inventory and guard
```

## Observed State

```text
subject=hakorune_mir_builder::context::BoxCompilationContext
current_landed_slice=BoxCompilationContext_ctor_is_empty_only
remaining_easy_tier_consultation_candidates=CoreContext, TypeContext, MetadataContext
crate_level_probe_opened=0
nightly_rustc_adapter_opened=0
route_selection_opened=0
runtime_fallback_opened=0
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_box_compilation_context_crate_smoke_readiness_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_box_compilation_context_crate_smoke_readiness_guard.sh
```

## Acceptance

```text
the readiness inventory is fixed in one machine-readable fixture
the current landed slice is explicit
remaining easy-tier consultation candidates are explicit
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
