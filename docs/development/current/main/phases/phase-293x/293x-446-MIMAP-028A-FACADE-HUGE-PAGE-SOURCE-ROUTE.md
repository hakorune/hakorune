# 293x-446 MIMAP-028A Facade Huge Page-Source Route

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-028A` is the selected post-MIMAP-027B allocator behavior row.

It adds one narrow facade-facing huge allocation route that attaches scalar
page-source backing identity before any later release/decommit row:

```text
huge request -> page-source reserve/commit backing identity
backing identity -> huge model allocation/register
scalar report -> page id / ptr / base / bytes / requested size / committed size
scalar report -> no release/unregister/decommit executed
```

This row is not an OS page return row. It proves that the huge allocation path
can carry enough backing identity for later decommit planning while keeping
release/unregister/decommit/unreserve, provider activation, and allocator
replacement closed.

## Scope

- Add a facade huge page-source owner:
  `lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_source_box.hako`.
- Reuse the existing MIMAP-023A huge page model route for huge allocation
  metadata.
- Reuse the existing page-source reserve/commit owner for backing identity.
- Publish scalar report fields that prove:
  - huge request classification
  - page-source reserve/commit backing identity
  - huge model allocation/register result
  - requested size and committed backing size
  - zero release/unregister/decommit activity in this row
- Add an EXE/VM proof app:
  `apps/mimalloc-facade-huge-page-source-proof/main.hako`.
- Add a guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_page_source_exe_guard.sh`.

## Stop Lines

- Do not add OSVM release, unreserve, decommit, purge, or reclaim behavior.
- Do not release or unregister the huge handle.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hook, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.
- Do not promote allocator provider activation or host allocator replacement.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `028A.1` | Add the facade huge page-source owner. | One `.hako` owner composes page-source reserve/commit with the huge model allocation route. | no release/unregister/decommit |
| `028A.2` | Add the proof app and guard. | Guard proves backing identity and huge allocation scalar report. | no provider activation |
| `028A.3` | Close docs and current pointers. | Current state points to the next row selected after 028A. | no OS page return |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_page_source_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
