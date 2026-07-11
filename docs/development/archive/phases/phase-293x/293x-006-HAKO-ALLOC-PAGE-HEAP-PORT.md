# 293x-006 Hako Alloc Page-Heap Port

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: move the allocator-shaped page/free-list policy-state slice from
  `apps/mimalloc-lite` into the public `hako_alloc` seam.

## Decision

Open the third live allocator row as a VM-only policy/state prototype:

```text
lang/src/hako_alloc/memory/layout_box.hako
lang/src/hako_alloc/memory/page_heap_box.hako
```

This row owns fixed-size page selection, free-list reuse, handle accounting,
peak usage, and deterministic requested-byte accounting for the real-app
allocator lane.

## Non-Goals

- No native allocator backend migration.
- No `RawBuf` / `MaybeInit` migration.
- No native layout or ABI ownership.
- No real-app EXE parity claim; the pure-first typed object plan is still
  required before general user-box `newbox` can lower through direct EXE.

## Changes

- Added `LayoutBox` for narrow fixed-size class policy.
- Added `HakoAllocHeap`, `HakoAllocPage`, and `HakoAllocHandle`.
- Exported the new boxes from `lang/src/hako_alloc/hako_module.toml`.
- Repointed `apps/mimalloc-lite` to use the `hako_alloc` public seam while
  preserving its smoke output.
- Updated allocator policy/state docs to record the third live row.

## Verification

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
