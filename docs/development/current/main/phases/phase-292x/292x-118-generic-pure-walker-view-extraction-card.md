---
Status: Active
Date: 2026-04-23
Scope: Plan the remaining generic pure walker split after 292x-117 reduced the guard to 3 files / 4 lines.
Related:
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-117-generic-pure-walker-residual-debt-card.md
  - tools/checks/inc_codegen_thin_shim_guard.sh
---

# 292x-118: Generic Pure Walker View Extraction

## Current Debt

`tools/checks/inc_codegen_thin_shim_guard.sh` reports 3 files / 3
analysis-debt lines:

- `hako_llvmc_ffi_pure_compile.inc`: entry-function `blocks` read
- `hako_llvmc_ffi_pure_compile_generic_lowering.inc`: generic block
  `instructions` view accessor
- `hako_llvmc_ffi_compiler_state.inc`: cross-block use finder instruction walk

These are no longer dead helper rows or route-specific exact matchers. They are
the remaining generic pure walker substrate that still reads MIR shape directly.

## Decision

Do not shave these lines by hiding JSON access behind a same-layer helper. The
next useful cleanup is a real boundary split:

- MIR/codegen owner prepares a small `GenericPureProgramView` or equivalent
  recipe/view for blocks, instruction lists, and single-use facts.
- `.inc` consumes that view for validation and emission.
- Route legality remains MIR-owned; `.inc` must not rediscover special method
  families while walking.

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
