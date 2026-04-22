---
Status: Active
Date: 2026-04-22
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
  - current-state token: `phase-292x .inc codegen thin tag cleanup`
  - active phase: `docs/development/current/main/phases/phase-292x/README.md`
  - current focus:
  - `.inc` is boundary glue, not planner
  - `tools/checks/inc_codegen_thin_shim_guard.sh` pins the no-growth baseline
  - first implementation card `array_rmw_window` MIR-owned route tag is landed
  - `array_string_len_window` len-only MIR-owned route tag is landed
  - `array_string_len_window` keep-live source reuse MIR-owned route tag is landed
  - `array_string_len_window` source-only direct-set reuse MIR-owned route tag is landed
  - legacy `array_string_len_window` C analyzer deletion is landed
  - legacy `array_rmw_window` C analyzer deletion is landed
  - string direct-set source-window metadata is landed
  - `generic_method.has` route policy metadata is landed
  - next implementation card is exact seed ladders to function-level backend route tags
  - CoreBox surface catalog work is landed and now a reference lane;
    `StringBox.length/len/size`, `StringBox.substring/substr`, and
    `StringBox.concat`, `StringBox.trim`, `StringBox.contains`, and one-arg
    `StringBox.lastIndexOf`, `StringBox.replace`, and `StringBox.indexOf` /
    `find`, plus `ArrayBox.length/size/len`, `ArrayBox.push`,
    `ArrayBox.slice`, `ArrayBox.get`, `ArrayBox.pop`, `ArrayBox.set`,
    `MapBox.size`, `MapBox.len`, and `MapBox.has`, are on the Unified value
    path; remaining cleanup is ArrayBox `remove/insert` and remaining MapBox
    rows
- perf blocker (observe-only):
  - `137x-H46 text-cell residence/materialization design`
- method anchor:
  - `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
- active entry:
  - `docs/development/current/main/phases/phase-292x/README.md`
- taskboard:
  - `docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md`
- current blocker token:
  - `move exact seed ladders to function-level backend route tags`

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
  - select the next exact seed ladder and move its function-level backend route to MIR metadata
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
- the current docs front is phase-292x `.inc` thin tag cleanup.
- phase-291x CoreBox first catalog and cleanup slices are landed references.
- active cleanup card is the next phase-292x metadata route card; `292x-93-array-rmw-window-route-card.md` is landed evidence.
- rejected slot-store boundary probe stays parked in `stash@{0}` as
  `wip/concat-slot-store-window-probe`; do not resurrect it unless explicitly
  reopening that rejected card.
- history lives in phase docs, not this file:
  - `docs/development/current/main/phases/phase-291x/README.md`
  - `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
  - `docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md`
  - `docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
  - `docs/development/current/main/phases/phase-292x/README.md`
  - `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
  - `docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
