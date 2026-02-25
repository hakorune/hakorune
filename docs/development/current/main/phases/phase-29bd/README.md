---
Status: Complete
Scope: CorePlan purity Stage-2 (fallback -> 0)
Related:
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/flowbox-fallback-observability-ssot.md
---

# Phase 29bd: CorePlan purity Stage-2 (fallback -> 0)

## Goal

strict/dev で fallback を 0 に収束させるため、Ok(None)/unsupported/unstructured の境界を SSOT 化し、
「許容」「Freeze すべき」を明示する。release 既定の挙動は不変。

## Non-goals

- subset 拡張（facts/extractors/planner の拡張）
- 新しい env var の追加
- release 既定のログ追加

## Plan

- P0: Ok(None) / unsupported の棚卸しと SSOT 化（docs-first）✅
- P1: strict/dev の fallback 収束（実装）✅
- P2: closeout（docs-only）✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29bd/P0-INVENTORY-OKNONE-UNSUPPORTED-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29bd/P1-CONVERGE-STRICTDEV-FALLBACK-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29bd/P2-CLOSEOUT-INSTRUCTIONS.md`

## Inventory (P0)

| Location | Condition | Current behavior | Strict/Dev policy | Notes |
| --- | --- | --- | --- | --- |
| `src/mir/builder/control_flow/joinir/patterns/router.rs:300` | planner_none (facts present, plan None) | flowbox/freeze + Err | Freeze(contract) | gate対象での fallback は禁止 |
| `src/mir/builder/control_flow/joinir/patterns/router.rs:265` | composer_reject (facts+plan present) | flowbox/freeze + Err | Freeze(contract) | strict/dev で fail-fast |
| `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs:119` | DomainPlan not supported | Ok(None) | Freeze(unsupported) | gate対象は fail-fast |
| `src/mir/builder/control_flow/plan/composer/coreloop_v0.rs:27` | v0 gate reject | Ok(None) | Freeze(contract) | value_join/exit/cleanup で拒否 |
| `src/mir/builder/control_flow/plan/composer/coreloop_v1.rs:21` | v1 gate reject | Ok(None) | Freeze(contract) | v1 専用条件の不一致 |
| `src/mir/builder/control_flow/plan/facts/skeleton_facts.rs:67` | skeleton multiple | Err(Freeze::unstructured) | Freeze(unstructured) | 既定のまま |
| `src/mir/builder/control_flow/plan/facts/loop_facts.rs:189` | contract violation | Err(Freeze::bug) | Freeze(contract) | バグ扱いで fail-fast |
| `src/mir/builder/control_flow/plan/facts/pattern_match_return_facts.rs:101` | unsupported match return | Err(Freeze::unsupported) | Freeze(unsupported) | strict/dev のみ採用対象 |
| `src/mir/join_ir/lowering/loop_pattern_router.rs:67` | no pattern matched | Ok(None) | Allow (tag only) | join_ir 側は対象外、段階2で再整理 |

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Summary

- strict/dev fallback を flowbox/freeze に収束（planner_none / composer_reject を fail-fast）
- inventory 表で Ok(None)/Freeze 境界を SSOT 固定
- gate は quick + phase29ae pack を維持（release 既定不変）

## Next

- 次フェーズ候補: Stage-2 継続（未分類の Ok(None) を追加棚卸し）
