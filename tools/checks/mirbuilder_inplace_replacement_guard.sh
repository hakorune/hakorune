#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-inplace-replacement-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/mirbuilder-inplace-replacement-v1.tsv"
LOWERING="$ROOT_DIR/src/mir/builder/calls/lowering.rs"
PORT_OWNER="$ROOT_DIR/src/mir/builder/port_aware_function_draft_impl.rs"
MODULE_LIFECYCLE="$ROOT_DIR/src/mir/builder/module_lifecycle.rs"
COMPILER="$ROOT_DIR/src/mir/compiler/mod.rs"
LEGACY_CANDIDATE="$ROOT_DIR/src/mir/compiler/legacy_candidate_session.rs"
MODULE_SESSION="$ROOT_DIR/src/mir/builder/module_invocation_session.rs"
STATEMENT_SURFACE="$ROOT_DIR/src/mir/builder/raw_expression_dispatch/statement_surface.rs"
LOCAL_DESCENT="$ROOT_DIR/src/mir/builder/stmts/local_statement_descent.rs"
LOCATED_LOCAL="$ROOT_DIR/src/mir/builder/located_legacy_lowering.rs"
VARIABLE_STMT="$ROOT_DIR/src/mir/builder/stmts/variable_stmt.rs"
LOCAL_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0.py"
RAW_DISPATCH="$ROOT_DIR/src/mir/builder/raw_expression_dispatch/mod.rs"
ASSIGNMENT_DESCENT="$ROOT_DIR/src/mir/builder/stmts/variable_assignment_descent.rs"
LOCATED_ASSIGNMENT="$ROOT_DIR/src/mir/builder/located_legacy_assignment.rs"
ASSIGNMENT_TESTS="$ROOT_DIR/src/mir/builder/stmts/variable_assignment_descent_tests.rs"
ASSIGNMENT_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment.py"
RETURN_DESCENT="$ROOT_DIR/src/mir/builder/stmts/return_statement_descent.rs"
RETURN_TESTS="$ROOT_DIR/src/mir/builder/stmts/return_statement_descent_tests.rs"
RETURN_OWNER="$ROOT_DIR/src/mir/builder/stmts/return_stmt.rs"
LOCATED_RETURN="$ROOT_DIR/src/mir/builder/located_legacy_return.rs"
RETURN_GUARD="$ROOT_DIR/tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0_return.py"

guard_require_command "$TAG" awk
guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files \
  "$TAG" \
  "$MANIFEST" \
  "$LOWERING" \
  "$PORT_OWNER" \
  "$MODULE_LIFECYCLE" \
  "$COMPILER" \
  "$LEGACY_CANDIDATE" \
  "$MODULE_SESSION" \
  "$STATEMENT_SURFACE" \
  "$LOCAL_DESCENT" \
  "$LOCATED_LOCAL" \
  "$VARIABLE_STMT" \
  "$LOCAL_GUARD" \
  "$RAW_DISPATCH" \
  "$ASSIGNMENT_DESCENT" \
  "$LOCATED_ASSIGNMENT" \
  "$ASSIGNMENT_TESTS" \
  "$ASSIGNMENT_GUARD" \
  "$RETURN_DESCENT" \
  "$RETURN_TESTS" \
  "$RETURN_OWNER" \
  "$LOCATED_RETURN" \
  "$RETURN_GUARD"

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
  count="$(rg -o -F "$symbol" "$MODULE_LIFECYCLE" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "ordinary collector production edge drift: $symbol count=$count expected=$expected"
  fi
done <<'EOF'
ModuleDraftCollectorV1::default() 1
RawInvocationChildPortV1::new 1
.try_add_functions_atomic 1
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
  'self.builder.build_module'
do
  if rg -n -F "$retired_edge" "$COMPILER" >/dev/null; then
    guard_fail "$TAG" "live compiler build edge returned: $retired_edge"
  fi
done

while IFS=$'\t' read -r file symbol expected; do
  count="$(rg -o -F "$symbol" "$file" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "candidate compiler edge drift: $symbol count=$count expected=$expected"
  fi
done <<EOF
$COMPILER	compile_legacy_candidate	1
$LEGACY_CANDIDATE	.build_module(ast)	1
$LEGACY_CANDIDATE	.finish_built_module	1
$LEGACY_CANDIDATE	.prepare_external_commit()	1
$LEGACY_CANDIDATE	.commit(&mut self.builder)	1
EOF

while IFS=$'\t' read -r file pattern expected label; do
  count="$(rg -o -P "$pattern" "$file" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "$label count drift: count=$count expected=$expected"
  fi
done <<EOF
$STATEMENT_SURFACE	\\bdrive_local_statement_v1\\s*\\(	1	raw/default Local owner caller
$STATEMENT_SURFACE	\\bRawLegacyLocalInputV1::new\\s*\\(	1	raw/default Local owned input
$LOCATED_LOCAL	\\bdrive_local_statement_v1\\s*\\(	1	detached located Local owner caller
EOF

if rg -n -P '\b(?:fn\s+)?build_local_statement\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired build_local_statement facade returned"
fi
if rg -n -P '\b(?:fn\s+)?drive_raw_local_statement_v1\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired drive_raw_local_statement_v1 facade returned"
fi
if rg -n -w 'retry|fallback' "$LOCAL_DESCENT" "$LOCATED_LOCAL" >/dev/null; then
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
$LOCATED_ASSIGNMENT	\\bdrive_variable_assignment_v1\\s*\\(	1	detached located Assignment owner caller
EOF

assignment_external_count="$(
  rg -n -P '\bdrive_variable_assignment_v1\s*\(' \
    "$ROOT_DIR/src" --glob '*.rs' \
    | awk -F ':' \
        -v owner="$ASSIGNMENT_DESCENT" \
        -v tests="$ASSIGNMENT_TESTS" \
        '$1 != owner && $1 != tests { count += 1 } END { print count + 0 }'
)"
if [[ "$assignment_external_count" != "3" ]]; then
  guard_fail "$TAG" \
    "Assignment external owner sites must be two raw/default plus one detached: count=$assignment_external_count"
fi
if rg -n -P '\b(?:fn\s+)?drive_raw_variable_assignment_v1\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired drive_raw_variable_assignment_v1 facade returned"
fi
if rg -n -P '\b(?:fn\s+)?build_grouped_assignment\s*\(' \
  "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
  guard_fail "$TAG" "retired build_grouped_assignment facade returned"
fi
if rg -n -w 'retry|fallback' "$ASSIGNMENT_DESCENT" "$LOCATED_ASSIGNMENT" >/dev/null; then
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
$LOCATED_RETURN	\\bdrive_value_return_statement_v1\\s*\\(	1	detached located Return owner caller
EOF

return_external_count="$(
  rg -n -P '\bdrive_value_return_statement_v1\s*\(' \
    "$ROOT_DIR/src" --glob '*.rs' \
    | awk -F ':' \
        -v owner="$RETURN_DESCENT" \
        -v tests="$RETURN_TESTS" \
        '$1 != owner && $1 != tests { count += 1 } END { print count + 0 }'
)"
if [[ "$return_external_count" != "2" ]]; then
  guard_fail "$TAG" \
    "Return external owner sites must be one raw/default plus one detached: count=$return_external_count"
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

for file in \
  "$LOWERING" \
  "$PORT_OWNER" \
  "$MODULE_LIFECYCLE" \
  "$COMPILER" \
  "$LEGACY_CANDIDATE" \
  "$MODULE_SESSION" \
  "$STATEMENT_SURFACE" \
  "$LOCAL_DESCENT" \
  "$LOCATED_LOCAL" \
  "$VARIABLE_STMT" \
  "$ROOT_DIR/src/mir/builder/stmts/README.md" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/wiring_tests.rs" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/raw_parity_tests.rs" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/if_general.rs" \
  "$ROOT_DIR/src/mir/builder/control_flow/plan/parts/stmt/tests.rs" \
  "$LOCAL_GUARD" \
  "$RAW_DISPATCH" \
  "$ASSIGNMENT_DESCENT" \
  "$LOCATED_ASSIGNMENT" \
  "$ASSIGNMENT_TESTS" \
  "$ROOT_DIR/src/mir/builder/stmts/variable_assignment_raw_tests.rs" \
  "$ROOT_DIR/src/mir/builder/stmts/variable_assignment_parity_tests.rs" \
  "$ROOT_DIR/src/mir/builder/builder_build.rs" \
  "$ASSIGNMENT_GUARD" \
  "$RETURN_DESCENT" \
  "$RETURN_TESTS" \
  "$RETURN_OWNER" \
  "$LOCATED_RETURN" \
  "$ROOT_DIR/src/mir/builder/stmts/return_statement_raw_tests.rs" \
  "$ROOT_DIR/src/mir/builder/stmts/return_statement_parity_tests.rs" \
  "$RETURN_GUARD" \
  "$ROOT_DIR/tools/checks/mirbuilder_inplace_replacement_guard.sh"
do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "file exceeds boundary: ${file#"$ROOT_DIR/"} lines=$lines"
  fi
done

echo "[$TAG] ok"
