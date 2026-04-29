---
Status: Landed
Date: 2026-04-29
Scope: lowering constructor surface cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/join_ir/lowering/if_select.rs
---

# 291x-717: IfSelect Debug Shim Prune

## Why

`IfSelectLowerer::with_debug(bool)` was a Phase 33 legacy shim with no
production or test callers.

The active lowering router already passes the numeric debug level through
`IfSelectLowerer::new(debug_level)` or `IfSelectLowerer::with_context(...)`.

## Decision

Keep the IfSelect constructor vocabulary to the active routes:

- `new(debug_level)` for pure if-select lowering
- `with_context(debug_level, context)` for router-provided loop context

Do not keep a bool debug compatibility constructor in the production surface.

## Changes

- removed `IfSelectLowerer::with_debug(bool)`
- removed the local `#[allow(dead_code)]` marker attached to that shim

## Result

IfSelect debug construction now has one active representation: the same numeric
debug level used by the router.

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
