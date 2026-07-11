---
Status: Landed
Date: 2026-04-29
Scope: recipe-tree cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/verified.rs
  - src/mir/builder/control_flow/plan/parts/wiring_tests.rs
---

# 291x-697: Verified Recipe Shelf Prune

## Why

`recipe_tree/verified.rs` still carried two dead shelves: the unused
`ObligationState::MaybeUndefined` variant and an orphaned recursive local-var
collector. The active verifier path only materializes `Defined` and
`OutOfScope`, so the extra surface was stale.

## Changes

- removed `ObligationState::MaybeUndefined`
- removed `collect_local_vars_from_block_recursive()`
- kept the live verifier/port-sig behavior unchanged

## Result

- `cargo build --release` warning count moved from **30** to **28**
- the verified-recipe contract surface now matches the active obligation states

## Proof

```bash
cargo build --release
cargo test --release --lib exit_allowed_port_sig -- --nocapture
```
