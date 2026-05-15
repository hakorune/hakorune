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
- New `loop_*_v0` modules are rejected by default.
- New acceptance shapes should prefer recipe-first/coreloop slots before adding another `*_v0` module.
- A new `loop_*_v0` module is allowed only when an active blocker explicitly
  selects a one-shape proof lane and the row documents `retire_when` /
  `promote_when`.
- No AST rewrite in legacy-v0 modules (analysis-only observation only).
- Acceptance expansion remains strict/dev `planner_required` scoped unless explicitly promoted by SSOT decision.
- If the same kind of one-shape extension appears twice, stop adding v0 boxes
  and promote the common part to skeleton + feature composition first.

## Retire / promote vocabulary

Use these fields in the registry before changing v0 wiring:

```text
retire_when:
  The generic skeleton/feature route that can replace this v0 box is green
  under the focused fixture and fast gate.

promote_when:
  The shape is no longer one-off and appears as a reusable compiler capability
  across at least two independent routes or apps.

hold_reason:
  Why this v0 box still exists today.
```

## Removal playbook (one box at a time)

1. Prove replacement route is wired and green (fast gate + focused fixture).
2. Freeze behavior with fixture + gate update.
3. Remove one legacy-v0 module (and related facts/planner wiring) in a single commit.
4. Update:
   - `src/mir/builder/control_flow/plan/mod.rs`
   - `src/mir/builder/control_flow/plan/REGISTRY.md`
   - this file.
