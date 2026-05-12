---
Status: Complete
Date: 2026-05-12
Scope: D195 hako_alloc SSOT refresh after M184.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/README.md
  - lang/src/hako_alloc/memory/README.md
---

# 293x-196 D195 hako_alloc SSOT Refresh

## Goal

Refresh the hako_alloc ownership map after the secure-list rows landed.

Current algorithm owner chain:

```text
M171 page-map model
M172 page-map-backed release
M173 release invariant freeze
M174 same-class realloc
M175 alloc-copy-release realloc
M176 realloc failure contract
M177 alignment policy
M178 aligned small path
M179 huge threshold/routing
M180 huge page model
M181 huge release seam
M182 secure-list policy inventory
M183 secure-list diagnostics
M184 secure-list encoded-next policy
```

## Durable Boundaries

- release/realloc ownership remains in the M171-M176 page-map owners.
- alignment ownership remains split between M177 policy and M178 small-path
  execution.
- huge ownership remains split between M179 routing, M180 huge model, and M181
  release seam.
- secure-list ownership is split between M183 diagnostics and M184 encoded-next
  policy.
- provider activation, hooks, process allocator replacement, OS unreserve, and
  `.inc` allocator-name matching remain out of scope.

## Next

M185 owns the numeric field inventory delta before any broader `usize` or
record-backed metadata migration.
