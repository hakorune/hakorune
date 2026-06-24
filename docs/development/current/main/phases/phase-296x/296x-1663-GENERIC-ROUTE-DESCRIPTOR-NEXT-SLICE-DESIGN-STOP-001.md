---
Status: Design Stop
Date: 2026-06-24
Scope: Choose the next generic route descriptor generation slice after
helper-symbol variants moved into route descriptors.
---

# GENERIC-ROUTE-DESCRIPTOR-NEXT-SLICE-DESIGN-STOP-001

## Current State

Generic method route descriptor generation now owns:

```text
route_id
tier
emit_kind
c_need_kind
c_helper_variants
```

`array_store_any` C registry rows select `c_helper_variant`; generated
Rust/C/Python registry output remains unchanged.

## Design Choice

The next task-order rows are both real, but they have different blast radius:

```text
Option A:
  Same-module / extern route descriptor generation

Option B:
  Set-route value-shape table generation
```

Option A follows task-order order, but it spans two route families and several
C shim consumers. Option B is thinner because it can reuse the just-landed
`c_helper_variants` descriptor data to generate the local
`LoweringPlanSetRouteRule` table.

## Stop Rule

Do not continue implementation until the next owner is selected.

## Non-Claims

```text
same-module descriptor generation selected = 0
extern descriptor generation selected = 0
set-route value-shape generation selected = 0
new backend behavior = 0
```
