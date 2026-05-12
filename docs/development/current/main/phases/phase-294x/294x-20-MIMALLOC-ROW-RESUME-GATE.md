---
Status: Landed
Date: 2026-05-12
Scope: resume mimalloc algorithm rows after usize preflight.
Related:
  - docs/development/current/main/phases/phase-293x/293x-175-M167-MIMALLOC-ALLOC-FAST-PATH.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-19-PRODUCTION-USIZE-MIGRATION-PREFLIGHT.md
---

# 294x-20 Mimalloc Row Resume Gate

## Decision

Mimalloc algorithm implementation may resume after 294x-19, but production
`hako_alloc` numeric fields remain `i64` until non-VM exact numeric typed-object
storage exists.

The first resumed row is M167:

```text
M167 alloc fast path plus generic fallback
```

## Boundary

Allowed:

- pure `.hako` allocator algorithm rows;
- current-lane `i64` production state;
- isolated `usize` probes that do not feed production facade proofs.

Still deferred:

- production `usize` field migration;
- native exact numeric typed-object slots;
- exact numeric field get/set ABI;
- OSVM page source composition, local-free retire, remote-free integration,
  page-map, provider activation, hooks, and process allocator replacement.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
