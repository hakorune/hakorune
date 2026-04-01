---
Status: Complete
Scope: CorePlan Step-A（lowerer/verifier の穴埋め、仕様不変で段階導入）
Related:
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29am: CorePlan Step-A implementation (lowerer/verifier)

Goal: CorePlan で“組み立てる”移行を進める前に、`CorePlan` の語彙が単独でも lower/verify できる状態へ近づける。

SSOT 道筋: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## P0: Implement CorePlan::If / CorePlan::Exit lowering (minimal)

- 指示書: `docs/development/current/main/phases/phase-29am/P0-COREPLAN-LOWERER-IF-EXIT-INSTRUCTIONS.md`
- ねらい: 既存のルーティング/観測を変えずに、CorePlan 側の “未対応エラー” を減らす（仕様不変）
- Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P1: Allow Seq-of-effects in CoreLoopPlan body (flatten)

- 指示書: `docs/development/current/main/phases/phase-29am/P1-CORELOOP-BODY-SEQ-FLATTEN-INSTRUCTIONS.md`
- ねらい: ループ body_bb では “Effect だけ” を前提にしつつ、`Seq([Effect...])` を安全に flatten して emit できるようにする
- 非目的: `If/Exit` を body_bb に入れる（これは Frag/ExitMap 側の語彙で表現する）

## P2: Verifier enforces Loop.body Effect-only (Seq flatten allowed)

- 指示書: `docs/development/current/main/phases/phase-29am/P2-VERIFY-CORELOOP-BODY-EFFECTONLY-INSTRUCTIONS.md`
- ねらい: lowerer の “Non-Effect plan in Loop body” エラーを、PlanVerifier の fail-fast で先に検出して局所化する（仕様不変）

## P3: ExitMap/CoreExitPlan alignment (design + minimal code)

- 指示書: `docs/development/current/main/phases/phase-29am/P3-EXITMAP-COREEXIT-ALIGNMENT-INSTRUCTIONS.md`
- ねらい: CorePlan 内で `Exit` を “独立ノード” として増やさず、Frag/ExitMap と整合する表現へ寄せる（仕様不変）

## Next

- Phase 29an（Skeleton/Feature Facts）へ進む: `docs/development/current/main/phases/phase-29an/README.md`
