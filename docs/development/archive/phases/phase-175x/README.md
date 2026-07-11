# Phase 175x: substring concat host-boundary publication sink

- Status: Landed
- Purpose: land the first narrow host-boundary publication slice by sinking a direct `substring_concat3_hhhii` helper birth to a same-block `RuntimeDataBox.set(...)` boundary when the current `publication_sink` plan already proves the route.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/string_corridor_sink.rs`
  - focused string unit/guard contracts for same-block host-boundary publication
- Non-goals:
  - no `phi_merge` widening
  - no generic `Call` / `BoxCall` barrier relaxation
  - no `setField(...)` widening
  - no exact-seed or boundary `pure-first` rewrite
  - no final emitted-MIR return-carrier cleanup

## Decision Now

- treat this as a narrow `publication_sink` consumer cut, not a new host/runtime vocabulary phase
- the semantic authority stays:
  - canonical MIR facts / relations / candidates
  - `publication_sink` plan on the helper result
- this cut is allowed to:
  - remove copy-only same-block host-boundary write chains
  - move the direct helper birth to just before a same-block `RuntimeDataBox.set(...)` call
- this cut is not allowed to:
  - cross `phi_merge`
  - cross `call` / `boxcall`
  - reinterpret helper names or invent a new string-only host-boundary fact

## Acceptance

- when a direct `substring_concat3_hhhii` helper result flows through only single-use same-block copy aliases to `RuntimeDataBox.set(key, value)`, `string_corridor_sink` may:
  - delete the copy-only chain
  - sink the helper birth to just before the host-boundary call
  - rewrite the host-boundary value argument to use the helper root directly
- the route must still require current `publication_sink.plan.proof = concat_triplet`
- focused unit guard lives at:
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_string_publication_host_boundary_unit.sh`
- current string guardrail smokes stay green
- exact `kilo_micro_substring_concat` asm/perf stays green

## Landed Result

- same-block `RuntimeDataBox.set(...)` is now covered as the first host-boundary publication sink in `string_corridor_sink`
- remaining string backlog is now only:
  - final emitted-MIR return-carrier cleanup if that route later needs a direct guard
