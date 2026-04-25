# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-26
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history と rejected history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Current Docs Policy

- Current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Update policy SSOT:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Normal card work must not append landed history here.
- Per-card updates are limited to `CURRENT_STATE.toml` latest-card fields and
  the active card unless lane / blocker / restart order / durable policy changes.

## Quick Restart Pointer

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/05-Restart-Quick-Resume.md`
3. `docs/development/current/main/10-Now.md`
4. Read `active_phase`, `phase_status`, `method_anchor`, `taskboard`, and
   `latest_card_path` from `CURRENT_STATE.toml`
5. `git status -sb`
6. `bash tools/checks/current_state_pointer_guard.sh`
7. `tools/checks/dev_gate.sh quick` when a code slice is ready
8. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
   only when returning to phase-29bq

## Current Lane

- active lane: `phase-291x CoreBox surface contract cleanup`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `phase-291x next compiler-clean cleanup card selection pending`
- primary mode: compiler cleanup lane
- phase-137x: observe-only unless app work reopens a real blocker

## Restart Handoff

- last landed: `291x-273` RuntimeData has declaration need-flag cleanup made
  `nyash.runtime_data.has_hh` demand-driven
- then landed: `291x-272` MIR-call MapBox+has surface fallback closeout kept
  the final two no-growth rows as an intentional paired fallback baseline
- then landed: `291x-271` generic has emit-kind fallback prune removed the
  method-name branch from `classify_generic_method_emit_kind`
- then landed: `291x-270` RuntimeDataBox.has route fallback prune removed the
  RuntimeDataBox receiver-name branch from `classify_generic_method_has_route`
- then landed: `291x-269` direct ArrayBox.has route fallback prune removed the
  ArrayBox receiver-name branch from `classify_generic_method_has_route`
- then landed: `291x-268` ArrayHas CoreMethod carrier covers direct ArrayBox.has
  and Array-origin RuntimeDataBox.has through `array_contains_any`
- then landed: `291x-267` RuntimeData push route fallback and RuntimeDataAppendAny vocabulary pruned
- then landed: `291x-266` RuntimeData set route fallback and RuntimeDataStoreAny vocabulary pruned
- then landed: `291x-265` RuntimeData get route fallback pruned; dispatch E2E set metadata boundary repaired
- then landed: `291x-264` set emit-kind fallback pruned; RuntimeDataBox set route remains pinned
- then landed: `291x-263` direct ArrayBox push route fallback pruned; RuntimeDataBox push remains pinned
- then landed: `291x-262` len route bname fallbacks pruned
- then landed: `291x-261` has-family remaining classifier rows reviewed with no safe prune
- then landed: `291x-260` MIR-call need-policy dead method-surface fallback pruned
- then landed: `291x-259` MIR-call MapBox.has need-policy branch pruned
- then landed: `291x-258` MIR-call RuntimeDataBox receiver-surface mirror pruned
- then landed: `291x-257` MIR-call MapBox receiver-surface review completed with no safe prune
- then landed: `291x-256` MIR-call ArrayBox receiver-surface mirror pruned
- then landed: `291x-255` post-birth cleanup task order landed
- then landed: `291x-254` birth emit-kind mirror pruned
- then landed: `291x-253` birth compatibility deletion criteria landed
- then landed: `291x-252` constructor/birth marker helper landed
- then landed: `291x-251` constructor/birth owner shape decision landed
- then landed: `291x-250` constructor/birth carrier design landed
- then landed: `291x-249` constructor/birth compatibility contract landed
- then landed: `291x-248` RuntimeDataBox get route-policy review completed with no safe prune
- then landed: `291x-247` ArrayBox get route-policy prune landed
- then landed: `291x-246` push route mirror prune review completed with no safe prune
- then landed: `291x-245` len route mirror prune review completed with no safe prune
- worktree: current slice includes the 291x-240..273 cleanup/review batch plus
  this restart handoff refresh; next restart should treat that batch as the
  active local diff set
- branch: `public-main` is ahead of remote by 114 commits
- resume point: choose the next compiler-clean cleanup card; the has-route
  fallback series is closed with no-growth `classifiers=2 rows=2` as an
  intentional MIR-call `MapBox + has` surface fallback baseline, and
  `nyash.runtime_data.has_hh` declaration is now demand-driven
- restart checks: `git status -sb` -> `bash tools/checks/current_state_pointer_guard.sh` ->
  `tools/checks/dev_gate.sh quick` when the next slice is ready

## Task Order

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
- source of task order:
  `docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md`
- keep BoxShape cleanup separate from BoxCount feature rows
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not reopen landed CoreBox router rows without an owner-path change

## Detail Pointers

- CoreBox surface phase:
  `docs/development/current/main/phases/phase-291x/README.md`
- CoreBox design brief:
  `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
- StringBox taskboard:
  `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
- CoreBox inventory:
  `docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md`
- Perf owner-first policy:
  `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- Hotline/CoreMethodContract SSOT:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
