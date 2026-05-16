# 293x-527 MIMAP-047B Post-Fast-Path-Unreserve-Closeout Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-047B` is a planning-only row after `MIMAP-047A`.

It selects exactly one next allocator/compiler/language row after the
OSVM-backed fast-path unreserve closeout guard lands. It must not implement
allocator behavior, compiler acceptance, or cleanup by itself.

## Scope

- Inspect `MIMAP-047A` closeout evidence.
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
- No OS release, post-unreserve reuse, reclaim execution, or page ownership
  migration unless selected as a separate focused row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `047B.1` | Read `MIMAP-047A` closeout evidence. | blocker is classified as allocator / compiler / language / cleanup. | no implementation |
| `047B.2` | Select the next row and write a focused card. | one current blocker token is named. | no multi-row bundle |
| `047B.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
