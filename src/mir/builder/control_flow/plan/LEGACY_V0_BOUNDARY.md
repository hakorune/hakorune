# Legacy v0 Boundary (SSOT helper)

Purpose: keep `loop_*_v0` modules explicit and prevent accidental drift while parser-handoff and recipe-first work continues.

Scope: `src/mir/builder/control_flow/plan/**` only.

## Routed legacy-v0 modules (active)

These modules are still part of the routed planner path (`registry`/handlers/composer wiring exists):

- `loop_scan_v0`
- `loop_scan_methods_v0`
- `loop_scan_methods_block_v0`
- `loop_scan_phi_vars_v0`
- `loop_collect_using_entries_v0`
- `loop_bundle_resolver_v0`

See the evidence snapshot in `src/mir/builder/control_flow/plan/REGISTRY.md` (`loop_*_v0 audit snapshot` section).

## Retired modules

- `loop_flag_exit_v0` is physically removed (`CLEAN-PLAN-V0-REMOVE-1`).

No module is considered retired unless:

1. route wiring is removed,
2. related facts fields are removed,
3. fixture/gate contract is pinned for the replacement path.

## Boundary rules

- `loop_*_v0` modules are temporary compatibility boxes, not the long-term vocabulary.
- New acceptance shapes should prefer recipe-first/coreloop slots before adding another `*_v0` module.
- No AST rewrite in legacy-v0 modules (analysis-only observation only).
- Acceptance expansion remains strict/dev `planner_required` scoped unless explicitly promoted by SSOT decision.

## Removal playbook (one box at a time)

1. Prove replacement route is wired and green (fast gate + focused fixture).
2. Freeze behavior with fixture + gate update.
3. Remove one legacy-v0 module (and related facts/planner wiring) in a single commit.
4. Update:
   - `src/mir/builder/control_flow/plan/mod.rs`
   - `src/mir/builder/control_flow/plan/REGISTRY.md`
   - this file.
