---
Status: Active
Date: 2026-06-24
Scope: Generate backend route contract fields from the neutral generic-method
route descriptors instead of treating C registry rows as a second authority.
---

# GENERIC-ROUTE-DESCRIPTOR-FULL-GENERATION-001

## Decision

Selected next after `REGION-SLOT-CLASSIFIER-LIVE-FACT-OWNER-001`.

This slice moves generic-method route contract fields toward one manifest-owned
descriptor:

```text
spec/mir/generic_method_routes.toml
  [[routes]]
    route_id
    helper_symbol
    tier
    emit_kind
    c_need_kind
        ↓
tools/generic_method_route_descriptor_codegen.py
        ↓
generated Rust / C / Python tables
```

`[[c_registry_rows]]` may still describe C-policy rows such as `core_op`,
`route_proof`, and `route_result`, but it is not the owner of route-id, tier,
emit-kind, or declaration need-kind.

## Implementation Slice

```text
GENERIC-ROUTE-DESCRIPTOR-FULL-GENERATION-001

selected owner:
  generic method route descriptor manifest

selected code:
  tools/generic_method_route_descriptor_codegen.py
  spec/mir/generic_method_routes.toml

first generated fields:
  C emit kind from route.emit_kind
  C need kind from route.c_need_kind
  route_id / tier consistency from route descriptor

non-authority:
  C registry row handwritten emit_kind / need_kind integers
  backend-side route classifier fallback
```

## Acceptance

```text
python3 tools/generic_method_route_descriptor_codegen.py --check
bash tools/checks/current_state_pointer_guard.sh

generated C/Python/Rust route descriptors unchanged unless the manifest
intentionally changes

if a C registry row disagrees with route descriptor fields:
  fail-fast in the generator
```

## Non-Claims

```text
helper_symbol row override drain = 0
same-module view generation = 0
extern route descriptor generation = 0
new backend route = 0
runtime fallback = 0
```
