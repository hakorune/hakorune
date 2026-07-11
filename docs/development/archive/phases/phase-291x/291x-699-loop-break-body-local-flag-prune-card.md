---
Status: Landed
Date: 2026-04-29
Scope: loop-break facts cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/loop_break/facts/body_local_facts.rs
  - src/mir/builder/control_flow/plan/loop_break/facts/body_local_facts_helpers.rs
  - src/mir/builder/control_flow/plan/loop_break/facts/body_local_facts_shape_matchers/body_local_digit_matcher.rs
  - src/mir/builder/control_flow/plan/loop_break/facts/body_local_facts_shape_matchers/body_local_trim_matcher.rs
---

# 291x-699: Loop-Break Body-Local Flag Prune

## Why

`LoopBreakBodyLocalFacts.break_uses_body_local` was a dead boolean shelf. The
helper only ever built facts with `break_uses_body_local: true`, and all owner
paths already rely on the presence of the facts record itself rather than the
flag value.

## Changes

- removed `LoopBreakBodyLocalFacts.break_uses_body_local`
- removed redundant `true` initialization in body-local facts helpers
- trimmed matcher tests that only asserted the dead always-true flag

## Result

- `cargo build --release` warning count moved from **26** to **25**
- the loop-break body-local facts record now carries only live shape/binding data

## Proof

```bash
cargo build --release
cargo test --release --lib loop_break_body_local_facts_detect_digit_pos -- --nocapture
cargo test --release --lib loop_break_body_local_facts_detect_trim_seg -- --nocapture
```
