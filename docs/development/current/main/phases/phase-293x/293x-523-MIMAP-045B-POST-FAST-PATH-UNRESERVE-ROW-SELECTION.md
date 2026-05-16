# 293x-523 MIMAP-045B Post-Fast-Path-Unreserve Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-045B` is a planning-only row after `MIMAP-045A`.

It selects:

```text
MIMAP-046A OSVM-backed fast-path unreserve fail-fast diagnostics
```

The MIMAP-045A success route did not expose a compiler or language acceptance
blocker. The next smallest allocator row is the fail-fast companion for
duplicate/stale/unknown fast-path unreserve requests, mirroring the earlier huge
unreserve success -> fail-fast split while keeping provider activation, hooks,
host allocator replacement, remote-free/TLS/atomic execution changes, reclaim
execution, post-unreserve reuse, and user-facing concurrency work closed.

## Scope

- Inspect `MIMAP-045A` unreserve evidence.
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
- No OS release, post-unreserve reuse, or reclaim execution unless selected as
  a separate focused allocator row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `045B.1` | Read `MIMAP-045A` unreserve evidence. | blocker classified as allocator fail-fast companion. | no implementation |
| `045B.2` | Select the next row and write a focused card. | `MIMAP-046A` card exists. | no multi-row bundle |
| `045B.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
