---
Status: SSOT
Decision: provisional
Date: 2026-03-24
Scope: execution-lane migration 中に見つかった legacy/delete-candidate を 1 箇所で triage し、lane-local retire 実装と混線させない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md
  - docs/development/current/main/design/code-retirement-history-policy-ssot.md
  - docs/development/current/main/design/selfhost-smoke-retirement-inventory-ssot.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29y/README.md
---

# Execution Lanes Legacy Retirement Inventory (SSOT)

## Goal

- execution-lane migration 中に見つかった legacy/delete 候補を lane-local README だけに散らさない。
- 「見つけた」「delete-ready」「まだ keep」の triage を 1 本で固定する。
- 実際の削除は既存の retire policy と phase owner に従って行う。

## Status Buckets

| bucket | meaning |
| --- | --- |
| `daily keep` | current daily lane で必要 |
| `stage1 proof keep` | stage1 bridge/proof/snapshot で必要 |
| `reference lane keep` | vm-hako reference/debug/bootstrap-proof lane で必要 |
| `bootstrap/recovery keep` | rust-vm or stage0 keep として必要 |
| `compat keep` | daily owner ではないが compat で必要 |
| `archive candidate` | live caller/gate から外れ、archive home を決めれば移せる |
| `delete-blocked` | remove target だが blocker が残る |
| `delete-ready` | successor proof と absence proof がそろっている |

## Rules

1. migration 中に legacy/delete 候補を見つけたら、まずここへ row を追加する。
2. actual removal は lane-local doc で owner を決めてから行う。
3. `delete-ready` にする条件:
   - successor or replacement proof exists
   - active docs/gates/callers no longer require the item
   - current owner phase agrees it is removable
4. code copy は作らない。
   - path / artifact token / proof link だけを記録する。
5. deletion procedure itself still follows `code-retirement-history-policy-ssot.md`.

## Seed Inventory (2026-03-24)

| item | current role | bucket | owner | delete blocker / note | successor proof |
| --- | --- | --- | --- | --- | --- |
| `src/stage1/program_json_v0/**` | bootstrap-only stage1 proof boundary | `delete-blocked` | `phase-29ci` | remaining caller inventory is not empty | `source -> direct MIR(JSON v0) -> backend/VM` convergence plus phase-29ci delete order |
| `src/runner/stage1_bridge/**` | future-retire bridge lane | `delete-blocked` | `phase-29ci` | bridge cluster still carries live bootstrap proof/workflow | phase-29ci bridge delete order and caller removal proof |
| `lang/bin/hakorune` | stage1 stable snapshot artifact | `stage1 proof keep` | `lang/README.md` + distribution docs | stage2+ distribution artifact is not active yet | future stage2+ distribution packaging |
| `vm-hako` runtime lane | semantic/reference/debug/bootstrap-proof lane | `reference lane keep` | `phase-29y` | lane remains active as reference/debug/bootstrap-proof | none; not a delete target in the current wave |
| `rust-vm` runtime lane | bootstrap/recovery/compat lane | `bootstrap/recovery keep` | `phase-29y` | stage0 bootstrap/recovery keep is still explicit | none; not a delete target in the current wave |

## Non-Goals

- replacing lane-local retirement orders
- storing old code in docs
- turning every keep row into an immediate delete target
