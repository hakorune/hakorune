---
Status: Complete
Scope: pattern5_infinite_early_exit decomposition (LoopTrueSkeleton + ExitIfMap + CarrierUpdate)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29bw: pattern5_infinite_early_exit decomposition

Goal: pattern5_infinite_early_exit を `LoopTrueSkeleton + FeatureSet` 合成へ寄せる。
release 既定は不変、strict/dev + planner_required のみで表現力を広げる。

Non-goals:
- AST rewrite
- by-name 特例
- 既定挙動変更

Entry (gates):
- `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`

Acceptance:
- 上記 2 gate が PASS

## Plan (P0-P3)

- P0: 対象ファイルを SSOT で固定（この README と `src/mir/builder/control_flow/plan/REGISTRY.md`）
- P1: pattern5 の normalizer を skeleton + pipeline 呼び出しのみに寄せる
- P2: exit-map / carrier / conditional_update の重複を feature へ集約
- P3: legacy 経路を deprecated→削除し、gate green を証跡として closeout

## Targets (SSOT)

- `src/mir/builder/control_flow/plan/normalizer/pattern5_infinite_early_exit.rs`
- `src/mir/builder/control_flow/plan/facts/pattern5_infinite_early_exit_facts.rs`

## Notes

- selfhost canary は主目的にしない（secondary）。

## Progress

- P0: targets fixed in this README + REGISTRY.
- P1: pattern5 normalizer → pipeline 呼び出しに寄せた。
- P2: exit/phi/branch 合流は feature helper へ集約。
- P3: legacy 経路削除、post-change green（`phase29bq_fast_gate_vm.sh` / `phase29bp_planner_required_dev_gate_v4_vm.sh`）。
