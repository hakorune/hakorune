# 296x-1561 CORE-CONTEXT-SCALAR-COUNTER-FACTS-PILOT-001

Status: landed
Date: 2026-06-22

## Purpose

Record the CoreContext scalar-counter facts pilot without opening route
selection or the nightly rustc adapter path.

This row stays consultation-only:

```text
inventory_only
```

That keeps the bounded CoreContext facts slice parked while the next easy-tier
follow-up remains a separate planning decision.

## Scope

```text
BoxCount: one consultation inventory
owner: CoreContext scalar-counter facts pilot
input: CoreContext facts fixture, readiness inventory, scalar-counter vocabulary fixture
output: one durable facts pilot inventory and guard
```

## Decision

```text
keep CoreContext scalar-counter facts pilot bounded to the extracted facts slice
keep the pilot separate from route selection and nightly rustc adapter work
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/extract_core_context_facts.py --check-reference
bash tools/checks/rust_lifecycle_core_context_facts_guard.sh
bash tools/checks/rust_mirbuilder_core_context_scalar_counter_vocabulary_guard.sh
bash tools/checks/rust_mirbuilder_core_context_readiness_guard.sh
```

## Acceptance

```text
core_context_facts_extraction_green=1
core_context_scalar_counter_vocabulary_recorded=1
core_context_readiness_recorded=1
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
