---
Status: Landed
Date: 2026-04-27
Scope: GenericMethodRoute route_kind metadata helper
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-443-generic-method-route-metadata-string-inventory-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-444: Generic Method RouteKind Metadata Helper

## Goal

Make exported `GenericMethodRoute` metadata tokens delegate to the decided
`GenericMethodRouteKind` instead of the raw `method` string.

This is BoxShape-only. JSON spellings and `.inc` behavior stay unchanged.

## Implementation

- Add `route_id()`, `emit_kind()`, and `effect_tags()` helpers on
  `GenericMethodRouteKind`.
- Make `GenericMethodRoute` delegate those helpers to `route_kind`.
- Keep `box_name` and `method` JSON fields for compatibility/debug.
- Add a regression test proving exported metadata tokens do not depend on the
  raw `method` field.

## Non-Goals

- Do not touch `.inc` readers.
- Do not remove `box_name` or `method` from JSON.
- Do not add hot lowering or new accepted method surfaces.

## Verification

```bash
cargo test -q generic_method_route_metadata_tokens_come_from_route_kind
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

Result: PASS.

Note: `cargo fmt -- --check` was also probed, but it is not used as this
slice gate because the repository currently has unrelated rustfmt drift in
other files.
