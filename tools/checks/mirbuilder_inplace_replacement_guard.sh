#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-inplace-replacement-guard"; source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/joinir_logical_demand_contract.sh"; source "$ROOT_DIR/tools/checks/lib/joinir_loop_compile_candidate_scope.sh"; source "$ROOT_DIR/tools/checks/lib/loop_family_observation_contract.sh"
MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/mirbuilder-inplace-replacement-v1.tsv"
GENERIC_LEGACY_MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/generic-loop-legacy-disposition-v1.tsv"
GENERIC_LEGACY_GUARD="$ROOT_DIR/tools/checks/lib/generic_legacy_corpus_universe_guard.py"
CALLER_MANIFEST="$ROOT_DIR/tools/checks/manifests/raw_public_cutover_caller_manifest_v1.json"
STRUCTURAL_RATCHET="$ROOT_DIR/docs/development/current/main/design/fixtures/mirbuilder-structural-ratchet.tsv"
LOWERING="$ROOT_DIR/src/mir/builder/calls/lowering.rs"
PORT_OWNER="$ROOT_DIR/src/mir/builder/port_aware_function_draft_impl.rs"
MODULE_LIFECYCLE="$ROOT_DIR/src/mir/builder/module_lifecycle.rs"
PROGRAM_ROOT="$ROOT_DIR/src/mir/builder/program_root_lowering.rs"
COMPILER="$ROOT_DIR/src/mir/compiler/mod.rs"
MODULE_SESSION="$ROOT_DIR/src/mir/builder/module_invocation_session.rs"
NORMAL_PIPELINE="$ROOT_DIR/src/mir/compiler/normal_default_pipeline.rs"
NORMAL_ROOT_LIFECYCLE="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
RAW_CHILD_PORT="$ROOT_DIR/src/mir/builder/recursive_child_lowering.rs"
LOOP_ROUTING="$ROOT_DIR/src/mir/builder/control_flow/joinir/routing.rs"
CONTROL_FLOW_ROOT="$ROOT_DIR/src/mir/builder/control_flow/mod.rs"
STATEMENT_SURFACE="$ROOT_DIR/src/mir/builder/raw_expression_dispatch/statement_surface.rs"
LOCAL_DESCENT="$ROOT_DIR/src/mir/builder/stmts/local_statement_descent.rs"
VARIABLE_STMT="$ROOT_DIR/src/mir/builder/stmts/variable_stmt.rs"
LOCAL_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0.py"
RAW_DISPATCH="$ROOT_DIR/src/mir/builder/raw_expression_dispatch/mod.rs"
MATCH_OWNER="$ROOT_DIR/src/mir/builder/exprs_peek.rs"
BUILDER_BUILD="$ROOT_DIR/src/mir/builder/builder_build.rs"
ASSIGNMENT_DESCENT="$ROOT_DIR/src/mir/builder/stmts/variable_assignment_descent.rs"
ASSIGNMENT_TESTS="$ROOT_DIR/src/mir/builder/stmts/variable_assignment_descent_tests.rs"
ASSIGNMENT_PARITY_TESTS="$ROOT_DIR/src/mir/builder/stmts/variable_assignment_parity_tests.rs"
ASSIGNMENT_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment.py"
RETURN_DESCENT="$ROOT_DIR/src/mir/builder/stmts/return_statement_descent.rs"
RETURN_TESTS="$ROOT_DIR/src/mir/builder/stmts/return_statement_descent_tests.rs"
RETURN_PARITY_TESTS="$ROOT_DIR/src/mir/builder/stmts/return_statement_parity_tests.rs"
RETURN_OWNER="$ROOT_DIR/src/mir/builder/stmts/return_stmt.rs"
RETURN_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0_return.py"
OPS_ROOT="$ROOT_DIR/src/mir/builder/ops/mod.rs"
BINARY_DESCENT="$ROOT_DIR/src/mir/builder/ops/binary_expression_descent.rs"
BINARY_TESTS="$ROOT_DIR/src/mir/builder/ops/binary_expression_descent_tests.rs"
SHORT_CIRCUIT_DESCENT="$ROOT_DIR/src/mir/builder/ops/short_circuit_expression_descent.rs"
SHORT_CIRCUIT_TESTS="$ROOT_DIR/src/mir/builder/ops/short_circuit_expression_descent_tests.rs"
BINARY_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py"
METHOD_CALL_DESCENT="$ROOT_DIR/src/mir/builder/calls/method_call_descent.rs"
METHOD_CALL_TERMINAL="$ROOT_DIR/src/mir/builder/calls/method_call_terminal.rs"
CALLS_MOD="$ROOT_DIR/src/mir/builder/calls/mod.rs"
CALLS_BUILD="$ROOT_DIR/src/mir/builder/calls/build.rs"
FUNCTION_CALL_ROUTE="$ROOT_DIR/src/mir/builder/calls/function_call_preflight_route.rs"
FUNCTION_SPECIAL="$ROOT_DIR/src/mir/builder/calls/special_method_handlers.rs"
FASTMEM_CALLS="$ROOT_DIR/src/mir/builder/fastmem/calls.rs"; ENUM_MATCH="$ROOT_DIR/src/mir/builder/exprs_enum_match.rs"
SCOPEBOX_ENUM="$ROOT_DIR/src/mir/builder/enum_match_scopebox.rs"; METHOD_CALL_HANDLERS="$ROOT_DIR/src/mir/builder/method_call_handlers.rs"
PROPERTY_READS="$ROOT_DIR/src/mir/builder/property_reads.rs"
FIELDS="$ROOT_DIR/src/mir/builder/fields.rs"
PROPERTY_TESTS="$ROOT_DIR/src/tests/mir_unified_members_property_read.rs"
RECORD_HELPER="$ROOT_DIR/src/mir/builder/record_helper_args.rs"
RECORD_HELPER_TESTS="$ROOT_DIR/src/mir/builder/record_helper_args_tests.rs"
INDEXING="$ROOT_DIR/src/mir/builder/indexing.rs"; PRINT_STMT="$ROOT_DIR/src/mir/builder/stmts/print_stmt.rs"
METHOD_CALL_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py"
RECORD_HELPER_GUARD="$ROOT_DIR/tools/checks/impl/k2_wide_allocator_record_construction_read_guard.sh"
guard_exact_counts() {
  while IFS='|' read -r file pattern expected label; do
    count="$({ rg -o -P "$pattern" "$file" || true; } | wc -l | tr -d '[:space:]')"
    if [[ "$count" != "$expected" ]]; then
      guard_fail "$TAG" "$label count drift: count=$count expected=$expected"
    fi
  done
}
for command in awk find python3 rg sed wc xargs; do guard_require_command "$TAG" "$command"; done
guard_require_files \
  "$TAG" \
  "$MANIFEST" \
  "$GENERIC_LEGACY_MANIFEST" \
  "$GENERIC_LEGACY_GUARD" \
  "$CALLER_MANIFEST" \
  "$STRUCTURAL_RATCHET" \
  "$LOWERING" \
  "$PORT_OWNER" \
  "$MODULE_LIFECYCLE" \
  "$PROGRAM_ROOT" \
  "$COMPILER" \
  "$MODULE_SESSION" \
  "$NORMAL_PIPELINE" \
  "$NORMAL_ROOT_LIFECYCLE" \
  "$RAW_CHILD_PORT" \
  "$LOOP_ROUTING" \
  "$CONTROL_FLOW_ROOT" \
  "$STATEMENT_SURFACE" \
  "$LOCAL_DESCENT" \
  "$VARIABLE_STMT" \
  "$LOCAL_GUARD" \
  "$RAW_DISPATCH" \
  "$MATCH_OWNER" \
  "$BUILDER_BUILD" \
  "$ASSIGNMENT_DESCENT" \
  "$ASSIGNMENT_TESTS" \
  "$ASSIGNMENT_GUARD" \
  "$RETURN_DESCENT" \
  "$RETURN_TESTS" \
  "$RETURN_OWNER" \
  "$RETURN_GUARD" \
  "$OPS_ROOT" \
  "$BINARY_DESCENT" \
  "$BINARY_TESTS" \
  "$SHORT_CIRCUIT_DESCENT" \
  "$SHORT_CIRCUIT_TESTS" \
  "$BINARY_GUARD" \
  "$METHOD_CALL_DESCENT" \
  "$METHOD_CALL_TERMINAL" \
  "$CALLS_MOD" \
  "$CALLS_BUILD" \
  "$FUNCTION_CALL_ROUTE" \
  "$FUNCTION_SPECIAL" \
  "$FASTMEM_CALLS" \
  "$ENUM_MATCH" \
  "$SCOPEBOX_ENUM" \
  "$METHOD_CALL_HANDLERS" \
  "$PROPERTY_READS" \
  "$FIELDS" \
  "$PROPERTY_TESTS" \
  "$RECORD_HELPER" \
  "$RECORD_HELPER_TESTS" \
  "$INDEXING" \
  "$PRINT_STMT" \
  "$METHOD_CALL_GUARD" \
  "$RECORD_HELPER_GUARD"
expected_header=$'record_kind\tid\tpack\tproduction_caller\tnew_owner\tdelete_target\tparity_gate\tdisposition\tstate'
actual_header="$(head -n1 "$MANIFEST")"
if [[ "$actual_header" != "$expected_header" ]]; then
  guard_fail "$TAG" "manifest header drift"
fi
if ! awk -F '\t' '
  NR == 1 { next }
  NF != 9 { exit 1 }
  $1 !~ /^(cell|asset)$/ { exit 1 }
  $9 !~ /^(pending|active|closed|explicit-residual)$/ { exit 1 }
  $1 == "asset" && $8 !~ /^(IntegrateNow|ReuseNeutral|FixtureOnly|Delete)$/ { exit 1 }
  END { if (NR < 2) exit 1 }
' "$MANIFEST"; then
  guard_fail "$TAG" "manifest row contract failed"
fi
for sunset in \
  MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001 \
  RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001
do
  if [[ "$(rg -F -c "\"$sunset\": {" "$CALLER_MANIFEST")" != "1" ]]; then
    guard_fail "$TAG" "compatibility sunset must have one first-class record: $sunset"
  fi
done
if [[ "$(rg -F -c '"production_build_module_edges": 0' "$CALLER_MANIFEST")" != "2" ]]; then
  guard_fail "$TAG" "arbitrary-AST production sunsets must both be retired"
fi
if [[ "$(rg -F -c 'fn lower_loop_or_freeze_v1(' "$LOOP_ROUTING")" != "1" ]] ||
   [[ "$(rg -F -c 'lower_loop_or_freeze_v1(' "$RAW_CHILD_PORT")" != "1" ]] ||
   [[ "$(rg -F -c 'lower_loop_or_freeze_v1(' "$ROOT_DIR/src/mir/builder/raw_loop_child_entry.rs")" != "1" ]]; then
  guard_fail "$TAG" "raw Loop callers must share one JoinIR route/freeze owner"
fi
if rg -n -F '.cf_loop(' "$ROOT_DIR/src/mir/builder" --glob '*.rs' >/dev/null ||
   rg -n -F 'fn cf_loop(' "$ROOT_DIR/src/mir/builder" --glob '*.rs' >/dev/null ||
   rg -n -F 'planner_reject_detail' "$CONTROL_FLOW_ROOT" >/dev/null; then
  guard_fail "$TAG" "retired generic cf_loop authority returned"
fi
guard_exact_counts <<EOF
$BUILDER_BUILD|struct\\s+PreparedRawNewExpressionV1\\b|1|prepared raw New route
$BUILDER_BUILD|fn\\s+lower_prepared_raw_new_expression_with_port_v1\\s*<Port>|1|prepared raw New lowering owner
$BUILDER_BUILD|is_record_constructor_class\\(&class\\)|1|single raw New record classifier
$RAW_DISPATCH|PreparedRawNewExpressionV1::prepare\\s*\\(|1|sole raw New route issuer
$RAW_DISPATCH|lower_prepared_raw_new_expression_with_port_v1\\s*\\(|1|sole prepared raw New caller
EOF
for retired_new_edge in \
  'fn build_new_expression(' \
  'fn build_new_expression_with_port_v1' \
  'fn build_new_expression_with_field_initializers(' \
  'fn build_new_expression_with_field_initializers_with_port_v1'
do
  if rg -n -F "$retired_new_edge" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired raw New edge returned: $retired_new_edge"
  fi
done
if rg -n -F 'RawLegacyChildLoweringPortV1' "$BUILDER_BUILD" >/dev/null; then
  guard_fail "$TAG" "builder_build restored a caller-selected legacy New port"
fi
guard_exact_counts <<EOF
$CALLS_BUILD|struct\\s+PreparedRawFromCallV1\\b|1|opaque prepared raw From route
$CALLS_BUILD|enum\\s+PreparedRawFromCallRouteV1\\b|1|private raw From route vocabulary
$CALLS_BUILD|fn\\s+lower_prepared_raw_from_call_with_port_v1\\s*<Port>|1|prepared raw From lowering owner
$ENUM_MATCH|fn\\s+prepare_raw_enum_variant_header_v1\\s*\\(|1|single raw enum variant classifier
$ENUM_MATCH|fn\\s+lower_prepared_raw_enum_variant_with_port_v1\\s*<Port>|1|prepared raw enum lowering owner
$CALLS_BUILD|lower_prepared_raw_enum_variant_with_port_v1\\s*\\(|1|sole prepared raw enum caller
$RAW_DISPATCH|PreparedRawFromCallV1::prepare\\s*\\(|1|sole raw From route issuer
$RAW_DISPATCH|lower_prepared_raw_from_call_with_port_v1\\s*\\(|1|sole prepared raw From caller
EOF
for retired_from_edge in \
  'fn build_from_expression(' \
  'fn build_from_expression_with_port_v1' \
  'fn try_build_enum_variant_constructor(' \
  'fn try_build_enum_variant_constructor_with_port_v1'
do
  if rg -n -F "$retired_from_edge" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired raw From edge returned: $retired_from_edge"
  fi
done
guard_exact_counts <<EOF
$SCOPEBOX_ENUM|struct\\s+PreparedRawScopeBoxV1\\b|1|opaque prepared raw ScopeBox route
$SCOPEBOX_ENUM|enum\\s+PreparedRawScopeBoxRouteV1\\b|1|private raw ScopeBox route vocabulary
$SCOPEBOX_ENUM|fn\\s+lower_prepared_raw_scopebox_with_port_v1\\s*<Port>|1|prepared raw ScopeBox lowering owner
$STATEMENT_SURFACE|PreparedRawScopeBoxV1::prepare\\s*\\(|1|sole raw ScopeBox route issuer
$STATEMENT_SURFACE|lower_prepared_raw_scopebox_with_port_v1\\s*\\(|1|sole prepared raw ScopeBox caller
$INDEXING|struct\\s+PreparedRawIndexReadV1\\b|1|opaque prepared raw Index-read route
$INDEXING|enum\\s+PreparedRawIndexReadRouteV1\\b|1|private raw Index-read route vocabulary
$INDEXING|fn\\s+lower_prepared_raw_index_read_with_port_v1\\s*<Port>|1|prepared raw Index-read lowering owner
$RAW_DISPATCH|PreparedRawIndexReadV1::prepare\\s*\\(|1|sole raw Index-read route issuer
$RAW_DISPATCH|lower_prepared_raw_index_read_with_port_v1\\s*\\(|1|sole prepared raw Index-read caller
$ENUM_MATCH|struct\\s+PreparedRawEnumMatchV1\\b|1|opaque prepared raw EnumMatch route
$ENUM_MATCH|enum\\s+PreparedRawEnumMatchRouteV1\\b|1|private raw EnumMatch route vocabulary
$ENUM_MATCH|fn\\s+lower_prepared_raw_enum_match_with_port_v1\\s*<Port>|1|prepared raw EnumMatch lowering owner
$RAW_DISPATCH|PreparedRawEnumMatchV1::prepare\\s*\\(|1|sole raw EnumMatch route issuer
$RAW_DISPATCH|lower_prepared_raw_enum_match_with_port_v1\\s*\\(|2|sole dispatch owner has both scoped EnumMatch branches
EOF
if rg -n -P 'fn\s+try_build_guard_let_scopebox(?:_with_port_v1)?\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired Option-based ScopeBox route returned"
fi
for retired_raw_edge in \
  'fn build_index_expression(' \
  'fn build_index_expression_with_port_v1' \
  'fn build_enum_match_expression(' \
  'fn build_enum_match_expression_with_port_v1' \
  'fn try_build_guard_let_payload_projection_with_port_v1' \
  'fn build_guard_let_variant_bool_select_with_port_v1'
do
  if rg -n -F "$retired_raw_edge" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired raw edge returned: $retired_raw_edge"
  fi
done
stageb_asset_count="$(awk -F '\t' '
  $1 == "asset" &&
  $2 == "PRELOOP-STAGEB-SPECIAL-ACTIVATION" &&
  $8 == "Delete" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$stageb_asset_count" != "1" ]]; then
  guard_fail "$TAG" "Stage-B special activation asset must be Delete/closed"
fi
for retired_path in \
  "$ROOT_DIR/src/mir/preloop_stageb_candidate_shell.rs" \
  "$ROOT_DIR/src/mir/preloop_stageb_carrier" \
  "$ROOT_DIR/src/mir/builder/calls/preloop_stageb_instance_function_session" \
  "$ROOT_DIR/src/mir/builder/calls/preloop_located_argument_port.rs" \
  "$ROOT_DIR/src/mir/builder/calls/preloop_located_outer_completion.rs"
do
  if [[ -e "$retired_path" ]]; then
    guard_fail "$TAG" "retired Stage-B activation path returned: ${retired_path#"$ROOT_DIR/"}"
  fi
done
for retired_symbol in \
  PreloopStageBWholeSourceProducerV1 \
  PreparedPreloopStageBModuleActivationV1 \
  PreparedPreloopStageBFunctionActivationV1 \
  collect_preloop_stageb_instance_function_v1 \
  lower_root_with_preinstalled_catalog_v1
do
  if rg -n -F "$retired_symbol" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired Stage-B activation symbol returned: $retired_symbol"
  fi
done
ratchet_header=$'source_files\tsource_loc\ttest_files\ttest_loc'
if [[ "$(head -n1 "$STRUCTURAL_RATCHET")" != "$ratchet_header" ]] ||
   [[ "$(wc -l < "$STRUCTURAL_RATCHET" | tr -d '[:space:]')" != "2" ]]; then
  guard_fail "$TAG" "structural observation must contain one baseline row"
fi
read -r source_files_baseline source_loc_baseline test_files_baseline test_loc_baseline \
  < <(tail -n1 "$STRUCTURAL_RATCHET")
if [[ ! "$source_files_baseline" =~ ^[0-9]+$ ]] ||
   [[ ! "$source_loc_baseline" =~ ^[0-9]+$ ]] ||
   [[ ! "$test_files_baseline" =~ ^[0-9]+$ ]] ||
   [[ ! "$test_loc_baseline" =~ ^[0-9]+$ ]]; then
  guard_fail "$TAG" "structural observation baseline row must be numeric"
fi
builder_roots=(
  "$ROOT_DIR/src/mir/builder"
  "$ROOT_DIR/crates/hakorune_mir_builder"
)
source_files="$(find "${builder_roots[@]}" -type f -name '*.rs' ! -name '*test*.rs' -print | wc -l)"
source_loc="$(
  find "${builder_roots[@]}" -type f -name '*.rs' ! -name '*test*.rs' -print0 \
    | xargs -0 wc -l | tail -n1 | awk '{ print $1 }'
)"
test_files="$(find "${builder_roots[@]}" -type f -name '*test*.rs' -print | wc -l)"
test_loc="$(
  find "${builder_roots[@]}" -type f -name '*test*.rs' -print0 \
    | xargs -0 wc -l | tail -n1 | awk '{ print $1 }'
)"
printf \
  '[%s] structural observation: source_files=%d (%+d), source_loc=%d (%+d), test_files=%d (%+d), test_loc=%d (%+d)\n' \
  "$TAG" \
  "$source_files" "$((source_files - source_files_baseline))" \
  "$source_loc" "$((source_loc - source_loc_baseline))" \
  "$test_files" "$((test_files - test_files_baseline))" \
  "$test_loc" "$((test_loc - test_loc_baseline))"
current_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "CALLABLE-DRAFT-PORT-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$current_row_count" != "1" ]]; then
  guard_fail "$TAG" "callable draft cutover must have one closed manifest row"
fi
collector_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "CALLABLE-DRAFT-COLLECTOR-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$collector_row_count" != "1" ]]; then
  guard_fail "$TAG" "callable collector cutover must have one closed manifest row"
fi
candidate_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "MODULE-CANDIDATE-SESSION-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
normal_pipeline_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$normal_pipeline_row_count" != "1" ]]; then
  guard_fail "$TAG" "normal default published pipeline must have one closed manifest row"
fi
normal_lifecycle_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$normal_lifecycle_row_count" != "1" ]]; then
  guard_fail "$TAG" "normal root/catalog lifecycle must have one closed manifest row"
fi
if rg -n 'ExistingGeneralModuleCompatibilityV1|\.build_module\(|session\.builder_mut\(' \
  "$NORMAL_PIPELINE" >/dev/null; then
  guard_fail "$TAG" "selected normal compatibility owner or direct Builder edge returned"
fi
if [[ "$(rg -o 'complete_normal_default_program_root_catalog_lifecycle\(' "$NORMAL_PIPELINE" | wc -l | tr -d '[:space:]')" != "1" ]]; then
  guard_fail "$TAG" "selected normal lifecycle caller must be exactly one"
fi
if [[ "$candidate_row_count" != "1" ]]; then
  guard_fail "$TAG" "module candidate cutover must have one closed manifest row"
fi
local_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "LOCAL-STATEMENT-DESCENT-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$local_row_count" != "1" ]]; then
  guard_fail "$TAG" "Local descent cutover must have one closed manifest row"
fi
assignment_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$assignment_row_count" != "1" ]]; then
  guard_fail "$TAG" "Variable Assignment descent cutover must have one closed manifest row"
fi
return_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "RETURN-SOURCE-PARTITION-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$return_row_count" != "1" ]]; then
  guard_fail "$TAG" "Return source partition cutover must have one closed manifest row"
fi
binary_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "BINARY-SOURCE-PARTITION-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$binary_row_count" != "1" ]]; then
  guard_fail "$TAG" "Binary source partition cutover must have one closed manifest row"
fi
record_helper_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "RECORD-HELPER-BODY-DESCENT0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$record_helper_row_count" != "1" ]]; then
  guard_fail "$TAG" "record-helper body descent must have one closed manifest row"
fi
property_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "FIELD-PROPERTY-GETTER-DESCENT0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$property_row_count" != "1" ]]; then
  guard_fail "$TAG" "Field property getter descent must have one closed manifest row"
fi
for symbol in \
  build_static_method_draft_v1 \
  build_instance_method_draft_v1 \
  lower_function_body \
  lower_method_body
do
  if rg -n -w "$symbol" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired callable body symbol returned: $symbol"
  fi
done
while read -r symbol expected; do
  count="$(rg -o -F "$symbol" "$LOWERING" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "ordinary production caller count drift: $symbol count=$count expected=$expected"
  fi
done <<'EOF'
build_static_method_draft_with_port_v1 1
build_instance_method_draft_with_port_v1 1
finalize_port_aware_draft_for_legacy_v1 2
EOF
while read -r symbol expected; do
  count="$(rg -o -F "$symbol" "$PROGRAM_ROOT" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "ordinary collector production edge drift: $symbol count=$count expected=$expected"
  fi
done <<'EOF'
ModuleDraftCollectorV1::with_brand(brand) 1
RawInvocationChildPortV1::new 1
.prepare_normal_collector_drain 1
EOF
for retired_edge in \
  lower_static_method_as_function \
  lower_method_as_function \
  build_static_main_box_typed \
  '.cf_block('
do
  if rg -n -F "$retired_edge" "$MODULE_LIFECYCLE" >/dev/null; then
    guard_fail "$TAG" "direct ordinary callable edge returned: $retired_edge"
  fi
done
for retired_edge in \
  compile_with_source_internal \
  compile_legacy_candidate \
  compile_legacy_request \
  'enum MirLoweringRequestV1' \
  'self.builder.build_module'
do
  if rg -n -F "$retired_edge" "$COMPILER" >/dev/null; then
    guard_fail "$TAG" "live compiler build edge returned: $retired_edge"
  fi
done
for required_edge in \
  'fn compile_public_program' \
  'NormalCompileRequestV1::for_mir_mode' \
  'self.compile_normal(request)'
do
  if [[ "$(rg -F -c "$required_edge" "$COMPILER")" != "1" ]]; then
    guard_fail "$TAG" "public Program compiler edge drift: $required_edge"
  fi
done
while IFS=$'\t' read -r file pattern expected label; do
  count="$(rg -o -P "$pattern" "$file" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "$label count drift: count=$count expected=$expected"
  fi
done <<EOF
$STATEMENT_SURFACE	\\bdrive_local_statement_v1\\s*\\(	1	raw/default Local owner caller
$STATEMENT_SURFACE	\\bRawLegacyLocalInputV1::new\\s*\\(	1	raw/default Local owned input
EOF
if rg -n -P '\b(?:fn\s+)?build_local_statement\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired build_local_statement facade returned"
fi
if rg -n -P '\b(?:fn\s+)?drive_raw_local_statement_v1\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired drive_raw_local_statement_v1 facade returned"
fi
if rg -n -w 'retry|fallback' "$LOCAL_DESCENT" >/dev/null; then
  guard_fail "$TAG" "Local owner gained retry or fallback"
fi
while IFS=$'\t' read -r file pattern expected label; do
  count="$(rg -o -P "$pattern" "$file" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "$label count drift: count=$count expected=$expected"
  fi
done <<EOF
$STATEMENT_SURFACE	\\bdrive_variable_assignment_v1\\s*\\(	1	exact Variable Assignment owner caller
$STATEMENT_SURFACE	\\bRawLegacyVariableAssignmentInputV1::new\\s*\\(	1	exact Variable Assignment owned input
$RAW_DISPATCH	\\bdrive_variable_assignment_v1\\s*\\(	1	Grouped Assignment owner caller
$RAW_DISPATCH	\\bRawLegacyVariableAssignmentInputV1::new\\s*\\(	1	Grouped Assignment owned input
EOF
assignment_external_count="$(
  rg -n -P '\bdrive_variable_assignment_v1\s*\(' \
    "$ROOT_DIR/src" --glob '*.rs' \
    | awk -F ':' \
        -v owner="$ASSIGNMENT_DESCENT" \
        -v tests="$ASSIGNMENT_TESTS" \
        -v parity="$ASSIGNMENT_PARITY_TESTS" \
        '$1 != owner && $1 != tests && $1 != parity { count += 1 } END { print count + 0 }'
)"
if [[ "$assignment_external_count" != "2" ]]; then
  guard_fail "$TAG" \
    "Assignment external owner sites must be two raw/default: count=$assignment_external_count"
fi
if rg -n -P '\b(?:fn\s+)?drive_raw_variable_assignment_v1\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired drive_raw_variable_assignment_v1 facade returned"
fi
if rg -n -P '\b(?:fn\s+)?build_grouped_assignment\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired build_grouped_assignment facade returned"
fi
if rg -n -w 'retry|fallback' "$ASSIGNMENT_DESCENT" >/dev/null; then
  guard_fail "$TAG" "Assignment owner gained retry or fallback"
fi
while IFS=$'\t' read -r file pattern expected label; do
  count="$(rg -o -P "$pattern" "$file" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "$label count drift: count=$count expected=$expected"
  fi
done <<EOF
$STATEMENT_SURFACE	\\bdrive_value_return_statement_v1\\s*\\(	1	value-bearing Return owner caller
$STATEMENT_SURFACE	\\bRawLegacyValueReturnInputV1::new\\s*\\(	1	value-bearing Return owned input
$STATEMENT_SURFACE	\\bbuild_void_return_statement\\s*\\(	1	exact Void Return owner caller
EOF
return_external_count="$(
  rg -n -P '\bdrive_value_return_statement_v1\s*\(' \
    "$ROOT_DIR/src" --glob '*.rs' \
    | awk -F ':' \
        -v owner="$RETURN_DESCENT" \
        -v tests="$RETURN_TESTS" \
        -v parity="$RETURN_PARITY_TESTS" \
        '$1 != owner && $1 != tests && $1 != parity { count += 1 } END { print count + 0 }'
)"
if [[ "$return_external_count" != "1" ]]; then
  guard_fail "$TAG" \
    "Return external owner sites must be one raw/default: count=$return_external_count"
fi
if rg -n -P '\b(?:fn\s+)?drive_raw_value_return_statement_v1\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired drive_raw_value_return_statement_v1 facade returned"
fi
if rg -n -P '\b(?:fn\s+)?build_return_statement\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired build_return_statement facade returned"
fi
if rg -n -P '\btry_apply_match_return_optimization\s*\(\s*builder,\s*None' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired no-value Match observation returned"
fi
if rg -n -w 'retry|fallback' "$RETURN_DESCENT" >/dev/null; then
  guard_fail "$TAG" "Return value owner gained retry or route fallback"
fi
while IFS=$'\t' read -r file pattern expected label; do
  count="$(rg -o -P "$pattern" "$file" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "$label count drift: count=$count expected=$expected"
  fi
done <<EOF
$RAW_DISPATCH	\\bRawLegacyBinaryInputV1::new\\s*\\(	1	raw/default ordinary Binary input
$RAW_DISPATCH	\\bdrive_ordinary_binary_expression_v1\\s*\\(	1	raw/default ordinary Binary owner caller
$RAW_DISPATCH	\\bRawLegacyShortCircuitInputV1::new\\s*\\(	1	raw/default short-circuit input
$RAW_DISPATCH	\\bdrive_short_circuit_expression_v1\\s*\\(	1	raw/default short-circuit owner caller
EOF
ordinary_binary_external_count="$(
  rg -n -P '\bdrive_ordinary_binary_expression_v1\s*\(' \
    "$ROOT_DIR/src" --glob '*.rs' \
    | awk -F ':' \
        -v owner="$BINARY_DESCENT" \
        -v tests="$BINARY_TESTS" \
        '$1 != owner && $1 != tests { count += 1 } END { print count + 0 }'
)"
if [[ "$ordinary_binary_external_count" != "1" ]]; then
  guard_fail "$TAG" "ordinary Binary external sites must be one raw/default: count=$ordinary_binary_external_count"
fi
short_circuit_external_count="$(
  rg -n -P '\bdrive_short_circuit_expression_v1\s*\(' \
    "$ROOT_DIR/src" --glob '*.rs' \
    | awk -F ':' \
        -v owner="$SHORT_CIRCUIT_DESCENT" \
        -v tests="$SHORT_CIRCUIT_TESTS" \
        '$1 != owner && $1 != tests { count += 1 } END { print count + 0 }'
)"
if [[ "$short_circuit_external_count" != "1" ]]; then
  guard_fail "$TAG" "short-circuit external sites must be one raw/default: count=$short_circuit_external_count"
fi
for retired_pattern in \
  '\b(?:fn\s+)?build_binary_op\s*\(' \
  '\b(?:fn\s+)?drive_raw_ordinary_binary_expression_v1\s*\(' \
  '\b(?:fn\s+)?drive_raw_short_circuit_expression_v1\s*\('
do
  if rg -n -P "$retired_pattern" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired Binary selector/facade returned: $retired_pattern"
  fi
done
if rg -n -w 'retry|fallback' "$BINARY_DESCENT" "$SHORT_CIRCUIT_DESCENT" >/dev/null; then
  guard_fail "$TAG" "Binary owner gained retry or route fallback"
fi
guard_exact_counts <<EOF
$RAW_CHILD_PORT|normalize_instance_method_params\\(|1|single instance params normalization
$RAW_CHILD_PORT|normalize_instance_method_param_decls\\(|1|single instance param-decls normalization
$METHOD_CALL_DESCENT|\\benum\\s+CatalogHelperChildV1\\b|1|catalog helper child vocabulary
$METHOD_CALL_DESCENT|\\[method-call-descent/catalog-helper-child-unsupported\\]|1|fail-closed custom-port default
$RECORD_HELPER|CatalogHelperChildV1::Expression\\(\\*expression\\)|1|owned catalog helper expression terminal
$RECORD_HELPER|CatalogHelperChildV1::Statement\\(statement\\)|1|owned catalog helper statement terminal
EOF
for retired_pattern in \
  'self\.build_expression\(\*expr\.clone\(\)\)' \
  'self\.build_statement\(stmt\.clone\(\)\)' \
  '\btry_inline_same_module_helper_setter_call\s*\(' \
  '\btry_inline_same_module_helper_setter_call_with_descent\s*\(' \
  '\btry_inline_same_module_helper_setter_call_from_receiver_with_descent\s*\('
do
  if rg -n -P "$retired_pattern" "$RECORD_HELPER" >/dev/null; then
    guard_fail "$TAG" "retired record-helper edge returned: $retired_pattern"
  fi
done
if rg -n -w 'retry|fallback|reselection' \
  "$METHOD_CALL_DESCENT" "$RECORD_HELPER" >/dev/null; then
  guard_fail "$TAG" "record-helper descent gained retry, fallback, or reselection"
fi
guard_exact_counts <<EOF
$RAW_DISPATCH|PreparedRawFieldReadV1::prepare\\s*\\(|1|sole raw/default FieldAccess route issuer
$RAW_DISPATCH|lower_prepared_raw_field_read_with_port_v1\\s*\\(|1|sole prepared FieldAccess caller
$PRINT_STMT|struct\\s+PreparedRawPrintV1\\b|1|opaque prepared raw Print route
$PRINT_STMT|enum\\s+PreparedRawPrintRouteV1\\b|1|private raw Print route vocabulary
$PRINT_STMT|fn\\s+lower_prepared_raw_print_with_port_v1\\s*<Port>|1|prepared raw Print lowering owner
$PRINT_STMT|let\\s+prepared\\s*=\\s*PreparedRawPrintV1::prepare\\s*\\(|1|sole raw Print route issuer
$PRINT_STMT|let\\s+value\\s*=\\s*lower_prepared_raw_print_with_port_v1\\s*\\(|1|sole prepared raw Print caller
$FUNCTION_CALL_ROUTE|struct\\s+PreparedRawFunctionPreflightV1\\b|1|opaque direct FunctionCall preflight
$FUNCTION_CALL_ROUTE|enum\\s+PreparedRawFunctionPreflightRouteV1\\b|1|private direct FunctionCall route vocabulary
$FUNCTION_CALL_ROUTE|fn\\s+lower_prepared_raw_function_preflight_with_port_v1\\s*<Port>|1|prepared direct FunctionCall lowering owner
$RAW_DISPATCH|PreparedRawFunctionPreflightV1::prepare\\s*\\(|1|sole direct FunctionCall route issuer
$RAW_DISPATCH|lower_prepared_raw_function_preflight_with_port_v1\\s*\\(|1|sole prepared direct FunctionCall caller
$FIELDS|try_lower_property_read_with_port_v1\\s*\\(port, object_value, &field\\)|1|port-aware property caller
$PROPERTY_READS|struct\\s+PropertyGetterCompletionV1\\b|1|exact zero-argument property adapter
$PROPERTY_READS|fn\\s+try_lower_property_read_with_port_v1\\s*<Port>|1|port-aware property owner
$PROPERTY_READS|handle_standard_method_call_with_descent\\s*\\(|1|shared standard orchestration caller
$PROPERTY_READS|lower_catalog_helper_child\\(self\\.port, builder, child\\)|1|selected catalog-child loan
$PROPERTY_READS|emit_standard_value_terminal_raw_v1\\s*\\(|1|A1 lookup-none property terminal
$METHOD_CALL_HANDLERS|fn\\s+handle_standard_method_call_with_descent\\s*<Completion>|1|sole standard orchestration owner
$METHOD_CALL_TERMINAL|trait\\s+StandardMethodCallCompletionV1\\b|1|standard completion capability
$METHOD_CALL_TERMINAL|impl<Port>\\s+StandardMethodCallCompletionV1|1|associated standard completion
EOF
for retired_pattern in \
  '\b(?:fn\s+)?try_lower_property_read\s*\(' \
  '\b(?:fn\s+)?handle_standard_method_call\s*\(' \
  '\bLegacyMethodCallArgumentsV1\b' \
  '\b(?:fn\s+)?build_field_access\s*\(' \
  '\b(?:fn\s+)?(?:build_field_access_with_port_v1|try_lower_record_field_read_from_ast(?:_with_port_v1)?)\s*\('
do
  if rg -n -P "$retired_pattern" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired property facade returned: $retired_pattern"
  fi
done
if rg -n -P '\bbuild_print_statement(?:_with_port_v1)?\s*\(|\bCallExpr\b|\.clone\s*\(' \
  "$PRINT_STMT" >/dev/null; then
  guard_fail "$TAG" "retired raw Print facade, wrapper, or AST clone returned"
fi
if [[ -e "$ROOT_DIR/src/mir/builder/calls/function_preflight.rs" ]] ||
   rg -n -P '\b(?:build_function_call|try_handle_function_preflight|try_build_typeop_function|try_handle_math_function|lower_fastmem_function_call)\s*\(' \
     "$ROOT_DIR/src/mir/builder" --glob '*.rs' >/dev/null ||
   rg -n -P '\bCallExpr\b' "$ROOT_DIR/src/mir/builder" --glob '*.rs' >/dev/null ||
   rg -n -P '\.clone\s*\(|\b(?:retry|fallback|reselection)\b' \
     "$FUNCTION_CALL_ROUTE" "$FUNCTION_SPECIAL" >/dev/null; then
  guard_fail "$TAG" "retired FunctionCall probe/facade/clone or route retry returned"
fi
function_prepare_external_files="$(
  rg -l -P '\bPreparedRawFunctionPreflightV1::prepare\s*\(' \
    "$ROOT_DIR/src/mir/builder" --glob '*.rs' |
    rg -v '/calls/function_call_preflight_route\.rs$' |
    wc -l | tr -d '[:space:]'
)"
function_lower_external_files="$(
  rg -l -P '\blower_prepared_raw_function_preflight_with_port_v1\s*\(' \
    "$ROOT_DIR/src/mir/builder" --glob '*.rs' |
    rg -v '/calls/function_call_preflight_route\.rs$' |
    wc -l | tr -d '[:space:]'
)"
if [[ "$function_prepare_external_files" != "1" ]] ||
   [[ "$function_lower_external_files" != "1" ]]; then
  guard_fail "$TAG" \
    "direct FunctionCall must have one external issuer/consumer file: prepare=$function_prepare_external_files lower=$function_lower_external_files"
fi
for forbidden in \
  RawLegacyMethodCallInputV1 \
  MethodCallValueTerminalPortV1 \
  with_function_headers \
  drive_raw_legacy_expression_v1 \
  drive_raw_legacy_statement_v1 \
  'build_expression('
do
  if rg -n -F "$forbidden" "$PROPERTY_READS" >/dev/null; then
    guard_fail "$TAG" "property adapter acquired forbidden authority: $forbidden"
  fi
done
if rg -n -w 'retry|fallback|reselection' \
  "$PROPERTY_READS" "$FIELDS" "$METHOD_CALL_HANDLERS" >/dev/null; then
  guard_fail "$TAG" "property descent gained retry, fallback, or reselection"
fi
if rg -n -P '\.build_expression\s*\(' \
  "$ROOT_DIR/src/mir/builder" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired MirBuilder build_expression caller returned"
fi
if rg -n -P '\bfn\s+build_expression\s*\(' \
  "$ROOT_DIR/src/mir/builder" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired MirBuilder build_expression facade returned"
fi
match_branch="$(sed -n '/ASTNode::MatchExpr {/,/ASTNode::EnumMatchExpr {/p' "$RAW_DISPATCH")"
if rg -n -F '.clone()' <<<"$match_branch" >/dev/null ||
   ! rg -n -F 'arms.into_iter().enumerate()' "$MATCH_OWNER" >/dev/null ||
   rg -n -F 'arms.iter().cloned()' "$MATCH_OWNER" >/dev/null; then
  guard_fail "$TAG" "Match owned input must have one consuming production owner"
fi
if rg -n -P '\b(?:callee|arguments|expression|record_type_name|fields|base|updates)\.clone\s*\(' "$RAW_DISPATCH" >/dev/null; then guard_fail "$TAG" "owned compound expression dispatcher clone returned"; fi
python3 "$GENERIC_LEGACY_GUARD" "$GENERIC_LEGACY_MANIFEST" "$ROOT_DIR" || guard_fail "$TAG" "Generic legacy corpus universe manifest failed"
guard_joinir_logical_demand_contract "$ROOT_DIR" "$TAG"; guard_joinir_if_recipe_contract "$ROOT_DIR" "$TAG"; guard_joinir_loop_compile_candidate_scope "$ROOT_DIR" "$TAG"; guard_loop_family_observation_contract "$ROOT_DIR" "$TAG"; guard_generic_g0_observation_contract "$ROOT_DIR" "$TAG"; guard_generic_candidate_envelope_contract "$ROOT_DIR" "$TAG"; guard_loop_family_row_context_retention_contract "$ROOT_DIR" "$TAG"; guard_loop_family_window_lease_contract "$ROOT_DIR" "$TAG"; guard_loop_family_admission_contract "$ROOT_DIR" "$TAG"; guard_loop_family_selector_contract "$ROOT_DIR" "$TAG"
for file in \
  "$LOWERING" \
  "$PORT_OWNER" \
  "$MODULE_LIFECYCLE" \
  "$COMPILER" \
  "$MODULE_SESSION" \
  "$NORMAL_PIPELINE" \
  "$NORMAL_ROOT_LIFECYCLE" \
  "$STATEMENT_SURFACE" \
  "$LOCAL_DESCENT" \
  "$VARIABLE_STMT" \
  "$ROOT_DIR/src/mir/builder/stmts/README.md" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/wiring_tests.rs" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/raw_parity_tests.rs" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/if_general.rs" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/stmt/tests.rs" \
  "$LOCAL_GUARD" \
  "$RAW_DISPATCH" \
  "$ASSIGNMENT_DESCENT" \
  "$ASSIGNMENT_TESTS" \
  "$ROOT_DIR/src/mir/builder/stmts/variable_assignment_raw_tests.rs" \
  "$ROOT_DIR/src/mir/builder/stmts/variable_assignment_parity_tests.rs" \
  "$ROOT_DIR/src/mir/builder/builder_build.rs" \
  "$ASSIGNMENT_GUARD" \
  "$RETURN_DESCENT" \
  "$RETURN_TESTS" \
  "$RETURN_OWNER" \
  "$ROOT_DIR/src/mir/builder/stmts/return_statement_raw_tests.rs" \
  "$ROOT_DIR/src/mir/builder/stmts/return_statement_parity_tests.rs" \
  "$RETURN_GUARD" \
  "$OPS_ROOT" \
  "$BINARY_DESCENT" \
  "$BINARY_TESTS" \
  "$ROOT_DIR/src/mir/builder/ops/binary_expression_raw_tests.rs" \
  "$ROOT_DIR/src/mir/builder/ops/binary_expression_parity_tests.rs" \
  "$SHORT_CIRCUIT_DESCENT" \
  "$SHORT_CIRCUIT_TESTS" \
  "$ROOT_DIR/src/mir/builder/ops/short_circuit_expression_raw_tests.rs" \
  "$ROOT_DIR/src/mir/builder/ops/short_circuit_expression_parity_tests.rs" \
  "$BINARY_GUARD" \
  "$METHOD_CALL_DESCENT" \
  "$METHOD_CALL_TERMINAL" \
  "$CALLS_MOD" \
  "$CALLS_BUILD" \
  "$FUNCTION_CALL_ROUTE" \
  "$FUNCTION_SPECIAL" \
  "$FASTMEM_CALLS" \
  "$ENUM_MATCH" \
  "$SCOPEBOX_ENUM" \
  "$METHOD_CALL_HANDLERS" \
  "$PROPERTY_READS" \
  "$FIELDS" \
  "$PROPERTY_TESTS" \
  "$RECORD_HELPER" \
  "$RECORD_HELPER_TESTS" \
  "$METHOD_CALL_GUARD" \
  "$RECORD_HELPER_GUARD" \
  "$ROOT_DIR/tools/checks/mirbuilder_inplace_replacement_guard.sh" \
  "$GENERIC_LEGACY_GUARD"
do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "file exceeds boundary: ${file#"$ROOT_DIR/"} lines=$lines"
  fi
done
echo "[$TAG] ok"
