---
Status: Landed
Date: 2026-04-29
Scope: lowering route-analysis shelf cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/join_ir/README.md
  - src/mir/join_ir/lowering/mod.rs
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - src/mir/join_ir/lowering/loop_update_summary/assignment_scan.rs
  - src/mir/join_ir/lowering/loop_update_summary/rhs_classification.rs
  - src/mir/join_ir/lowering/loop_update_summary/tests.rs
---

# 291x-720: LoopUpdateSummary Shelf Prune

## Why

`loop_update_summary` no longer had production callsites. Its only remaining
crate references were its public module declaration, internal tests, and
historical documentation.

The active carrier-update analysis owner is `loop_update_analyzer`, while route
shape facts have moved to the current planner/lowering vocabulary.

## Decision

Remove the stale `loop_update_summary` shelf instead of shrinking individual
reserved fields.

Do not keep a public-looking lowering module whose analyzer output is not part
of any active route contract.

## Changes

- removed `lowering::loop_update_summary`
- removed its private helper files and tests
- removed the stale entry from the JoinIR README loop-route cluster

## Result

JoinIR lowering now exposes one fewer dead route-analysis module, and carrier
update analysis ownership stays with `loop_update_analyzer`.

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
