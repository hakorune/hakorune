# 173x-90: return-boundary publication sink SSOT

Status: SSOT
Date: 2026-04-12
Scope: sink direct helper-result births to a same-block `return` boundary using already-landed `publication_sink` plan metadata.

## Goal

- keep canonical MIR as the only semantic owner
- consume existing `publication_sink` candidate proof instead of rediscovering a new string-only route
- reopen broader string publication one boundary at a time after the exact front is already green

## Diagnosis

Current broader string state is asymmetric:

- landed:
  - helper-result `length()` consumes `publication_sink`
  - helper-result `substring()` consumes `publication_sink`
  - local `ArrayBox.set` birth sink consumes `materialization_sink`
  - first post-store trailing `length()` observer is handled
- still open:
  - helper-result direct `return`

That leaves a same-block copy-only return chain where the helper birth still sits earlier in the block even though the current candidate plan already says publication may sink to the visible boundary.

## Fix

### 1. Keep the semantic contract unchanged

Do not add a new MIR dialect or another boundary vocabulary.

The route must keep reading:

- `string_corridor_candidates[*].kind = publication_sink`
- `plan.proof.kind = concat_triplet`

### 2. Add a same-block return consumer in `string_corridor_sink`

Accept only this narrow shape:

- direct `substring_concat3_hhhii` helper result
- same-block copy-only aliases
- same-block `Return { value: Some(...) }` whether it currently lives in:
  - block terminator
  - trailing instruction form
- helper root has no extra users outside that chain

### 3. Rewrite by sinking birth, not relaxing barriers

The transform may:

- delete the copy-only aliases
- reinsert the helper call immediately before the `return`
- rewrite the terminator to return the helper root directly

The transform must not:

- cross `phi_merge`
- cross `call` / `boxcall`
- cross blocks
- treat this as generic `return` barrier relaxation

## Acceptance

- focused unit contract proves:
  - copy-only return chain disappears
  - helper call is reinserted immediately before `return`
  - unrelated pure instructions stay above the sunk helper
- focused unit guard lives at:
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_string_publication_return_unit.sh`
- current string guardrail smokes stay green
- exact `kilo_micro_substring_concat` asm/perf stays green

## Non-Goals

- no host-boundary publication wave
- no `phi_merge` continuity widening
- no generic lifecycle extraction
- no exact-seed bridge rewrite
- no final emitted-MIR return-carrier normalization
