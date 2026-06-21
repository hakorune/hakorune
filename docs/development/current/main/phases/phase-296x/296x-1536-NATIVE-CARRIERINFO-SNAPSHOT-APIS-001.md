# 296x-1536 NATIVE-CARRIERINFO-SNAPSHOT-APIS-001

Status: landed
Date: 2026-06-21

## Purpose

Adopt native `.hako` CarrierInfo snapshot APIs after the generated owned
snapshot carrier paths are green.

## Required Contract

```text
VariableContextNativeApi.snapshot(ctx)
  -> CarrierInfoNativeApi.from_snapshot(...)

VariableContextNativeApi.snapshot(ctx)
  -> CarrierInfoNativeApi.with_explicit_carriers_from_snapshot(...)
```

The native APIs must not expose raw `OrderedMapBox` read aliases through
`VariableContext::variable_map()`.

## Acceptance

```text
apps/lib/hakorune_mir_builder/carrier_info.hako exists
from_snapshot native EXE=green
with_explicit_carriers_from_snapshot native EXE=green
missing requested carrier fail-fast=green
snapshot/context/output alias isolation=green
raw variable_map alias generated=0
```

## Result

```text
apps/lib/hakorune_mir_builder/carrier_info.hako=added
from_snapshot native EXE=green
with_explicit_carriers_from_snapshot native EXE=green
missing requested carrier fail-fast=green
snapshot/context/output alias isolation=green
raw variable_map alias=0
OrderedMapReadViewBox=0
```

## Stop Line

```text
do_not_add_OrderedMapReadViewBox=1
do_not_open_phi_or_join_id_assignment=1
do_not_change_full_variable_context_claim=1
do_not_use_runtime_try_hako_then_rust_fallback=1
```
