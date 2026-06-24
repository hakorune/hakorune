# 296x-1552 CARRIER-SENSITIVE-ALIAS-PROOF-001

Status: landed
Date: 2026-06-22

## Purpose

Record the carrier-sensitive alias proof decision without opening a true
carrier-sensitive route.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps carrier-sensitive consumers parked until a separate hard-tier
contract names the alias model.

## Scope

```text
BoxCount: one consultation inventory
owner: CarrierSensitiveAlias proof
input: current carrier-sensitive readiness inventory and read-view decision
output: one durable proof inventory and guard
```

## Decision

```text
keep carrier-sensitive consumers parked until a separate hard-tier contract names the alias model
keep read-only BorrowView and owned snapshot readiness separate
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_carrier_sensitive_alias_proof_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_carrier_sensitive_alias_proof_guard.sh
```

## Acceptance

```text
the carrier-sensitive alias proof space is fixed in one machine-readable fixture
NoReturnedAlias remains the current read contract
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
