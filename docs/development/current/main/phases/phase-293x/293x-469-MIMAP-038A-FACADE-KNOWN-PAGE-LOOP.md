# 293x-469 MIMAP-038A Facade Known-Page Loop

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-038A` is the BoxShape cleanup selected by `MIMAP-037B`.

It replaces the fixed three-page lookup in:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById(page_id)
```

with a queue-length loop.

## Scope

- Update `objectLifecycleKnownPageIndexById` to scan
  `me.object_lifecycle_queue.pages.length()`.
- Add a proof app that adds four pages and releases a block from the fourth
  page through the facade.
- Add a focused guard that rejects the old fixed `page0/page1/page2` lookup
  shape.

## Stop Lines

- Do not rewrite `HakoAllocObjectLifecyclePageQueue.selectPage`.
- Do not split `HakoAllocObjectLifecycleFacade`.
- Do not add allocator behavior.
- Do not add provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `038A.1` | Add SSOT and proof app. | Four-page lookup proof is explicit. | no behavior widening |
| `038A.2` | Replace fixed lookup with loop. | Fourth known page can be found and released. | no queue selection rewrite |
| `038A.3` | Add focused guard. | Old `page0/page1/page2` lookup cannot return. | no backend matcher |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_known_page_loop_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Landed Implementation

```text
owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
proof app:
  apps/mimalloc-facade-known-page-loop-proof/main.hako
guard:
  tools/checks/k2_wide_mimalloc_facade_known_page_loop_guard.sh
```

Closeout:

```text
current blocker moves to MIMAP-038B post-known-page-loop row selection.
```
