# CURRENT_TASK (moved)

Status: SSOT  
Scope: Repo root の旧リンク互換。現行の入口は `docs/development/current/main/10-Now.md`。

- Now: `docs/development/current/main/10-Now.md`
- Backlog: `docs/development/current/main/30-Backlog.md`
- Archive（過去ログ）: `docs/development/current/main/phases/phase-29ao/CURRENT_TASK-archive.md`

## Stop-the-line memo (Phase 29br)

- Repro: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Failure case: `apps/tests/phase118_pattern3_if_sum_min.hako`
- Note: selfhost/.hako 側のループ変形追跡は一旦中止し、CorePlan（Facts/Normalize）の表現力強化を先行する。
- Direction (SSOT):
  - Condition の形揺れ（例: `j+m<=n`）は raw を書き換えず、analysis-only view（CondCanon/UpdateCanon）で受理範囲を増やす。
  - 最初の着手: generic loop v0 の `loop_var` を “候補列挙→step一致で一意決定” に拡張（`src/mir/builder/control_flow/plan/generic_loop/canon.rs`）。

## Stop-the-line memo (Phase 29bq rerun)

- Repro: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Latest log: `/tmp/phase29bq_selfhost_phase118_pattern3_if_sum_min.hako.log`
- Current blocker:
  - Stage-B compile fails with: `[plan/freeze:unsupported] generic loop v0.2: control flow after in-body step`
  - Trigger shape: `loop(cond)` の step（loop_increment）が body の末尾ではなく、step の後ろに `exit-if`（例: `if (...) { break }`）が続く形
- Hypothesis (2 max):
  - A: Facts が step を抽出した時点で「step を step_bb に移動できる」前提を強く持っており、body-step + tail-exit を表現する LoopFrame（step-in-body）語彙が不足している
  - B: step placement を Facts で保持していないため、normalizer が “step を body に残す” 形へ分岐できず、結果として strict/dev で fail-fast している
- Next action (design-first):
  - `StepPlacement` を Facts に保持し、step-in-body を表現できる最小 plan/lowerer（LoopFrame v1）を設計SSOT→実装（strict/dev + planner_required 限定、release既定不変、AST rewrite禁止）。
