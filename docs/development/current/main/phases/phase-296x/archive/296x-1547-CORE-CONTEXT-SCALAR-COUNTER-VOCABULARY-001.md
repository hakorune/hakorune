# 296x-1547 CORE-CONTEXT-SCALAR-COUNTER-VOCABULARY-001

Status: landed
Date: 2026-06-22

## Purpose

Record the exact scalar-counter vocabulary that the CoreContext easy-tier facts
slice exposes, without opening route selection or Hako lifecycle planning.

This is a consultation-only row. It keeps the next easy-tier stop explicit:

```text
scalar counter field initialization
increment / saturating_add
ID constructor calls
struct-return construction
```

## Scope

```text
BoxCount: one consultation inventory
owner: MirBuilder CoreContext scalar-counter vocabulary
input: extracted CoreContext facts fixture
output: one durable vocabulary summary and guard
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_core_context_scalar_counter_vocabulary_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_core_context_scalar_counter_vocabulary_guard.sh
bash tools/checks/rust_mirbuilder_core_context_readiness_guard.sh
```

## Acceptance

```text
the CoreContext scalar-counter vocabulary is fixed in one machine-readable fixture
the allowed operations stay limited to the easy-tier scalar-counter shapes
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
