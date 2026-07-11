# 167x-91: user-box direct method sealing task board

## Board

- [x] `167xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `167xB` deterministic member traversal owner
  - add one MIR builder seam for method/constructor iteration
  - stop direct `HashMap` traversal on live box-member lowering paths
- [x] `167xC` shared finalize owner
  - route instance methods through `finalize_function()`
  - seal per-function `value_types` on the shared owner path
- [x] `167xD` receiver metadata seed
  - seed `me` as `MirType::Box(<box>)`
  - register instance-method parameter kinds
- [x] `167xE` regression lock
  - sorted traversal unit tests
  - `Counter.step_chain/0` receiver metadata + canonical method shape regression tests
  - repeated release direct emit probe stays green
- [x] `167xF` verify + doc sync
  - targeted cargo tests
  - release direct repeat probe
  - `git diff --check`

## Notes

- this is a narrow BoxShape repair inside `phase-163x`
- do not treat backend helper retry as the fix
- pure-first build/asm failure after direct emit is a separate backend seed contract issue, not part of this cut
