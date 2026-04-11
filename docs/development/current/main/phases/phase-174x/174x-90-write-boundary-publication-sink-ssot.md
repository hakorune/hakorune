# 174x-90: write-boundary publication sink SSOT

Status: SSOT
Date: 2026-04-12
Scope: sink direct helper-result births to same-block canonical write boundaries using already-landed `publication_sink` plan metadata.

## Goal

- keep canonical MIR as the only semantic owner
- consume existing `publication_sink` candidate proof instead of rediscovering a new string-only route
- reopen broader string publication one boundary at a time after the exact front is already green

## Diagnosis

Current broader string state after `phase-173x` still left canonical write boundaries open:

- landed:
  - helper-result `length()` consumes `publication_sink`
  - helper-result `substring()` consumes `publication_sink`
  - direct helper-result same-block `return` consumes `publication_sink`
  - local `ArrayBox.set` birth sink consumes `materialization_sink`
  - first post-store trailing `length()` observer is handled
- still open:
  - same-block canonical `Store { value, .. }`
  - same-block canonical `FieldSet { value, .. }`

That left copy-only write chains where the helper birth still sat earlier in the block even though the current candidate plan already said publication may sink to the visible boundary.

## Fix

### 1. Keep the semantic contract unchanged

Do not add a new MIR dialect or another boundary vocabulary.

The route must keep reading:

- `string_corridor_candidates[*].kind = publication_sink`
- `plan.proof.kind = concat_triplet`

### 2. Add same-block write-boundary consumers in `string_corridor_sink`

Accept only this narrow shape:

- direct `substring_concat3_hhhii` helper result
- same-block copy-only aliases
- same-block canonical write boundary in instruction form:
  - `Store { value, .. }`
  - `FieldSet { value, .. }`
- helper root has no extra users outside that chain
- helper is not moved past another effectful instruction

### 3. Rewrite by sinking birth, not relaxing barriers

The transform may:

- delete the copy-only aliases
- reinsert the helper call immediately before the write boundary
- rewrite the write boundary to use the helper root directly

The transform must not:

- cross `phi_merge`
- cross `call` / `boxcall`
- cross blocks
- treat this as generic store/escape barrier relaxation

## Acceptance

- focused unit contract proves:
  - copy-only write chain disappears
  - helper call is reinserted immediately before the write boundary
  - unrelated pure instructions stay above the sunk helper
- focused unit guard lives at:
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_string_publication_store_unit.sh`
- current string guardrail smokes stay green
- exact `kilo_micro_substring_concat` asm/perf stays green

## Non-Goals

- no host-boundary publication wave
- no `phi_merge` continuity widening
- no generic lifecycle extraction
- no exact-seed bridge rewrite
- no loop-carry shaping reopen
