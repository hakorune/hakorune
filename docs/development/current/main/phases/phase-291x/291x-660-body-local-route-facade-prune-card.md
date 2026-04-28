---
Status: Landed
Date: 2026-04-28
Scope: prune BodyLocalRoute type facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/body_local_policy.rs
  - src/mir/builder/control_flow/plan/body_local_policy_helpers.rs
  - src/mir/builder/control_flow/plan/loop_break/api/promote_prepare_helpers.rs
---

# 291x-660: BodyLocalRoute Facade Prune

## Goal

Remove the `BodyLocalRoute` type re-export from the `body_local_policy` facade.

This is BoxShape cleanup. It must not change body-local policy routing,
promotion behavior, read-only slot behavior, derived-slot behavior, or
loop-break lowering.

## Evidence

Worker inventory found that `body_local_policy.rs` exposes both the routing
entrypoint and the `BodyLocalRoute` enum:

- `classify_loop_break_body_local_route`
- `BodyLocalRoute`

The entrypoint belongs in the facade. The enum is owned by
`body_local_policy_types.rs`; only two callers were relying on the facade type
re-export.

## Decision

Keep `classify_loop_break_body_local_route` in `body_local_policy.rs`.

Move `BodyLocalRoute` imports to `body_local_policy_types.rs` and remove the
facade re-export.

## Boundaries

- Do not change `classify_body_local_policy_route`.
- Do not change promotion/read-only/derived-slot matching.
- Do not touch loop-break prep data flow beyond import ownership.
- Do not mix this with feature pipeline or recipe-tree facade cleanup.

## Acceptance

```bash
cargo fmt
cargo test loop_break_body_local --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `BodyLocalRoute` imports now point to `body_local_policy_types`.
- `body_local_policy.rs` remains the entrypoint facade only.
- Body-local policy behavior is unchanged.
