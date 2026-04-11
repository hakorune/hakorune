# Phase 166x: semantic refresh and generic relation cleanup

- Status: Active
- Purpose: fix the owner seams around semantic metadata so later optimization widening does not reintroduce helper-name, alias-root, or PHI-base reanalysis in domain passes.
- Scope:
  - MIR authority only
  - metadata refresh orchestration
  - generic `value_origin` / `phi_relation` ownership
  - compat semantic-recovery quarantine
  - no broad transform rewrite in this phase

## Decision Now

- keep semantic authority in canonical MIR
- keep `used_values()` as generic def-use only
- keep operand-role escape vocabulary separate from generic def-use
- move `copy root` / `phi carry` ownership out of domain passes and into generic MIR seams
- quarantine helper/runtime-name semantic recovery into compat layers instead of leaving it inside domain fact builders
- postpone generic `boundary_fact` / `lifecycle_outcome` extraction until refresh and relation owners are stable

## Restart Handoff

- parent lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- precursor:
  - `docs/development/current/main/phases/phase-165x/README.md`
- current snapshot:
  - `docs/development/current/main/10-Now.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- design owner:
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
- string-domain anchor:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- SSOT:
  - `docs/development/current/main/phases/phase-166x/166x-90-semantic-refresh-and-generic-relation-ssot.md`

## Current Cut

- landed first cut:
  - `src/mir/semantic_refresh.rs` now owns MIR semantic metadata refresh entry points
  - `MirCompiler` now refreshes semantic metadata through that single module-level owner
  - `string_corridor_sink` now refreshes its function-local fact/relation/candidate stack through the same owner seam
- next:
  - define the fixed order:
    - generic `value_origin` / `phi_relation` owner next
    - compat semantic recovery quarantine after that
    - generic boundary/lifecycle extraction only after that
- keep current domain `fact -> candidate -> transform` layering

## Stop Line

- do not push policy into runtime helpers or LLVM
- do not make domain passes own copy-root or PHI semantics
- do not extract new generic lifecycle vocabulary before refresh ownership is fixed
- do not mix this structural cleanup with new optimization acceptance shapes
