---
Status: Landed
Date: 2026-04-29
Scope: parts cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/parts/exit_kind_depth_view.rs
  - src/mir/builder/control_flow/plan/facts/exit_only_block.rs
---

# 291x-698: ExitKindDepthView Shelf Prune

## Why

`exit_kind_depth_view.rs` still carried two dead shelves: the unused
`from_facts_exit_kind()` constructor and the `depth` field that no caller ever
read. The only active owner-path is `from_recipe_exit_kind(kind).kind`.

## Changes

- removed `ExitKindDepthView::from_facts_exit_kind()`
- removed `ExitKindDepthView.depth`
- kept `from_recipe_exit_kind()` as the active thin adapter for `exit_only_block`

## Result

- `cargo build --release` warning count moved from **28** to **26**
- the parts adapter surface now matches the active recipe-exit path

## Proof

```bash
cargo build --release
cargo test --release --lib exit_allowed_port_sig -- --nocapture
```
