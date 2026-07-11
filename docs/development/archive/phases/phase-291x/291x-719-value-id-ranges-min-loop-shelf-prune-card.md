---
Status: Landed
Date: 2026-04-29
Scope: lowering ValueId range cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/join_ir/lowering/value_id_ranges.rs
---

# 291x-719: ValueId Ranges MinLoop Shelf Prune

## Why

`value_id_ranges::min_loop` and `base::MIN_LOOP` had no crate callsites outside
their own range-boundary test.

The active ValueId range owners are the specific route lowerers such as
`skip_ws`, `funcscanner_trim`, `stage1_using_resolver`, append-defs, and Stage-B
routes.

## Decision

Remove the inactive min-loop range vocabulary from `value_id_ranges`.

Keep `value_id_ranges` itself, because multiple production lowerers still use
the active route ranges.

## Changes

- removed `base::MIN_LOOP`
- removed the `min_loop` ValueId helper module
- updated the allocation table and range-boundary test to current active ranges

## Result

`value_id_ranges` now describes only active route owners, and its next-range
guidance no longer points at already allocated ranges.

Follow-up: the remaining fixed `lowering/min_loop.rs` direct lowerer shelf was
retired later in `291x-736`.

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
