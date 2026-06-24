# 296x-1575 BOX-COMPILATION-CONTEXT-CRATE-SMOKE-PROBE-VERIFICATION-BUNDLE-RUN-001

Status: landed
Date: 2026-06-22

## Purpose

Run the landed BoxCompilationContext crate-smoke consultation bundle without
opening the actual probe.

## Scope

```text
BoxCount: one consultation execution
owner: MirBuilder crate-smoke probe verification bundle run
input: landed verification bundle
output: durable bundle execution guard and run entry
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
bash tools/checks/rust_mirbuilder_box_compilation_context_crate_smoke_probe_verification_bundle_guard.sh
```

## Acceptance

```text
the representative verification bundle is executable in one guarded entry
the probe candidate remains explicit
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
