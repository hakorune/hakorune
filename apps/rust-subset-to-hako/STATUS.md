# rust-subset-to-hako Status

Status: v0 embedded-fixture, FileBox-input, and adapter-fixture `.hako` converter paths pass EXE/AOT parity.

## Current State

The reference converter is complete enough for v0 fixtures:

```bash
python3 apps/rust-subset-to-hako/selftest.py
```

The native `.hako` converter wrapper exists:

```text
apps/rust-subset-to-hako/convert.hako
apps/rust-subset-to-hako/convert_file.hako
apps/rust-subset-to-hako/convert_adapter_fixture.hako
```

The converter core is separate from the current input route:

```text
apps/rust-subset-to-hako/converter_core.hako
```

It uses the `.hako` JSON library:

```text
apps/lib/json_native/parser/parser.hako
```

The external Rust parser adapter boundary is documented here:

```text
apps/rust-subset-to-hako/schema/external-adapter-boundary-v0.md
```

## Verified Working

```text
python_reference_selftest=ok
json_native_type_reserved_word_blocker=fixed
hako_json_probe_mir_json_emit=ok
hako_converter_mir_json_emit=ok
json_native_probe_exe=ok
hako_converter_exe=ok
hako_converter_parity=ok
embedded_handoff_core_separated=ok
filebox_minimal_exe_aot=ok
file_input_converter_parity=ok
adapter_fixture_handoff_parity=ok
else_if_fixture_parity=ok
returnless_void_body_fixture_parity=ok
vec_method_fixture_parity=ok
loop_without_break_fixture_parity=ok
```

## Current Scope Boundary

The current accepted slice uses a host-generated embedded fixture module:

```text
apps/rust-subset-to-hako/fixtures/simple_subset_embedded.hako
```

`convert.hako` is a startup wrapper that passes this embedded JSON to
`converter_core.hako`. `convert_file.hako` reads the same JSON from disk through
FileBox. `convert_adapter_fixture.hako` reads a host-produced adapter fixture
from disk. All wrappers verify generated output against checked-in expected
`.hako` files.

```text
route=EXE/AOT
vm_product_route=retired
file_input_enabled=1
file_input_probe_status=exe_aot_green
stdin_input_enabled=0
external_rust_parser_adapter_enabled=0
external_adapter_boundary=accepted
adapter_fixture_handoff=exe_aot_green
filebox_probe_status=exe_aot_green
```

Stdin input and external adapter invocation remain outside this row. The
current acceptance proves the app-front core path:

```text
embedded RustSubset JSON -> json_native -> JsonNode traversal -> .hako skeleton
```

`json_native` now uses generic object-key entry-table equality for
scanner-derived dynamic keys. The former RustSubset critical-key dictionary is
retired, and converter call sites do not carry schema-specific lookup logic.

`json_native` no longer uses a numeric token materialization seam. NUMBER token
payloads remain transport/diagnostic text, while `JsonParser.parse_number()`
constructs semantic integer values by scanning `source_text[token.start..end]`
directly.

## Task Board

```text
closed:
  JSON-NATIVE-SCHEMA-KEY-BRIDGE-CLOSEOUT-001
    result=temporary bridge isolated to json_native object-key context

  RUST-SUBSET-TO-HAKO-EMBEDDED-HANDOFF-001
    result=host-generated embedded fixture drives converter core on EXE/AOT

  RUST-SUBSET-PROBES-LAYOUT-001
    result=probes split into stable/regression/investigations

  RUST-SUBSET-SCHEMA-NORMALIZER-PROBE-001
    result=status-code schema helper probe is green

  JSON-NATIVE-KEY-MATERIALIZATION-001
    result=generic unknown-key fallback is green
    caveat=closed by object-key equality fix; critical-key bridge retired in 296x-1275

  SCHEMA-BOOL-HELPER-SHAPE-ACCEPTANCE-001
    result=bool-returning schema helper + not call-site shape is green
    evidence=probes/regression/schema_bool_shape_probe.hako

  BOOL-RETURN-CALL-BRANCH-NORMALIZATION-001
    result=user/global bool-return calls are normalized from ABI i64 to i1 before branch/not use
    owner=C ABI global call lowering / target_return_type contract
    evidence=probes/regression/bool_return_call_branch_probe.hako
    card=docs/development/current/main/phases/phase-296x/296x-1230-BOOL-RETURN-CALL-BRANCH-NORMALIZATION-001.md

  FILEBOX-EXE-AOT-MINIMAL-PROBE-001
    result=minimal FileBox new/open/read/close is green on EXE/AOT
    evidence=probes/regression/filebox_read_probe.hako

  RUST-SUBSET-TO-HAKO-FILE-INPUT-001
    result=converter FileBox input wrapper is green on EXE/AOT
    prerequisite=FILEBOX-EXE-AOT-MINIMAL-PROBE-001 green
    evidence=convert_file.hako + smoke.sh file converter parity

  RUST-SUBSET-EXTERNAL-ADAPTER-BOUNDARY-001
    result=external Rust parser adapter handoff boundary is documented
    owner=app-front boundary / producer contract
    evidence=schema/external-adapter-boundary-v0.md

  RUST-SUBSET-ADAPTER-FIXTURE-HANDOFF-PROBE-001
    result=host-produced adapter fixture JSON is validated through FileBox route
    owner=file handoff acceptance
    evidence=convert_adapter_fixture.hako + smoke.sh adapter fixture parity

  JSON-NATIVE-NONZERO-NUMBER-PARSE-HARDENING-001
    result=nonzero JSON integer parsing is stable on EXE/AOT for accepted v0 fixture values
    owner=json_native number token/value materialization
    evidence=probes/regression/json_nonzero_number_probe.hako + adapter fixture value+1 parity
    caveat=number materializer remains temporary until numeric value conversion owner is fixed
    card=docs/development/current/main/phases/phase-296x/296x-1231-JSON-NATIVE-NONZERO-NUMBER-PARSE-HARDENING-001.md

  RUST-SUBSET-ADAPTER-TOOL-SELECTION-001
    result=syn selected and scaffolded as the first host-side RustSubset JSON producer
    owner=external adapter boundary
    evidence=tools/syn_adapter + examples/adapter_fixture_input.rs
    optional_gate=RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
    card=docs/development/current/main/phases/phase-296x/296x-1232-RUST-SUBSET-ADAPTER-TOOL-SELECTION-001.md

  RUST-SUBSET-SYN-ADAPTER-COVERAGE-SELECTION-001
    result=tail-expression returns are normalized to RustSubset Return
    owner=syn adapter statement lowering
    evidence=simple_input.rs semantic parity + optional adapter smoke
    card=docs/development/current/main/phases/phase-296x/296x-1233-RUST-SUBSET-SYN-ADAPTER-COVERAGE-SELECTION-001.md

  RUST-SUBSET-SYN-ADAPTER-UNSUPPORTED-SHAPE-PROBE-001
    result=unsupported Rust trait item is explicit Unsupported handoff and converter TODO comment
    owner=syn adapter unsupported item lowering / converter Unsupported emission
    evidence=unsupported_trait_input.rs + unsupported_trait_expected.hako + optional adapter smoke
    card=docs/development/current/main/phases/phase-296x/296x-1234-RUST-SUBSET-SYN-ADAPTER-UNSUPPORTED-SHAPE-PROBE-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-001
    result=If statement selected and implemented as the next supported RustSubset source shape
    owner=RustSubset statement schema / converter emit / syn adapter lowering
    evidence=if_input.rs + if_subset.json + if_expected.hako + convert_if_fixture.hako
    card=docs/development/current/main/phases/phase-296x/296x-1235-RUST-SUBSET-SYN-ADAPTER-IF-STATEMENT-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-002
    result=Assign statement selected and implemented as the next supported RustSubset source shape
    owner=RustSubset statement schema / converter emit / syn adapter lowering
    evidence=assign_input.rs + assign_subset.json + assign_expected.hako + convert_assign_fixture.hako
    card=docs/development/current/main/phases/phase-296x/296x-1236-RUST-SUBSET-SYN-ADAPTER-ASSIGN-STATEMENT-001.md

  RUST-SUBSET-SYN-ADAPTER-MODULE-SPLIT-001
    result=tools/syn_adapter/src/main.rs split into cli/items/functions/stmts/exprs/types modules
    owner=syn adapter structure
    behavior_changed=0
    evidence=assign fixture JSON SHA unchanged + full adapter smoke green
    card=docs/development/current/main/phases/phase-296x/296x-1238-RUST-SUBSET-SYN-ADAPTER-MODULE-SPLIT-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-003
    result=While statement selected and implemented as the next supported RustSubset source shape
    owner=RustSubset statement schema / converter emit / syn adapter lowering
    evidence=while_input.rs + while_subset.json + while_expected.hako + convert_while_fixture.hako
    caveat=break/continue remain compiler backlog, not app-front support
    card=docs/development/current/main/phases/phase-296x/296x-1239-RUST-SUBSET-SYN-ADAPTER-WHILE-STATEMENT-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-004
    result=Vec literal selected and implemented as RustSubset ArrayLiteral transport
    owner=RustSubset expression schema / converter emit / syn adapter lowering
    evidence=vec_input.rs + vec_subset.json + vec_expected.hako + convert_vec_fixture.hako
    caveat=typed Array semantics remain owned by the Hakorune compiler
    card=docs/development/current/main/phases/phase-296x/296x-1240-RUST-SUBSET-SYN-ADAPTER-VEC-LITERAL-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
    result=Else-if selected and implemented as recursive RustSubset If in the parent else array
    owner=RustSubset statement schema / converter emit / syn adapter lowering
    evidence=else_if_input.rs + else_if_subset.json + else_if_expected.hako + convert_else_if_fixture.hako
    caveat=break/continue remain compiler backlog, not app-front support
    card=docs/development/current/main/phases/phase-296x/296x-1265-RUST-SUBSET-SYN-ADAPTER-ELSE-IF-STATEMENT-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-006
    result=Returnless void function body hardening selected and fixture-guarded
    owner=RustSubset function return_type contract / syn adapter fixture / converter emit
    evidence=void_body_input.rs + void_body_subset.json + void_body_expected.hako + convert_void_body_fixture.hako
    converter_core_changed=0
    card=docs/development/current/main/phases/phase-296x/296x-1266-RUST-SUBSET-SYN-ADAPTER-RETURNLESS-VOID-BODY-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-007
    result=Vec method calls selected and fixture-guarded through existing MethodCall schema
    owner=RustSubset MethodCall schema / syn adapter fixture / converter emit
    evidence=vec_method_input.rs + vec_method_subset.json + vec_method_expected.hako + convert_vec_method_fixture.hako
    schema_node_added=0
    converter_core_changed=0
    card=docs/development/current/main/phases/phase-296x/296x-1267-RUST-SUBSET-SYN-ADAPTER-VEC-METHOD-CALLS-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-008
    result=Loop without break selected and implemented through existing While schema with cond=true
    owner=RustSubset While schema / syn adapter loop lowering / converter emit
    evidence=loop_forever_input.rs + loop_forever_subset.json + loop_forever_expected.hako + convert_loop_forever_fixture.hako
    break_continue_supported=0
    compiler_recipe_acceptance_changed=0
    card=docs/development/current/main/phases/phase-296x/296x-1268-RUST-SUBSET-SYN-ADAPTER-LOOP-WITHOUT-BREAK-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-009
    result=match unsupported handoff selected and fixture-guarded
    owner=syn adapter Unsupported expression handoff / converter Unsupported.reason emission
    evidence=match_unsupported_input.rs + match_unsupported_subset.json + match_unsupported_expected.hako + convert_match_unsupported_fixture.hako
    schema_node_added=0
    match_semantics_enabled=0
    card=docs/development/current/main/phases/phase-296x/296x-1270-RUST-SUBSET-SYN-ADAPTER-MATCH-UNSUPPORTED-HANDOFF-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-010
    result=returnless typed unit function selected and fixture-guarded
    owner=RustSubset function return_type contract / existing syn adapter unit tuple type mapping / converter emit
    evidence=unit_return_input.rs + unit_return_subset.json + unit_return_expected.hako + convert_unit_return_fixture.hako
    schema_node_added=0
    adapter_code_changed=0
    card=docs/development/current/main/phases/phase-296x/296x-1271-RUST-SUBSET-SYN-ADAPTER-EXPLICIT-UNIT-RETURN-001.md

  RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-011
    result=for-loop unsupported handoff selected and fixture-guarded
    owner=syn adapter Unsupported expression handoff / tail-expression normalization guard / converter Unsupported.reason emission
    evidence=for_loop_unsupported_input.rs + for_loop_unsupported_subset.json + for_loop_unsupported_expected.hako + convert_for_loop_unsupported_fixture.hako
    schema_node_added=0
    iterator_semantics_enabled=0
    while_desugar_enabled=0
    card=docs/development/current/main/phases/phase-296x/296x-1272-RUST-SUBSET-SYN-ADAPTER-FOR-LOOP-UNSUPPORTED-HANDOFF-001.md

active_next:
  COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
    goal=capture the next real compiler acceptance candidate and reduce it to a minimal failing fixture
    active_next=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
    source_candidates=continue_inside_staged_loop,read_next_number_literal_full_shape,nested_break_continue,loop_carried_phi_scanner_shape,return_break_continue_interaction
    prerequisite=json_native object-key equality hardening closed by 296x-1275
    implementation_allowed=0
    status=active_next
    latest_taskboard=docs/development/current/main/phases/phase-296x/296x-1282-COREPLAN-REAL-SHAPE-TASKBOARD-REFRESH-003.md
    note=RustSubset while and Vec literal transport rows are already closed; remaining work is compiler Recipe/CorePlan loop-exit acceptance. read_next_number_literal stays taskized, but the immediate failing fixture is partial-carrier continue PHI.

hardening_queue:
  JSON-NATIVE-NUMBER-TOKEN-PAYLOAD-STABILITY-001
    result=app-level candidate fixes inventoried; final keeper came from ArrayBox element origin + backend result_origin_box consumption
    prerequisite=historical investigation probe json_number_scanner_payload_probe.hako exposed scanner-derived NUMBER payload instability
    nonkeeper_candidates=parser source-span recovery, JsonToken primitive sidecar, JsonNode source-span/object sidecar, char-by-char materializer
    owner_family=json_tokenizer_number_production_shape
    direct_json_token_dynamic_payload=green
    scanner_read_number_substring=green
    tokenizer_next_token_number=green
    tokenizer_number_probe_compile=unsupported_pure_shape
    tokenizer_number_value_only_compile=unsupported_pure_shape
    selected_owner=arraybox_element_origin_metadata_propagation
    owner_selection=JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001
    element_origin_shadow=green
    element_origin_shadow_card=docs/development/current/main/phases/phase-296x/296x-1254-JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ORIGIN-SHADOW-001.md
    backend_element_origin_consumer=green
    backend_element_origin_consumer_card=docs/development/current/main/phases/phase-296x/296x-1255-BACKEND-ARRAYBOX-ELEMENT-ORIGIN-CONSUMER-001.md
    tokenizer_number_payload_storage_probe=regression_green
    tokenizer_number_payload_storage_regression_card=docs/development/current/main/phases/phase-296x/296x-1257-JSON-NATIVE-TOKENIZER-NUMBER-PAYLOAD-STORAGE-REGRESSION-001.md
    number_materializer_retired=1
    number_materializer_retire_card=docs/development/current/main/phases/phase-296x/296x-1258-JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001.md
    number_materializer_retire_result=accepted_after_source_span_numeric_parse
    numeric_span_parse_card=docs/development/current/main/phases/phase-296x/296x-1261-JSON-NATIVE-NUMERIC-SPAN-PARSE-MATERIALIZER-RETIRE-001.md
    next_task=JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
    cards=docs/development/current/main/phases/phase-296x/296x-1245-JSON-NATIVE-NUMBER-TOKEN-PAYLOAD-STABILITY-INVENTORY-001.md,docs/development/current/main/phases/phase-296x/296x-1246-JSON-NATIVE-NUMBER-PAYLOAD-OWNER-SELECTION-001.md,docs/development/current/main/phases/phase-296x/296x-1247-JSON-NATIVE-TOKEN-TEXT-PAYLOAD-STORAGE-PROBE-001.md,docs/development/current/main/phases/phase-296x/296x-1248-TOKENIZER-NUMBER-PRODUCTION-SHAPE-TASKIZATION-001.md,docs/development/current/main/phases/phase-296x/296x-1249-JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001.md,docs/development/current/main/phases/phase-296x/296x-1250-JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001.md,docs/development/current/main/phases/phase-296x/296x-1251-JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001.md,docs/development/current/main/phases/phase-296x/296x-1252-JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001.md,docs/development/current/main/phases/phase-296x/296x-1254-JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ORIGIN-SHADOW-001.md,docs/development/current/main/phases/phase-296x/296x-1255-BACKEND-ARRAYBOX-ELEMENT-ORIGIN-CONSUMER-001.md,docs/development/current/main/phases/phase-296x/296x-1257-JSON-NATIVE-TOKENIZER-NUMBER-PAYLOAD-STORAGE-REGRESSION-001.md,docs/development/current/main/phases/phase-296x/296x-1258-JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001.md,docs/development/current/main/phases/phase-296x/296x-1261-JSON-NATIVE-NUMERIC-SPAN-PARSE-MATERIALIZER-RETIRE-001.md

  JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-001
    result=accepted in 296x-1275 after object-key equality fix
    prerequisite=kind/name/items dynamic key lookup green without materializer
    evidence=probes/regression/json_object_same_length_key_lookup_probe.hako + full smoke with regression green
    retire_allowed=1
    historical_reject_card=docs/development/current/main/phases/phase-296x/296x-1262-JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-PROBE-001.md
    retry_card=docs/development/current/main/phases/phase-296x/296x-1263-JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-001.md
    accepted_card=docs/development/current/main/phases/phase-296x/296x-1275-JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001.md

  JSON-NATIVE-CRITICAL-STRING-VALUE-PUBLICATION-PROBE-001
    result=replaced; remaining failure owner is object-key equality false positive, not string value publication
    prerequisite=JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-PROBE-001
    implementation_allowed=0
    status=closed_by_owner_reclassification

  JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
    result=selected and fixed JsonNodeInstance.object_key_equals same-length dynamic-key false positive
    prerequisite=JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-001 rejected by full smoke
    evidence=probes/investigations/json_object_same_length_pair_lookup_probe.hako reproduced kind/name false positive; probes/regression/json_object_same_length_key_lookup_probe.hako guards fix
    json_object_key_materializer_removed=1
    full_smoke_with_regression_green=1
    status=closed
    card=docs/development/current/main/phases/phase-296x/296x-1275-JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001.md

  JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001
    result=accepted in 296x-1261 after parser source-span integer conversion removed the adapter regression
    prerequisite=JsonToken NUMBER dynamic payload survives tokenize()->ArrayBox on EXE/AOT regression
    evidence=probes/regression/json_tokenizer_number_payload_storage_probe.hako + regression smoke
    historical_reject_card=docs/development/current/main/phases/phase-296x/296x-1258-JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001.md
    accepted_card=docs/development/current/main/phases/phase-296x/296x-1261-JSON-NATIVE-NUMERIC-SPAN-PARSE-MATERIALIZER-RETIRE-001.md

  JSON-NATIVE-NUMERIC-VALUE-CONVERSION-OWNER-SELECTION-001
    result=resolved by JsonParser source-span integer conversion
    prerequisite=number payload storage regression is green
    evidence=probes/investigations/json_object_number_node_publication_probe.hako green; regression smoke green
    selected_owner=jsonparser_source_span_integer_parse
    card=docs/development/current/main/phases/phase-296x/296x-1259-JSON-NATIVE-NUMERIC-VALUE-CONVERSION-OWNER-SELECTION-001.md
    resolution_card=docs/development/current/main/phases/phase-296x/296x-1261-JSON-NATIVE-NUMERIC-SPAN-PARSE-MATERIALIZER-RETIRE-001.md

  JSON-NATIVE-INT-TEXT-VALUE-OBJECT-PUBLICATION-PROBE-001
    result=reclassified; JsonNode int sidecar is not needed after parser source-span integer conversion
    prerequisite=JSON-NATIVE-NUMERIC-VALUE-CONVERSION-OWNER-SELECTION-001
    implementation_allowed=0
    status=closed_by_296x_1261

  FFI-SMOKE-SERIALIZATION-GUARD-001
    goal=prevent parallel smoke/regression rebuilds from corrupting libhako_llvmc_ffi.so
    current_policy=run smoke/regression sequentially

compiler_backlog:
  COREPLAN-LOOP-BREAK-MULTI-STAGE-RECIPE-ACCEPTANCE-001
    goal=accept read_next_number_literal-style staged loop with break through Recipe/CorePlan
    source_case=json_native read_next_number_literal scanner loop
    boundary=compiler acceptance, not rust-subset app converter
    note=previous WIP reverted to token payload route to keep EXE/AOT stable
    taskization_card=docs/development/current/main/phases/phase-296x/296x-1241-COREPLAN-LOOP-BREAK-RECIPE-BACKLOG-TASKIZATION-001.md

  COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001
    result=minimal staged loop + conditional break compiler canary captured
    fixture=apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
    default_exe_aot=green
    planner_required_claim=0
    json_native_route_changed=0
    card=docs/development/current/main/phases/phase-296x/296x-1242-COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001.md

  COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001
    result=captured canary is planner_required green through existing LoopSimpleWhile + flowbox/adopt break route
    prerequisite=COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001
    new_recipe_acceptance_required=0
    evidence=phase29bq_fast_gate_cases.tsv selfhost_read_next_number_literal_staged_loop_break_min
    card=docs/development/current/main/phases/phase-296x/296x-1243-COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001.md

  COREPLAN-RECURSIVE-RECIPE-SHAPE-BACKLOG-AUDIT-001
    result=recursive Recipe/CorePlan remains the direction for future compiler acceptance, but the captured read_next_number_literal canary is green
    implementation_allowed=0
    next_when_reopened=COREPLAN-RECURSIVE-RECIPE-MINIMAL-FAILING-FIXTURE-SELECTION-001
    known_compiler_shape_backlog=continue,nested_break_continue,multi_exit_scanner_loop,loop_carried_phi,return_break_continue_interaction
    card=docs/development/current/main/phases/phase-296x/296x-1253-COREPLAN-RECURSIVE-RECIPE-SHAPE-BACKLOG-AUDIT-001.md

  COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKIZATION-001
    result=reported read_next_number_literal multi-stage loop/break shape is taskized as real-shape intake, not immediate implementation
    implementation_allowed=0
    next_compiler_task=COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
    requires_minimal_failing_fixture=1
    other_unsupported_shapes_bucketed=1
    card=docs/development/current/main/phases/phase-296x/296x-1256-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKIZATION-001.md

  COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-001
    result=recursive Recipe remains the compiler-side direction, but implementation still requires a minimal failing fixture
    current_staged_loop_break_canary_green=1
    implementation_allowed=0
    compiler_recipe_queue=read_next_number_literal_full_shape,continue_inside_staged_loop,nested_break_continue,loop_carried_phi_scanner_shape,return_break_continue_interaction,multi_exit_scanner_loop
    rust_subset_app_front_queue=else_if,returnless_void_body,vec_method_calls,match_unsupported_handoff,trait_generic_unsupported_handoff
    json_native_hardening_queue=numeric_value_conversion_owner,number_materializer_retire,critical_key_bridge_retire
    current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
    card=docs/development/current/main/phases/phase-296x/296x-1260-COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-001.md

  COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-001
    result=read_next_number_literal full shape and adjacent unsupported compiler shapes are refreshed as compiler-only tasks
    read_next_number_literal_full_shape_taskized=1
    minimal_failing_fixture_required=1
    implementation_allowed=0
    next_compiler_task=COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
    current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
    card=docs/development/current/main/phases/phase-296x/296x-1264-COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-001.md

  COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-002
    result=read_next_number_literal remains compiler Recipe/CorePlan intake, while token payload and nonzero number stability stay json_native hardening concerns
    read_next_number_literal_full_shape_taskized=1
    recursive_recipe_direction_preserved=1
    implementation_allowed=0
    minimal_failing_fixture_required=1
    token_payload_route_is_json_native_stability_only=1
    nonzero_number_semantics_compiler_owner=0
    compiler_recipe_queue=read_next_number_literal_full_shape,continue_inside_staged_loop,nested_break_continue,loop_carried_phi_scanner_shape,return_break_continue_interaction,multi_exit_scanner_loop
    rust_subset_unsupported_handoff_queue=trait_generic_item_support,match_semantics,for_loop_semantics
    active_next_json_native_task=resolved_by_296x_1275
    card=docs/development/current/main/phases/phase-296x/296x-1274-COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-002.md

  COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
    goal=capture the next real compiler acceptance candidate and reduce it to a minimal failing fixture
    active_next=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
    source_candidates=read_number_sign_break_fullish,read_number_decimal_exponent,scanner_multi_exit,continue_inside_staged_loop,nested_break_continue,loop_carried_phi_scanner_shape,return_break_continue_interaction
    prerequisite=json_native object-key equality hardening closed by 296x-1275
    status=active_next
    latest_taskboard=docs/development/current/main/phases/phase-296x/296x-1276-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001.md

  COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001
    result=read_next_number_literal is split into a concrete compiler shape ladder
    active_next=COREPLAN-REAL-SHAPE-FIXTURE-SIGN-BREAK-001
    existing_staged_loop_canary_green=1
    while_app_front_transport_closed=1
    vec_literal_app_front_transport_closed=1
    implementation_allowed=0
    minimal_failing_fixture_required=1
    card=docs/development/current/main/phases/phase-296x/296x-1276-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001.md

  COREPLAN-REAL-SHAPE-FIXTURE-SIGN-BREAK-001
    result=optional sign + conditional break + loop-carried accumulator fixture is planner_required green
    fixture=apps/tests/phase29bq_selfhost_blocker_read_number_sign_break_fullish_min.hako
    gate_case=selfhost_read_number_sign_break_fullish_min
    new_recipe_acceptance_required=0
    implementation_allowed=0
    next_task=COREPLAN-REAL-SHAPE-FIXTURE-DECIMAL-EXPONENT-001
    card=docs/development/current/main/phases/phase-296x/296x-1277-COREPLAN-REAL-SHAPE-FIXTURE-SIGN-BREAK-001.md

  COREPLAN-REAL-SHAPE-FIXTURE-DECIMAL-EXPONENT-001
    result=decimal/exponent staged scanner loops are planner_required green
    fixture=apps/tests/phase29bq_selfhost_blocker_read_number_decimal_exponent_min.hako
    gate_case=selfhost_read_number_decimal_exponent_min
    new_recipe_acceptance_required=0
    implementation_allowed=0
    next_task=COREPLAN-REAL-SHAPE-FIXTURE-MULTI-EXIT-001
    card=docs/development/current/main/phases/phase-296x/296x-1278-COREPLAN-REAL-SHAPE-FIXTURE-DECIMAL-EXPONENT-001.md

  COREPLAN-REAL-SHAPE-FIXTURE-MULTI-EXIT-001
    result=scanner multi-exit fixture captured as expected-fail contract
    fixture=apps/tests/phase29bq_selfhost_blocker_scanner_multi_exit_min.hako
    gate_case=selfhost_scanner_multi_exit_min
    expected_fail_contract_green=1
    selected_owner=generic_loop_v1_recipe_body
    new_recipe_acceptance_required=1
    next_task=COREPLAN-GENERIC-LOOP-MULTI-EXIT-RECIPE-001
    card=docs/development/current/main/phases/phase-296x/296x-1279-COREPLAN-REAL-SHAPE-FIXTURE-MULTI-EXIT-001.md

  COREPLAN-GENERIC-LOOP-MULTI-EXIT-RECIPE-001
    result=scanner multi-exit fixture is accepted by generic_loop_v1 Recipe/CorePlan
    fixture=apps/tests/phase29bq_selfhost_blocker_scanner_multi_exit_min.hako
    gate_case=selfhost_scanner_multi_exit_min
    accepted_shape=IfMode::ThenOnlyExit
    new_method_name_branch=0
    next_task=COREPLAN-CONTINUE-IN-STAGED-LOOP-001
    card=docs/development/current/main/phases/phase-296x/296x-1280-COREPLAN-GENERIC-LOOP-MULTI-EXIT-RECIPE-001.md

  COREPLAN-CONTINUE-IN-STAGED-LOOP-FIXTURE-001
    result=staged scanner continue fixture captured as expected-fail contract
    fixture=apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
    gate_case=selfhost_read_number_continue_staged_min
    selected_owner=loop_cond_break_continue_partial_carrier_update
    failure_mode=mir/verify:dominator_violation
    next_task=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
    card=docs/development/current/main/phases/phase-296x/296x-1281-COREPLAN-CONTINUE-IN-STAGED-LOOP-FIXTURE-001.md

  COREPLAN-REAL-SHAPE-TASKBOARD-REFRESH-003
    result=read_next_number_literal family remains taskized, but active implementation is narrowed to the partial-carrier continue PHI fixture
    active_task=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
    queued_after_active=COREPLAN-READ-NEXT-NUMBER-LITERAL-MULTI-STAGE-LOOP-ACCEPTANCE-001
    already_closed_app_front_transport=while,vec_literal,vec_method_calls,loop_without_break
    unsupported_app_front_handoff_queue=match_semantics,for_loop_semantics,trait_generic_item_support,break_continue_transport_if_real_input_requires_it
    broad_recursive_recipe_rewrite_allowed=0
    card=docs/development/current/main/phases/phase-296x/296x-1282-COREPLAN-REAL-SHAPE-TASKBOARD-REFRESH-003.md

  COREPLAN-LOOP-BREAK-RECURSIVE-RECIPE-ACCEPTANCE-001
    status=not_needed_for_captured_canary
    goal=only reopen if json_native restore probe exposes a stronger failing shape
    prerequisite=COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001
    non_goal=read_next_number_literal by-name branch

  COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001
    result=json_native scanner read_number route is active; later token payload stability rows retired the number materializer
    prerequisite=COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001
    new_recipe_acceptance_required=0
    next_owner=json_native number token payload stability
    evidence=probes/investigations/json_number_scanner_payload_probe.hako expected-fails with Result: 2 for 123
    card=docs/development/current/main/phases/phase-296x/296x-1244-COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001.md

  TOKENIZER-NUMBER-PRODUCTION-SHAPE-TASKIZATION-001
    result=real scanner/tokenizer NUMBER production shape is taskized without blocking bool-return validation
    stability_route=token payload regression is active; small number materializer retired
    next_probe=JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001
    probe_ladder=scanner.read_number,next_token NUMBER,tokenize()->ArrayBox,compiler Recipe/CorePlan only if proved
    current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
    card=docs/development/current/main/phases/phase-296x/296x-1248-TOKENIZER-NUMBER-PRODUCTION-SHAPE-TASKIZATION-001.md

  JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001
    result=JsonScanner.read_number() direct substring route is EXE/AOT green
    scanner_read_number_owner=0
    evidence=probes/investigations/json_scanner_number_substring_probe.hako
    next_probe=JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001
    card=docs/development/current/main/phases/phase-296x/296x-1249-JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001.md

  JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001
    result=JsonTokenizer.next_token() NUMBER route is EXE/AOT green
    tokenizer_next_token_number_owner=0
    evidence=probes/investigations/json_tokenizer_next_token_number_probe.hako
    next_probe=JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001
    card=docs/development/current/main/phases/phase-296x/296x-1250-JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001.md

  JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001
    result=tokenize()->ArrayBox first reject is post-ArrayBox.get RuntimeDataBox.get_type/get_value route loss
    selected_owner=returned_token_array_element_route_recovery
    evidence=probes/investigations/json_tokenizer_number_payload_storage_probe.hako,probes/investigations/json_tokenizer_number_value_only_probe.hako
    next_probe=JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001
    card=docs/development/current/main/phases/phase-296x/296x-1251-JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001.md

  COREPLAN-CONTINUE-MULTI-STAGE-RECIPE-BACKLOG-001
    goal=park continue-in-loop acceptance until break fixture is accepted
    status=parked

  COREPLAN-LOOP-CARRIED-PHI-SCANNER-SHAPE-BACKLOG-001
    goal=park loop-carried PHI scanner shapes exposed by staged loops
    status=parked
```

Stop lines:

```text
do not replace json_native with a host JSON DLL
do not re-enable VM as the product app route
do not move RustSubset schema-specific lookup into converter callsites
do not run smoke/regression commands that rebuild libhako_llvmc_ffi.so in parallel
```

## Known Instability Inventory

Keep these out of stable acceptance until their probes are green:

```text
json_object_key_equality:
  status=regression_guarded
  owner=apps/lib/json_native/core/node.hako
  stable_policy=generic entry-table key equality; no RustSubset schema-key dictionary
  evidence=probes/regression/json_object_same_length_key_lookup_probe.hako

ffi_parallel_build:
  stable_policy=do not run smoke/regression commands that rebuild
    libhako_llvmc_ffi.so in parallel
  reason=parallel runs can leave target/release/libhako_llvmc_ffi.so invalid

number_token_payload:
  status=stable_for_token_transport
  stable_policy=NUMBER token payload is diagnostic/transport text; semantic integer conversion is parser source-span scanning
  evidence=probes/regression/json_tokenizer_number_payload_storage_probe.hako; probes/investigations/json_object_number_node_publication_probe.hako
  reason=JsonNumberTextMaterializer is retired; parser numeric semantics no longer depend on token payload parsing
```

Boundary SSOT:

```text
docs/development/current/main/design/hako-app-front-boundary-template-ssot.md
```

## Reproduction

```bash
bash apps/rust-subset-to-hako/smoke.sh
```
