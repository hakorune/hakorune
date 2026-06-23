---
Status: Landed
Date: 2026-06-23
Scope: RustStringOrdV1 total text ordering capability acceptance
---

# 296x-1646: Backend Accepted Total Text Ordering Capability

## Result

`TextOrder.compare_rust_string_v1` is now an explicit generic text ordering
capability:

```text
CompareTotal(RustStringOrdV1)
```

Acceptance covers:

```text
equal
less
greater
prefix short/long
non-ASCII ordering
```

## Guard

```bash
bash tools/checks/rust_lifecycle_text_order_rust_string_ord_v1_guard.sh
```

The guard verifies:

```text
vm_reference=green
mir_emit=green
exe_aot=green
ordered_map_special_case=0
region_observer_special_case=0
```

## Boundary

This row does not generate the RegionObserver artifact and does not add
collection-specific backend branches.

## Next Blocker

```text
USE-TOTAL-TEXT-ORDERING-IN-ORDEREDMAPBOX-001
```

Next task:

```text
Replace OrderedMapBox local string comparison with TextOrder.compare_rust_string_v1
and keep ordered-map behavior green.
```
