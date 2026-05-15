# 293x-382 MIMAP-021A Post-020 Allocator Row Selection

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-021A` is a small planning row after the `MIMAP-020A` page-source
adoption and `METADATA-CATALOG-004` post-promotion reconcile.

The current mimalloc row plan reached the first OSVM/page-source capability
checkpoint. Before adding another allocator behavior row, this card selects the
next smallest `.hako` / `hako_alloc` behavior slice and updates the taskboard so
implementation work does not drift into provider activation or broad runtime
changes.

## Scope

- Inspect the landed facade/page/free-list/release/realloc/stats/purge and
  page-source rows.
- Choose one next allocator behavior row, or explicitly open the smallest
  compiler/language sidecar only if that behavior row is blocked.
- Update the mimalloc taskboard and granularity SSOT with the selected row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  parked unless the user explicitly reopens the provider ladder.

## Candidate Directions

The selected row must fit one of these paths:

| Candidate | Use when | Stop line |
| --- | --- | --- |
| next `.hako` facade behavior | The in-memory/page-source facade can prove one more allocator policy with scalar observers. | no provider activation |
| capability verifier sidecar | The next allocator row needs unsupported capability fail-fast before it can be honest. | no allocator behavior in the same row |
| representation sidecar | The next allocator row truly needs packed/raw/static representation support. | one representation contract only |
| provider ladder reopen | The task requires provider selection, hooks, host allocator replacement, or `#[global_allocator]`. | explicit user instruction required |

## Stop Lines

- Do not implement allocator behavior in this selection row.
- Do not combine BoxShape metadata cleanup with the next allocator behavior
  row.
- Do not reopen provider hooks, host allocator replacement, or
  `#[global_allocator]` implicitly.
- Do not hide a compiler blocker in `.hako` source.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
```
