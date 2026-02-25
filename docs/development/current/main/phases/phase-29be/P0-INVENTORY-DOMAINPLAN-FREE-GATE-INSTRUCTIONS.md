---
Status: Ready
Scope: docs-only
---

# Phase 29be P0: Inventory — domain-plan-free gate（docs-first）

## Goal

Stage-3 の対象を “コード上の事実” で確定する。

「release の gate が CorePlan 合成で通っている」に見えても、内部で `DomainPlan → PlanNormalizer::normalize(...)`
を踏んでいる場合は “CorePlan purity Stage-3” の未完了扱いとする。

## SSOT

- Gate: `docs/development/current/main/phases/phase-29ae/README.md`
- Purity Stage-2: `docs/development/current/main/design/coreplan-purity-stage2-ssot.md`
- FlowBox fallback codes: `docs/development/current/main/design/flowbox-fallback-observability-ssot.md`

## What to inventory

### 1) composer: DomainPlan normalize 経路

候補:

- `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`
  - `try_compose_core_loop_for_domain_plan(...)` 内の `PlanNormalizer::normalize(builder, domain_plan.clone(), ctx)`

棚卸し結果として、この表を Phase README に追記する:

| Location | Why it uses DomainPlan | Gate impact | Replacement plan |
| --- | --- | --- | --- |
| `coreloop_single_entry.rs` | Pattern2/3/5 (value_join_needed=false) | gate内/外 | facts→parts→normalizer で置換 |

### 2) router: final fallback

候補:

- `src/mir/builder/control_flow/joinir/patterns/router.rs`
  - `lower_via_plan(builder, domain_plan, ctx)` を呼ぶ経路

ここは “最終fallback” として残してもよいが、Stage-3 の完了条件としては **gate が踏まない** ことが必要。

### 3) plan/normalizer: “facts → parts” の不足

Pattern2/3/5 の “value_join_needed=false” で DomainPlan normalize に寄っている理由が
「facts だけでは parts が作れない」なら、その不足を列挙する。

例（placeholder）:

- Pattern2Break: `Pattern2BreakPlan` 相当の parts を facts から復元できない

## Deliverables

- `docs/development/current/main/phases/phase-29be/README.md` に inventory 表を追加（P0 ✅）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` /
  `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` を Phase 29be に切替

## Acceptance

- docs-only（テスト不要）

