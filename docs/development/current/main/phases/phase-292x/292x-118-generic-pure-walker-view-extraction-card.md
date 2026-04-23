---
Status: Closed
Date: 2026-04-23
Scope: Plan the remaining generic pure walker split after 292x-117 reduced the guard to 3 files / 4 lines.
Related:
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-117-generic-pure-walker-residual-debt-card.md
  - tools/checks/inc_codegen_thin_shim_guard.sh
---

# 292x-118: Generic Pure Walker View Extraction

## Current Debt

`tools/checks/inc_codegen_thin_shim_guard.sh` reports 0 files / 0
analysis-debt lines, plus 1 file / 2 view-owner shape-read lines:

- view owner: `hako_llvmc_ffi_pure_compile.inc` / `generic_pure_view`

These are no longer dead helper rows, route-specific exact matchers, or
scattered generic walker scans. They are the explicit boundary view construction
surface.

## Decision

Do not shave these lines by hiding JSON access behind a same-layer helper. The
next useful cleanup is a real boundary split:

- MIR/codegen owner prepares a small `GenericPureProgramView` or equivalent
  recipe/view for blocks, instruction lists, and single-use facts.
- `.inc` consumes that view for validation and emission.
- Route legality remains MIR-owned; `.inc` must not rediscover special method
  families while walking.
- The guard distinguishes analysis debt from view-owner construction. New raw
  MIR shape reads outside explicit view-owner regions fail.

## Next Slice

Start with a docs/code map of the generic pure walker phases:

1. entry block validation / function selection
2. pre-scan state population
3. cross-block single-use refinement
4. emission walk

Then extract only one API seam at a time. The first implementation slice should
not change accepted MIR shapes; it should only make one walker phase consume a
named view or recipe produced earlier.

## Slice 118a Result

- introduced `hako_llvmc_generic_pure_program_view`
- moved entry function, metadata, blocks, block count, and rune selection setup
  behind `hako_llvmc_read_generic_pure_program_view(...)`
- kept accepted MIR shapes unchanged
- kept the guard at 3 files / 4 lines; this slice names the boundary, it does
  not hide or shave the remaining walker substrate

## Slice 118b Result

- introduced `GenericPureBlockView`
- made pre-scan and emission consume the same named block view accessor
- kept the raw `instructions` access visible in one accessor rather than hiding
  it behind unrelated helpers
- lowered the guard from 3 files / 4 lines to 3 files / 3 lines

## Slice 118c Result

- lifted `GenericPureBlockView` to the generic pure view owner in
  `pure_compile.inc`
- made cross-block use lookup consume the same block view accessor as pre-scan
  and emission
- removed raw walker access from `compiler_state.inc` and
  `pure_compile_generic_lowering.inc`
- lowered the guard from 3 files / 3 lines to 1 file / 2 lines

## Slice 118d Result

- added explicit `inc-codegen-view-owner` markers around the generic pure view
  construction reads
- added `tools/checks/inc_codegen_thin_shim_view_allowlist.tsv`
- updated `inc_codegen_thin_shim_guard.sh` so view-owner construction is
  accounted separately from analysis debt
- lowered analysis debt from 1 file / 2 lines to 0 files / 0 lines while keeping
  the view owner pinned at 1 file / 2 lines

## Acceptance

```bash
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh
bash tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh
git diff --check
```

## Closeout

Phase-292x stops here. The useful cleanup is no longer another same-layer
`.inc` shave:

- active `.inc` analysis debt is 0 files / 0 lines
- explicit view-owner construction remains 1 file / 2 lines
- `tools/checks/inc_codegen_thin_shim_guard.sh` prevents raw MIR analysis debt
  from growing again

Future work may replace this view owner with MIR/codegen-owned metadata, but
that is a new phase, not active phase-292x debt.
