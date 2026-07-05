# 3121 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-RUNTIME-PARSE-ERROR-READINESS-001

Status: active-partial

## Scope

Fix or precisely isolate the runtime `ProgramJsonV0PhaseStateBox.parse`
`err=1` result exposed by the 3120 heavy executable probe.

3120 proves the Layer4 loop Recipe DTO probe can now emit and run as an EXE
within the bounded guard. The remaining blocker is semantic runtime parity:
the same ProgramJSON rows that have green MIR route proof currently return
`snapshot_kind=LoopRecipeDtoSnapshotV1;err=1;reason=parse_error` under EXE.

## Prerequisites

```bash
bash tools/checks/hako_programjson_scanner_result_map_return_contract_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_heavy_exe_readiness_gate.sh
```

Required state:

```text
3109 scanner result-map contract remains green.
3109 remains the canonical cleanup task for scanner out-map/null returns:
  prefer TotalResultMapReturnContractV1; avoid body-proof widening and
  mixed_runtime_i64_or_handle scanner out-map publication.
3119 lightweight MIR route proof remains green.
3120 heavy emit-exe probe is green.
3120 exact_first_blocker = phase_state_parse_runtime_parse_error.
```

## Allowed Work

```text
inspect PhaseState parse runtime failure reason for covered Layer4 rows
make AOT runtime string/map result handling match the MIR route contract
move any remaining scanner `MapBox | null` call site to a total result-map
helper if the parse failure proves scanner-return related
add a narrow direct PhaseState parse EXE row if needed
upgrade the 3120 heavy guard to runtime_parity_green=1 if parity becomes exact
```

## Forbidden Work

```text
do not widen scanner void returns
do not use mixed_runtime_i64_or_handle for scanner out-map helpers
do not use body proof alone to publish object returns from void signatures
do not add a nullable out-map bridge without an explicit declared helper
contract, `remove_after`, and `new_consumers_allowed = false`
do not add MIR mutation, lowering, ID allocation, route switch, or new ABI
do not replace ProgramJSON-derived DTO traversal with prebuilt token strings
do not claim full RecipeMatcher execution or Source Selfhost
```

## Acceptance

```text
direct PhaseState parse EXE row identifies the exact failing sub-contract
runtime_parity_green = 1 only if executable output matches canonical DTO summaries
if runtime parity is still blocked, exact_first_blocker names the next owner
void_signature_object_return_widening = 0
source_selfhost_claim = 0
```

## 2026-07-06 Progress

Implemented the first narrow cleanup for the EXE-only parse failure:

```text
owner = ProgramJsonV0PhaseStateBox / ProgramJsonV0PhaseStateConsumerBox
fix = keep scanner-derived string tokens as raw StringBox values for dispatch
      and recipe classification; avoid `"" + map_get(...)` handle-id
      canonicalization on known Program(JSON v0) enum tokens.
```

Validated sub-rows under EXE:

```text
LocalStmtHandler.handle(Local Int) = err=0
RecipeFactsBox.from_stmt("Local") = err=0
RecipeVerifierBox.verify(Local recipe item) = err=0
```

The public heavy guard remains bounded and non-claiming:

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_heavy_exe_readiness_gate.sh
heavy_emit_exe_probe=green
runtime_parity_green=0
exact_first_blocker=phase_state_parse_runtime_parse_error
```

## Follow-up Task

```text
next_card:
  HAKO-AOT-PROGRAMJSON-PHASE-STATE-POSITIVE-PATH-HELPER-ROUTE-READINESS-001

reason:
  Once Local consumption reaches the positive append/summary path, AOT exposes
  helper-call route gaps such as PhaseState recipe append and DTO summary
  helper calls. Do not paper over them with scanner void widening or
  mixed_runtime_i64_or_handle.

allowed:
  make same-module positive-path helper calls publish narrow map_handle or
  string_handle return shapes, or refactor helpers only where it keeps source
  files under line limits and preserves DTO behavior.

forbidden:
  no body-proof object return publication
  no scanner nullable bridge
  no MIR mutation/lowering/id allocation/new ABI/runtime route switch
```
