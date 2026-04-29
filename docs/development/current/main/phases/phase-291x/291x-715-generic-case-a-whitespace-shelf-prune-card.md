---
Status: Landed
Date: 2026-04-29
Scope: lowering dead shelf cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/join_ir/lowering/generic_case_a/mod.rs
  - src/mir/join_ir/lowering/generic_case_a/trim.rs
  - src/mir/join_ir/lowering/generic_case_a/whitespace_check.rs
---

# 291x-715: Generic Case-A Whitespace Shelf Prune

## Why

`generic_case_a::whitespace_check` was a placeholder helper with no source
caller outside its own unit tests.

Current whitespace lowering is owned by the active inline emitter path, so the
generic Case-A helper was a stale shelf rather than structural vocabulary.

## Decision

Delete the unused helper module and remove current prose that listed it as an
active shared utility.

Future whitespace lowering changes should update the active emitter owner
directly instead of reviving this placeholder module.

## Changes

- removed `generic_case_a/whitespace_check.rs`
- removed the module declaration from `generic_case_a/mod.rs`
- removed stale helper prose from generic Case-A docs/comments

## Result

- one lowering dead shelf and its local `allow(dead_code)` markers are gone
- active whitespace lowering ownership remains with the inline emitter path

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
