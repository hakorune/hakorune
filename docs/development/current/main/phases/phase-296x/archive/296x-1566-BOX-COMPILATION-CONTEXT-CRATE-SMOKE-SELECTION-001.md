# 296x-1566 BOX-COMPILATION-CONTEXT-CRATE-SMOKE-SELECTION-001

Status: landed
Date: 2026-06-22

## Purpose

Select the first crate-level probe candidate after the landed
BoxCompilationContext readiness inventory.

## Scope

```text
BoxCount: one consultation selection
owner: MirBuilder crate-smoke probe selection
input: landed BoxCompilationContext readiness inventory
output: durable crate-level probe candidate selection and guard
```

## Observed State

```text
subject=hakorune_mir_builder::context::BoxCompilationContext
current_landed_slice=BoxCompilationContext_ctor_is_empty_only
crate_level_probe_candidate=BoxCompilationContext
crate_level_probe_opened=0
nightly_rustc_adapter_opened=0
route_selection_opened=0
runtime_fallback_opened=0
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_box_compilation_context_crate_smoke_selection.py --check-reference
bash tools/checks/rust_mirbuilder_box_compilation_context_crate_smoke_selection_guard.sh
```

## Acceptance

```text
the crate-level probe candidate is fixed in one machine-readable fixture
the landed BoxCompilationContext slice remains the subject
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
