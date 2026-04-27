---
Status: Landed
Date: 2026-04-28
Scope: prune the loop_bundle_resolver_v0 module-root compatibility shelf and keep only the pipeline owner path
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-585-legacy-v0-boundary-inventory-card.md
  - src/mir/builder/control_flow/plan/loop_bundle_resolver_v0/mod.rs
  - src/mir/builder/control_flow/plan/loop_bundle_resolver_v0/pipeline.rs
  - src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs
---

# 291x-587: Loop Bundle Resolver Wrapper Prune

## Goal

Trim `plan::loop_bundle_resolver_v0` down to its actual owner surface.

This is BoxShape-only cleanup. It does not change the facts contract, recipe
shape, or lowering behavior for the bundle resolver route.

## Evidence

The 291x-585 inventory showed that this family is the twin of
`loop_collect_using_entries_v0`:

- facts live under `facts::loop_bundle_resolver_v0`
- recipe data lives under `recipes::loop_bundle_resolver_v0`
- lowering logic lives in `plan::loop_bundle_resolver_v0::pipeline`

The module root was only re-exporting those surfaces, and the lowering entry had
exactly one caller in `plan::recipe_tree::loop_cond_composer`.

## Boundaries

- Keep the `pipeline` module as the physical lowering owner.
- Remove module-root re-exports for facts, recipe, and lowering.
- Update the lone lowering caller to the direct pipeline owner path.

## Acceptance

- No live caller uses `plan::loop_bundle_resolver_v0::lower_*` via the module
  root.
- `plan::loop_bundle_resolver_v0` no longer re-exports facts/recipe types.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Reduced `loop_bundle_resolver_v0` to a thin owner module that only exposes its
  `pipeline` submodule.
- Moved the sole lowering caller to the direct `pipeline` owner path.
- Removed dead compatibility re-exports for facts and recipe types.

## Verification

```bash
rg -n "plan::loop_bundle_resolver_v0::" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
