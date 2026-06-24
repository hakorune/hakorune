# 296x-1555 NONTRIVIAL-DROP-001

Status: landed
Date: 2026-06-22

## Purpose

Record the NonTrivialDrop decision without opening route selection or the
nightly rustc adapter path.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps nontrivial Drop parked until a positive TrivialMemory or
verifier-approved cleanup contract is named.

## Scope

```text
BoxCount: one consultation inventory
owner: NonTrivialDrop
input: drop boundary SSOTs and existing TrivialMemory pilots
output: one durable drop inventory and guard
```

## Decision

```text
keep NonTrivialDrop parked until a positive TrivialMemory or verifier-approved cleanup contract is named
keep nontrivial Drop separate from the simple-map and snapshot/restore pilots that already require TrivialMemory
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_nontrivial_drop_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_nontrivial_drop_guard.sh
```

## Acceptance

```text
the NonTrivialDrop decision space is fixed in one machine-readable fixture
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
