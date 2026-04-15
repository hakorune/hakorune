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
- next actual move:
  - `ssa::exit_binding`

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
- next actual move:
  - `cleanup::common`

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

- `control_flow/{facts,recipes,verify,lower,ssa,cleanup}/**`
- `control_flow/joinir/route_entry::owner_local_compat`

These wrappers are allowed to point at `plan/` until the implementation moves to its final owner folder.

### Owner-Local Keep-Plan-For-Now Symbols

- `plan/loop_cond/break_continue_types::LoopCondBreakAcceptKind`
  - consumed through `joinir/route_entry::owner_local_compat`
- test-only:
  - `plan/loop_scan_methods_block_v0::try_extract_loop_scan_methods_block_v0_facts`
  - `plan/loop_scan_methods_v0::try_extract_loop_scan_methods_v0_facts`
  - consumed through `joinir/route_entry::owner_local_compat`

### Movable Next

- `none confirmed`
- next movable symbols only appear after `loop_cond` or `loop_scan_methods_*` stops mixing owner responsibilities

## First Cut

- establish top-level `control_flow/facts/` as the descriptive owner surface
- keep implementations in `plan/` behind compatibility re-exports
- update non-`plan/` consumers first

## Removal Rule

- remove the `plan/` name only after:
  - shared infra imports point at top-level owners
  - owner-local families no longer mix responsibilities
  - docs / registry / restart pointers point at owner folders directly
