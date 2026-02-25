---
Status: SSOT
Scope: Migrate adopt/fallback observability to FlowBox schema (strict/dev only)
Related:
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
- docs/development/current/main/design/flowbox-fallback-observability-ssot.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# FlowBox adopt tag migration (SSOT)

目的: strict/dev の採用点・フォールバック点の観測を FlowBox schema に統一し、pattern 名や旧タグに依存しない SSOT に収束する。

## Invariants (SSOT)

- release 既定の挙動/恒常ログは不変（FlowBox タグは strict/dev のみ）
- 観測は “schema 固定”:
  - adopt: `[flowbox/adopt box_kind=... features=... via={shadow|release}]`
  - freeze: `[flowbox/freeze code=... box_kind=... features=...]`
- emit/merge は FlowBox タグ生成のために CFG/Facts を再解析しない（手元の Facts/CorePlan のみ）。

## Deprecation target

以下は段階的にゼロへ寄せ、最終的に撤去する（strict/dev only）:

- `[coreplan/shadow_adopt:*]`

Status: removed in Phase 29az P2 (code + smokes).

## Migration steps (SSOT)

1. Smokes を “raw output の FlowBox schema” で検証できるようにする（generic smoke は `filter_noise` 維持）
2. FlowBox schema が十分にカバーされてから、旧タグ参照を撤去する

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
