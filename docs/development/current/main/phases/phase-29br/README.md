---
Status: Complete
Scope: selfhost compiler の planner-required freeze（BundleResolver loop）を解消する
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/phases/phase-29bq/README.md
---

# Phase 29br: selfhost planner-required BundleResolver freeze

## Goal

- `apps/tests/phase29aq_string_parse_integer_min.hako` を selfhost compiler の planner-required で通す。
- 既定挙動は不変（release default unchanged）。

## Non-goals

- 他パターンの追加
- by-name 分岐や silent fallback の復活

## Repro (SSOT)

- LOG: `/tmp/phase29bq_selfhost_phase29aq_string_parse_integer_min.hako.log`
- Symptom: stage-b failed under planner-required (`BundleResolver.resolve/4` in compiler.hako)

## Current blocker (SSOT)

- Selfhost/Stage‑B canary の freeze は “`.hako` を JoinIR に合わせて変形し続ける” 方向だと箱が肥大化しやすく、移植戦略としても逆になりやすい。
- 方針: selfhost 側の追跡は一旦止め、CorePlan 側（Facts/Normalize の analysis-only view）を先に強化する（SSOT: `docs/development/current/main/design/compiler-expressivity-first-policy.md`）。
- 具体: generic loop v0 の `loop_var` 抽出を “condition.left が Variable のときだけ” から、候補列挙→body の step 一致で一意決定に拡張する（`j+m<=n` など）。

## P0–P3 status

- P0: docs-first (repro/goal/contract) ✅
- P1: target + direction fixed ✅
- P2: UpdateCanon + step extraction (analysis-only) ✅
- P3: gates green / handoff conditions ✅

## P2 done (compiler expressivity line)

1. `UpdateCanon`（analysis-only view）を追加し、loop update の形揺れ（例: `j=j+1` と `j=1+j`）を保守的に観測できるようにした（raw rewrite 禁止）。
2. `extract_loop_increment_plan` を拡張して `lit + var` を観測できるようにした（AST rewrite なし）。
3. Deferred: `generic_loop_v0` の step 検出を `UpdateCanon` ベースへ段階移行する（契約/理由は `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`）。
4. Deferred: planner 候補追加の優先度ガード（contract の迂回禁止）を精緻化する。
5. Next phase: Loop を “構造箱” として扱える v1（LoopFrame + `Break/Continue(depth)`）へ進み、selfhost 側の回避コードを減らせる土台を作る（SSOT: `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`）。

## Parser scan loop refactor (priority)

- Prioritize `parser_scan_loop_box.hako` to centralize single-increment scan loops, then slim string scan escape handling and using collector to reduce nested ifs.
- Design note (scheduled): 代入-only if による `IfEffect.then_effects empty` を、`.hako` 側のダミー leaf effect なしで解消するために Select/IfSelect 正規化（データフローの条件付き更新）を CorePlan 側へ追加する（SSOT: `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`）。

## Gate (SSOT)

- Selfhost planner-required entry: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- JoinIR regression pack: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- Planner-required dev gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`

## P2 status

- `phase29bp_planner_required_dev_gate_v4_vm.sh` green.
- `phase29ae_regression_pack_vm.sh` green (post-change).

## Acceptance criteria (RC)

- `phase29bq_selfhost_planner_required_dev_gate_vm.sh` が RC=0（target ケース含む）。
- `phase29ae_regression_pack_vm.sh` が RC=0。

## Handoff conditions (Loop structure box v1)

- UpdateCanon が analysis-only で追加済み（raw rewrite なし）。
- `extract_loop_increment_plan` が `lit + var` を観測できる。
- `phase29bp_planner_required_dev_gate_v4_vm.sh` が green を維持。
- `phase29ae_regression_pack_vm.sh` が green を維持。
