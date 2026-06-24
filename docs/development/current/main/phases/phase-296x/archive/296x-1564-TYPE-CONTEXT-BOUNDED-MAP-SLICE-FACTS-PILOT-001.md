# 296x-1564 TYPE-CONTEXT-BOUNDED-MAP-SLICE-FACTS-PILOT-001

Status: landed
Date: 2026-06-22

## Purpose

Record the TypeContext bounded map-slice facts pilot boundary without opening
route selection or the nightly rustc adapter path.

This row stays consultation-only:

```text
inventory_only
```

That keeps the bounded facts pilot parked while the next easy-tier follow-up
remains a separate planning decision.

## Scope

```text
BoxCount: one consultation inventory
owner: TypeContext bounded map-slice facts pilot
input: TypeContext readiness inventory and bounded map-slice fixture
output: one durable facts-pilot inventory and guard
```

## Decision

```text
keep TypeContext bounded map slice facts pilot parked until a bounded facts extractor is named
keep the pilot separate from route selection and nightly rustc adapter work
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_type_context_bounded_map_slice_facts_pilot.py --check-reference
bash tools/checks/rust_mirbuilder_type_context_bounded_map_slice_facts_pilot_guard.sh
```

## Acceptance

```text
type_context_bounded_map_slice_facts_pilot_recorded=1
route_selection=0
nightly_rustc_adapter=0
summary=ok
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```
