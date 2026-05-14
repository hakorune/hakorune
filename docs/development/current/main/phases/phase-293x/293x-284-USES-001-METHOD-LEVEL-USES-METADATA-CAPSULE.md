# 293x-284 USES-001 Method-Level Uses Metadata Capsule

Status: complete
Date: 2026-05-14

## Scope

Parse method-level `uses` clauses as read-only capability metadata.

## Landed changes

- Added ordered `uses` metadata to function declarations.
- Parsed `uses cap` and `uses cap, other` before function/method/constructor bodies.
- Kept `uses` contextual rather than a global keyword.
- Transported uses metadata through AST JSON and Program JSON v0 helper defs.
- Left capability checking, backend gates, runtime lowering, and provider/hook
  activation out of Stage0.
- Added parser and Program JSON tests plus a dedicated guard.

## Non-goals

- No `unsafe` keyword.
- No `cap` block syntax.
- No Stage0 capability checker.
- No backend route selection.
- No provider activation, allocator hooks, or `#[global_allocator]` coupling.

## Guard

```bash
bash tools/checks/k2_wide_uses_metadata_capsule_guard.sh
```

## Next selected row

`GEN-001 generic type annotation metadata capsule`.

`USES-002 capability checker` remains Stage1-owned.
