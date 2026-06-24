# 296x-1651: Boxed Sum I64 Payload ABI

Status: Complete
Date: 2026-06-24
Token: BOXED-SUM-I64-PAYLOAD-ABI-001

## Decision

Select generic boxed-sum scalar payload support as the next semantic slice.

```text
owner:
  boxed sum payload storage class = I64

canonical MIR:
  VariantMake / VariantTag / VariantProject unchanged

representation:
  BoxedSumAbiPlanV2
  payload_storage=None|I64|Handle
  site consumes abi_plan_id
```

This is not an `Option<i64>` or `MetadataContext` special case.

## Scope

```text
add I64 payload storage to boxed sum runtime ABI
prove None != Some(0)
prove Some(-1) projects as scalar -1, not handle
keep unit enum and handle-payload regressions green
allow Option-shape [None, I64] and [None, Handle] in the same module
```

## Non-Claims

```text
general Option payload support = 0
F64 payload support = 0
converter family addition = 0
MetadataContext region-parent AOT reopen = 0
new Hako syntax = 0
runtime fallback = 0
```

## Next If Green

```text
METADATA-CONTEXT-REGION-PARENT-AOT-REOPEN-001
```

## Closeout Evidence

```text
BoxedSumAbiPlanV2 shape key = green
payload_storage=None|I64|Handle = green
canonical MIR instruction changes = 0
Option-specific backend branch = 0
MetadataContext-specific backend branch = 0
unit enum cross-function EXE/AOT = green
handle payload cross-function EXE/AOT = green
MapBox boxed enum roundtrip EXE/AOT = green
MapBox Option boxed enum roundtrip EXE/AOT = green
I64 payload cross-function EXE/AOT = green
Option-shape [None,I64] and [None,Handle] same-module coexist = green
Some(-1) projects as scalar -1 = green
runtime fallback = 0
```
