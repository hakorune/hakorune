---
Status: Complete
Scope: CorePlan Step-B（Facts を Skeleton+Feature SSOT に寄せる、仕様不変）
Related:
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-ssot-registry.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an: Skeleton/Feature Facts (SSOT)

Goal: Facts を “complete pattern 列挙” ではなく **Skeleton + Feature** として表現できる状態へ寄せ、CorePlan 合成に向けた SSOT の足場を作る（仕様不変）。

SSOT 道筋: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## Closeout summary

- SkeletonFacts / FeatureFacts（ExitUsage/ExitMap/Cleanup/ValueJoin）の語彙を SSOT 化
- Canonical projections: `skeleton_kind` / `exit_usage` / `exit_kinds_present` / `cleanup_kinds_present` / `value_join_needed`
- debug-only invariants: exit_usage↔plan / exit_usage↔exitmap / cleanup↔exitkind / value_join→exitkind
- Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P0: Add SkeletonFacts (Loop/If/BranchN/StraightLine) as SSOT (code, no wiring)

- 指示書: `docs/development/current/main/phases/phase-29an/P0-SKELETONFACTS-SSOT-INSTRUCTIONS.md`
- ねらい: planner/emit が CFG を覗き直さなくても良いよう、骨格（Skeleton）の観測/導出を Facts に集約する
- 重要: **既定挙動は不変**（Ok(None) のまま、既存 planner-first/legacy ルーティングは触らない）
- Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

Status: ✅ COMPLETE（実装コミット: `ea32d61a5`）

## P1: Add FeatureFacts (ExitMap/ValueJoin/Cleanup materials) as SSOT (code, no wiring)

- 指示書: `docs/development/current/main/phases/phase-29an/P1-FEATUREFACTS-SSOT-INSTRUCTIONS.md`
- ねらい: Skeleton に直交する “特徴” を Facts に寄せ、complete pattern 増殖を止める（まずは ExitUsage から）
- 重要: **既定挙動は不変**（features だけで Ok(Some) にしない）
- Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

Status: ✅ COMPLETE（実装コミット: `c7fbcf3c8`）

## P2: Planner staging for Skeleton→Feature inference (structure only, behavior preserved)

- 指示書: `docs/development/current/main/phases/phase-29an/P2-PLANNER-SKELETON-FEATURE-STAGING-INSTRUCTIONS.md`
- ねらい: planner の内部構造を Skeleton→Feature→CandidateSet の段取りへ寄せる（候補/順序/挙動は不変）
- Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

Status: ✅ COMPLETE（実装コミット: `f866badb3`）

## P3: Add Freeze tag `unstructured` (taxonomy alignment)

- 指示書: `docs/development/current/main/phases/phase-29an/P3-FREEZE-UNSTRUCTURED-TAG-SSOT-INSTRUCTIONS.md`
- ねらい: taxonomy SSOT とコード語彙を一致（まずは未使用の足場）

Status: ✅ COMPLETE（実装コミット: `aa8c12bcf`）

## P4: Require Skeleton/Feature in LoopFacts (type-level SSOT)

- 指示書: `docs/development/current/main/phases/phase-29an/P4-LOOPFACTS-REQUIRE-SKELETON-FEATURES-INSTRUCTIONS.md`
- ねらい: LoopFacts が Some のとき、Skeleton/Feature は必ず揃っている（Option剥がしの散在を止める）
- 重要: `Ok(None)` gate は不変

Status: ✅ COMPLETE（実装コミット: `5ea120ca1`）

## P5: SkeletonFacts — classify `if` without `else` as If2 (SSOT correctness, unused)

- 指示書: `docs/development/current/main/phases/phase-29an/P5-SKELETON-IF-WITHOUT-ELSE-INSTRUCTIONS.md`
- ねらい: SkeletonFacts の If2 判定を “else有無に依存しない” 形に修正し、SSOTとしての正しさを上げる（未接続・仕様不変）

Status: ✅ COMPLETE（実装コミット: `1319cbfb2`）

## P6: Planner skeleton gate (Loop only)

- 指示書: `docs/development/current/main/phases/phase-29an/P6-PLANNER-SKELETON-GATE-INSTRUCTIONS.md`
- ねらい: skeleton.kind が Loop 以外なら `Ok(None)` に倒して fallback を維持（仕様不変）

Status: ✅ COMPLETE（実装コミット: `80c879fdb`）

## P7: CanonicalLoopFacts projections (skeleton_kind / exit_usage)

- 指示書: `docs/development/current/main/phases/phase-29an/P7-CANONICAL-LOOPFACTS-PROJECTIONS-INSTRUCTIONS.md`
- ねらい: normalize SSOT に “骨格/特徴” の入口を揃える（候補/順序/挙動は不変）

Status: ✅ COMPLETE（実装コミット: `5ac2f3586`）

## P8: exit_usage invariants (debug-only)

- 指示書: `docs/development/current/main/phases/phase-29an/P8-EXITUSAGE-INVARIANTS-INSTRUCTIONS.md`
- ねらい: exit_usage と DomainPlan（historical labels `1/2/4/5`）の整合を debug-only で固定（release は仕様不変）

Status: ✅ COMPLETE（実装コミット: `195b424cc`）

## P9: Skeleton unification (0/1/2+ boundary) in Facts (unused, SSOT groundwork)

- 指示書: `docs/development/current/main/phases/phase-29an/P9-SKELETON-UNIFICATION-INFERENCE-INSTRUCTIONS.md`
- ねらい: Skeleton 一意化の境界を Facts 側で SSOT 化（未接続、仕様不変）

Status: ✅ COMPLETE（実装コミット: `0354c17eb`）

## P10: ExitMap facts scaffold (types only, unused)

- 指示書: `docs/development/current/main/phases/phase-29an/P10-EXITMAP-FEATURE-FACTS-SCAFFOLD-INSTRUCTIONS.md`
- ねらい: ExitMapFacts の語彙（型）だけ先に追加して足場を作る（未接続・仕様不変）

Status: ✅ COMPLETE（実装コミット: `b71434ffe`）

## P11: ExitMap presence from ExitUsage (still conservative)

- 指示書: `docs/development/current/main/phases/phase-29an/P11-EXITMAP-PRESENCE-FROM-EXITUSAGE-INSTRUCTIONS.md`
- ねらい: ExitMapFacts を “存在集合” として最小で埋める（対応付け/CFGはやらない、仕様不変）

Status: ✅ COMPLETE（実装コミット: `8caa09768`）

## P12: CanonicalLoopFacts ExitMap projections (exit_kinds_present)

- 指示書: `docs/development/current/main/phases/phase-29an/P12-CANONICAL-EXITMAP-PROJECTIONS-INSTRUCTIONS.md`
- ねらい: ExitMapFacts を normalize 側で投影して、planner/合成が深掘りせず参照できる足場を作る（仕様不変）

Status: ✅ COMPLETE（実装コミット: `fa5a891bd`）

## P13: Cleanup facts scaffold (unused)

- 指示書: `docs/development/current/main/phases/phase-29an/P13-CLEANUP-VOCAB-SCAFFOLD-INSTRUCTIONS.md`
- ねらい: cleanup を Feature として合成するための語彙と projection の足場を作る（未接続、仕様不変）

Status: ✅ COMPLETE（実装コミット: `34ec46d13`）

## P14: ValueJoin facts scaffold (unused)

- 指示書: `docs/development/current/main/phases/phase-29an/P14-VALUEJOIN-VOCAB-SCAFFOLD-INSTRUCTIONS.md`
- ねらい: join値（post-phi）を Feature として合成するための語彙と projection の足場を作る（未接続、仕様不変）

Status: ✅ COMPLETE（実装コミット: `395f3b01d`）

## Next

- Phase 29ao（CorePlan composition: Feature合成→Normalizerへ）
  - 入口: `docs/development/current/main/phases/phase-29ao/README.md`
