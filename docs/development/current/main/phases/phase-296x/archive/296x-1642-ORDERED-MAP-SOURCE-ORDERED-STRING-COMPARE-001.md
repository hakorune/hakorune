---
Status: Landed
Date: 2026-06-23
Scope: MirBuilder Rust-to-Hako converter source-order blocker
---

# 296x-1642: OrderedMap Source-Ordered String Compare

## Decision

Do not claim `SourceOrdered` read-fold conversion for `OrderedMapBox` yet.

The RegionObserver `variable_map().iter()` slice requires Rust
`BTreeMap<String>` source order. The current `.hako` `OrderedMapBox` path does
not prove that order for the focused `b, a, args` insertion case, and an AOT
implementation attempt using `.hako` string content comparison makes
`OrderedMapBox.set/2` fail backend acceptance.

Therefore the converter must fail closed:

```text
Deny(UnsupportedKeyTransport)
detail=SourceOrderedStringKeyCompareUnavailable
```

## Stop Line

```text
source_ordered_read_fold_claim=0
runtime_fallback=0
insertion_order_substitution=0
region_observer_key_name_special_case=0
```

## Implemented Guardrail

```text
tools/rust_lifecycle/mirbuilder_ordered_map_source_order_inventory.py
docs/development/current/main/design/fixtures/rust-lifecycle/ordered-map-source-order-v0.json
```

The inventory is intentionally small. It records the deny contract and prevents
the next RegionObserver work from silently treating insertion order as Rust
source order.

## Validation

```bash
python3 tools/rust_lifecycle/mirbuilder_ordered_map_source_order_inventory.py
python3 tools/rust_lifecycle/mirbuilder_ordered_map_source_order_inventory.py --check-reference
bash tools/checks/current_state_pointer_guard.sh
```

## Next Blocker

```text
SOURCE-ORDERED-READ-FOLD-ROUTE-SELECTION-001
```

Required next step:

```text
Choose one:
  1. add a backend-accepted StringBox lexical comparison route for OrderedMapBox
  2. change the selected observer lowering so it does not require SourceOrdered map iteration
```
