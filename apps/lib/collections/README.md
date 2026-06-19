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

