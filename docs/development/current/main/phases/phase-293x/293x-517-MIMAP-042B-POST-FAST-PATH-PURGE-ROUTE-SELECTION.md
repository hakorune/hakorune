# 293x-517 MIMAP-042B Post-Fast-Path-Purge Route Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-042B` is a planning-only row after `MIMAP-042A`.

It selects the next single allocator/compiler/language row after the
OSVM-backed fast-path bounded purge route lands. It must not implement allocator
behavior or compiler acceptance by itself.

## Scope

- Inspect `MIMAP-042A` evidence and any route-preflight / EXE diagnostics.
- Choose exactly one next row:
  - an allocator behavior row if MIMAP can continue without compiler support;
  - a focused compiler acceptance sidecar if `MIMAP-042A` exposed a real route
    gap;
  - a language/selfhost row only if it is the immediate blocker for allocator
    completeness.
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
| `042B.1` | Read `MIMAP-042A` guard/proof result and compiler diagnostics. | blocker is classified as allocator / compiler / language / cleanup. | no implementation |
| `042B.2` | Select the next row and write a focused card. | one current blocker token is named. | no multi-row bundle |
| `042B.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
