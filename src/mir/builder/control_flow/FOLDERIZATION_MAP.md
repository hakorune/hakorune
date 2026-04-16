# Control-Flow Folderization Map

Status: working SSOT for the `plan/` -> owner-folder migration.

## Goal

- shrink `src/mir/builder/control_flow/plan/` into a temporary compatibility namespace
- move shared infrastructure to top-level owner folders first
- relocate owner-local route families only after they stop mixing facts/recipe/lower/ssa/cleanup responsibilities

## Shared Infra Destination Map

### `facts/`

- move first:
  - `plan/facts`
  - `plan/canon`
  - `plan/extractors`
  - `plan/route_shape_recognizers`
- rationale:
  - descriptive-only analysis surface
  - conservative observation helpers
  - route-shape questions that must stay separate from recipe/lowering
- actual owner surface already landed:
  - `facts::ast_feature_extractor`
  - `facts::route_shape_recognizers`
  - `facts::escape_shape_recognizer`
  - `facts::stmt_walk`
  - `facts::plan_residue`
  - `facts::extractors::common_helpers::condition`
  - `facts::extractors::common_helpers::control_flow`
  - `facts::extractors::common_helpers::increment`
  - `facts::extractors::common_helpers::loop_true_early_exit`
  - `facts::extractors::if_phi_join`
  - `facts::extractors::loop_simple_while`
  - `facts::loop_scan_methods_v0`
  - `facts::loop_scan_methods_block_v0`
  - `facts::loop_bundle_resolver_v0`
  - `facts::loop_cond_continue_only`
  - `facts::loop_cond_continue_with_return`
  - `facts::loop_cond_return_in_body`
  - `facts::loop_collect_using_entries_v0`
  - `facts::expr_value`
  - `facts::expr_bool`
  - `facts::no_exit_block`
  - `facts::stmt_view`
  - `facts::if_phi_join_facts`
  - `facts::canon::cond_block_view`
  - `facts::canon::generic_loop::update`
- keep-plan-for-now residue inside `plan/extractors`:
  - `none confirmed`
- facts-local residue still forwarded through `facts::plan_residue`:
  - `none confirmed`
- next actual move:
  - `none confirmed`

### `recipes/`

- move after `facts/` owner surface is live:
  - `plan/recipes`
  - `plan/recipe_tree`
  - `plan/parts`
  - `plan/steps`
  - `plan/features`
  - `plan/skeletons`
- rationale:
  - recipe/CorePlan vocabulary
  - route-local recipe composition without MIR-side repair
- actual owner surface already landed:
  - `recipes::RecipeBody`
  - `recipes::refs`
  - `recipes::scan_loop_segments`
  - `recipes::loop_bundle_resolver_v0`
  - `recipes::loop_collect_using_entries_v0`
  - `recipes::loop_scan_methods_block_v0`
  - `plan/recipe_tree` now imports `recipes::{RecipeBody, refs}` directly
- next actual move:
  - `none confirmed`

### `verify/`

- move after `recipes/`:
  - `plan/verifier`
  - `plan/diagnostics`
  - `plan/observability`
- rationale:
  - fail-fast validation and debug-contract surface
- actual owner surface already landed:
  - `verify/diagnostics::span_format`
  - `verify/observability::flowbox_tags`
  - `verify::verifier`
  - `verify::coreloop_body_contract`
- next actual move:
  - `none confirmed`

### `lower/`

- move after `verify/`:
  - `plan/lowerer`
  - `plan/emit`
  - `plan/composer`
  - `plan/planner`
  - `plan/single_planner`
  - `plan/normalize`
  - `plan/normalizer`
- rationale:
  - route orchestration, adoption, and MIR emission surface
- actual owner surface already landed:
  - `lower::expectations`
  - `lower::normalize`
- next actual move:
  - `none confirmed`

### `ssa/`

- move only when bindings/phi repair stop leaking into semantic lowering:
  - `plan/exit_binding`
  - `plan/exit_binding_applicator`
  - `plan/exit_binding_constructor`
  - `plan/exit_binding_validator`
- rationale:
  - PHI / binding repair must converge on a dedicated owner
- actual owner surface already landed:
  - `ssa::exit_binding`
  - `ssa::exit_binding_constructor`
  - `ssa::exit_binding_applicator`
  - `ssa::exit_binding_validator`

### `cleanup/`

- move after `lower/` / `ssa/` boundaries are stable:
  - `plan/common`
  - `plan/policies`
  - `plan/observability` (only if debug-contract cleanup ownership wins over verify ownership)
- rationale:
  - policy and post-lowering cleanup surface
- actual owner surface already landed:
  - `cleanup::common`
  - `cleanup::policies::body_local_derived_slot`
  - `cleanup::policies::cond_prelude_vocab`
  - `cleanup::policies::loop_true_read_digits_policy`
  - `cleanup::policies::read_digits_break_condition_box`
  - `cleanup::policies::loop_simple_while_subset_policy`
  - `cleanup::policies::balanced_depth_scan_policy_box`
  - `cleanup::policies::normalized_shadow_suffix_router_box`
  - `cleanup::policies::p5b_escape_derived_policy`
  - `cleanup::policies::trim_policy`
- keep-plan-for-now residue inside `plan/policies`:
  - `none confirmed`
- next actual move:
  - `none confirmed`
  - `plan/policies` is compat-only for already-moved cleanup policies

## Owner-Local Families (keep under temporary `plan/` namespace)

- `generic_loop`
- `loop_break`
- `loop_break_steps`
- `loop_bundle_resolver_v0`
- `loop_collect_using_entries_v0`
- `loop_cond`
- `loop_scan_methods_block_v0`
- `loop_scan_methods_v0`
- `loop_scan_phi_vars_v0`
- `loop_scan_v0`
- `loop_true_break_continue`
- `nested_loop_depth1`

These stay in `plan/` until each family is thin enough that the remaining code belongs clearly to one owner bucket.

## Direct `plan/` Import Residue Ledger

### Compatibility wrappers (expected while migration is in flight)

- `control_flow/facts::plan_residue`
- `control_flow/recipes/composer_compat.rs`
- `control_flow/lower/**`

These wrappers are allowed to point at `plan/` until the implementation moves to its final owner folder.

### Owner-Local Keep-Plan-For-Now Symbols

- `none confirmed`

### Movable Next

- safe tiny wrapper cleanup is exhausted for owner-preserving seams
- keep `recipes/`, `lower/`, `verify/`, `ssa/`, `cleanup/`, and `facts/` as the active top-level owner surfaces
- next movable symbols now concentrate on `loop_cond` family-local classifier cleanup and `loop_scan_phi_vars_v0` pipeline-side owner cleanup

## First Cut

- establish top-level `control_flow/facts/` as the descriptive owner surface
- keep implementations in `plan/` behind compatibility re-exports
- update non-`plan/` consumers first

## Removal Rule

- remove the `plan/` name only after:
  - shared infra imports point at top-level owners
  - owner-local families no longer mix responsibilities
  - docs / registry / restart pointers point at owner folders directly
