# phase21_5 / perf

This family is the next live semantic split after `phase29cc_wsm/p8`.
It currently hosts the `chip8` and `kilo` subfamilies.

## Active Split

- `chip8/`
  - chip8 kernel crosslang baseline
  - 1 smoke
- `kilo/`
  - kilo kernel crosslang + route-hotspot baseline
  - 7 smokes

## Migration Note

- The remaining `phase21_5_perf_*` scripts still live under `tools/smokes/v2/profiles/integration/apps/`.
- Keep new `phase21_5_perf` work under this family tree; do not add more `phase21_5_perf_*` files to `apps/`.
- After `chip8/` and `kilo/`, the next live family to inspect is `phase21_5/perf/apps`.
