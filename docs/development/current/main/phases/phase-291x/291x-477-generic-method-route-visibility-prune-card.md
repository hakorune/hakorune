---
Status: Landed
Date: 2026-04-27
Scope: Prune GenericMethodRoute component construction visibility
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-476-generic-method-route-visibility-inventory-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-477: GenericMethodRoute Visibility Prune

## Goal

Keep `GenericMethodRoute` readable as route metadata while making construction
internals crate-private.

This is BoxShape-only. It must not change route detection, JSON field names,
helper symbols, lowering tiers, or `.inc` behavior.

## Change

- Add public primitive/string route accessors:
  - `route_kind_tag()`
  - `helper_symbol()`
  - `proof_tag()`
- Update JSON emission to use those accessors instead of route-kind/proof enums.
- Make component construction records and constructors crate-private.
- Make route-kind/proof enum accessors crate-private.

## Acceptance

- `cargo check -q` passes without private-interface warnings.
- Generic method route JSON fixture remains unchanged.
- Generic method route focused tests pass.
- Map lookup fusion focused tests pass.
- Formatting and current-state guards pass.

## Result

Landed.

- `GenericMethodRoute` now exposes primitive/string read accessors for JSON:
  - `route_kind_tag()`
  - `helper_symbol()`
  - `proof_tag()`
- JSON emission no longer needs route-kind/proof enums.
- Component construction records, construction enums, and route construction are
  crate-private.
- `route_kind()` and `proof()` are test-only internal helpers.
- Route refresh behavior, JSON field names, helper symbols, lowering tiers, and
  `.inc` behavior are unchanged.

Verification:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q generic_method_route
cargo test -q map_lookup_fusion
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: quick gate still reports the existing chip8 release artifact sync warning,
then completes with `[dev-gate] profile=quick ok`.
