# phase21_5 / perf / apps

Apps wallclock family. This bundle is the remaining `phase21_5/perf/apps` split after `chip8/`, `kilo/`, `entry_mode/`, and `mir_mode/` landed.

## Active Split

- `entry_mode/`
  - app entry-mode compare family
  - 5 smokes
- `mir_mode/`
  - app MIR input-mode compare family
  - 5 smokes
- `case_breakdown/`
  - per-app case breakdown contract smoke
  - 1 smoke
- `compile_run_split/`
  - compile/run split contract smoke
  - 1 smoke
- `crosslang_bundle/`
  - crosslang bundle contract smoke
  - 1 smoke
- `emit_mir_jsonfile_route/`
  - `--emit-mir-json` / `--mir-json-file` route contract smoke
  - 1 smoke
- `startup_subtract/`
  - startup subtraction contract smoke
  - 1 smoke
- bundle root:
  - compatibility README only
  - no live scripts remain here

## Migration Note

- The remaining `phase21_5_perf_*` scripts have been split out of the bundle root.
- Keep new `phase21_5_perf` work under this family tree; do not add more `phase21_5_perf_*` files to the bundle root.
- The active subfamilies are now `entry_mode/`, `mir_mode/`, `case_breakdown/`, `compile_run_split/`, `crosslang_bundle/`, `emit_mir_jsonfile_route/`, and `startup_subtract/`.
