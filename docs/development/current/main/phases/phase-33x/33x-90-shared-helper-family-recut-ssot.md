---
Status: SSOT
Decision: provisional
Date: 2026-04-03
Scope: shared helper family の path-truth と caller-drain order を固定し、`emit_mir` thin wrapper を no-op rehome ではなく route-preset thin shim として truthify する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-33x/README.md
  - docs/development/current/main/phases/phase-33x/33x-91-task-board.md
  - docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md
---

# 33x-90 Shared Helper Family Recut

## Goal

- shared helper family を top-level keep のまま増やさず、thin wrapper と family-local smoke helper を truthful home に寄せる。
- delete より先に `rehome / shim-only / caller-drain` を進める。
- `tools/hako_check.sh` と `tools/hakorune_emit_mir.sh` の broad live integration は keep しつつ、その周辺の thin surfaces から細くする。

## Fixed Rules

- prefer `family-home rehome` over forcing helper implementation rewrites
- keep top-level explicit entry only when current/live callers are broad
- move thin wrappers before moving broad helpers
- leave raw backend default/token truthification for a later lane
- do not widen `vm` residue while touching helper families

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `33xA helper family inventory` | landed | exact caller pressure and canonical homes を固定する | `hako_check` / `emit_mir` family の `keep / rehome / shim-only` が読める |
| `33xB hako_check family path truth` | landed | family-local smoke helper を `tools/hako_check/**` に寄せる | deadcode/deadblocks 系の canonical home が family path になり `hako_check.sh` keep reason も固定される |
| `33xC emit_mir thin wrapper path truth` | active | thin compat/mainline wrappers の truthful role を固定する | top-level thin wrappers は route-preset compatibility wrappers と読め、routing truth は `emit_mir_route.sh` に寄る |
| `33xD closeout/docs cleanup` | active | current/public docs を helper-family truth に揃える | landed keep/rehome/shim-only 読みが root/current docs に揃う |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `33xA1` | landed | helper family caller inventory | current/live caller map と canonical home candidates が読める |
| `33xB1` | landed | `hako_check_deadblocks_smoke.sh` family-home rehome | `tools/hako_check/deadblocks_smoke.sh` が canonical で top-level は shim-only |
| `33xB2` | landed | `hako_check.sh` top-level keep gate | broad live caller pressure と future drain 条件が固定される |
| `33xC1` | landed | `emit_mir` thin wrapper caller inventory | `mainline` / `compat` wrapper の exact live callers が読める |
| `33xC2` | landed | `emit_mir` thin wrapper route-preset lock | wrappers stay top-level thin compatibility shims and operational routing truth is `tools/smokes/v2/lib/emit_mir_route.sh` |
| `33xC3` | landed | `hakorune_emit_mir.sh` top-level keep gate | broad live integration が exact keep reason として固定される |
| `33xD1` | active | closeout/docs cleanup | current/public docs が truthful family paths に揃う |

## Current Focus

- active macro wave: `33xC emit_mir thin wrapper path truth`
- active micro task: `33xD1 closeout/docs cleanup`
- next queued micro task: `phase-33x closeout review`
- current blocker: `none`

## 33xA1 Result

### `hako_check` family

- broad top-level keep:
  - `tools/hako_check.sh`
- family-local smoke helpers:
  - `tools/hako_check/deadcode_smoke.sh`
  - `tools/hako_check_deadblocks_smoke.sh` (still top-level before this phase)
- current/live caller pressure:
  - `selfhost-coreplan-unblocking-policy.md`
  - `tools/hako_check_loopless_gate.sh`
  - older current-phase bringup docs under `phase-29bg` / `phase-29ca`

Read as:
- `hako_check.sh` still has broad live callers and stays top-level keep.
- deadcode/deadblocks helper pair belongs under `tools/hako_check/**`.

### `emit_mir` family

- broad helper keep:
  - `tools/hakorune_emit_mir.sh`
- thin wrappers:
  - `tools/hakorune_emit_mir_mainline.sh`
  - `tools/hakorune_emit_mir_compat.sh`
- current/live caller pressure:
  - `docs/tools/README.md`
  - `tools/selfhost/README.md`
  - `selfhost-bootstrap-route-ssot.md`
  - `frontend-owner-proof-index.md`
  - `phase-29bq` current checklists and proof docs

Read as:
- `hakorune_emit_mir.sh` still has broad docs/script integration and is not the first move target.
- `mainline` / `compat` wrappers are thin and are the lower-blast first cut.

## 33xB1 Result

- `hako_check_deadblocks_smoke.sh` moved to:
  - `tools/hako_check/deadblocks_smoke.sh`
- top-level path:
  - `tools/hako_check_deadblocks_smoke.sh`
  - reduced to a compatibility shim only
- current/live policy docs now point at the family home:
  - `selfhost-coreplan-unblocking-policy.md`
- `tools/hako_check/README.md` now treats `deadblocks_smoke.sh` as a canonical helper next to `deadcode_smoke.sh`

Read as:
- `hako_check` family-local smoke helpers now share one truthful home under `tools/hako_check/**`.
- remaining top-level `hako_check` work is about the broad entry `tools/hako_check.sh`, not the smoke helper pair.

## 33xB2 Result

- executable caller pressure is still present:
  - family-local helper:
    - `tools/hako_check/deadblocks_smoke.sh`
  - analyze smoke:
    - `tools/smokes/v2/profiles/integration/analyze/dot_cluster_smoke.sh`
- current-doc pressure keeps the path user-facing:
  - `docs/tools/script-index.md`
  - `docs/development/current/main/design/selfhost-coreplan-unblocking-policy.md`
  - current and historical phase bringup docs still name `tools/hako_check.sh` as the analyzer entry
- therefore `tools/hako_check.sh` stays top-level keep in this phase

Future drain conditions:
- a dedicated `tools/hako_check/**` family entry is accepted as the canonical analyzer path
- current/public docs and script index are repointed to that family entry
- the analyze smoke either follows that family entry or is re-cut behind a narrower helper
- only after that drain should shim-only/archive be reconsidered

Read as:
- `tools/hako_check.sh` is still the canonical analyzer entry, not just a leftover wrapper.
- `phase-33x` should pin that keep reason instead of forcing a cosmetic rehome.

## 33xC1 Result

- thin wrapper live callers were fixed as:
  - `docs/tools/README.md`
  - `tools/selfhost/README.md`
  - `selfhost-bootstrap-route-ssot.md`
  - `frontend-owner-proof-index.md`
  - `phase-29bq/29bq-92-parser-handoff-checklist.md`

Read as:
- current thin wrapper pressure is docs/proof oriented, not broad script integration.
- wrapper truthification is low blast compared with touching `hakorune_emit_mir.sh` itself.

## 33xC2 Result

- thin wrappers remain at:
  - `tools/hakorune_emit_mir_mainline.sh`
  - `tools/hakorune_emit_mir_compat.sh`
- as top-level route-preset compatibility wrappers
- operational routing truth stays in:
  - `tools/smokes/v2/lib/emit_mir_route.sh`
- no executable caller pressure justifies a family-home move right now
- current/live docs now explicitly read these wrappers as route presets while keeping `emit_mir_route.sh` as the operational entry for new scripts

Read as:
- `emit_mir` thin wrappers are truthful as thin top-level compatibility wrappers; they are not the right family-home rehome target.
- remaining `emit_mir` work is about documenting/pinning the wrapper role, then gating the broad helper `tools/hakorune_emit_mir.sh`.

## 33xC3 Result

- executable caller pressure stays broad across multiple roles:
  - route owner:
    - `tools/smokes/v2/lib/emit_mir_route.sh`
  - perf:
    - `tools/perf/bench_hakorune_emit_mir.sh`
  - check/debug helper:
    - `tools/checks/route_env_probe.sh`
  - explicit proof canary:
    - `tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
- source/diagnostic pressure remains:
  - `src/mir/builder/builder_build.rs`
  - `src/runner/mir_json_emit/helpers.rs`
  - `src/runner/mir_json_emit/mod.rs`
- current-doc pressure remains broad:
  - `tools/selfhost/README.md`
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
  - `docs/development/current/main/design/frontend-owner-proof-index.md`
  - current `phase-29bq` proof/checklist docs
- therefore `tools/hakorune_emit_mir.sh` stays top-level keep in this phase

Future drain conditions:
- route-selecting scripts keep converging on `tools/smokes/v2/lib/emit_mir_route.sh`
- direct helper references in current docs are limited to explicit proof/troubleshooting lanes
- perf/check callers either stay as explicit helper-local keep or are moved behind route-owned helpers
- only after that drain should rehome/archive be reconsidered

Read as:
- `tools/hakorune_emit_mir.sh` is still a broad shared helper, not a low-blast move target.
- `phase-33x` should not force a no-op path recut for this helper; it should only pin why the helper still stays top-level.
