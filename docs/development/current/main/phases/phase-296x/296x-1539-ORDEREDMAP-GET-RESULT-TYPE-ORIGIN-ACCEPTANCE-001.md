# 296x-1539 ORDEREDMAP-GET-RESULT-TYPE-ORIGIN-ACCEPTANCE-001

Status: closed
Date: 2026-06-21

## Purpose

Preserve result type origin through `OrderedMapBox.get()` when the stored value
has a known object type, so native and generated smokes can inspect nested
objects without falling to `RuntimeDataBox`.

Current workaround:

```text
avoid:
  local names = info.get("carrier_names")
  names.get(0)

because:
  names.get(0) can become RuntimeDataBox.get with no route
```

Desired accepted shape:

```hako
local names = carrier_data.get("carrier_names")
names.get(0)
```

where `names` is known as `ArrayBox`.

## Scope

```text
BoxCount: one result-origin propagation shape
owner: value origin / route result box inference
input: OrderedMapBox.set(key, ArrayBox) followed by OrderedMapBox.get(key)
output: returned value keeps ArrayBox origin for subsequent method route
```

## Acceptance

```text
focused EXE smoke:
  OrderedMapBox.set("carrier_names", ArrayBox)
  local names = map.get("carrier_names")
  names.get(0)

expected:
  generated EXE green
  no RuntimeDataBox.get fallback widening
```

Result:

```text
orderedmap_get_result_type_origin=green
runtime_data_get_for_carrier_arrays=0
variable_context_carrier_snapshot generated EXE green
variable_context_explicit_carrier_snapshot generated EXE landed in 296x-1540
```

## Stop Line

```text
do_not_claim_general_dependent_map_typing=1
do_not_change_mixed_RuntimeDataBox_get_contract=1
do_not_infer_from_string_key_names_globally=1
do_not_add_requested_names_type_special_case=1
```
