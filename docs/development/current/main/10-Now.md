---
Status: SSOT
Date: 2026-04-26
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Self Current Task — Now (main)

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-291x CoreBox surface contract cleanup`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-291x next compiler-clean cleanup card selection pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- first completed: `mir_call_receiver_surface ArrayBox` pruned in `291x-256`
- `mir_call_receiver_surface MapBox` review completed in `291x-257` with no safe prune
- `mir_call_receiver_surface RuntimeDataBox` pruned in `291x-258`
- has family reviewed in `291x-261` with no further safe prune
- len family pruned in `291x-262`
- push family reviewed in `291x-263`; direct ArrayBox route pruned,
  RuntimeDataBox route remains pinned
- set family reviewed in `291x-264`; method-name emit-kind pruned,
  RuntimeDataBox set-route remains pinned
- RuntimeData `get` cleanup landed in `291x-265`; the get route-policy fallback
  is pruned after repairing the dispatch E2E set metadata boundary
- RuntimeData `set` route cleanup landed in `291x-266`; RuntimeDataStoreAny
  route vocabulary is retired
- RuntimeData `push` route cleanup landed in `291x-267`; RuntimeDataAppendAny
  route vocabulary is retired
- `ArrayHas` CoreMethod carrier landed in `291x-268`; direct `ArrayBox.has` and
  Array-origin `RuntimeDataBox.has` now carry `array_contains_any` metadata
- direct `ArrayBox.has` route fallback pruned in `291x-269`; the backend now
  relies on `ArrayHas` route metadata instead of receiver-name rediscovery
- `RuntimeDataBox.has` route fallback pruned in `291x-270`; metadata-present
  RuntimeData cases now cover `runtime_data_contains_any`, `ArrayHas`, and
  `MapHas` routes without receiver-name rediscovery
- generic `has` emit-kind fallback pruned in `291x-271`; metadata-present has
  routes and route-state `runtime_map_has` now select emit-kind without
  method-name rediscovery
- MIR-call `MapBox + has` surface fallback closeout landed in `291x-272`; the
  final two no-growth rows remain pinned by
  `map_has_no_metadata_min_v1.mir.json`
- RuntimeData has declaration need-flag cleanup landed in `291x-273`;
  `runtime_data_contains_any` metadata and `route.runtime_map_has` now set
  `needs.runtime_data_has`
- next: select the next compiler-clean cleanup card outside the closed has
  fallback series
- task-order card:
  `docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md`
- keep BoxShape and BoxCount separate
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not update current mirrors for every landed card
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/README.md`
3. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
4. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
5. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
