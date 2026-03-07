---
Status: SSOT
Scope: JoinIR smoke script stem retirement
Decision: accepted
Related:
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
- CURRENT_TASK.md
---

# JoinIR smoke legacy stem retirement

## Goal

Active docs and regression guidance should point at semantic route names, while old smoke
script stems remain available as compatibility entrypoints until all callers move.

## Rules

- Do not rename legacy smoke scripts in-place during cleanup slices.
- Add semantic alias wrappers first.
- Migrate active docs and pack filters to the alias wrappers.
- Keep legacy script stems for traceability and backward compatibility until grep shows
  no active callers outside history/archive inventories.

## Phase status

- Phase A: inventory fixed
- Phase B: semantic alias wrappers added for current route-facing smokes
- Phase C: active docs and regression packs may switch to semantic alias wrappers
- Phase D: old stems can retire only after active callers reach zero

## Alias map

| Semantic alias wrapper | Legacy target |
| --- | --- |
| `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_strict_shadow_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_subset_reject_extra_stmt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_subset_reject_extra_stmt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern2_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/if_phi_join_vm.sh` | `tools/smokes/v2/profiles/integration/apps/archive/phase118_pattern3_if_sum_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/if_phi_join_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern3_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_continue_only_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ap_pattern4_continue_min_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_vm.sh` | `tools/smokes/v2/profiles/integration/apps/archive/phase286_pattern5_break_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern5_strict_shadow_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern5_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/scan_with_init_strict_shadow_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern6_strict_shadow_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/scan_with_init_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern6_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/scan_with_init_regression_pack_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ae_pattern6_scan_with_init_pack_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/nested_loop_minimal_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ap_pattern6_nested_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/nested_loop_minimal_strict_shadow_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ap_pattern6_nested_strict_shadow_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/split_scan_strict_shadow_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern7_strict_shadow_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/split_scan_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern7_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/split_scan_regression_pack_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ae_pattern7_scan_split_pack_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_plan_subset_vm.sh` | `tools/smokes/v2/profiles/integration/apps/archive/phase29ai_pattern2_break_plan_subset_ok_min_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_realworld_vm.sh` | `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_vm.sh` | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_seg_vm.sh` | `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh` |

## Acceptance

- Regression packs may filter semantic alias wrappers without changing fixture names.
- Active docs should reference alias wrappers when they need a current-facing smoke path.
- Legacy script stems stay in historical inventories, archive packs, and traceability notes only.
