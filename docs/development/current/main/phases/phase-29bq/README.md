---
Status: Active
Scope: planner-required master list を selfhost 入口へ接続し、selfhost lane を failure-driven canary として運用するための active front page。
Related:
- docs/development/current/main/15-Workstream-Map.md
- docs/development/current/main/design/compiler-task-map-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/lego-composability-policy.md
- docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
- docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md
- docs/development/current/main/phases/phase-29bq/29bq-114-hako-cleanup-integration-prep-lane.md
- docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md
- docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
---

# Phase 29bq: planner-required -> selfhost entry wiring

## Goal

- planner-required (`strict/dev`) を selfhost 側からも 1 コマンドで回せる入口に接続する。
- selfhost lane を failure-driven canary として維持し、next exact blocker capture の入口にする。
- release default と既存 gate を壊さない。

## Non-goals

- 新しい language feature の追加
- by-name 分岐
- silent fallback の復活
- selfhost のための ad hoc workaround 追加

## Priority (SSOT)

- 最優先は compiler 側 (`Facts / Normalizer / CorePlan`) の表現力と構造の収束。
- selfhost を通すこと自体は目的ではなく、fixture + fast gate で exact blocker を固定する入口として使う。
- task order is owned by `compiler-task-map-ssot.md`; this phase must not pull that order around.

## Task Order Snapshot

1. `CondProfile` stabilization
2. `VerifiedRecipe` boundary tightening
3. `GenericLoopV1` acceptance closure
4. `LoopCond` recipe-only tag pinning
5. historical `DomainPlan` wording cleanup
6. `idx_var` contract
7. selfhost / OOM

## Canonical Child Docs

- selfhost / gate operations:
  - `29bq-90-selfhost-checklist.md`
- mirbuilder progress ledger:
  - `29bq-91-mirbuilder-migration-progress-checklist.md`
- parser handoff operations:
  - `29bq-92-parser-handoff-checklist.md`
- `.hako` recipe-first migration lane:
  - `29bq-113-hako-recipe-first-migration-lane.md`
- `.hako` cleanup integration prep:
  - `29bq-114-hako-cleanup-integration-prep-lane.md`
- selfhost closeout checklist:
  - `29bq-115-selfhost-to-go-checklist.md`
- session log / historical updates:
  - `29bq-session-updates-2026-02-08.md`

## Current Read

- phase status: `active`
- current blocker: `none`
- operation mode: `failure-driven`
- current exact implementation leaf: `none while blocker=none`
- latest landed blocker:
  - `program_json_contract_pin` / `joinir_port04` / `joinir_port07`
  - fixed by program-json compat bridge, parser/helper simplification, and removal of disabled legacy lowers from mainline owners
- while blocker=`none`, the next cleanup cut is dedicated `legacy lowerer removal`

## Read Order

1. `29bq-90-selfhost-checklist.md`
2. `29bq-91-mirbuilder-migration-progress-checklist.md`
3. `29bq-92-parser-handoff-checklist.md`
4. `29bq-113-hako-recipe-first-migration-lane.md`
5. `29bq-114-hako-cleanup-integration-prep-lane.md`
6. `29bq-115-selfhost-to-go-checklist.md`

## Gates

- loopless subset:
  - `./tools/hako_check_loopless_gate.sh`
- planner-required dev gate v4:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- JoinIR regression pack:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- fast iteration:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- selfhost planner-required entry:
  - `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- baseline safety:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`

Fast gate case ownership:

- `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`
- `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`

## Current Contracts

- selfhost entry command:
  - `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- log contract:
  - failures end with `LOG: /tmp/phase29bq_selfhost_<case>.log`
- default list:
  - `planner_required_selfhost_subset.tsv`
- optional master list:
  - `SMOKES_SELFHOST_LIST=tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv`
- de-rust done handshake is not owned here:
  - done判定の正本は `phase-29x/29x-62-derust-done-sync-ssot.md`

## Exact Next

- keep selfhost canary failure-driven
- do not widen `29bq` while blocker=`none`
- next cleanup cut:
  - `29bq-118-legacy-lowerer-removal-lane.md`
- next work stays compiler-first:
  - `cleanupwrap-cleanup-region-boundary-ssot.md`
  - `condblockview-desugar-consult.md`
  - `block-expressions-and-condition-blocks-ssot.md`
- if a new blocker appears:
  - pin fixture
  - pin fast gate
  - capture the first freeze/reject
  - fix one accepted shape only

## BoxShape / BoxCount Rule

- `1 blocker = 1 fixture = 1 fast gate = 1 commit`
- but prefer `BoxShape` over `BoxCount` when:
  - the issue grows only by count (`clusterN`, nested-loop depth, similar families)
  - shared pipeline machinery can be centralized instead
  - allowlist/composer exceptions would scatter the change

Special-rule triggers stay:

- irreducible / multi-entry loop
- unwind / finally / cleanup boundary
- coroutine / yield
- transformations that would violate no-rewrite / evaluation-order constraints

## Acceptance Summary

- selfhost entry remains executable via one command with RC=0 when enabled
- subset TSV can grow gradually and stops on the first freeze
- planner-required dev gate, JoinIR regression pack, and loopless gate remain green
- README is front-page only; frontier tables, blocker history, and long fixture mapping stay in TSVs, session logs, and child docs

## Detail Owners

- frontier progress / migration detail:
  - `29bq-91-mirbuilder-migration-progress-checklist.md`
- selfhost closeout / acceptance detail:
  - `29bq-115-selfhost-to-go-checklist.md`
- blocker and session history:
  - `29bq-session-updates-2026-02-08.md`
- fixture / case inventory:
  - the TSV owners above
