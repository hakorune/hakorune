---
Status: Active
Date: 2026-06-24
Scope: Drain generic-method helper-symbol row overrides into descriptor-owned
helper variants.
---

# GENERIC-ROUTE-HELPER-VARIANT-DRAIN-001

## Decision

Continue `GENERIC-ROUTE-DESCRIPTOR-FULL-GENERATION-001` by moving the remaining
generic-method helper override into the neutral route descriptor manifest.

The first concrete case is `array_store_any`:

```text
before:
  c_registry_rows encode concrete helper_symbol values

after:
  routes.array_store_any owns c_helper_variants
  c_registry_rows select c_helper_variant only
```

## Implementation Slice

```text
selected manifest:
  spec/mir/generic_method_routes.toml

selected generator:
  tools/generic_method_route_descriptor_codegen.py

new descriptor field:
  c_helper_variants = [
    { key, helper_symbol }
  ]

row field:
  c_helper_variant
```

The generator resolves `c_helper_variant` through route-owned
`c_helper_variants` and fails closed for unknown variants.

## Acceptance

```text
python3 tools/generic_method_route_descriptor_codegen.py --check
bash tools/checks/current_state_pointer_guard.sh

generated route registry outputs unchanged
array_store_any c_registry_rows contain no concrete helper_symbol spelling
unknown c_helper_variant fails in the generator
```

## Non-Claims

```text
same-module route descriptor generation = 0
extern route descriptor generation = 0
set-route value-shape table generation = 0
new backend route = 0
runtime fallback = 0
```
