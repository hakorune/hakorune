# 296x-1535 EXPLICIT-CARRIERINFO-OWNED-SNAPSHOT-CONVERSION-001

Status: active
Date: 2026-06-21

## Purpose

Move `CarrierInfo::with_explicit_carriers` off the raw
`VariableContext::variable_map()` alias route and onto the same owned snapshot
contract as `CarrierInfo::from_variable_map`.

## Required Contract

```text
VariableContext::variable_map standalone conversion:
  Deny(ReturnedReadBorrow)

CarrierInfo::with_explicit_carriers known consumer:
  VariableContextApi.snapshot(ctx)
  -> CarrierInfoApi.with_explicit_carriers_from_snapshot(
       carrier_data,
       loop_var_name,
       loop_var_id,
       requested_names,
       snapshot
     )
```

## Acceptance

```text
raw variable_map alias generated=0
missing requested carrier fail-fast=green
requested names owned copy=green
snapshot/context/output alias isolation=green
generated Hako MIR=green
generated Hako EXE=green
```

## Stop Line

```text
do_not_reintroduce_return_ctx_variable_map=1
do_not_add_OrderedMapReadViewBox=1
do_not_open_phi_or_join_id_assignment=1
do_not_change_full_variable_context_claim=1
```
