---
Status: Complete
Scope: generic_loop_v0/v1 decomposition (Skeleton + FeatureSet)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md
---

# Phase 29bu: generic_loop_v0/v1 decomposition

Goal: generic_loop_v0/v1 を `LoopSkeleton + FeatureSet` 合成へ寄せる。
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

- P0: decomposition の入口（skeleton/feature）を `generic_loop_v0/v1` に接続（挙動不変）
- P1: loop_var/cond/step の判定を `canon` に寄せ、Facts は観測だけに薄くする（no rewrite）
- P2: `ExitMap` / `Carrier` / `StepMode` / `ContinueTarget` を feature 化して、normalizer を “compose only” にする
- P3: legacy 経路を deprecated→削除（入口一本化）し、gate green を証跡として closeout

## Targets (decompose order)

1. `generic_loop_v0`（release 既定維持）
2. `generic_loop_v1`（strict/dev + planner_required のみ）

## Notes

- このフェーズでは selfhost canary を主目的にしない（secondary）。表現力の拡張は “小さい feature の合成” に寄せる。

## Progress

- P0: `generic_loop_v0/v1` の normalizer を skeleton 経由に接続（挙動不変）。
- P1: loop_var/cond/step 判定を canon 側へ寄せ、Facts を薄くした（no rewrite）。
- P2: normalizer を skeleton + feature 呼び出しに寄せ、generic_loop step/body を features に分離（`phase29bq_fast_gate_vm.sh` / `phase29bp_planner_required_dev_gate_v4_vm.sh` PASS）。
