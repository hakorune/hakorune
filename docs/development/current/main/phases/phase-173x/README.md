# Phase 173x: substring concat return-boundary publication sink

- Status: Landed
- Purpose: land the next narrow broader-corridor string slice by sinking a direct `substring_concat3_hhhii` helper birth to a same-block `return` boundary when the current `publication_sink` plan already proves the route.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/string_corridor_sink.rs`
  - focused string unit/guard contracts for the same-block return route
- Non-goals:
  - no `phi_merge` widening
  - no `call` / `boxcall` / `return` barrier relaxation
  - no host-boundary publication widening
  - no exact-seed or boundary `pure-first` rewrite
  - no broader post-store window reopening

## Decision Now

- treat this as a narrow `publication_sink` consumer cut, not a barrier-policy cut
- the current semantic authority stays:
  - canonical MIR facts / relations / candidates
  - `publication_sink` plan on the helper result
- this cut is allowed to:
  - remove copy-only return chains in one block
  - move the direct helper birth to the final same-block `return` boundary
- this cut is not allowed to:
  - cross `phi_merge`
  - cross `call` / `boxcall`
  - reinterpret helper names or invent new bridge-local facts

## Restart Handoff

- parent implementation lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- sibling string guardrail:
  - `docs/development/current/main/phases/phase-137x/README.md`
- previous exact-front cut:
  - `docs/development/current/main/phases/phase-172x/README.md`
- current snapshot:
  - `docs/development/current/main/10-Now.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- SSOT:
  - `docs/development/current/main/phases/phase-173x/173x-90-return-boundary-publication-sink-ssot.md`
  - `docs/development/current/main/phases/phase-173x/173x-91-task-board.md`

## Acceptance

- when a direct `substring_concat3_hhhii` helper result flows through only single-use same-block copy aliases to `return`, `string_corridor_sink` may:
  - delete the copy-only chain
  - sink the helper birth to just before `return`
  - leave unrelated pure instructions ahead of the sunk helper
- the route must still require current `publication_sink.plan.proof = concat_triplet`
- focused unit guard lives at:
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_string_publication_return_unit.sh`
- no widening across `phi_merge` or host-boundary routes
- current string guardrail smokes stay green
- exact `kilo_micro_substring_concat` asm/perf stays green

## Stop Line

- do not claim this completes broader `return` / `store` / host-boundary publication
- do not move helper birth across block boundaries
- do not claim final emitted MIR already keeps the same return shape; return-carrier normalization stays outside this cut
- do not attach new generic lifecycle/boundary vocabulary here
- do not widen `materialization_sink` in the same cut

## Landed Result

- same-block direct-helper `return` publication sink is now covered in `string_corridor_sink`
- the route accepts both:
  - block terminator `Return`
  - trailing instruction-form `Return`
- copy-only same-block return chains now collapse under the focused unit guard:
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_string_publication_return_unit.sh`
- existing string guardrail smokes, exact asm/perf, and `tools/checks/dev_gate.sh quick` stay green
- remaining string publication backlog is now narrowed to:
  - broader `store`
  - host-boundary publication
  - final emitted-MIR return-carrier cleanup if that route needs to become a direct guard later
