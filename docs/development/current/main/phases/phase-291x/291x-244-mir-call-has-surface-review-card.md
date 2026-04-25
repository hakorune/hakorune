---
Status: Done
Date: 2026-04-25
Scope: Review mir_call_route_policy has surface cleanup after 291x-241 MapBox generic_method.has mirror prune.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-241-mapbox-has-route-policy-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-211-runtime-data-has-compat-contract-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-216-receiver-surface-fallback-sunset-design-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-244 MIR Call Has Surface Review Card

## Goal

Determine whether any mir_call_route_policy has surface row is safely removable
after 291x-241 removed the direct MapBox branch from generic_method.has policy.

## Analysis

291x-241 removed the direct `MapBox` fallback from
`classify_generic_method_has_route(...)` in
`hako_llvmc_ffi_generic_method_has_policy.inc`. That prune was correct because
generic_method.has policy is **metadata-first**: it tries metadata, and only
falls back to surface classifiers for metadata-absent boundaries.

However, `mir_call_route_policy.inc` serves a **different role**: it is the
**fallback layer** for when CoreMethod metadata is absent. The flow is:

1. `classify_generic_method_route(...)` line 291-294: try metadata path first
2. If `route_kind == FALLBACK` (metadata absent), line 295-303: fall back to
   surface classifiers
3. Surface classifiers use `MapBox + has` → `RUNTIME_MAP_HAS` (line 93-96)

Current mir_call_route_policy has surface rows:

- `classify_mir_call_receiver_surface box MapBox` (line 68)
- `classify_mir_call_receiver_surface box ArrayBox` (line 69)
- `classify_mir_call_receiver_surface box RuntimeDataBox` (line 70-71)
- `classify_mir_call_method_surface method has` (line 79)

All are **actively used** in the metadata-absent fallback path (line 298-303).

## Blocker Review

According to 291x-211:

> The `has` mirror rows may be pruned only after one of these happens:
> 1. `ArrayHas` lands as a CoreMethod contract and the Array-origin RuntimeData
>    boundary fixtures carry that metadata.
> 2. Array-origin `RuntimeDataBox.has` is retired or replaced by an explicit
>    non-generic contract that no longer needs method-name fallback.

According to 291x-216:

> Do not prune receiver-surface rows until the corresponding method-surface and
> compat rows are gone or exact non-use is proven by fixtures and smokes.
>
> Required blockers:
> - `MapBox` receiver row: keep while `has` fallback remains.
> - `ArrayBox` receiver row: keep while Array-origin `has` and constructor/set
>   compat remain.
> - `RuntimeDataBox` receiver row: keep while RuntimeData set/has/String
>   compatibility fallback remains.

The mir_call_route_policy **is** the fallback layer. Metadata-absent boundaries
still exist (e.g., legacy MIR JSON, certain Stage-B outputs) and rely on this
fallback path.

## Decision

**No safe prune is justified.** All current mir_call has surface rows remain
required as the metadata-absent fallback layer. The 291x-241 prune was correct
for the metadata-first generic_method.has policy, but mir_call_route_policy
serves the intentionally separate fallback role.

The review is complete. No code changes are needed.

## Next Work

The mir_call has surface rows may be pruned only after:

1. Metadata-absent `MapBox.has` boundaries are retired or proven non-existent
   by fixtures and smokes.
2. Array-origin `RuntimeDataBox.has` is covered by `ArrayHas` CoreMethod
   metadata or retired (291x-211 blocker).
3. The fallback path itself is retired in favor of metadata-only routing.

Until then, keep all four tracked mir_call has surface rows pinned.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Review complete. No mir_call has surface rows are safely removable at this
time. The remaining rows serve the metadata-absent fallback layer and are
actively used when CoreMethod metadata is not present.

The no-growth allowlist baseline stays unchanged: `classifiers=12 rows=12` for
mir_call_route_policy surface rows.
