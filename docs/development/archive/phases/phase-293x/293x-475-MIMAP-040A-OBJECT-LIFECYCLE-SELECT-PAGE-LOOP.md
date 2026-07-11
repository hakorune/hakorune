# 293x-475 MIMAP-040A Object-Lifecycle SelectPage Loop

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-040A` is the BoxShape cleanup selected by `MIMAP-039C`.

It replaces the fixed three-slot selection shape in:

```text
HakoAllocObjectLifecyclePageQueue.selectPage()
```

with a queue-length loop that carries the selected page object as a nullable
loop value.

## Scope

- Update `selectPage()` to scan `me.pages.length()`.
- Return the selected `HakoAllocPageModel` object directly from the loop-carried
  nullable selected value.
- Update the object queue and facade proof apps so the active page selected by
  the second request lives in the fourth queue slot.
- Update focused guards to reject the old fixed `page0/page1/page2` selection
  shape.

## Stop Lines

- Do not expose a facade API that returns the selected page object.
- Do not rewrite `objectLifecycleKnownPageIndexById`; it already owns the
  facade known-page queue-length lookup.
- Do not split `HakoAllocObjectLifecyclePageQueue`.
- Do not change allocation, release, decommit, or reuse semantics.
- Do not add OSVM/page-source, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `040A.1` | Document row and proof owner. | Current selection points at a narrow queue owner. | no code before docs |
| `040A.2` | Replace fixed selection slots with a queue-length loop. | Fourth-slot page is reachable and returned as object. | no facade API widening |
| `040A.3` | Update proofs/guards. | Old `page0/page1/page2` selection cannot return. | no backend matcher |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
bash tools/checks/k2_wide_userbox_nullable_loop_return_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row closes when `selectPage()` is no longer fixed to three slots and
current moves to the next planning row.

## Landed Implementation

```text
owner:
  lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
proof apps:
  apps/mimalloc-object-lifecycle-queue-proof/main.hako
  apps/mimalloc-facade-object-lifecycle-queue-proof/main.hako
guards:
  tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh
  tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
```

Closeout:

```text
current blocker moves to MIMAP-040B post-selectPage-loop row selection.
```
