# 296x-1558 NON-ASCII-ORDERED-KEY-001

Status: landed
Date: 2026-06-22

## Purpose

Record the NonAsciiOrderedKey decision without opening route selection or the
nightly rustc adapter path.

The current contract remains inventory-only for this slice:

```text
inventory_only
```

That keeps non-ASCII key collation parked until a dedicated key collation
contract is named.

## Scope

```text
BoxCount: one consultation inventory
owner: NonAsciiOrderedKey
input: ordered-map boundary SSOT and task-order SSOT
output: one durable non-ASCII ordered-key inventory and guard
```

## Decision

```text
keep NonAsciiOrderedKey parked until a dedicated key collation contract is named
keep deterministic String-key ordering separate from non-ASCII collation questions
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_non_ascii_ordered_key_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_non_ascii_ordered_key_guard.sh
```

## Acceptance

```text
the non-ASCII ordered key decision space is fixed in one machine-readable fixture
String-only OrderedMapBox remains separate from non-ASCII collation policy
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
