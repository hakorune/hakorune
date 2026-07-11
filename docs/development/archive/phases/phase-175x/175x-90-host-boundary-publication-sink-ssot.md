# 175x-90: host-boundary publication sink SSOT

Status: SSOT
Date: 2026-04-12
Scope: sink direct helper-result births to a same-block `RuntimeDataBox.set(...)` boundary using already-landed `publication_sink` plan metadata.

## Goal

- keep canonical MIR as the only semantic owner
- consume existing `publication_sink` candidate proof instead of rediscovering a new string-only host route
- close the current planned publication backlog one boundary class at a time

## Diagnosis

Current broader string state after `phase-174x` still left one host-visible method boundary open:

- landed:
  - helper-result `length()` consumes `publication_sink`
  - helper-result `substring()` consumes `publication_sink`
  - direct helper-result same-block `return` consumes `publication_sink`
  - same-block canonical `Store { value, .. }` consumes `publication_sink`
  - same-block canonical `FieldSet { value, .. }` consumes `publication_sink`
- still open:
  - same-block `RuntimeDataBox.set(key, value)` when the value is a copy-only publication helper chain

That left a narrow host-boundary case where the helper birth still sat earlier in the block even though the current candidate plan already said publication may sink to the visible boundary.

## Fix

### 1. Keep the semantic contract unchanged

Do not add a new MIR dialect or another boundary vocabulary.

The route must keep reading:

- `string_corridor_candidates[*].kind = publication_sink`
- `plan.proof.kind = concat_triplet`

### 2. Add a same-block `RuntimeDataBox.set(...)` consumer in `string_corridor_sink`

Accept only this narrow shape:

- direct `substring_concat3_hhhii` helper result
- same-block copy-only aliases
- same-block host-boundary method call:
  - `RuntimeDataBox.set(key, value)`
- helper root has no extra users outside that chain
- helper is not moved past another effectful instruction

### 3. Rewrite by sinking birth, not relaxing barriers

The transform may:

- delete the copy-only aliases
- reinsert the helper call immediately before the host-boundary call
- rewrite the host-boundary value argument to use the helper root directly

The transform must not:

- cross `phi_merge`
- cross `call` / `boxcall`
- widen to `setField(...)`
- treat this as generic call/escape barrier relaxation

## Acceptance

- focused unit contract proves:
  - copy-only host-boundary chain disappears
  - helper call is reinserted immediately before `RuntimeDataBox.set(...)`
  - unrelated pure instructions stay above the sunk helper
- focused unit guard lives at:
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_string_publication_host_boundary_unit.sh`
- current string guardrail smokes stay green
- exact `kilo_micro_substring_concat` asm/perf stays green

## Non-Goals

- no generic method-call host-boundary widening
- no `setField(...)` widening
- no `phi_merge` continuity widening
- no generic lifecycle extraction
- no exact-seed bridge rewrite
- no final emitted-MIR return-carrier cleanup
