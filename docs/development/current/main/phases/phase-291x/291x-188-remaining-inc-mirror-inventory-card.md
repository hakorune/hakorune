---
Status: Landed
Date: 2026-04-25
Scope: Inventory remaining `.inc` method/box mirror rows after the Set cleanup slice.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-187-mir-call-set-surface-prune-card.md
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-188 Remaining `.inc` Mirror Inventory Card

## Goal

Stop one-row prune guessing and record the remaining mirror rows by cleanup
type after the Set cleanup slice.

The current guard baseline is:

```text
core-method-contract-inc-no-growth-guard:
  ok classifiers=26 rows=26
```

## Inventory

### Generic Emit-Kind Fallback Rows

These rows are still required by metadata-absent boundary coverage or explicit
constructor compatibility:

```text
birth
set
get
len
length
size
push
substring
has
ArrayBox
MapBox
```

Current status:

- `get`, `len` family, `substring`, `push`, and `set` prune probes were
  rejected by boundary evidence.
- `has` still has metadata-absent RuntimeData fallback coverage.
- `birth` plus the `ArrayBox`/`MapBox` constructor checks need a constructor
  contract before any cleanup.

### Generic Set Storage-Route Rows

```text
MapBox
RuntimeDataBox
```

Current status:

- `ArrayBox` was pruned in `291x-183`.
- `MapBox` is pinned by `291x-184`.
- `RuntimeDataBox` is pinned by `291x-185`.

### MIR-Call Route-Policy Surface Rows

```text
receiver surface:
  MapBox
  ArrayBox
  StringBox
  RuntimeDataBox

method surface:
  get
  set
  size
  len
  length
  has
  push
  indexOf
  substring
```

Current status:

- `set` method surface prune was rejected in `291x-187`.
- The other route-policy rows should not be probed one by one until the
  route-policy layer can consume MIR-owned CoreMethod metadata. This file still
  classifies box/method names directly and builds `GenericMethodRouteState`
  from that classification.

## Decision

Next cleanup should not be another blind prune. The next structural seam is:

```text
MIR generic_method_routes metadata
  -> hako_llvmc_ffi_mir_call_route_policy.inc
  -> GenericMethodRouteState
```

This should be done as a preflight + small consumer card, with fallback kept for
metadata-absent routes.

## Result

The Set cleanup slice is closed for now:

- metadata-first emit-kind: landed
- metadata-first storage-route: landed
- ArrayBox storage-route mirror: pruned
- MapBox/RuntimeData fallback contracts: pinned
- MIR-call `set` surface prune: rejected

The next active target is the MIR-call route-policy metadata consumer preflight.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
