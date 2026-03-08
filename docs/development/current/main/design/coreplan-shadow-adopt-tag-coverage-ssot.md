---
Status: Deprecated
Scope: Historical - replaced by FlowBox schema tags
Related:
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
- docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md
---

# CorePlan shadow-adopt tag coverage (historical)

目的: 旧 `[coreplan/shadow_adopt:*]` 観測タグの運用履歴を残す。

現行の SSOT は FlowBox schema:

- `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md`

## 前提

- タグは strict/dev の診断・観測用途であり、release の既定挙動を変えない。
- 一部のスモークは `filter_noise` によりタグが落ちるため、タグ検証は raw output を参照する。

## Historical tag vocabulary (deprecated)

Note:
- tag suffix には pattern-era token が残るが、これは traceability-only。
- smoke path は semantic alias wrapper を優先し、legacy file name は wrapper の転送先または archive pin としてのみ保持する。
- current/legacy stem の対応は `docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md` を正本にする。
- current route semantics は左側の semantic route 名を主語に読む。

- `loop_simple_while` route: `[coreplan/shadow_adopt:pattern1_simplewhile]`
- `loop_break` route: `[coreplan/shadow_adopt:pattern2_break_subset]`
- `if_phi_join` route: `[coreplan/shadow_adopt:pattern3_ifphi]`
- Match (return-only subset): `[coreplan/shadow_adopt:match_return]`
- `loop_true_early_exit` route: `[coreplan/shadow_adopt:pattern5_infinite_early_exit]`
- `scan_with_init` route: `[coreplan/shadow_adopt:pattern6_scan_with_init]`
- `split_scan` route: `[coreplan/shadow_adopt:pattern7_split_scan]`
- Return-in-loop: retired from shadow-adopt coverage (strict path is fail-fast reject as of 2026-03-06)

## Required tags (positive gates)

| Scenario | Smoke path (current semantic wrapper) | Tag suffix (legacy token) |
|---|---|---|
| `loop_simple_while` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh` | `pattern1_simplewhile` |
| `loop_break` planner route | `tools/smokes/v2/profiles/integration/joinir/loop_break_plan_subset_vm.sh` | `pattern2_break_subset` (regression-pack archive-fixed keep) |
| `loop_break` realworld route (phase263) | `tools/smokes/v2/profiles/integration/joinir/loop_break_realworld_vm.sh` | `pattern2_break_subset` (regression-pack archive-fixed keep) |
| `loop_break` body-local route (2 cases) | `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_vm.sh` | `pattern2_break_subset` (coverage-only archive-fixed keep) |
| `loop_break` body-local-seg route (2 cases) | `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_seg_vm.sh` | `pattern2_break_subset` (coverage-only archive-fixed keep) |
| `if_phi_join` route | `tools/smokes/v2/profiles/integration/joinir/if_phi_join_vm.sh` | `pattern3_ifphi` (regression-pack archive-fixed keep) |
| `loop_true_early_exit` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh` | `pattern5_infinite_early_exit` |
| `scan_with_init` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/scan_with_init_strict_shadow_vm.sh` | `pattern6_scan_with_init` |
| `split_scan` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/split_scan_strict_shadow_vm.sh` | `pattern7_split_scan` |

## Forbidden tags (negative gates)

| Scenario | Smoke path (current wrapper or archived legacy stem) | Forbidden tag suffix (legacy token) |
|---|---|---|
| `loop_break` route NotApplicable | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_notapplicable_min_vm.sh` (archived legacy stem) | `pattern2_break_subset` |
| `loop_break` route Freeze | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_freeze_min_vm.sh` (archived legacy stem) | `pattern2_break_subset` |
| `loop_simple_while` subset reject (extra stmt) | `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_subset_reject_extra_stmt_vm.sh` | `pattern1_simplewhile` |

## Gate (SSOT)

- Integration gate:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - このパックに “positive/negative” のタグ検証が含まれていることが前提。
