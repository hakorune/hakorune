---
Status: Landed
Date: 2026-04-29
Scope: lowering helper shelf cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/join_ir/lowering/carrier_update_emitter/mod.rs
  - src/mir/join_ir/lowering/carrier_update_emitter/conditional_step.rs
  - src/mir/join_ir/lowering/common/conditional_step_emitter.rs
---

# 291x-718: Carrier Update Conditional Step Shelf Prune

## Why

`carrier_update_emitter::conditional_step` had no production or test callsites.

The active conditional-step lowering path is already owned by
`common::conditional_step_emitter`, which is used by the loop-with-break carrier
update route.

## Decision

Keep conditional-step emission ownership in `common::conditional_step_emitter`.

Do not keep the stale `carrier_update_emitter` re-export, because it creates a
second apparent owner for the same lowering vocabulary.

## Changes

- removed `carrier_update_emitter::conditional_step`
- removed the stale parent module declaration and re-export
- kept `carrier_update_emitter::emit_carrier_update_with_env` as the active
  carrier-update surface

## Result

Carrier-update lowering has one fewer dead helper shelf, and conditional-step
ownership remains at the active common emitter path.

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
