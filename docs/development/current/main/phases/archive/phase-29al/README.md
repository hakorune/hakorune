---
Status: Complete
Scope: JoinIR / PlanFrag “仕上げ”の設計SSOT（仕様不変）
Related:
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-ssot-registry.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/design/joinir-plan-frag-ssot.md
---

# Phase 29al: CorePlan composition hardening (docs-first)

Goal: “pattern が重なる/増殖する” を設計で根治し、JoinIR/PlanFrag を **美しく閉じる**ための SSOT を揃える。

## P0: Skeleton/Feature model SSOT（docs-only）

- 指示書: `docs/development/current/main/phases/phase-29al/P0-SKELETON-FEATURE-SSOT-INSTRUCTIONS.md`
- ねらい: “骨格→特徴→合成” を SSOT として固定し、通らない/危険なパターンも Freeze taxonomy に落とす
- 成果: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`

## P1: post-phi（join 入力の最終表現）SSOT

- ねらい: join 値の “最終表現” と “局所 verify” の不変条件を SSOT 化する（emit/merge の再解析禁止を強化）
- 指示書: `docs/development/current/main/phases/phase-29al/P1-POST-PHI-FINAL-FORM-SSOT-INSTRUCTIONS.md`
- SSOT: `docs/development/current/main/design/post-phi-final-form-ssot.md`

## P2: effect classification SSOT（docs-first）

- ねらい: effect 分類と “許される変形” を SSOT 化し、最適化/RC insertion/観測が相互に壊さない境界を固定する
- 指示書: `docs/development/current/main/phases/phase-29al/P2-EFFECT-CLASSIFICATION-SSOT-INSTRUCTIONS.md`
- SSOT: `docs/development/current/main/design/effect-classification-ssot.md`

## P3: ExitKind/Cleanup/Effect contract SSOT（docs-first）

- ねらい: cleanup を ExitKind と effect の契約として固定し、Control/Io を跨いだ移動や DCE 消去の事故を防ぐ
- 指示書: `docs/development/current/main/phases/phase-29al/P3-EXITKIND-CLEANUP-EFFECT-CONTRACT-INSTRUCTIONS.md`
- SSOT: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`

## P5: CorePlan migration roadmap SSOT（docs-only）

- ねらい: CorePlan へ移行完了と言える条件と、安全順のタスク列を 1 枚 SSOT として固定する
- SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## Next (planned)

### P4: ExitKind に unwind を追加する設計（design only）

- ねらい: ExitKind/cleanup/effect の契約を unwind を含めて破綻しない形に拡張する（実装は別フェーズ）
  - 注: unwind の “予約” は Phase 29au（docs-first）で完了済み:
    `docs/development/current/main/phases/phase-29au/README.md`
