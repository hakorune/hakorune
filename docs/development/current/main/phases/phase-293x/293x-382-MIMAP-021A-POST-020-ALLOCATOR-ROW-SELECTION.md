# 293x-382 MIMAP-021A Post-020 Allocator Row Selection

Status: landed
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

## Closeout

`MIMAP-021A` selects `MIMAP-021B facade page-source fresh-page attach` as the
next allocator behavior row.

Rationale:

- `MIMAP-020A` adopted the page-source capability owner
  (`HakoAllocPageSourcePolicy` and `HakoAllocProductionFacade.pageSource*`).
- The smallest post-020 behavior row is to attach one freshly
  reserved/committed `HakoAllocPageModel` to the current object-lifecycle
  facade as a known page.
- This bridges the active MIMAP facade lane to page-source without
  allocation-on-miss, provider activation, page-map lookup, or purge/reclaim
  execution.
- Existing M168 still proves the older heap/page-source composition seam, but
  the selected MIMAP row should stay on the object-lifecycle facade owner path.

Selected next row:

```text
docs/development/current/main/phases/phase-293x/293x-383-MIMAP-021B-FACADE-PAGE-SOURCE-FRESH-PAGE-ATTACH.md
```

Evidence:

```text
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
# [m168-mir-json] ok
# [k2-wide-mimalloc-osvm-page-source-composition] ok

# Selection note:
# M168 remains green, but MIMAP-021B selects the facade fresh-page attach seam
# because it is the smallest row that connects the active facade lane to
# page-source.
```

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
# [current-state-pointer-guard] ok
```
