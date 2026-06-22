# 296x-1560 CORE-CONTEXT-PILOT-SELECTION-001

Status: landed
Date: 2026-06-22

## Purpose

Select the next easy-tier family pilot after the CoreContext readiness
inventory.

CoreContext is the next bounded candidate. This row only selects the pilot;
it does not open route selection, behavior generation, or the nightly rustc
adapter path.

## Scope

```text
BoxCount: one consultation selection
owner: CoreContext pilot selection
input: CoreContext readiness inventory and scalar-counter vocabulary fixtures
output: one durable pilot selection fixture and guard
```

## Decision

```text
select CoreContext as the next easy-tier family pilot
keep the pilot bounded to the scalar-counter slice
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_core_context_pilot_selection.py --check-reference
bash tools/checks/rust_mirbuilder_core_context_pilot_selection_guard.sh
```

## Acceptance

```text
selected_next_pilot=CoreContext
pilot_scope=CoreContext_scalar_counter_only
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
