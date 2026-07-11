---
Status: Landed
Date: 2026-04-29
Scope: lowering helper surface cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/join_ir/lowering/generic_case_a/entry_builder.rs
---

# 291x-716: Generic Case-A Entry Builder Shrink

## Why

`generic_case_a::entry_builder` production callers only use
`EntryFunctionBuilder::{new, add_var, get_map}`.

The pinned/carrier fields and helper methods were test-only residue with local
`#[allow(dead_code)]` markers. They duplicated vocabulary that is now owned by
LoopScopeShape and route-specific lowering inputs.

## Decision

Shrink `EntryFunctionBuilder` to its active responsibility: deterministic
`name -> ValueId` map construction.

Pinned/carrier route semantics must stay in the route owners and
LoopScopeShape, not in this entry map helper.

## Changes

- removed unused pinned/carrier fields
- removed unused pinned/carrier helper methods and tests
- kept `new`, `add_var`, and `get_map` as the active map-building surface

## Result

- one generic Case-A helper now has a smaller responsibility boundary
- local `allow(dead_code)` markers in `entry_builder.rs` are gone

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
