# 296x-1562 CORE-CONTEXT-SCALAR-COUNTER-PLAN-ORACLE-001

Status: landed
Date: 2026-06-22

## Purpose

Record the CoreContext scalar-counter plan/oracle boundary without opening
route selection or the nightly rustc adapter path.

This row stays consultation-only:

```text
inventory_only
```

That keeps the bounded plan/oracle question parked while the next easy-tier
follow-up remains a separate planning decision.

## Scope

```text
BoxCount: one consultation inventory
owner: CoreContext scalar-counter plan/oracle
input: CoreContext readiness inventory and scalar-counter vocabulary fixture
output: one durable plan/oracle inventory and guard
```

## Decision

```text
keep CoreContext scalar-counter plan/oracle parked until bounded plan and oracle fixtures are named
keep the plan/oracle question separate from route selection and nightly rustc adapter work
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_core_context_plan_oracle_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_core_context_plan_oracle_guard.sh
bash tools/checks/rust_mirbuilder_core_context_readiness_guard.sh
bash tools/checks/rust_mirbuilder_core_context_scalar_counter_vocabulary_guard.sh
```

## Acceptance

```text
core_context_plan_oracle_recorded=1
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
