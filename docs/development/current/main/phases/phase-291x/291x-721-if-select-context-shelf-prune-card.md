---
Status: Landed
Date: 2026-04-29
Scope: lowering constructor/context cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/join_ir/lowering/if_lowering_router.rs
  - src/mir/join_ir/lowering/if_select.rs
---

# 291x-721: IfSelect Context Shelf Prune

## Why

`IfSelectLowerer` stored an optional `IfPhiContext`, but never read it.

The router already tries `IfMergeLowerer` first with context, then falls back to
IfSelect for pure value selection. Keeping a context-bearing IfSelect
constructor made the Select route look context-sensitive when it is not.

## Decision

Keep IfSelect construction context-free.

If-in-loop context remains owned by IfMerge and IfPhi-specific lowering paths.

## Changes

- removed the unused `IfSelectLowerer.context` field
- removed `IfSelectLowerer::with_context(...)`
- simplified the router fallback to always construct IfSelect with
  `new(debug_level)`

## Result

IfSelect now exposes one constructor and one responsibility: lower supported
if/else value-selection shapes to `JoinInst::Select`.

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
