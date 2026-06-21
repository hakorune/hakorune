# 296x-1534 ORDEREDMAP-PARAM-RECEIVER-ROUTE-ACCEPTANCE-001

Status: landed
Date: 2026-06-21

## Purpose

Unblock the owned-snapshot `CarrierInfo` converter slice by accepting one
backend receiver shape:

```text
static method parameter typed as OrderedMapBox
  -> parameter.set/get/keys/values/length method calls
  -> EXE backend route resolves the receiver origin
```

This is a BoxCount blocker. Do not change the converter ownership contract in
this row.

## Current Split

```text
Meaning/converter layer:
  ReturnedReadBorrow is resolved as NoReturnedAlias +
  OwnedReadSnapshotProjection.

  VariableContext::variable_map standalone conversion:
    Deny(ReturnedReadBorrow)

  known CarrierInfo bulk read consumer:
    VariableContextApi.snapshot(ctx)
    -> CarrierInfoApi.from_snapshot(carrier_data, loop_var_name, snapshot)

Backend acceptance layer:
  generated .hako reaches MIR.
  EXE currently fails on OrderedMapBox method calls through typed static
  method parameters.
```

## Current Failure

```text
command:
  bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh

failure:
  unsupported pure shape for current backend recipe
  first_op=mir_call
  reason=mir_call_no_route
  callee_symbol=set
  next_check_hint=check_callee_route_or_receiver_origin

shape:
  CarrierInfoApi.from_snapshot(
      carrier_data: OrderedMapBox,
      loop_var_name,
      snapshot: OrderedMapBox
  )

  carrier_data.set(...)
```

## Task Order

1. Add a focused backend/MIR acceptance fixture for method calls on typed
   `OrderedMapBox` parameters.
2. Extend receiver-origin route recovery so the pure EXE route recognizes the
   typed parameter as an `OrderedMapBox` receiver.
3. Keep the unsupported-shape diagnostic stable and retain the callee /
   receiver-origin hint.
4. Re-run the owned-snapshot `CarrierInfo` artifact guard and verify EXE alias
   isolation.
5. Close the WIP converter commit only after the artifact guard is green.

## Acceptance

```text
bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh

expected:
  generated_hako_mir_emit=green
  generated_hako_exe=green
  owned_snapshot_alias_isolation=green
  publishes_variable_map=0
  returned_read_borrow_deny=green
```

## Stop Line

```text
do_not_reintroduce_return_ctx_variable_map=1
do_not_add_OrderedMapReadViewBox=1
do_not_use_runtime_try_hako_then_rust_fallback=1
do_not_weaken_owned_snapshot_alias_proof=1
do_not_open_nightly_rustc_adapter=1
do_not_expand_to_phi_or_loop_lowering=1
```
