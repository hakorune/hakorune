# Phase 179x: string kernel plan export and seed retirement

- Status: Active
- Purpose: recut the remaining string exact-seed bridge so `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc` can move from benchmark-shape reanalysis toward a metadata-first consumer of a dedicated `StringKernelPlan`.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  - `src/mir/string_corridor_placement.rs`
  - `src/runner/mir_json_emit/mod.rs`
  - `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`
- Non-goals:
  - no new public MIR dialect
  - no helper-name semantic recovery widening
  - no benchmark-specific new matcher family
  - no DCE widening in this phase

## Decision Now

- the external reread is adopted in direction:
  - `hako_llvmc_ffi_string_loop_seed.inc` is still too shape-driven
  - retirement should move toward variant-seed style metadata consumption
- one correction is fixed here:
  - MIR JSON already exports `string_corridor_facts`, `string_corridor_relations`, and `string_corridor_candidates`
  - the next missing seam is an explicit backend-consumable `StringKernelPlan`, not raw metadata export itself
- therefore this phase starts with schema/export ownership cleanup, not another exact matcher proof

## Acceptance

- `StringKernelPlan` is described as a minimal backend-consumable schema
- plan ownership stays:
  - `.hako` / canonical MIR vocabulary
  - MIR facts / relations / candidates
  - exported plan JSON
  - backend consumer
- `hako_llvmc_ffi_string_loop_seed.inc` remains bridge-only
- retirement rule stays:
  - metadata-first lane must prove the same keeper before shape matchers shrink

## Exit

- the repo has one explicit `StringKernelPlan` schema to target
- the next implementation cut can add metadata-first consumption without reopening ownership arguments
