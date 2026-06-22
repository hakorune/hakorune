# 296x-1557 NULLABLE-MAP-VALUE-001

Status: landed
Date: 2026-06-22

## Purpose

Record the NullableMapValue decision without opening route selection or the
nightly rustc adapter path.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps nullable map payloads parked until an explicit missing-vs-null
carrier contract is named.

## Scope

```text
BoxCount: one consultation inventory
owner: NullableMapValue
input: option/null policy SSOTs and the negative converter corpus
output: one durable nullable map-value inventory and guard
```

## Decision

```text
keep NullableMapValue parked until an explicit missing-vs-null carrier contract is named
keep null-free Option and nullable map payload disambiguation separate
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_nullable_map_value_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_nullable_map_value_guard.sh
```

## Acceptance

```text
the nullable map value decision space is fixed in one machine-readable fixture
null-free Option remains separate from nullable map payload handling
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
