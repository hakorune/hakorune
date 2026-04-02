---
Status: Active
Decision: provisional
Date: 2026-04-02
Scope: `phase-31x engineering lane isolation` の concrete task order と evidence command をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-31x/README.md
  - docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md
---

# 31x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `31xA engineering home lock` | landed | phase switch and canonical engineering home |
| 2 | `31xB low-blast tool rehome` | landed | actual move of low-blast engineering tools |
| 3 | `31xC shared helper family inventory` | landed | exact `keep / rehome / archive` map for helper family |
| 4 | `31xD orchestrator isolation prep` | landed | no-touch-first orchestrator keep vs rehome split |
| 5 | `31xE shim drain and legacy sweep` | landed | delete/archive after rehome drain is explicit |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `31xA1` | landed | root/current mirrors and phase index switch to `phase-31x` |
| 2 | `31xA2` | landed | `tools/engineering/**` is fixed as the canonical home |
| 3 | `31xB1` | landed | `run_vm_stats.sh` actual move + shim |
| 4 | `31xB2` | landed | `parity.sh` actual move + shim |
| 5 | `31xC1` | landed | shared helper family inventory |
| 6 | `31xC2` | landed | shared helper disposition |
| 7 | `31xD1` | landed | orchestrator keep vs rehome split |
| 8 | `31xD2` | landed | docs/live path repoint for moved orchestrators |
| 9 | `31xE1` | landed | drained shim deletion |
| 10 | `31xE2` | landed | stale wrapper archive/delete |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
git diff --check
rg -n 'tools/(engineering/run_vm_stats|engineering/parity|hako_check|hako_check_deadcode_smoke|hakorune_emit_mir|bootstrap_selfhost_smoke|plugin_v2_smoke|selfhost/selfhost_smoke|selfhost/selfhost_vm_smoke|selfhost/selfhost_stage3_accept_smoke)\.sh' \
  README.md README.ja.md docs tools src Makefile
bash -n tools/engineering/run_vm_stats.sh tools/engineering/parity.sh \
  tools/selfhost/selfhost_smoke.sh tools/selfhost/selfhost_vm_smoke.sh tools/selfhost/selfhost_stage3_accept_smoke.sh
bash tools/engineering/parity.sh --help >/dev/null
```

## 31xB Result

- moved actual scripts:
  - `tools/engineering/run_vm_stats.sh`
  - `tools/engineering/parity.sh`
- active next:
  - `phase-31x closeout review`

## 31xC Result

- `tools/hako_check.sh` = keep here
- `tools/hako_check_deadcode_smoke.sh` = keep here with the `hako_check` family
- `tools/hakorune_emit_mir.sh` = keep here as a shared route/helper script
- result:
  - no helper in this family is a low-blast `tools/engineering/**` move candidate
  - `31xD1` was the next front

## 31xD Result

- keep top-level:
  - `tools/plugin_v2_smoke.sh`
  - `tools/bootstrap_selfhost_smoke.sh`
- moved to dedicated selfhost home with top-level shims:
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- keep in profile home:
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- result:
  - `31xE1` is the current front

## 31xE Result

- deleted drained top-level shims for:
  - `run_vm_stats`
  - `parity`
  - `selfhost_smoke`
  - `selfhost_vm_smoke`
  - `selfhost_stage3_accept_smoke`
- no stale top-level wrapper remained for an extra archive move
- current front:
  - `phase-31x closeout review`
