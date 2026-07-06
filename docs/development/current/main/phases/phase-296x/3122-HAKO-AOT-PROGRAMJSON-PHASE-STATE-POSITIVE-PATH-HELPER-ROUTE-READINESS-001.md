# 3122 - HAKO-AOT-PROGRAMJSON-PHASE-STATE-POSITIVE-PATH-HELPER-ROUTE-READINESS-001

Status: selected-next

## Scope

Stabilize the same-module helper route boundary exposed after 3121 narrowed the
EXE-only PhaseState parse failure.

3121 proved the first Local positive sub-rows under EXE:

```text
LocalStmtHandler.handle(Local Int) = err=0
RecipeFactsBox.from_stmt("Local") = err=0
RecipeVerifierBox.verify(Local recipe item) = err=0
```

The remaining blocker is not the scanner result-map contract and not the public
`ProgramJsonV0PhaseStateBox.parse/2` route itself. It is the transitive
positive-path helper boundary:

```text
PhaseState recipe append helpers
DTO summary helpers
same-module DirectAbi helper route publication / same-module body prepass
```

## Responsibility Split

```text
.hako owns:
  Program(JSON v0) traversal semantics
  raw StringBox token preservation for known ProgramJSON enum tokens
  Recipe DTO shape and summary behavior

Rust MIR route planner owns:
  global-call DirectAbi vs Unsupported classification
  same-module helper return contract publication
  exact blocker metadata such as missing_multi_function_emitter

C AOT same-module emitter owns:
  consuming explicit same_module_function_definitions
  prepass value/origin publication for already-accepted same-module helpers
  failing fast with module_generic_prepass_failed when metadata is insufficient
```

## In Scope

```text
make one positive-path helper family route-clean first:
  ProgramJsonV0PhaseStateBox._append_recipe_item_result/5
  ProgramJsonV0PhaseStateBox._append_recipe_children_result/6

if the append family is green, then inspect the DTO summary family:
  ProgramJsonLoopRecipeDtoSnapshotBox._summary_base/4
  ProgramJsonLoopRecipeDtoSnapshotBox._assignment_tail/2
  ProgramJsonLoopRecipeDtoSnapshotBox._if_tail/3

publish narrow map_handle or string_handle return contracts only when the
declared return type/body shape supports it.
```

## Out of Scope

```text
do not inline or erase append/summary helpers only to hide route gaps
do not widen scanner void returns
do not use mixed_runtime_i64_or_handle for scanner out-map helpers
do not use body proof alone to publish object returns from void signatures
do not add nullable scanner bridges
do not add raw callee-name special cases in C
do not add a whole-module C scan
do not add MIR mutation, lowering, ID allocation, runtime route switch, or new ABI
do not claim full RecipeMatcher execution, HakoAdoption, or Source Selfhost
```

## Candidate Owners

```text
Rust route / contract:
  src/mir/global_call_route_plan.rs
  src/mir/global_call_route_plan/same_module_static_helper_contract.rs
  src/mir/same_module_body_shape.rs

C same-module backend, only if route publication is already green but emit fails:
  lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_function_emit.inc
```

## Acceptance

```text
3109 scanner result-map contract guard = green
3119 lightweight DTO route guard = green
3120 heavy EXE readiness guard = green

same-module positive-path helper routes publish DirectAbi where accepted
PhaseState append helper family returns map_handle or a narrower accepted shape
DTO summary helper family returns string_handle if reached in this slice

runtime_parity_green = 1
  OR exact_first_blocker names the next non-helper owner

void_signature_object_return_widening = 0
mixed_runtime_i64_or_handle_for_scanner_out_map = 0
```

## 2026-07-06 Progress Note

Status: active, not green.

Resolved within this slice:

```text
ProgramJsonV0ScannerBox.read_node_type_at_result now reads the node object's
own top-level "type" field instead of the last nested "type" field.

PhaseState/handler state propagation now preserves known ProgramJSON token
StringBox values instead of routing them through "" + map_get at state seams.

LoopStmtHandler now has a scalar state-value entry for the Loop positive path,
so the Loop handler does not depend on a broad state MapBox argument for the
minimal Local -> Loop route.

Loop token comparisons use a local token equality helper for ProgramJSON
identifier tokens.

PhaseStateBox.parse no longer stringifies the ProgramJSON input at entry.

Rust map value metadata now keeps per-literal-key value facts even when the
broad MapBox value type is heterogeneous.

BoxHelpers.map_get/2 now declares its receiver as MapBox, so the helper's own
internal get route is a MapBox read instead of an untyped RuntimeData fallback.

Generic method origin publication now treats MirType::String as StringBox for
handle-origin purposes, and receiver origin lookup now checks metadata value
types before requiring an origin instruction. This is intentionally kept in
generic method route metadata, not in scanner nullable bridges or C by-name
special cases.
```

Observed green probes:

```text
ProgramJsonV0PhaseStateBox.parse(row0 positive fixture) = err=0, next_idx=396
LocalStmtHandler positive consume = err=0
Loop cond lhs mismatch blocker = closed for direct PhaseState parse
RecipeItemBox.kind_of/1 route is string_handle, and BoxHelpers.map_get/2 uses
MapBox get metadata in the inspected heavy probe.
```

Remaining blocker:

```text
ProgramJsonLoopRecipeDtoSnapshotBox.build_summary/1 still returns
snapshot_kind=LoopRecipeDtoSnapshotV1;err=1;reason=parse_error when called
through the heavy EXE route.

The intermediate recipe_root_not_seq failure is closed. The heavy EXE readiness
gate now classifies the exact first blocker as
phase_state_parse_runtime_parse_error, while the route metadata guards remain
green:

  phase_state_parse_route = DirectAbi
  phase_state_parse_return_shape = map_handle
  snapshot_route = DirectAbi
  snapshot_return_shape = string_handle

This points back at the PhaseState parse runtime path under the heavy EXE
caller, not at RecipeItem kind token comparison and not at ProgramJSON scanner
result-map returns.
```

Next owner to inspect:

```text
PhaseState parse heavy EXE runtime path after MapBox/string token route cleanup:
  ProgramJsonV0PhaseStateBox.parse/2
  ProgramJsonV0PhaseStateConsumerBox.consume_next_state/5
  lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  src/mir/global_call_route_plan/value_type_publish.rs

Keep generic void object return widening = 0.
Keep scanner nullable bridge = 0.
```

## Required Guards

```bash
bash tools/checks/hako_programjson_scanner_result_map_return_contract_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_phase_state_aot_call_blocker_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_heavy_exe_readiness_gate.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
full_recipe_matcher_execution = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
route_selection_migration = 0
runtime_route_switch = 0
new_backend_route = 0
new_abi = 0
scanner_nullable_bridge = 0
```
