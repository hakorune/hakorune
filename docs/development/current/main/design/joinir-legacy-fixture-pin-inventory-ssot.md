---
Status: SSOT
Scope: Active legacy fixture pin inventory for JoinIR / selfhost guidance
Decision: accepted
Related:
- docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md
- docs/development/current/main/design/pattern6-7-contracts.md
- docs/development/current/main/design/pattern-p5b-escape-design.md
- docs/development/current/main/design/coreloop-stepmode-inline-in-body-ssot.md
- CURRENT_TASK.md
---

# JoinIR legacy fixture pin inventory

## Goal

Keep current docs route-first while preserving the small set of legacy fixture and case
tokens that still act as traceability pins.

## Rules

- Active design docs should describe runtime behavior with semantic route names.
- Legacy fixture filenames and case ids stay as pins only.
- Do not rename pinned fixture filenames in-place during cleanup slices.
- If a rename is needed later, use alias-first retirement and keep the old token in this
  ledger until active callers reach zero.

## Pin categories

- `legacy fixture key`
  - by-name / filter contract token that is still required by a live entrypoint.
- `legacy fixture pin token`
  - fixture filename or case id kept for traceability, gate pinning, or command examples.
- `legacy selfhost test stem`
  - selfhost-side test filename stem kept only for traceability.
- `semantic fixture alias`
  - preferred current fixture basename for active gate/selfhost guidance.

## Active pins

| Legacy pin token | Current route semantics | Pin category |
| --- | --- | --- |
| `phase118_pattern3_if_sum_min.hako` | `if_phi_join` | legacy fixture pin token |
| `if_phi_join_min.hako` | `if_phi_join` | semantic fixture alias |
| `loop_break_plan_subset_min.hako` | `loop_break subset` | semantic fixture alias |
| `loop_break_realworld_min.hako` | `loop_break realworld` | semantic fixture alias |
| `loop_break_body_local_min.hako` | `loop_break body-local` | semantic fixture alias |
| `loop_break_body_local_seg_min.hako` | `loop_break body-local-seg` | semantic fixture alias |
| `loop_true_early_exit_min.hako` | `loop_true_early_exit` | semantic fixture alias |
| `string_is_integer_min.hako` | `string_is_integer` | semantic fixture alias |
| `bool_predicate_scan_frag_min.hako` | `bool_predicate_scan` | semantic fixture alias |
| `accum_const_loop_frag_poc.hako` | `accum_const_loop` | semantic fixture alias |
| `loop_continue_only_multidelta_min.hako` | `loop_continue_only multi-delta` | semantic fixture alias |
| `loop_continue_only_min.hako` | `loop_continue_only` | semantic fixture alias |
| `scan_with_init_ok_min.hako` | `scan_with_init` | semantic fixture alias |
| `scan_with_init_match_ok_min.hako` | `scan_with_init match` | semantic fixture alias |
| `scan_with_init_reverse_ok_min.hako` | `scan_with_init reverse` | semantic fixture alias |
| `scan_with_init_contract_min.hako` | `scan_with_init contract` | semantic fixture alias |
| `scan_with_init_match_contract_min.hako` | `scan_with_init match contract` | semantic fixture alias |
| `scan_with_init_reverse_contract_min.hako` | `scan_with_init reverse contract` | semantic fixture alias |
| `split_scan_ok_min.hako` | `split_scan` | semantic fixture alias |
| `split_scan_nearmiss_ok_min.hako` | `split_scan near-miss` | semantic fixture alias |
| `split_scan_contract_min.hako` | `split_scan contract` | semantic fixture alias |
| `loop_break_recipe_only_min.hako` | `loop_break recipe-only` | semantic fixture alias |
| `loop_simple_while_inline_explicit_step_min.hako` | `loop_simple_while explicit-step` | semantic fixture alias |
| `loop_simple_while_strict_shadow_min.hako` | `loop_simple_while strict shadow` | semantic fixture alias |
| `loop_simple_while_subset_reject_extra_stmt_min.hako` | `loop_simple_while subset reject extra stmt` | semantic fixture alias |
| `selfhost_cleanup_only_min.hako` | `selfhost cleanup-only` | semantic fixture alias |
| `selfhost_trim_generic_loop_min.hako` | `selfhost trim generic-loop` | semantic fixture alias |
| `selfhost_breakfinder_parse_int_min.hako` | `selfhost BreakFinder parse-int` | semantic fixture alias |
| `phase29bq_pattern1_inline_explicit_step_min.hako` | `loop_simple_while explicit-step` | legacy fixture pin token |
| `pattern1_inline_explicit_step_min` | `loop_simple_while explicit-step` | legacy fixture pin token |
| `phase29ap_pattern4_continue_min.hako` | `loop_continue_only` | legacy fixture pin token |
| `phase29ab_pattern6_*` | `scan_with_init` | legacy fixture pin token |
| `phase29ab_pattern7_*` | `split_scan` | legacy fixture pin token |
| `phase29bq_pattern2_break_recipe_only_min.hako` | `loop_break recipe-only` | legacy fixture pin token |
| `phase263_p0_pattern2_seg_min.hako` | `loop_break minimal seg` | legacy fixture pin token |
| `phase29ai_pattern2_break_plan_subset_ok_min.hako` | `loop_break subset` | legacy fixture pin token |
| `phase29ab_pattern2_loopbodylocal_min.hako` | `loop_break body-local` | legacy fixture pin token |
| `phase29ab_pattern2_loopbodylocal_seg_min.hako` | `loop_break body-local-seg` | legacy fixture pin token |
| `phase263_pattern2_seg_realworld_min.hako` | `loop_break realworld seg` | legacy fixture pin token |
| `phase286_pattern5_break_min.hako` | `loop_true_early_exit` | legacy fixture pin token |
| `phase269_p0_pattern8_frag_min.hako` | `bool_predicate_scan` | legacy fixture pin token |
| `phase286_pattern9_frag_poc.hako` | `accum_const_loop` | legacy fixture pin token |
| `phase29bq_pattern4continue_multidelta_min.hako` | `loop_continue_only multi-delta` | legacy fixture pin token |
| `p4_multidelta` | `loop_continue_only multi-delta` | legacy fixture pin token |
| `test_pattern3_skip_whitespace.hako` | `skip_whitespace` | legacy selfhost test stem |
| `test_pattern5b_escape_minimal.hako` | `escape route P5b` | legacy selfhost test stem |
| `test_pattern5b_escape_*` | `escape route P5b` | legacy selfhost test stem |

## Current interpretation

### Still-live contract tokens

These tokens are still part of a live contract and should not be retired without an alias-first
or caller-migration phase.

| Token | Why still live |
| --- | --- |
| `SMOKES_SELFHOST_FILTER=<substring>` matches on fixture/reason/planner_tag/filter_alias | selfhost gate contract is substring-based, so pinned fixture stems may still be used operationally |
| Program JSON by-name keys in `src/mir/join_ir/frontend/ast_lowerer/route.rs` | frontend allowlist remains a live entry contract; legacy retired aliases are handled separately |

Current hotspot files:
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `src/mir/join_ir/frontend/ast_lowerer/route.rs`

### Retirement conditions for still-live contract tokens

| Token / lane | Retirement precondition |
| --- | --- |
| `SMOKES_SELFHOST_FILTER=<substring>` exact legacy token examples | all active how-to/checklist examples have semantic route substrings or semantic fixture names, and exact legacy token examples are inventory-only |
| current Program JSON by-name keys in `route.rs` | repo-local active callers are non-historical and require the key; otherwise retire via alias-first or direct reject lane |

For `SMOKES_SELFHOST_FILTER` legacy-example retirement, use this caller scope:
`phase29bq_selfhost_planner_required_dev_gate_vm.sh` header contract,
`planner_required_selfhost_subset.tsv` semantic pin row (`apps/tests/if_phi_join_min.hako`),
and active docs (`joinir-planner-required-gates-ssot.md`, `phase-29bq/README.md`).

Selfhost filter interpretation:
- allowed current examples: semantic route substrings or semantic fixture aliases such as `if_phi_join`
  / `apps/tests/if_phi_join_min.hako`
- current gate also derives semantic aliases from the pinned fixture basename
  (for example `phase29bq_selfhost_blocker_parse_map_min.hako` -> `parse-map`,
  `phase29bq_selfhost_blocker_localssa_block_insts_end_min.hako` -> `localssa-block-insts-end`)
- current subset TSV may also expose `filter_alias` so semantic basename filters such as
  `selfhost_cleanup_only_min.hako`, `selfhost_trim_generic_loop_min.hako`, and
  `selfhost_breakfinder_parse_int_min.hako` stay current without rewriting the pinned historical fixture path
- inventory-only legacy examples: historical if-phi-join basenames kept in the Active pins table
- current docs/how-to should teach the semantic examples first and keep the exact historical basename in
  this ledger or other historical notes only

Current closeout decision:
- exact selfhost legacy basename examples are inventory-only
- active how-to/checklists should keep using semantic route substrings or semantic fixture aliases
- retire exact legacy filter examples only after caller scope is zero in the gate header contract, current TSV subset, and current active docs

### Inventory-only pins

These tokens should be read as traceability pins, not as the preferred semantic names in current
guidance.

| Token family | Why inventory-only |
| --- | --- |
| `historical if_phi_join basename family` | historical selfhost/joinir fixture basename family; current gates/selfhost subset now pin `if_phi_join_min.hako` |
| `loop_simple_while explicit-step historical pins` | runtime/gate pin family for explicit-step fixture; say `loop_simple_while explicit-step` in prose |
| `phase29bq_pattern4continue_multidelta_min.hako` / `p4_multidelta` | planner-required multi-delta fixture pin; say `loop_continue_only multi-delta` in prose |
| `phase29ab_pattern6_*` / `phase29ab_pattern7_*` | route-family fixture pin tokens for scan regressions; current docs should say `scan_with_init` / `split_scan` |
| `phase29ai_pattern2_break_plan_subset_ok_min.hako` / `phase29ab_pattern2_loopbodylocal_min.hako` / `phase29ab_pattern2_loopbodylocal_seg_min.hako` / `phase263_pattern2_seg_realworld_min.hako` / `phase286_pattern5_break_min.hako` / `phase269_p0_pattern8_frag_min.hako` / `phase286_pattern9_frag_poc.hako` | historical coverage pins kept for replay and gate traceability |
| semantic fixture aliases such as `if_phi_join_min.hako` / `loop_break_plan_subset_min.hako` / `loop_break_realworld_min.hako` / `loop_break_body_local_min.hako` / `loop_break_body_local_seg_min.hako` / `loop_true_early_exit_min.hako` / `string_is_integer_min.hako` / `bool_predicate_scan_frag_min.hako` / `accum_const_loop_frag_poc.hako` / `loop_continue_only_min.hako` / `loop_continue_only_multidelta_min.hako` / `scan_with_init_*_min.hako` / `split_scan_*_min.hako` / `loop_break_recipe_only_min.hako` / `loop_simple_while_inline_explicit_step_min.hako` / `loop_simple_while_strict_shadow_min.hako` / `loop_simple_while_subset_reject_extra_stmt_min.hako` | preferred current gate/selfhost entry names; old basenames are now historical pin tokens only |

## Usage

- In active docs:
  - say `scan_with_init`, not `pattern6`
  - say `split_scan`, not `pattern7`
  - say `if_phi_join`, and use `if_phi_join_min.hako` when an exact current fixture basename matters
  - say `loop_break subset`, and use `loop_break_plan_subset_min.hako` when an exact current fixture basename matters
  - say `loop_break realworld`, and use `loop_break_realworld_min.hako` when an exact current fixture basename matters
  - say `loop_break body-local`, and use `loop_break_body_local_min.hako` when an exact current fixture basename matters
  - say `loop_break body-local-seg`, and use `loop_break_body_local_seg_min.hako` when an exact current fixture basename matters
  - say `loop_true_early_exit`, and use `loop_true_early_exit_min.hako` when an exact current fixture basename matters
  - say `string_is_integer`, and use `string_is_integer_min.hako` when an exact current fixture basename matters
  - say `loop_continue_only`, and use `loop_continue_only_min.hako` when an exact current fixture basename matters
  - say `bool_predicate_scan`, and use `bool_predicate_scan_frag_min.hako` when an exact current fixture basename matters
  - say `loop_simple_while strict shadow`, and use `loop_simple_while_strict_shadow_min.hako` when an exact current fixture basename matters
  - say `loop_simple_while subset reject extra stmt`, and use `loop_simple_while_subset_reject_extra_stmt_min.hako` when an exact current fixture basename matters
  - say `selfhost cleanup-only`, and use `selfhost_cleanup_only_min.hako` when an exact current fixture basename matters
  - say `selfhost trim generic-loop`, and use `selfhost_trim_generic_loop_min.hako` when an exact current fixture basename matters
  - say `selfhost BreakFinder parse-int`, and use `selfhost_breakfinder_parse_int_min.hako` when an exact current fixture basename matters
  - say `accum_const_loop`, and use `accum_const_loop_frag_poc.hako` when an exact current fixture basename matters
  - say `loop_break recipe-only`, and use `loop_break_recipe_only_min.hako` when an exact current fixture basename matters
  - say `loop_continue_only multi-delta`, and use `loop_continue_only_multidelta_min.hako` when an exact current fixture basename matters
  - say `scan_with_init`, and use `scan_with_init_*_min.hako` aliases when an exact current fixture basename matters
  - say `split_scan`, and use `split_scan_*_min.hako` aliases when an exact current fixture basename matters
  - say `loop_simple_while explicit-step`, and use `loop_simple_while_inline_explicit_step_min.hako` when an exact current fixture basename matters
- When the filename itself matters for a command or grep, label it explicitly as
  `semantic fixture alias`, `legacy fixture key`, `legacy fixture pin token`, or `legacy selfhost test stem`.
- For `SMOKES_SELFHOST_FILTER`, prefer semantic route substrings first; use an exact historical basename only
  when a replay/debug note explicitly needs that archived token.
