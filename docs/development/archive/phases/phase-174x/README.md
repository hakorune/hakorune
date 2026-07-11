# Phase 174x: substring concat write-boundary publication sink

- Status: Landed
- Purpose: land the next narrow broader-corridor string slice by sinking a direct `substring_concat3_hhhii` helper birth to a same-block canonical write boundary when the current `publication_sink` plan already proves the route.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/string_corridor_sink.rs`
  - focused string unit/guard contracts for same-block canonical write boundaries
- Non-goals:
  - no `phi_merge` widening
  - no host-boundary publication widening
  - no exact-seed or boundary `pure-first` rewrite
  - no broader post-store observer reopening
  - no loop-carry shaping reopen

## Decision Now

- treat this as a narrow `publication_sink` consumer cut, not a new lifecycle/boundary vocabulary phase
- the semantic authority stays:
  - canonical MIR facts / relations / candidates
  - `publication_sink` plan on the helper result
- this cut is allowed to:
  - remove copy-only same-block write chains
  - move the direct helper birth to just before a same-block canonical write boundary
- this cut is not allowed to:
  - cross `phi_merge`
  - cross `call` / `boxcall`
  - reinterpret helper names or invent new bridge-local facts

## Acceptance

- when a direct `substring_concat3_hhhii` helper result flows through only single-use same-block copy aliases to:
  - `Store { value, .. }`
  - `FieldSet { value, .. }`
  `string_corridor_sink` may:
  - delete the copy-only chain
  - sink the helper birth to just before the write boundary
  - leave unrelated pure instructions ahead of the sunk helper
- the route must still require current `publication_sink.plan.proof = concat_triplet`
- focused unit guard lives at:
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_string_publication_store_unit.sh`
- current string guardrail smokes stay green
- exact `kilo_micro_substring_concat` asm/perf stays green

## Landed Result

- same-block canonical write-boundary publication sink is now covered in `string_corridor_sink`
- the focused unit guard now keeps both:
  - `Store { value, .. }`
  - `FieldSet { value, .. }`
  under the same `publication_sink` plan contract
- remaining string publication backlog is now narrowed to:
  - host-boundary publication
  - final emitted-MIR return-carrier cleanup if that route later needs a direct guard
