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

## Active pins

| Legacy pin token | Current route semantics | Pin category |
| --- | --- | --- |
| `phase118_pattern3_if_sum_min.hako` | `if_phi_join` | legacy fixture key |
| `phase29bq_pattern1_inline_explicit_step_min.hako` | `loop_simple_while explicit-step` | legacy fixture pin token |
| `pattern1_inline_explicit_step_min` | `loop_simple_while explicit-step` | legacy fixture pin token |
| `phase29ab_pattern6_*` | `scan_with_init` | legacy fixture pin token |
| `phase29ab_pattern7_*` | `split_scan` | legacy fixture pin token |
| `phase29bq_pattern2_break_recipe_only_min.hako` | `loop_break recipe-only` | legacy fixture pin token |
| `phase263_p0_pattern2_seg_min.hako` | `loop_break minimal seg` | legacy fixture pin token |
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
| `phase118_pattern3_if_sum_min.hako` | selfhost planner-required subset row is still pinned to this fixture basename |
| `SMOKES_SELFHOST_FILTER=<substring>` matches on fixture/reason/planner_tag | selfhost gate contract is substring-based, so pinned fixture stems may still be used operationally |
| Program JSON by-name keys in `src/mir/join_ir/frontend/ast_lowerer/route.rs` | frontend allowlist remains a live entry contract; legacy retired aliases are handled separately |

### Inventory-only pins

These tokens should be read as traceability pins, not as the preferred semantic names in current
guidance.

| Token family | Why inventory-only |
| --- | --- |
| `phase29bq_pattern1_inline_explicit_step_min.hako` / `pattern1_inline_explicit_step_min` | runtime/gate pin for explicit-step fixture; say `loop_simple_while explicit-step` in prose |
| `phase29bq_pattern4continue_multidelta_min.hako` / `p4_multidelta` | planner-required multi-delta fixture pin; say `loop_continue_only multi-delta` in prose |
| `phase29ab_pattern6_*` / `phase29ab_pattern7_*` | route-family fixture pin tokens for scan regressions; current docs should say `scan_with_init` / `split_scan` |
| `phase263_pattern2_seg_realworld_min.hako` / `phase286_pattern5_break_min.hako` / `phase269_p0_pattern8_frag_min.hako` / `phase286_pattern9_frag_poc.hako` | historical coverage pins kept for replay and gate traceability |

## Usage

- In active docs:
  - say `scan_with_init`, not `pattern6`
  - say `split_scan`, not `pattern7`
  - say `if_phi_join`, not `phase118_pattern3_if_sum_min`
  - say `loop_break recipe-only`, not `phase29bq_pattern2_break_recipe_only_min`
  - say `loop_continue_only multi-delta`, not `phase29bq_pattern4continue_multidelta_min` / `p4_multidelta`
- When the filename itself matters for a command or grep, label it explicitly as
  `legacy fixture key`, `legacy fixture pin token`, or `legacy selfhost test stem`.
