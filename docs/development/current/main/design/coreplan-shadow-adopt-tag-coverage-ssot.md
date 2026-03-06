---
Status: Deprecated
Scope: Historical - replaced by FlowBox schema tags
Related:
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
---

# CorePlan shadow-adopt tag coverage (historical)

目的: 旧 `[coreplan/shadow_adopt:*]` 観測タグの運用履歴を残す。

現行の SSOT は FlowBox schema:

- `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md`

## 前提

- タグは strict/dev の診断・観測用途であり、release の既定挙動を変えない。
- 一部のスモークは `filter_noise` によりタグが落ちるため、タグ検証は raw output を参照する。

## Tag vocabulary (SSOT)

Note:
- tag suffix には pattern-era token が残るが、これは traceability-only。
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

| Scenario | Smoke | Tag |
|---|---|---|
| `loop_simple_while` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_strict_shadow_vm.sh` | `pattern1_simplewhile` |
| `loop_break` planner route | `tools/smokes/v2/profiles/integration/apps/archive/phase29ai_pattern2_break_plan_subset_ok_min_vm.sh` | `pattern2_break_subset` |
| `loop_break` realworld route (phase263) | `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh` | `pattern2_break_subset` |
| `loop_break` body-local route (2 cases) | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh` | `pattern2_break_subset` |
| `loop_break` body-local-seg route (2 cases) | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh` | `pattern2_break_subset` |
| `if_phi_join` route | `tools/smokes/v2/profiles/integration/apps/phase118_pattern3_if_sum_vm.sh` (legacy smoke name) | `pattern3_ifphi` |
| `loop_true_early_exit` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern5_strict_shadow_vm.sh` | `pattern5_infinite_early_exit` |
| `scan_with_init` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern6_strict_shadow_vm.sh` | `pattern6_scan_with_init` |
| `split_scan` strict shadow adopt | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern7_strict_shadow_vm.sh` | `pattern7_split_scan` |

## Forbidden tags (negative gates)

| Scenario | Smoke | Forbidden tag |
|---|---|---|
| `loop_break` route NotApplicable | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_notapplicable_min_vm.sh` | `pattern2_break_subset` |
| `loop_break` route Freeze | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_freeze_min_vm.sh` | `pattern2_break_subset` |
| `loop_simple_while` subset reject (extra stmt) | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_subset_reject_extra_stmt_vm.sh` | `pattern1_simplewhile` |

## Gate (SSOT)

- Integration gate:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - このパックに “positive/negative” のタグ検証が含まれていることが前提。
