# phase21_5 / perf

This family is the next live semantic split after `phase29cc_wsm/p8`.
It currently hosts the `chip8`, `kilo`, and `apps` subfamilies.

## Active Split

- `chip8/`
  - chip8 kernel crosslang baseline
  - 1 smoke
- `kilo/`
  - kilo kernel crosslang + route-hotspot baseline
  - 7 smokes
- `apps/`
  - app wallclock bundle
  - first semantic subfamilies: `entry_mode/` and `mir_mode/`
  - singleton subfamilies now live under:
    - `case_breakdown/`
    - `compile_run_split/`
    - `crosslang_bundle/`
    - `emit_mir_jsonfile_route/`
    - `startup_subtract/`
  - the bundle root now only keeps the README / compatibility note

## Migration Note

- The remaining `phase21_5_perf_*` scripts have been split out of `tools/smokes/v2/profiles/integration/apps/`.
- Keep new `phase21_5_perf` work under this family tree; do not add more `phase21_5_perf_*` files to the bundle root.
- After `chip8/`, `kilo/`, `entry_mode/`, `mir_mode/`, and the singleton app subfamilies, this bundle is considered done enough for the current smoke-taxonomy pass.
