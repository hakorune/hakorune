---
Status: Complete
Scope: CorePlan purity Stage-3 (domain-plan-free gate)
Related:
- docs/development/current/main/design/coreplan-purity-stage2-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/flowbox-fallback-observability-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29be: CorePlan purity Stage-3（domain-plan-free gate）

## Goal

release 既定の gate 対象（phase-29ae regression pack）が、`Facts → composer(single entry) → CorePlan → lowerer`
の導線だけで通る状態へ収束させる。

strict/dev の “candidateっぽいのに Ok(None)” は Stage-2（Phase 29bd）で `flowbox/freeze` に収束済みなので、
Stage-3 では **DomainPlan 経由の合成（PlanNormalizer::normalize(domain_plan)) を gate から段階的に外す**。

## Non-goals

- subset 拡張（新しい stdlib 形状を増やす）
- 新しい env var の追加
- release 既定の恒常ログ/エラー文字列の変更

## Plan

- P0: Inventory（docs-first）— gate が “DomainPlan を踏んでいる箇所” を列挙し、置換順を決める ✅
- P1: coreloop composer を facts-first に寄せる（DomainPlan normalize 経路を削減）✅
- P2: router の `lower_via_plan(domain_plan)` を “最終fallback” に縮退（gate では踏まない）✅
- P3: closeout（docs-only）✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29be/P0-INVENTORY-DOMAINPLAN-FREE-GATE-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29be/P1-REMOVE-DOMAINPLAN-NORMALIZE-IN-COMPOSER-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29be/P2-NO-LOWER_VIA_PLAN-IN-GATE-INSTRUCTIONS.md`

## Inventory (P0)

### 1) composer: DomainPlan normalize 経路

| Location | Why it uses DomainPlan | Gate impact | Replacement plan |
| --- | --- | --- | --- |
| `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs` | Pattern2/3/5 の `value_join_needed=false` を typed normalizer（`normalize_pattern2_break/3_if_phi/5_infinite_early_exit`）で合成 | gate 内 | facts → parts → typed entry へ置換済み |

### 2) router: final fallback

| Location | Why it uses DomainPlan | Gate impact | Replacement plan |
| --- | --- | --- | --- |
| `src/mir/builder/control_flow/joinir/route_entry/router.rs` | `lower_via_plan(domain_plan)` が最終 fallback | gate では strict/dev で freeze（`composer_reject`）、lower_via_plan は非gateのみ | strict/dev は fail-fast、release は最終fallback（historical path token: `joinir/patterns/router.rs`） |

### 3) plan/normalizer: facts → parts の不足

- Pattern2Break: `Pattern2BreakPlan` 相当の parts を facts だけで復元できず DomainPlan に依存
- Pattern3IfPhi: branch updates の構成が DomainPlan に寄っている
- Pattern5InfiniteEarlyExit: exit kind/value の parts を facts から復元できない

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
