# 293x-383 MIMAP-021B Facade Page-Source Fresh-Page Attach

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-021B` is the next allocator behavior row selected by `MIMAP-021A`.
It connects the active object-lifecycle facade lane to the page-source seam by
reserving and committing exactly one fresh page, constructing one
`HakoAllocPageModel`, and attaching it as a facade-known page.

This is not allocation-on-miss yet. It is the smallest post-020 behavior row
that uses the newly adopted page-source capability while preserving provider
and global-allocator stop lines.

## Selected Owner

New narrow owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako
```

Existing collaborators:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
lang/src/hako_alloc/memory/page_source_policy_box.hako
lang/src/hako_alloc/memory/page_box.hako
```

Proof and guard:

```text
apps/mimalloc-facade-page-source-fresh-page-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
```

## Scope

- Prove `reservePage -> commitPage -> new HakoAllocPageModel ->
  objectLifecycleAddPage`.
- Expose scalar proof fields such as `source_reserved`, `source_committed`,
  `added_page_id`, `facade_page_count`, and `source_reject`.
- Keep the adapter narrow and facade-facing; the object-lifecycle facade remains
  the owner of known-page attachment.
- Keep M168 as historical heap/page-source composition evidence, not the active
  MIMAP facade row.

## Planned Task Order

| Step | Task | Output | Stop line |
| --- | --- | --- | --- |
| `021B.1` | Add the narrow facade page-source adapter owner. | One `.hako` module that returns scalar attach evidence. | no allocation-on-miss |
| `021B.2` | Add one proof app and guard. | MIR JSON + EXE prove reserve/commit/page model/facade attach. | no broad smoke or provider ladder |
| `021B.3` | Update memory README / task docs. | Owner boundary and stop lines are discoverable. | no unrelated cleanup |

## Stop Lines

- No allocation-on-miss retry.
- No release/realloc/alignment behavior changes.
- No purge/reclaim/decommit/recommit execution.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, page-map lookup, unreserve, or OS release.
- No backend `.inc` matcher shortcut.

## Closeout

`MIMAP-021B` adds the narrow facade-facing page-source adapter, proof app, and
guard.

Implemented owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako
```

The row proves one fresh page can be reserved/committed, modeled as a
`HakoAllocPageModel`, and attached through
`HakoAllocObjectLifecycleFacade.objectLifecycleAddPage`. It intentionally stops
before allocation-on-miss retry.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
# [mimap021b-mir-json] ok
# [k2-wide-mimalloc-facade-page-source-fresh-page-exe] ok

bash tools/checks/current_state_pointer_guard.sh
# [current-state-pointer-guard] ok

tools/checks/dev_gate.sh quick
# [dev-gate] profile=quick ok
```
