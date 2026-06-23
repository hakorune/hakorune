---
Status: Landed
Date: 2026-06-23
Scope: MirBuilder Rust-to-Hako converter route selection
---

# 296x-1643: RegionObserver SourceOrdered Read-Fold Deny

## Decision

The RegionObserver `variable_map().iter()` slice is now mechanically denied
before artifact generation.

This closes the route-selection blocker without generating `.hako` that would
substitute insertion order for Rust `BTreeMap<String>` order.

## Implemented Shape

```text
src/mir/region/observer.rs
  classify_slots_from_variable_map()
  for (name, &vid) in builder.variable_ctx.variable_map().iter()

    -> live RustLifecycleFacts
    -> ordered read-fold compiler
    -> Deny(UnsupportedKeyTransport)
       detail=SourceOrderedStringKeyCompareUnavailable
```

Implemented files:

```text
tools/rust_lifecycle/extract_region_observer_variable_map_facts.py
tools/rust_lifecycle/mirbuilder_ordered_read_fold_converter.py
tools/rust_lifecycle/mirbuilder_region_observer_variable_map_route.py
docs/development/current/main/design/fixtures/rust-lifecycle/region-observer-variable-map-route-v0.json
```

## Stop Line

```text
generated_hako=0
source_ordered_read_fold_claim=0
runtime_fallback=0
insertion_order_substitution=0
region_observer_key_name_special_case=0
```

## Validation

```bash
python3 tools/rust_lifecycle/mirbuilder_region_observer_variable_map_route.py
python3 tools/rust_lifecycle/mirbuilder_region_observer_variable_map_route.py --check-reference
python3 -m py_compile tools/rust_lifecycle/extract_region_observer_variable_map_facts.py tools/rust_lifecycle/mirbuilder_ordered_read_fold_converter.py tools/rust_lifecycle/mirbuilder_region_observer_variable_map_route.py
```

## Next Blocker

```text
SOURCE-ORDERED-UNBLOCK-ROUTE-DESIGN-001
```

This is a design choice, not a mechanical conversion step.

Choices:

```text
1. add a backend-accepted StringBox lexical comparison route for OrderedMapBox
2. change RegionObserver lowering so it does not require SourceOrdered map iteration
```
