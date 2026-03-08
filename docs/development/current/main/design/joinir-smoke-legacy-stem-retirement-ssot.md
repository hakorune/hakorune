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

## Stem categories

- `active semantic wrapper`
  - current-facing smoke entrypoint that docs and daily gates should prefer.
- `compat wrapper`
  - old stem kept as a forwarding script to the active semantic wrapper.
- `archived smoke stem`
  - archived smoke path kept only for traceability or negative coverage.
- `legacy pack stem`
  - old planner-pack stem kept until all callers move to the semantic pack alias.

## Phase status

- Phase A: inventory fixed
- Phase B: semantic alias wrappers added for current route-facing smokes
- Phase C: active docs and regression packs may switch to semantic alias wrappers
- Phase D: old stems can retire only after active callers reach zero
- Phase E1: selected release-adopt/current route wrappers promote the semantic entry to the real script body, and legacy stems become thin forwarders
- Phase E2: strict-shadow / regression-pack / planner-pack wrappers also promote the semantic entry to the real script body; remaining `exec bash` current wrappers are limited to archive replay entries
- Phase E3: archive-backed current wrappers promote the semantic entry to the real script body, and archived stems become historical replay forwarders
- Phase E4: archived replay forwarders retire only after current docs, packs, and manual replay lanes no longer depend on the archived basename

## Alias map

### Route smoke aliases

| Active semantic wrapper | Compat target |
| --- | --- |
| `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_strict_shadow_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_strict_shadow_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_simple_while_subset_reject_extra_stmt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern1_subset_reject_extra_stmt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern2_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/if_phi_join_vm.sh` | `tools/smokes/v2/profiles/integration/apps/archive/phase118_pattern3_if_sum_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/if_phi_join_release_adopt_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern3_release_adopt_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_continue_only_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29ap_pattern4_continue_min_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/loop_continue_only_multidelta_planner_required_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29bq_pattern4continue_multidelta_planner_required_vm.sh` |
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

Note:
- Entries under `apps/archive/` are `archived smoke stems`, not compat wrappers.
- Entries under `joinir/phase29ao*` / `joinir/phase29ap*` / `joinir/phase29ae*` are compat/legacy stems still kept for forwarding or direct pinning.
- No current semantic wrapper should `exec bash` into an archive stem.
- The archive-backed six-route group now uses semantic script bodies, while the archived stems are
  retained as historical replay forwarders:
  - `regression-pack semantic-body wrapper`: `loop_break_plan_subset_vm.sh`, `loop_break_realworld_vm.sh`, `if_phi_join_vm.sh`, `loop_true_early_exit_vm.sh`
  - `coverage-only semantic-body wrapper`: `loop_break_body_local_vm.sh`, `loop_break_body_local_seg_vm.sh`
  - archived replay stems forward back into the semantic wrappers via `LEGACY_STEM_OVERRIDE`, so manual replay keeps the historical basename while current gates stay semantic-first
  - current `phase29ae_regression_pack_vm.sh` now calls the body-local pair via semantic wrapper names, not via the historical `phase29ab_pattern2_` family filter

Resolution rule:
- `semantic-body promotion` for the six archive-backed current wrappers is accepted.
- `caller 0` now applies to the archived replay forwarders and to any remaining exact-doc references,
  not to the current semantic wrappers.
- Retire or collapse the archive stems only after current docs/packs/manual lanes no longer need the
  historical basename as a replay handle.
- repo-local current semantic wrappers no longer `exec bash` into `apps/archive/**`; remaining
  replay forwarding is intentional and lives in the archived stems themselves.

### Archived replay forwarder resolution conditions

| Group | Current active callers | Current structure | Retirement / collapse precondition |
| --- | --- | --- | --- |
| `regression-pack semantic-body wrapper + archived replay forwarder` | `phase29ae_regression_pack_vm.sh` via `loop_break_plan_subset_vm`, `loop_break_realworld_vm`, `if_phi_join_vm`, `loop_true_early_exit_vm` | semantic wrapper owns the real body; archive script is replay-only forwarder | `phase29ae` pack and active docs use only semantic wrappers, and no manual replay/how-to lane still points at the archived basename |
| `coverage-only semantic-body wrapper + archived replay forwarder` | flowbox/coreplan coverage docs and `phase29ae_regression_pack_vm.sh` for the body-local pair | semantic wrapper owns the real body; archive script is replay-only forwarder | coverage docs no longer require the archived basename as a replay handle, and archive/manual guidance either moves to semantic wrappers or is explicitly archived-only |

Operational rule:
- prefer semantic wrapper names in current scripts/docs
- archived stems are replay handles only; they should not be the primary current entry
- do not treat `caller 0` as the sole next-step condition; archive/manual guidance must also stop pointing at the archived basename
- retire only after repo-local caller inventory and active-doc caller inventory both reach zero
- `rg`/grep zero-hit alone is not retirement evidence; include `tools/smokes/v2/run.sh` auto-discovery scope and active-doc/manual replay inventory

### Planner-required pack aliases

| Active semantic wrapper | Legacy pack stem |
| --- | --- |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_planner_required_pack_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/scan_split_planner_required_pack_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_scan_split_pack_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/core_loop_routes_planner_required_pack_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29bl_planner_required_pattern1_4_5_pack_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/if_phi_join_planner_required_pack_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29bn_planner_required_pattern3_pack_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/bool_predicate_accum_planner_required_pack_vm.sh` | `tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh` |

Current semantic planner-case lists:
- `tools/smokes/v2/profiles/integration/joinir/loop_break_planner_required_cases.tsv`
- `tools/smokes/v2/profiles/integration/joinir/core_loop_routes_planner_required_cases.tsv`
- `tools/smokes/v2/profiles/integration/joinir/if_phi_join_planner_required_cases.tsv`
- `tools/smokes/v2/profiles/integration/joinir/bool_predicate_accum_planner_required_cases.tsv`

## Acceptance

- Regression packs may filter semantic alias wrappers without changing fixture names.
- Active docs should reference alias wrappers when they need a current-facing smoke path.
- Planner-required dev gates should call semantic pack alias wrappers rather than legacy pack stems.
- Strict-shadow / regression-pack / planner-pack semantic wrappers should own the real script body.
- Legacy script stems stay in historical inventories, archive packs, and traceability notes only.

## Low-ref inventory (2026-03-07)

The following inventory separates `self-only grep hits` from actual removal candidates.
`rg` counts inside the repo are advisory only; they do not prove that a smoke is dead.

### Keep: current semantic entrypoints

These scripts are current-facing route smoke entrypoints even when repo grep shows only
their own filename.

| Script | Evidence | Why it stays |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/joinir/if_phi_join_release_adopt_vm.sh` | repo grep: self-only | canonical semantic entry for if-phi release-adopt smoke |
| `tools/smokes/v2/profiles/integration/joinir/loop_break_release_adopt_vm.sh` | repo grep: self-only | canonical semantic entry for loop-break release-adopt smoke |
| `tools/smokes/v2/profiles/integration/joinir/loop_continue_only_vm.sh` | repo grep: self-only | canonical semantic entry for continue-only route smoke |
| `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_release_adopt_vm.sh` | repo grep: self-only | canonical semantic entry for loop-true-early-exit release-adopt smoke |
| `tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_vm.sh` | repo grep: self-only | canonical semantic entry for loop-true-early-exit route smoke |
| `tools/smokes/v2/profiles/integration/joinir/nested_loop_minimal_release_adopt_vm.sh` | repo grep: self-only | canonical semantic entry for nested-loop release-adopt smoke |
| `tools/smokes/v2/profiles/integration/joinir/nested_loop_minimal_strict_shadow_vm.sh` | repo grep: self-only | canonical semantic entry for nested-loop strict-shadow smoke |

### Keep: standalone coverage smokes

These scripts are not wrapper aliases. Their low ref count comes from being direct,
single-purpose checks.

| Script | Evidence | Why it stays |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/joinir/json_missing_vm.sh` | repo grep: self-only | unique MissingBox/file-using observation coverage |
| `tools/smokes/v2/profiles/integration/joinir/method_resolution_is_eof_vm.sh` | repo grep: self-only | unique method-resolution regression coverage |
| `tools/smokes/v2/profiles/integration/joinir/phase29bh_planner_first_parse_integer_ws_single_case_vm.sh` | repo grep: self-only | unique single-case planner-first probe |

### Future retire groups

These are not safe hard-delete targets yet; retire only when caller inventory is zero and the
lane-specific precondition below is also satisfied.

| Group | Current role | Retirement precondition |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/joinir/phase29ao_pattern*.sh` | compat wrapper targets for route smoke aliases | semantic wrappers or packs are the only remaining callers |
| `tools/smokes/v2/profiles/integration/joinir/phase29ap_pattern*.sh` | compat wrapper targets for continue/nested route aliases | semantic wrappers are the only remaining callers |
| `tools/smokes/v2/profiles/integration/joinir/phase29ae_pattern{6,7}_*.sh` | legacy regression pack stems | semantic regression-pack wrappers are the only remaining callers |
| `tools/smokes/v2/profiles/integration/joinir/phase29bi/phase29bj/phase29bl/phase29bn/phase29bo_*pack_vm.sh` | legacy planner-pack stems | semantic planner-pack wrappers are the only remaining callers |
| `tools/smokes/v2/profiles/integration/apps/archive/{phase29ai_pattern2_break_plan_subset_ok_min_vm,phase263_pattern2_seg_realworld_min_vm,phase29ab_pattern2_loopbodylocal_min_vm,phase29ab_pattern2_loopbodylocal_seg_min_vm,phase118_pattern3_if_sum_vm,phase286_pattern5_break_vm}.sh` | archived replay forwarders | active docs/packs/manual lanes no longer need the archived basename as a replay handle |
| `tools/smokes/v2/profiles/integration/joinir/phase143_legacy_pack.sh` / `phase286_pattern9_legacy_pack.sh` | archived legacy pack stems | retained only until historical phase docs/archive references are explicitly retired |

### Decision

- No top-level smoke script is a safe hard-delete candidate as of 2026-03-07.
- The next cleanup step is to keep thinning `compat wrapper` callers and move archive-only
  mentions into historical ledgers, not to delete low-ref semantic entrypoints.
- `phase143_legacy_pack.sh` and `phase286_pattern9_legacy_pack.sh` are `archived legacy pack stems`:
  they are intentionally skipped and kept for historical phase replay only, not for current gates.

## Manual-lane and one-case wrappers (2026-03-08)

These scripts are intentionally low-ref, but they are not retirement-ready without an explicit
archive/manual-lane decision.

| Script | Classification | Current canonical reference |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/joinir/phase29bq_mir_preflight_lowered_away_reject_vm.sh` | historical compat wrapper | `phase29bq_mir_preflight_unsupported_reject_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/phase29bq_continue_target_header_planner_required_vm.sh` | one-case semantic wrapper | `phase29bq_fast_gate_vm.sh --only continue_target_header` |
| `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase5_min_vm.sh` | manual single-fixture pin | `phase29bq_hako_mirbuilder_quick_suite_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase7_min_vm.sh` | manual single-fixture pin | `phase29bq_hako_mirbuilder_quick_suite_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase9_min_vm.sh` | manual single-fixture pin | `phase29bq_hako_mirbuilder_quick_suite_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port02_if_merge_minimal_vm.sh` | historical standalone probe | later if-merge ports in fast-gate / current route docs |
| `tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_conditional_update_release_route_vm.sh` | manual release-route probe | `phase29bq_conditional_update_join_planner_required_vm.sh` |
| `tools/smokes/v2/profiles/integration/joinir/phase1883_nested_minimal_vm.sh` | historical compat wrapper | `nested_loop_minimal_strict_shadow_vm.sh` |

Operational rule:
- keep these scripts as long as current docs or manual replay instructions still point at them
- when active guidance moves to the canonical reference and repo-local callers drop to zero, retire
  them via the same archive-first process used for other compat wrappers
