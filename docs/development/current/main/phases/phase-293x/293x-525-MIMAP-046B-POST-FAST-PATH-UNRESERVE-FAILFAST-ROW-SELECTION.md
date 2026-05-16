# 293x-525 MIMAP-046B Post-Fast-Path-Unreserve-FailFast Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-046B` is a planning-only row after `MIMAP-046A`.

It selects exactly one next allocator/compiler/language row after the
OSVM-backed fast-path unreserve fail-fast diagnostics land. It must not
implement allocator behavior, compiler acceptance, or cleanup by itself.

## Scope

- Inspect `MIMAP-046A` fail-fast evidence.
- Classify the next blocker as allocator / compiler / language / cleanup.
- Choose exactly one next row and write its focused card.
- Update current pointers and taskboard after selection.

## Stop Lines

- No `.hako` behavior changes.
- No Rust compiler changes.
- No broad cleanup bundle.
- No provider activation, host allocator replacement, hook, or
  `#[global_allocator]` route.
- No remote-free/TLS/atomic/user-facing concurrency expansion unless selected as
  the next row by explicit evidence.
- No post-unreserve reuse, reclaim execution, or OS release beyond named future
  rows.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `046B.1` | Read `MIMAP-046A` fail-fast evidence. | blocker is classified as allocator / compiler / language / cleanup. | no implementation |
| `046B.2` | Select the next row and write a focused card. | one current blocker token is named. | no multi-row bundle |
| `046B.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
