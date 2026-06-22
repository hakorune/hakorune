# 296x-1559 CONSTRUCTOR-LIFECYCLE-MISMATCH-001

Status: landed
Date: 2026-06-22

## Purpose

Record the ConstructorLifecycleMismatch decision without opening route
selection or the nightly rustc adapter path.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps field-initializer versus birth-time constructor questions parked
until a dedicated contract is named.

## Scope

```text
BoxCount: one consultation inventory
owner: ConstructorLifecycleMismatch
input: constructor lifecycle SSOTs plus BoxCompilationContext plan / recipe / verifier fixtures
output: one durable constructor lifecycle inventory and guard
```

## Decision

```text
keep ConstructorLifecycleMismatch parked until a dedicated field-initializer-vs-birth contract is named
keep declaration-site stored field initializers separate from birth-time constructor logic
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_constructor_lifecycle_mismatch_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_constructor_lifecycle_mismatch_guard.sh
```

## Acceptance

```text
the constructor lifecycle decision space is fixed in one machine-readable fixture
birth-time constructor logic remains separate from stored field initializers
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
