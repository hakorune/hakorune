# 296x-1554 LOOP-TRIM-ROUTE-LOWERING-001

Status: landed
Date: 2026-06-22

## Purpose

Record the loop / trim route lowering decision without opening executable
lowering, route selection, or the nightly rustc adapter path.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps trim route lowering parked until a concrete trim fixture is
selected.

## Scope

```text
BoxCount: one consultation inventory
owner: Loop / trim route lowering
input: trim route lowering inventory + readiness gate + route-boundary probe
output: one durable route-lowering inventory and guard
```

## Decision

```text
keep trim route lowering parked until a concrete trim fixture is selected
keep route-boundary readiness and boundary probes separate from executable lowering
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_loop_trim_route_lowering_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_loop_trim_route_lowering_guard.sh
```

## Acceptance

```text
the loop/trim route lowering space is fixed in one machine-readable fixture
route selection remains unopened
nightly rustc adapter remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```
