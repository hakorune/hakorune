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

## 2026-07-06 Cleanup Phase Split

Claude/worker review found that the same root issue appears in three places:
`.hako` helper contracts, Rust route metadata publication, and C `.inc`
mirrors. Keep these as separate BoxShape phases; do not mix them with new
ProgramJSON acceptance rows.

Phase A - helper route metadata hygiene (this card):

```text
normalize collection receiver contracts already used by ProgramJSON helpers
protect polymorphic predicate helpers from single-observation type publication
keep array_len/array_get untyped until caller inventory proves receiver narrowing
keep map_set untyped until its null-creates-new-MapBox contract is audited
```

Phase B - C `.inc` responsibility cleanup (next BoxShape series):

```text
extract DirectArray metadata out of lowering_plan_metadata.inc
move ORG_* enum/state ownership into compiler_state.inc
table-drive pure_compile exact_seed dispatch
deduplicate same_module method view registry against generated route registry
```

Phase C - token/origin preservation follow-up:

```text
audit remaining ORG_STRING/StringBox mirrors
audit receiver contract strings baked into map/direct metadata includes
add guards only where they prevent drift; do not add another docs-only loop
```

Acceptance for Phase A:

```text
BoxHelpers.map_get/2 remains MapBox receiver
BoxHelpers.array_len/1 and array_get/2 are selected as the next small receiver
annotation slice; they are early-return array readers like map_get/2
BoxHelpers.map_set/3 is split into a typed mutation helper while the old
nullable-builder behavior is named BoxHelpers.map_put_or_new/3
BoxHelpers.is_map/1 and is_array/1 are polymorphic predicate inputs in Rust
route metadata policy
BoxTypeInspectorBox predicate publication remains parked until route/caller
inventory proves it does not regress the heavy EXE path

no scanner nullable bridge
no generic void object return widening
no new backend route or ABI
```

Phase A.1 - Array receiver annotation:

```text
card type: BoxShape-only helper contract cleanup
owner file:
  lang/src/shared/common/box_helpers.hako

annotate:
  BoxHelpers.array_len(arr: ArrayBox)
  BoxHelpers.array_get(arr: ArrayBox, idx)

split:
  BoxHelpers.map_set(obj: MapBox, key, val)
  BoxHelpers.map_put_or_new(obj, key, val)

why:
  array_len/array_get are read helpers with null early-return behavior,
  matching the already-accepted map_get receiver contract pattern.
  map_set is now the typed mutation helper. map_put_or_new is the explicit
  nullable builder helper for callers that want null -> new MapBox behavior.
```

Acceptance for Phase A.1:

```text
box_helpers.hako MIR verify stays green
current AOT route value-type publication contract stays green
Layer4 heavy EXE readiness keeps exact_first_blocker =
  phase_state_parse_runtime_parse_error
map_set is typed as MapBox receiver
map_put_or_new carries the nullable-builder contract
no nullable scanner bridge is introduced
no source_selfhost, lowering, mutation, route-selection, ABI, or backend claims
```

Cleanliness backlog after Phase A.1/B.1:

```text
1. done: pure_compile exact-seed dispatcher extraction
2. done-for-structure: ORG_* enum and GenericPure state exposure extraction
3. done: same-module method view registry drift guard against generated route rows
4. map lookup fusion route-shape vs site/register matcher split
5. BoxTypeInspectorBox predicate publication inventory
6. shared same-token helper pilot
```

Phase B.4 - same-module method view registry drift guard:

```text
status: done
guard:
  tools/checks/hako_aot_same_module_method_view_registry_drift_guard.sh

boundary:
  generated generic method route registry remains the route tuple source
  same-module static rules may keep receiver/value-shape matching details
  guard compares route_id/core_op/route_kind/helper/proof/tier only

non-claims:
  no route-family unification
  no same-module route acceptance change
  no ABI, lowering, backend, or Source Selfhost claim
```

Phase B.2 - pure_compile exact-seed dispatcher extraction:

```text
status: done
owner file:
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
new include:
  lang/c-abi/shims/hako_llvmc_ffi_exact_seed_backend_route_dispatch.inc

boundary:
  registry remains the only acceptance table for tag/source/proof tuples
  dispatcher maps an already-accepted tag to the existing seed consumer
  compile_json_compat_pure no longer carries the 12-branch seed dispatcher

non-claims:
  no supported exact-seed tags added
  no route acceptance, fallback, ABI, lowering, or Source Selfhost claim
```

Phase B.3 - ORG vocabulary and generic pure state exposure extraction:

```text
status: done-for-structure
owner file:
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
new include:
  lang/c-abi/shims/hako_llvmc_ffi_origin_kind_vocabulary.inc
  lang/c-abi/shims/hako_llvmc_ffi_generic_pure_lowering_state.inc

done:
  ORG_* enum values are moved to one include with integer values unchanged
  GenericPureFunctionLoweringState and its historical macro exposure are moved
    to one include with macro names and reset defaults unchanged

still pending:
  ORG_STRING -> ORG_STRINGBOX naming decision is not part of this slice

non-claims:
  no origin semantics, route acceptance, ABI, lowering, or Source Selfhost claim
```

Phase B.1 - DirectArray metadata include extraction:

```text
card type: BoxShape-only mechanical refactor
owner file:
  lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
new include:
  lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_direct_array_access_metadata.inc
  lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_view_types.inc
  lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_extern_call_metadata.inc

move only:
  lowering-plan view struct declarations
  LoweringPlanDirectArrayAccessView
  read_lowering_plan_direct_array_access_view
  lowering_plan_direct_array_access_view_* predicates
  lowering_plan_direct_array_access_site_* matchers
  LoweringPlanExternCallView read/match/result-origin helpers

do not change:
  route metadata fields
  accepted DirectArray routes
  fallback policy
  receiver/value/result register matching
  generic method, same-module, extern, or user-box metadata
```

Acceptance for Phase B.1:

```text
lowering_plan_metadata.inc keeps the common tier/view helpers and includes the
DirectArray-specific file after common site lookup helpers are defined
the new includes are below the 800-line source limit
lowering_plan_metadata.inc is reduced below the 800-line source limit
all existing DirectArray call sites continue to call the same function names
no source_selfhost, lowering, mutation, route-selection, ABI, or backend claims
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
