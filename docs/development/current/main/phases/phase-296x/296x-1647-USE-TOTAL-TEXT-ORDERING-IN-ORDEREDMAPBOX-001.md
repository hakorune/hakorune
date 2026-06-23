---
Status: Landed
Date: 2026-06-23
Scope: OrderedMapBox consumption of RustStringOrdV1 ordering capability
---

# 296x-1647: Use Total Text Ordering In OrderedMapBox

## Result

`OrderedMapBox.set` now uses the generic text ordering capability:

```text
TextOrder.compare_rust_string_v1
```

The local comparator expression was removed from `OrderedMapBox`.

## Guard

```bash
bash tools/checks/rust_lifecycle_ordered_map_text_order_guard.sh
```

The guard verifies:

```text
ordered_map_uses_text_order=1
ordered_map_local_compare=0
ordered_map_exe=green
```

Behavior coverage includes:

```text
a,args,b,c lexical order
update existing key
remove
clear
fresh instance isolation
```

## Next Blocker

```text
LOWER-REGION-OBSERVER-SOURCE-ORDERED-READ-FOLD-001
```

Next task:

```text
Use KeyAscending(RustStringOrdV1) plus comparator proof to lower the
RegionObserver variable_map().iter() read-fold.
```
