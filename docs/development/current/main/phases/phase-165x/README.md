# Phase 165x: escape barrier vocabulary widening

- Status: Landed
- Purpose: widen MIR-side escape analysis by introducing operand-role escape barrier vocabulary that stays separate from generic `used_values()` def-use queries.
- Scope:
  - MIR authority only
  - no runtime helper policy
  - no LLVM-side rediscovery
  - no generic cross-block escape pass in this slice

## Decision Now

- keep semantic authority in MIR
- keep implementation in Rust under `src/mir/**`
- do not encode escape meaning into `used_values()`
- classify escape by operand role, not by instruction kind alone

## Restart Handoff

- parent lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- current snapshot:
  - `docs/development/current/main/10-Now.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- SSOT:
  - `docs/development/current/main/phases/phase-165x/165x-90-escape-barrier-vocabulary-ssot.md`

## Current Cut

- introduce `src/mir/escape_barrier.rs`
- move escape operand classification out of `src/mir/passes/escape.rs`
- make `escape.rs` a consumer of the new classifier API
- pin method-receiver and `FieldSet.value` operand-role proofs
- structural follow-on after this landed slice:
  - `docs/development/current/main/phases/phase-166x/README.md`
  - keep `boundary_fact` / lifecycle extraction after refresh-owner and generic-relation ownership, not before

## Stop Line

- do not widen into PHI/cross-block object graph reasoning in this phase
- do not move escape policy into runtime helpers or LLVM
