---
Status: Active
Date: 2026-04-21
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
  - `phase-137x-H owner-first optimization return` (active; H45 post-observer-guard memmove/materialization owner)
- blocker:
  - `137x-H45 post-observer-guard memmove/materialization owner`
- method anchor:
  - `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- active entry:
  - `docs/development/current/main/phases/phase-137x/137x-current.md`
- taskboard:
  - `docs/development/current/main/phases/phase-137x/137x-91-task-board.md`

## Current Perf Snapshot

- H44.1 keeper:
  - runtime-private observer all-hit guard
  - no MIR metadata shape change
  - no `.inc` emit change
  - no public ABI change
- whole guard:
  - `kilo_kernel_small = C 86 ms / Ny AOT 6 ms`
  - `ny_aot_instr=24129815`
  - `ny_aot_cycles=5615809`
- exact guard:
  - `kilo_micro_array_string_store = C 11 ms / Ny AOT 3 ms`
  - `ny_aot_instr=9265721`
  - `ny_aot_cycles=2317791`
- meso guard:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 4 ms`
  - `ny_aot_instr=17651018`
  - `ny_aot_cycles=4247395`
- latest top after H44.1:
  - combined executor closure `59.07%`
  - external `__memmove_avx512_unaligned_erms` `24.01%`
  - `_int_malloc` `3.03%`

## Immediate Next

- H45 first step:
  - split the post-H44.1 owner around external `memmove` / materialization
  - preserve or regenerate top/annotate only if source mapping is needed
- allowed next code shape:
  - source-pinned copy/materialization transition only
  - or broader text-cell residence/materialization design if the sampled owner is not narrow
- forbidden drift:
  - no more suffix/left-copy micro leaves without a new sampled source block
  - no `.inc` planner regression
  - no runtime legality/provenance inference
  - no benchmark-name/source-content assumptions

## Restart Notes

- worktree should be clean after the last commit.
- branch may be ahead of `hakorune/public-main`; do not push unless requested.
- rejected slot-store boundary probe stays parked in `stash@{0}` as
  `wip/concat-slot-store-window-probe`; do not resurrect it unless explicitly
  reopening that rejected card.
- history lives in phase docs, not this file:
  - `docs/development/current/main/phases/phase-137x/137x-current.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
