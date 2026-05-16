# 293x-529 MIMAP-048B Post-Release-Inventory Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-048B` is a planning-only row after `MIMAP-048A`.

It selects exactly one next allocator/compiler/language row after the OSVM
release capability inventory lands. It must not implement allocator behavior,
compiler acceptance, or cleanup by itself.

## Scope

- Inspect `MIMAP-048A` release inventory evidence.
- Classify the next blocker as allocator / compiler / language / cleanup.
- Choose exactly one next row and write its focused card.
- Update current pointers and taskboard after selection.

## Stop Lines

- No `.hako` behavior changes.
- No Rust compiler changes.
- No broad cleanup bundle.
- No provider activation, host allocator replacement, hook, or
  `#[global_allocator]` route.
- No OS release implementation unless selected as a separate focused row.
- No reclaim execution, page ownership migration, remote-free/TLS/atomic
  execution changes, or user-facing concurrency expansion unless selected by
  explicit evidence.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `048B.1` | Read `MIMAP-048A` inventory evidence. | blocker is classified as allocator / compiler / language / cleanup. | no implementation |
| `048B.2` | Select the next row and write a focused card. | one current blocker token is named. | no multi-row bundle |
| `048B.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
