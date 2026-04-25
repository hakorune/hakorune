---
Status: Active
Date: 2026-04-26
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Heavy gates are not first-step restart work. Run them only when the next code
slice is ready:

```bash
tools/checks/dev_gate.sh quick
cargo check -q
```

## Current Lane

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-291x CoreBox surface contract cleanup`
- active phase: read `active_phase` from `CURRENT_STATE.toml`
- latest card: read `latest_card_path` from `CURRENT_STATE.toml`
- current blocker token: `phase-291x next compiler-clean cleanup card selection pending`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Handoff Snapshot

- latest card: `291x-273`
- last landed work: RuntimeData has declaration need-flag cleanup, then
  MIR-call MapBox+has surface fallback closeout, then generic
  has emit-kind fallback prune, then RuntimeDataBox.has route fallback prune, then direct
  ArrayBox.has route fallback prune, then ArrayHas
  CoreMethod carrier, then RuntimeData push route fallback prune, then RuntimeData set
  route fallback prune, then RuntimeData get route fallback prune, then set
  emit-kind fallback prune, then direct ArrayBox push
  route fallback prune, then len route bname fallback prune, then has-family
  remaining classifier review with no safe prune, then MIR-call need-policy
  dead method-surface fallback prune, then
  MIR-call MapBox.has need-policy branch prune, then MIR-call RuntimeDataBox
  receiver-surface mirror prune, then MIR-call MapBox receiver-surface review
  with no safe prune, then MIR-call ArrayBox receiver-surface mirror prune,
  then post-birth cleanup task order, then birth emit-kind mirror prune, then
  birth compatibility deletion criteria, then constructor/birth marker helper,
  then constructor/birth owner shape decision, then constructor/birth carrier
  design, then constructor/birth compatibility contract, then RuntimeDataBox get
  route-policy review, then ArrayBox get route-policy prune
- worktree: current local diff now includes the 291x-240..273 cleanup/review
  batch plus this restart handoff refresh
- resume point: choose the next compiler-clean cleanup card; the has-route
  fallback series is closed with no-growth `classifiers=2 rows=2` as an
  intentional MIR-call `MapBox + has` surface fallback baseline

## Immediate Next

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
- keep docs mirrors thin; update `CURRENT_STATE.toml` and the active card first
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- keep phase-137x observe-only unless app work reopens a real blocker

## Restart Notes

- worktree should be clean after the last commit
- do not run heavy perf ladders during restart unless explicitly requested
