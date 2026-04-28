---
Status: Landed
Date: 2026-04-28
Scope: prune dead JoinIR route registry handler shards
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs
---

# 291x-631: Route Registry Dead Handler Shards Prune

## Goal

Remove route registry handler shard files that were no longer part of the Rust
module tree.

This is BoxShape cleanup only. It does not change route ordering, predicates,
lowering behavior, or the active handler surface.

## Evidence

`handlers.rs` currently declares:

- `generic`
- `routes`

The stale `cond.rs`, `nested.rs`, and `recipe.rs` files were not declared by
`handlers.rs` and had no `handlers::{cond,nested,recipe}` callsites. Their
contents duplicated active handlers that are still provided by `routes.rs`.

## Boundaries

- Delete only unreachable handler shard files.
- Do not split or rewrite the active `routes.rs` monolith in this card.
- Do not change the `ENTRIES` table or route handler exports.

## Acceptance

- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed three unreachable route registry handler shard files.
- Kept active routing behavior unchanged through `handlers::routes`.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
