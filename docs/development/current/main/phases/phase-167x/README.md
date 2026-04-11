# Phase 167x: user-box direct method sealing for keepers

- Status: Landed
- Purpose: stabilize direct user-box method lowering by keeping instance-method receiver type facts and function metadata sealing on one owner path, so the current known-receiver keeper shapes do not drift between direct emits.
- Scope:
  - MIR builder box-member traversal order
  - shared finalize owner for static/instance lowered methods
  - receiver `Box(...)` metadata seeding for instance methods
  - direct-route `Counter.step_chain` known-receiver stability
  - no pure-first seed widening
  - no backend recipe widening

## Decision Now

- fix direct MIR sealing, not backend fallback
- do not absorb this flake in helper retry or pure-first exact seeds
- keep rewrite authority unchanged in this phase; only remove the direct-route metadata drift that currently leaks into it
- keep this as a narrow BoxShape repair inside `phase-163x`

## Restart Handoff

- parent lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- current snapshot:
  - `docs/development/current/main/10-Now.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- SSOT:
  - `docs/development/current/main/phases/phase-167x/167x-90-method-lowering-determinism-ssot.md`
  - `docs/development/current/main/phases/phase-167x/167x-91-task-board.md`
- code owner seam:
  - `src/mir/builder/calls/lowering.rs`
  - `src/mir/builder/calls/parameter_setup.rs`

## Current Cut

- landed deterministic traversal owner:
  - `src/mir/builder/declaration_order.rs` now owns sorted method/constructor traversal for box declarations
- landed direct sealing owner:
  - `src/mir/builder/calls/lowering.rs` now routes instance methods through the same `finalize_function()` metadata sealing path as static lowered methods
  - `src/mir/builder/calls/parameter_setup.rs` now seeds `me` as `MirType::Box(<box>)` and records parameter kinds for instance methods
- landed regression coverage:
  - builder-order unit tests now pin sorted traversal independent of Rust `HashMap` order
  - `src/tests/mir_user_box_method_determinism.rs` now pins `Counter.step_chain/0` receiver metadata and canonical known-receiver `Method` call shape
- result:
  - direct `Counter.step_chain` lowering no longer depends on missing instance-method metadata sealing
  - release direct emit now stays on canonical `Method` shape in repeated probes (`6/6`)
  - pure-first AOT build/asm still has a separate stop-line in backend seed matching; that is not fixed in this phase

## Stop Line

- do not widen pure-first matcher tolerance in this phase
- do not change `try_known_rewrite` authority to a new declaration-presence registry here
- if AOT build still stops on `unsupported pure shape`, reopen `phase-163x` boundary seed contract instead of changing builder authority again
- do not mix this determinism repair with broader user-box local-method parity widening
