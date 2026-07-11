# Phase 179x: string kernel plan export and seed retirement

- Status: Landed
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
- `179xA` is now landed:
  - docs/schema ownership is fixed
- `179xB` is now landed:
  - MIR-side `derive_string_kernel_plan(...)` now derives a backend-consumable plan from existing string corridor candidates
  - MIR JSON now exports `metadata.string_kernel_plans`
- `179xC` is now landed:
  - `hako_llvmc_ffi_string_loop_seed.inc` now consumes exported `metadata.string_kernel_plans` first for the stable-length `substring_concat` len route
  - the old body matcher remains only as shape-fallback for the remaining full-loop bridge
- `179xD` is now landed:
  - exact asm/perf keeper parity on `kilo_micro_substring_concat` stays green:
    - `ny_main = mov $0x10 ; xor %ecx,%ecx ; ... ; add $0x12,%rax ; ... ; ret`
    - `ny_aot_instr=1,665,875 / ny_aot_cycles=1,027,222 / ny_aot_ms=3`
- `179xE` is now landed:
  - the old loop matcher no longer accepts the 14-op len-route fallback
  - only the remaining full-loop bridge stays on shape fallback

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
- the first metadata-first consumer now proves the same keeper as the prior exact bridge
- the next implementation cut can return to broader DCE widening without reopening string bridge ownership
