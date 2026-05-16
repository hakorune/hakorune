# 293x-521 MIMAP-044B Post-Fast-Path-Closeout Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-044B` is a planning-only row after `MIMAP-044A`.

It selects:

```text
MIMAP-045A OSVM-backed fast-path unreserve route
```

The MIMAP-044A closeout did not expose a compiler or language acceptance
blocker. The next smallest allocator row is to explicitly open the
previously-closed fast-path unreserve seam by composing the MIMAP-043A
OSVM-backed fast-path route with the MIMAP-033A page-source unreserve adapter.
This keeps provider activation, hooks, host allocator replacement, remote-free
execution, TLS/atomic execution changes, reclaim execution, and user-facing
concurrency work closed.

## Scope

- Inspect `MIMAP-044A` closeout evidence.
- Choose exactly one next row:
  - an allocator behavior row if MIMAP can continue without compiler support;
  - a focused compiler acceptance sidecar if closeout exposes a real route gap;
  - a language/selfhost row only if it is the immediate blocker for allocator
    completeness or if the lane switch is selected explicitly.
- Update current pointers and taskboard after selection.

## Stop Lines

- No `.hako` behavior changes.
- No Rust compiler changes.
- No broad cleanup bundle.
- No provider activation, host allocator replacement, hook, or
  `#[global_allocator]` route.
- No remote-free/TLS/atomic/user-facing concurrency expansion unless selected as
  the next row by explicit evidence.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `044B.1` | Read `MIMAP-044A` closeout evidence. | blocker classified as allocator continuation. | no implementation |
| `044B.2` | Select the next row and write a focused card. | `MIMAP-045A` card exists. | no multi-row bundle |
| `044B.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
