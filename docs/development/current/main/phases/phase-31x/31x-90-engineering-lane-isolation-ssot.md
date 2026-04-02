---
Status: SSOT
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
  - `tools/selfhost_smoke.sh`
  - `tools/selfhost_vm_smoke.sh`
  - `tools/selfhost_stage3_accept_smoke.sh`
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
| `31xD orchestrator isolation prep` | active | no-touch-first orchestrators の rehome 可否を exact に固定する | bootstrap/selfhost/plugin orchestrators are split into `keep here` vs `rehome later` |
| `31xE shim drain and legacy sweep` | queued | drained shims と stale top-level wrappers を archive/delete する | drained wrappers leave top-level or become explicit archive-only residue |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `31xA1` | landed | phase switch in root/current mirrors | `CURRENT_TASK`, `05`, `10`, `15`, and phase index read `phase-31x` as active |
| `31xA2` | landed | engineering home lock | `tools/engineering/**` is the canonical home for rehomed engineering-only tools |
| `31xB1` | landed | rehome `run_vm_stats.sh` | actual script lives under `tools/engineering/`; top-level copy is shim-only |
| `31xB2` | landed | rehome `parity.sh` | actual script lives under `tools/engineering/`; top-level copy is shim-only |
| `31xC1` | landed | inventory shared helper family | `hako_check.sh`, `hako_check_deadcode_smoke.sh`, and `hakorune_emit_mir.sh` are grouped by contract and blast radius |
| `31xC2` | landed | choose shared helper disposition | each helper is fixed as `keep here / rehome / archive later` |
| `31xD1` | active | orchestrator keep vs rehome split | bootstrap/selfhost/plugin smokes are fixed as `keep top-level` or `candidate for engineering home` |
| `31xD2` | queued | docs and live path repoint for moved orchestrators | current docs stop pointing at old top-level paths where rehome landed |
| `31xE1` | queued | delete drained compatibility shims | shim deletion starts only after live callers/docs are zero |
| `31xE2` | queued | archive stale top-level wrappers | wrappers that should not stay live move under archive/historical homes |

## Current Focus

- active macro wave: `31xD orchestrator isolation prep`
- active micro task: `31xD1 orchestrator keep vs rehome split`
- next queued micro task: `31xD2 docs and live path repoint for moved orchestrators`
- current blocker: `none`

## 31xA Result

- `phase-31x engineering lane isolation` is now the active lane
- `phase-30x` is a landed precursor
- `tools/engineering/**` is the canonical home for rehomed engineering-only tools

## 31xB Result

- rehomed:
  - `tools/engineering/run_vm_stats.sh`
  - `tools/engineering/parity.sh`
- compatibility shims remain at:
  - `tools/run_vm_stats.sh`
  - `tools/parity.sh`
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

## Delete / Archive Gate

- do not delete a top-level shim until:
  - current docs stop pointing to it
  - live non-archive callers are zero
  - the engineering-home path has at least one verification pass after rehome
- archive over delete when the script still explains a historical route that should stay runnable for replay
