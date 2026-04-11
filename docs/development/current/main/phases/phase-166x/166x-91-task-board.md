# 166x-91: semantic refresh and generic relation cleanup task board

## Board

- [x] `166xA` docs lock
  - phase README
  - SSOT
  - root/current/workstream pointers
- [x] `166xB` semantic refresh owner
  - add one MIR-side refresh entry point
  - move scattered refresh ordering behind that entry
  - keep first cut behavior-preserving
- [x] `166xC` generic `value_origin` owner
  - stop letting domain modules own `copy root` normalization
  - prepare a shared MIR seam for alias-root queries
- [ ] `166xD` generic `phi_relation` owner
  - keep PHI carry/base traversal in one generic seam
  - let domain layers consume the result, not own the traversal
- [ ] `166xE` compat semantic recovery quarantine
  - move helper/runtime-name semantic recovery out of domain fact builders
  - keep canonical-op reading as the domain-pass direction
- [ ] `166xF` generic boundary/lifecycle extraction decision
  - only after `166xB` through `166xE`
  - do not start this before refresh and relation ownership are stable
- [ ] `166xG` verification + doc sync
  - pointer docs
  - targeted tests / `git diff --check`

## Notes

- this is structural cleanup, not a new optimization acceptance wave
- do not mix fact-owner cleanup with runtime/LLVM policy moves
- if `166xB` is not landed first, later vocabulary extraction will create another owner seam instead of removing one
