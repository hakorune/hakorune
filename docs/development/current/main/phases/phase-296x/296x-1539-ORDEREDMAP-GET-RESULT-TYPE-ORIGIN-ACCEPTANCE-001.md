# 296x-1539 ORDEREDMAP-GET-RESULT-TYPE-ORIGIN-ACCEPTANCE-001

Status: pending
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

## Stop Line

```text
do_not_claim_general_dependent_map_typing=1
do_not_change_mixed_RuntimeDataBox_get_contract=1
do_not_infer_from_string_key_names_globally=1
```
