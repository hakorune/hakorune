# apps/lib/collections

Small `.hako` collection utilities used by apps and compiler-construction
probes.

## Boundary

This directory owns library-level collection behavior. It does not own runtime
substrates, ring0 capabilities, provider registration, or `MapBox` semantics.

## OrderedMapBox

`ordered_map.hako` provides deterministic String-key ordered map behavior.

Reference:

```text
docs/reference/boxes-system/ordered-mapbox.md
docs/development/current/main/design/ordered-map-box-boundary-ssot.md
```

## ValueIdOrderedMapBox

`value_id_ordered_map.hako` provides deterministic i64 `ValueId`-key ordered
map behavior for bounded compiler-construction artifacts.

It is separate from `OrderedMapBox` so String-key ordering and ValueId ordering
do not share a silent transport contract.

Consumers should use `set/get/clone_owned/length/key_at/value_at`; direct
`keys_value` / `values_value` access is an implementation detail.
