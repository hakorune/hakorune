---
Status: Landed
Decision: provisional
Date: 2026-04-02
Scope: engineering lane isolation の fixed order、home rules、no-touch-first surfaces、delete/archive gate を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-31x/README.md
  - docs/development/current/main/phases/phase-31x/31x-91-task-board.md
  - docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md
---

# 31x-90 Engineering Lane Isolation

## Goal

- engineering-only tools を `tools/engineering/**` に rehome する。
- product/mainline front に不要な top-level shell helpers を薄くする。
- top-level old paths は compatibility shim としてだけ残し、drain 後に delete/archive する。
- shared helper/orchestrator surfaces は inventory 後にしか触らない。

## Fixed Rules

- prefer `rehome -> shim -> drain -> delete` over forced deletion.
- do not start from selfhost/bootstrap/orchestrator surfaces.
- current no-touch-first keep remains:
  - `tools/bootstrap_selfhost_smoke.sh`
  - `tools/plugin_v2_smoke.sh`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/selfhost_build.sh`
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- current shared-helper inventory starts before rehoming:
  - `tools/hako_check.sh`
  - `tools/hako_check_deadcode_smoke.sh`
  - `tools/hakorune_emit_mir.sh`

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `31xA engineering home lock` | landed | phase switch と `tools/engineering/**` の home contract を固定する | current docs read `phase-31x` as active and engineering tools have a canonical home |
| `31xB low-blast tool rehome` | landed | low-blast engineering tools を top-level から rehome する | actual scripts live under `tools/engineering/**` and old top-level paths are shim-only |
| `31xC shared helper family inventory` | landed | helper family を `keep / rehome / archive` に分ける | `hako_check` / `deadcode smoke` / `emit_mir` family has an exact disposition map |
| `31xD orchestrator isolation prep` | landed | no-touch-first orchestrators の rehome 可否を exact に固定する | bootstrap/selfhost/plugin orchestrators are split into `keep here` vs `rehome later`, and selfhost-only smokes land under `tools/selfhost/**` |
| `31xE shim drain and legacy sweep` | landed | drained shims と stale top-level wrappers を archive/delete する | no stale top-level compatibility wrapper remains on the current/public surface |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `31xA1` | landed | phase switch in root/current mirrors | `CURRENT_TASK`, `05`, `10`, `15`, and phase index read `phase-31x` as active |
| `31xA2` | landed | engineering home lock | `tools/engineering/**` is the canonical home for rehomed engineering-only tools |
| `31xB1` | landed | rehome `run_vm_stats.sh` | actual script lives under `tools/engineering/`; top-level copy is shim-only |
| `31xB2` | landed | rehome `parity.sh` | actual script lives under `tools/engineering/`; top-level copy is shim-only |
| `31xC1` | landed | inventory shared helper family | `hako_check.sh`, `hako_check_deadcode_smoke.sh`, and `hakorune_emit_mir.sh` are grouped by contract and blast radius |
| `31xC2` | landed | choose shared helper disposition | each helper is fixed as `keep here / rehome / archive later` |
| `31xD1` | landed | orchestrator keep vs rehome split | bootstrap/selfhost/plugin smokes are fixed as `keep top-level`, `move to dedicated selfhost home`, or `stay in profile home` |
| `31xD2` | landed | docs and live path repoint for moved orchestrators | current public docs point at `tools/selfhost/**` for moved selfhost smokes |
| `31xE1` | landed | delete drained compatibility shims | drained top-level shims are deleted after live callers/docs reach zero |
| `31xE2` | landed | archive stale top-level wrappers | no stale top-level wrapper remained; deletion closed the sweep without an extra archive move |

## Current Focus

- successor lane: `phase-32x product / engineering split`
- current blocker: `none`

## 31xA Result

- `phase-31x engineering lane isolation` was the active lane for the engineering rehome sweep
- `phase-30x` is a landed precursor
- `tools/engineering/**` is the canonical home for rehomed engineering-only tools

## 31xB Result

- rehomed:
  - `tools/engineering/run_vm_stats.sh`
  - `tools/engineering/parity.sh`
- current/public docs can move to engineering-home paths without breaking historical callers

## 31xC Result

| Helper | Disposition | Read as |
| --- | --- | --- |
| `tools/hako_check.sh` | keep here | shared helper keep; still live in current design gates, current phase docs, and analyze smoke callers |
| `tools/hako_check_deadcode_smoke.sh` | keep here | stays with the `hako_check` family; loopless/deadcode gates still call the top-level path |
| `tools/hakorune_emit_mir.sh` | keep here | shared route/helper script with live `README`, `smokes/v2/lib`, `route_env_probe`, and runner integrations |

- `31xC1` confirmed blast radius before any move.
- `31xC2` fixed the family as `keep here` for this phase.
- none of the three helpers is a low-blast `tools/engineering/**` rehome candidate.
- any future move requires a dedicated shared-helper phase, not `31x` low-blast cleanup.

## 31xD Result

| Surface | Disposition | Read as |
| --- | --- | --- |
| `tools/plugin_v2_smoke.sh` | keep top-level | plugin lane smoke is still referenced by plugin guard hints and current plugin-lane docs |
| `tools/bootstrap_selfhost_smoke.sh` | keep top-level | bootstrap smoke is still called from `Makefile` and selfhost-pilot docs |
| `tools/selfhost/selfhost_smoke.sh` | keep in dedicated selfhost home | dedicated selfhost smoke belongs under the selfhost home |
| `tools/selfhost/selfhost_vm_smoke.sh` | keep in dedicated selfhost home | dedicated selfhost VM smoke belongs under the selfhost home |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | keep in dedicated selfhost home | dedicated selfhost stage3 acceptance smoke belongs under the selfhost home |
| `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh` | keep in profile home | mixed aggregator spans product/probe/native/selfhost lanes and is not an engineering-home candidate |

- `31xD1` fixed the orchestrator split.
- `31xD2` moved the selfhost-only smoke trio under `tools/selfhost/**` and repointed current public docs there.
- drained top-level wrappers are ready for deletion once current/public references are zero.

## 31xE Result

- deleted drained top-level compatibility wrappers for:
  - `run_vm_stats`
  - `parity`
  - `selfhost_smoke`
  - `selfhost_vm_smoke`
  - `selfhost_stage3_accept_smoke`
- repointed remaining current/public references to canonical homes:
  - `tools/engineering/run_vm_stats.sh`
  - `tools/engineering/parity.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- no stale top-level compatibility wrapper remains on the current/public surface.

## Delete / Archive Gate

- do not delete a top-level shim until:
  - current docs stop pointing to it
  - live non-archive callers are zero
  - the engineering-home path has at least one verification pass after rehome
- archive over delete when the script still explains a historical route that should stay runnable for replay
