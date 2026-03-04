# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-04
Scope: repo root の再起動入口。詳細ログは `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で `Current Blocker` と `next fixed order` に到達する。
- 本ファイルは薄い入口に保ち、長文履歴はアーカイブへ逃がす。
- runtime lane の Next は `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` を単一正本に固定する。

## Focus Lock (2026-03-02)

- primary target: `kernel-mainline`（`.hako` kernel）を日常既定経路に固定。
- no-fallback: `NYASH_VM_USE_FALLBACK=0`（silent fallback 禁止 / fail-fast）。
- compiler lane は `phase-29bq` を monitor-only 運用（failure-driven reopen のみ）。

## Current Blocker (SSOT)

- compiler lane: `phase-29bq / monitor-only`
  - current blocker: `none`
  - reopen condition: `emit_fail > 0` または `route_blocker > 0`
  - task SSOT:
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
    - `docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`
- runtime lane: `phase-29y / none`
  - fixed order SSOT:
    - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- compiler pipeline lane: `hako-using-resolver-parity / monitor-only`
  - SSOT:
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- de-rust orchestration lane: `phase-29cc / monitor-only`
  - SSOT:
    - `docs/development/current/main/phases/phase-29cc/README.md`
    - `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
- perf lane: `phase-21.5 / monitor-only`
  - SSOT:
    - `docs/private/roadmap/phases/phase-21.5/PLAN.md`

## Immediate Next (this round)

- docs-first / compiler lane SSOT:
  - `docs/development/current/main/design/compiler-task-map-ssot.md`
  - `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
- execution rule:
  - 1 blocker = 1受理形 = fixture+gate = 1 commit
  - BoxCount と BoxShape を同コミットで混在させない
- compiler fixed order:
  1. `phase29x-probe` を定点観測し、`emit_fail=0` / `route_blocker=0` を維持確認。
  2. D5 cleanup 継続: facts/planner 側の dead entry を isolate -> delete。
  3. D5 cleanup 継続: test-only wire の `#[cfg(test)]` 化と不要 module wire 削除。
  4. D5 cleanup 継続: planner/build の legacy negative test 群を DomainPlan-only 契約へ整形。

## Compiler Cleanup Order (2026-03-04, SSOT)

- D5-A: facts/planner dead staging 削除（挙動不変）
- D5-B: runtime 未参照の legacy module を isolate -> delete
- D5-C: diagnostic-only vocabulary を semantic key に揃える
- D5-D: test-only module wire を runtime build から分離
- D5-E: planner/build test 群を DomainPlan-only の事実に合わせて縮退

## Latest Probe Snapshot (direct route)

- command:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
- latest result (2026-03-04):
  - `emit_fail=0`
  - `run_nonzero=18`
  - `run_ok=101`
  - `route_blocker=0`
  - total=`119`
- class_counts:
  - `run:nonzero-empty=6`（intentional contract exit）
  - `run:nonzero=9`（direct runner bridge limitation cluster）
  - `run:vm-error=3`（provider-dependent cluster）
  - `run:ok=101`
- note:
  - monitor-known 扱い。compiler blocker は `none` 維持。

## Restart Handoff (2026-03-04)

- this round commits:
  - `49c97d94f` fix D5 restore pattern2 promotion hint tag emission
  - `c2ab89104` refactor D5 prune dead single planner recipe-only guards
  - `936b7766a` docs D5 sync planner SSOT comments with domain-plan-only flow
  - `4d4effb81` docs phase29x classify run_nonzero monitor-known clusters
  - `17431bbae` refactor D5 collapse dead loop facts staging toggles
  - `22532f0bf` refactor D5 delete dead pattern8 plan-side module
  - `6d5b1ab7c` refactor D5 align planner shadow with semantic rule keys
  - `32f735d9c` refactor D5 gate facts loop_tests as test-only module

- key behavior lock (kept green):
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh`
  - `cargo build --release --bin hakorune`

- known note:
  - `cargo test -q --lib facts_extracts_pattern9_const_accum_success` は現作業ツリーで既存 mismatch（本ラウンド差分では未変更）

## next fixed order (resume point)

1. D5-E: `planner/build_tests.rs` の legacy-heavy negative tests を DomainPlan-only 契約へ整理。
2. D5-E: `single_planner/rule_order.rs` の legacy label 互換を最小化（gate互換を維持する範囲）。
3. D5-E: `facts/planner` 側の dead import / dead comment を掃除して SSOT と同期。
4. 各ステップで `bq` + `phase29x-probe` を回し、`emit_fail=0` / `route_blocker=0` を維持確認。

## Quick Restart (After Reboot)

1. `git status -sb`
2. `sed -n '1,220p' CURRENT_TASK.md`
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
4. `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`

## Read-First Navigation

- root pointer: `CURRENT_TASK.md`（このファイル）
- compiler map SSOT: `docs/development/current/main/design/compiler-task-map-ssot.md`
- cleanliness SSOT: `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
- planner gate SSOT: `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- ai/debug contract SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`

## Quick Entry: Selfhost Migration

1. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
2. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
4. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`

## Daily Commands

- fast gate:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- planner-required packs:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_pattern6_7_pack_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh`
- probe:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`

## Archive

- full historical log (2111 lines, archived 2026-03-04):
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
- policy:
  - 長文の時系列ログは以後 archive 側へ追記し、`CURRENT_TASK.md` は再起動用の薄い入口を維持する。
