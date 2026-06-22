# 296x-1550 RETURNED-MUTABLE-BORROW-REPLACEMENT-DECISION-001

Status: landed
Date: 2026-06-22

## Purpose

Record the design stop for `VariableContext::variable_map_mut()` without
selecting a replacement route.

The mutable borrow boundary is already denied. This row keeps the replacement
decision space explicit while leaving route selection unopened.

## Scope

```text
BoxCount: one consultation inventory
owner: VariableContext returned mutable borrow replacement decision
input: current VariableContext mutable-borrow boundary shape
output: one durable decision inventory and guard
```

## Current Boundary

```text
VariableContext::variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId>
```

## Candidate Replacements

```text
explicit mutation APIs
bounded with-map operation
ReplaceOwned-style ownership transfer
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_returned_mutable_borrow_replacement_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_returned_mutable_borrow_replacement_guard.sh
```

## Acceptance

```text
the replacement decision space is fixed in one machine-readable fixture
the mutable borrow boundary remains denied
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
