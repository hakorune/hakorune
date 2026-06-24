# 296x-1556 UNSAFE-OR-FFI-001

Status: landed
Date: 2026-06-22

## Purpose

Record the UnsafeOrFFI decision without opening route selection or the
nightly rustc adapter path.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps broad unsafe surface and FFI parked until a restricted unsafe
capability contract or explicit CompatShim row is named.

## Scope

```text
BoxCount: one consultation inventory
owner: UnsafeOrFFI
input: substrate capability ladder, hako lifecycle plan, ownership reference
output: one durable unsafe / FFI inventory and guard
```

## Decision

```text
keep UnsafeOrFFI parked until a restricted unsafe capability contract or explicit CompatShim row is named
keep broad unsafe surface and FFI separate from the easy-tier converter
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_unsafe_or_ffi_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_unsafe_or_ffi_guard.sh
```

## Acceptance

```text
the unsafe / FFI decision space is fixed in one machine-readable fixture
broad unsafe surface remains parked
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
