# 293x-390 MIMAP-SUBSTRATE-CONC-002 Route Inventory Guard

Status: ready
Date: 2026-05-15

## Decision

Before adding new allocator concurrency substrate behavior, inventory and guard
the existing narrow route facts for:

```text
hako.atomic
hako.tls
hako.osvm
hako.mem
```

This row should prove that backend lowering uses MIR-owned route facts through
`extern_call_routes` / `lowering_plan`, not raw helper-name rediscovery.

## Scope

- Inventory existing route rows and proof apps for atomic/TLS/OSVM/hako.mem.
- Add or update a focused guard only if the current route inventory is not
  already protected by an indexed check.
- Do not add new substrate behavior.
- Do not widen language-level concurrency.

## Stop Lines

- No new extern route row unless an inventory gap is proven.
- No source-level `worker_local`.
- No generic TLS cells.
- No generic atomic surface beyond existing rows.
- No provider hook or host allocator replacement.

## Required Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
