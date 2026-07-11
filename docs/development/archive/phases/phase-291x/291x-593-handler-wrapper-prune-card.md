---
Status: Landed
Date: 2026-04-28
Scope: prune orphan joinir handler wrapper files that are not wired into the live module tree
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers/common.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers/standard.rs
---

# 291x-593: Orphan Handler Wrapper Prune

## Goal

Remove dead joinir handler wrapper files that duplicate live handler logic but
are not actually part of the compiled module tree.

This is BoxShape-only cleanup. It does not change any active route behavior.

## Evidence

`handlers.rs` only declares:

- `mod generic;`
- `mod routes;`

It does **not** declare `mod common;` or `mod standard;`.

The orphan files duplicated live helper/standard-route logic:

- `handlers/common.rs`
- `handlers/standard.rs`

`standard.rs` was the only file referencing `common.rs`, and no live registry
module imported either file.

## Boundaries

- Keep the compiled `handlers.rs`, `handlers/routes.rs`, and `handlers/generic.rs`
  surfaces unchanged.
- Delete only files that are not wired into the module tree.
- Do not refactor the live route implementation in this card.

## Acceptance

- `handlers/common.rs` and `handlers/standard.rs` are removed.
- No live registry file references those orphan paths.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Removed two dead wrapper files that mirrored live handler logic.
- Reduced confusion around which joinir handler shelf is authoritative.
- Left the actual live route shelf untouched for future targeted cleanup.

## Verification

```bash
rg -n "common::route_standard|handlers/common.rs|handlers/standard.rs" src/mir/builder/control_flow/joinir/route_entry/registry -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
