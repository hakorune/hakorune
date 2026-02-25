---
Status: Complete
Scope: Plan boxes decomposition (Skeleton + FeatureSet) to keep the compiler compositional
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- src/mir/builder/control_flow/plan/REGISTRY.md
---

# Phase 29bt: Plan decomposition (compiler cleanliness first)

Goal: “箱（plan rule）を増やして凌ぐ”のではなく、`Skeleton + FeatureSet` 合成へ寄せて CorePlan 周りをシュッとさせる。
結果として、以後の selfhost/unblock も「最小部品の追加」で進むようにする。

## Non-goals

- 既定挙動の変更（release default unchanged）
- AST rewrite（見かけ等価の式変形・コード移動）
- selfhost/.hako 側 workaround の追加

## Entry (gates)

- Fast: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- Checkpoint: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`

## P0 (entry and slots)

- composer/feature の入口と置き場を作る（分解の導線だけ先に整える）
- 既存の normalizer/planner から直接参照しない（SSOT は feature へ寄せる）
- Status: ✅ done（`features/` / `skeletons/` の責務を README で固定）

## P1 (first extraction)

- `pattern3_if_phi` の if-join（pred 収集/phi 挿入規則）を JoinFeature として抽出
- 影響範囲は最小（既定挙動不変・strict/dev の観測強化のみ）
- P1 post-change green: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh` PASS
- Status: ✅ done（`features/if_join.rs` に集約、lowerer は呼び出しのみ）

## P2 (second extraction)

- `loop_true_break_continue` を LoopTrueSkeleton + NestedLoopFeature に分解
- nested loop(true) 1段 + general-if 拡張は feature に移す（箱肥大化を止める）
- Status: ✅ done（挙動不変、fast/dev gate green）

## P3 (third extraction)

- `loop_cond_break_continue` を `LoopCondSkeleton + FeatureSet` に分解
  - `ExitIfMap` / `ConditionalUpdateJoin` / `CarrierMerge` / `GuardBreak` を features に切り出す
  - `ContinueWithPhiArgs` / join PHI の扱いを “feature 側”へ寄せて箱肥大化を止める
  - normalizer は skeleton+feature の呼び出しだけに薄くする（AST 再解析禁止）
- P3 post-change green: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh` PASS
- Status: ✅ done（feature 分解 + gate green）

## Acceptance criteria

- `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh` PASS
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh` PASS
