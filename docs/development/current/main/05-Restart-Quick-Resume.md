---
Status: Active
Date: 2026-04-23
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/137x-current.md
  - docs/development/current/main/phases/phase-137x/137x-91-task-board.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Heavy gates are not first-step restart work. Run them only when the next code
slice is ready:

```bash
tools/checks/dev_gate.sh quick
cargo check -q
```

## Current Lane

- lane:
  - compiler cleanup lane is primary
  - current-state token: `phase-291x CoreBox surface contract cleanup`
  - active phase: `docs/development/current/main/phases/phase-291x/README.md`
  - phase status SSOT: `docs/development/current/main/phases/phase-291x/README.md`
  - current focus:
  - phase-292x is closed: `.inc` analysis debt is 0 files / 0 lines, with
    1 file / 2 explicit view-owner lines guarded separately
  - `.inc` remains boundary glue, not planner
  - `MapBox.length()` is landed as a read-only contract-first alias for the
    existing Map size surface
  - non-empty source-level vm-hako `MapBox.values().size()` is landed and pinned
  - non-empty source-level vm-hako `MapBox.keys().size()` is landed and pinned
  - source-level vm-hako `MapBox.remove(key)` delete-owner alias is landed and pinned
  - source-level vm-hako `MapBox.clear()` state reset is landed and pinned
  - source-level vm-hako `MapBox.set(...)` duplicate-receiver routing is landed
  - `keys()/values()` element publication is landed and pinned by the
    291x-102 acceptance smoke
  - MapBox write-return receipt implementation is landed and pinned
  - MapBox bad-key normalization implementation is landed and pinned
  - Rust `MapBox.values()` sorted-key order fix is landed (291x-102 slice 1)
  - `ArrayCoreBox.get` VM-local-first metadata check is landed (291x-102 slice 2)
  - MapBox get missing-key contract is landed and pinned
  - `StringBox.lastIndexOf(needle, start_pos)` is landed and pinned by the
    291x-103 acceptance smoke
  - CoreBox surface catalog work is landed and now a reference lane;
    `StringBox.length/len/size`, `StringBox.substring/substr`,
    `StringBox.concat`, `StringBox.trim`, `StringBox.contains`, one-arg and
    two-arg `StringBox.lastIndexOf`, `StringBox.replace`, and `StringBox.indexOf` /
    `find`, plus `ArrayBox.length/size/len`, `ArrayBox.push`,
    `ArrayBox.slice`, `ArrayBox.get`, `ArrayBox.pop`, `ArrayBox.set`,
    `ArrayBox.remove`, `ArrayBox.insert`, `MapBox.size`, `MapBox.length`,
    `MapBox.len`, and `MapBox.has`, `MapBox.get`, `MapBox.set`,
    `MapBox.keys`, and `MapBox.values`, are on the Unified value path;
    remaining cleanup is MapBox contract-first rows
- perf blocker (observe-only):
  - `137x-H46 text-cell residence/materialization design`
- method anchor:
  - `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
- active entry:
  - `docs/development/current/main/phases/phase-291x/README.md`
- taskboard:
  - `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
- current blocker token:
  - `phase-291x StringBox lastIndexOf start-position landed — next app card selection pending`

## Current Perf Snapshot

- H45 refresh:
  - H44.1 keeper remains accepted
  - saved bundle + dwarf callgraph pinned the residual owner to the combined
    `ArrayTextCell` edit/materialization family
- whole guard:
  - `kilo_kernel_small = C 83 ms / Ny AOT 5 ms`
  - `ny_aot_instr=24122891`
  - `ny_aot_cycles=5842445`
- latest reject/revert:
  - bounded `MidGap + bridge` H46.1 probe regressed to `Ny AOT 22 ms`,
    `ny_aot_instr=142651499`, `ny_aot_cycles=90126830`
  - perf top moved cost into `__memmove 54.59%` and `_int_malloc 21.74%`
  - post-revert whole guard is back at `Ny AOT 5 ms`,
    `ny_aot_instr=24123290`, `ny_aot_cycles=6044833`
- exact guard:
  - `kilo_micro_array_string_store = C 11 ms / Ny AOT 3 ms`
  - `ny_aot_instr=9265721`
  - `ny_aot_cycles=2317791`
- meso guard:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 4 ms`
  - `ny_aot_instr=17651018`
  - `ny_aot_cycles=4247395`
- latest top after H45 refresh:
  - combined executor closure `54.33%`
  - external `__memmove_avx512_unaligned_erms` `27.50%`
  - `realloc` `2.41%`
  - `_int_malloc` `0.51%`

## Immediate Next

- app priority:
  - select the next app cleanup card without reopening the landed 291x witnesses
  - do not reopen the landed MapBox get missing-key contract without an owner-path change
  - `pure_compile_minimal_paths` is removed; phase-292x analysis debt is now 0
    files / 0 lines, with 1 file / 2 view-owner lines
  - keep `.inc` on metadata read / validation / emit / skip / fail-fast only
  - old C analyzers are temporary fallback only until each route family is pinned
  - keep `src/boxes/array/surface_catalog.rs` and `src/boxes/basic/string_surface_catalog.rs` as CoreBox precedent references
  - keep phase-137x at observe-only unless app work produces a real blocker
- perf reopen rule:
  - `137x-E` is already closed enough (`E0` closed, `E1` landed)
  - do not reopen helper-local perf cards from curiosity or local neatness
  - open a new perf card only when app work is actually blocked, the owner is
    pinned to one family, and the slice fits in one rollback-safe card
- forbidden drift:
  - no more suffix/left-copy micro leaves without a new sampled source block
  - no `.inc` planner regression
  - no runtime legality/provenance inference
  - no benchmark-name/source-content assumptions

## Restart Notes

- worktree should be clean after the last commit.
- branch may be ahead of `hakorune/public-main`; do not push unless requested.
- the current docs front is phase-291x CoreBox surface contract cleanup.
- phase-291x CoreBox first catalog and cleanup slices are landed references.
- phase-292x closeout card was `292x-118-generic-pure-walker-view-extraction-card.md`;
  `292x-112` deleted path #1/#2 after `292x-114` fixed the predelete blockers,
  `292x-115` deleted paths #3/#4, `292x-116` deleted paths #5/#6,
  `292x-117a` deleted the string loop seed copy-graph helper, and `292x-117b`
  tightened cross-block use lookup; `292x-118a` introduced the generic pure
  program view shell, and `292x-118b` introduced the generic pure block view
  accessor; `292x-118c` consolidated raw walker access into the view owner;
  `292x-118d` split view-owner reads out of analysis debt.
- rejected slot-store boundary probe stays parked in `stash@{0}` as
  `wip/concat-slot-store-window-probe`; do not resurrect it unless explicitly
  reopening that rejected card.
- history lives in phase docs, not this file:
  - `docs/development/current/main/phases/phase-291x/README.md`
  - `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
  - `docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md`
  - `docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md`
  - `docs/development/current/main/phases/phase-292x/README.md`
  - `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
  - `docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
