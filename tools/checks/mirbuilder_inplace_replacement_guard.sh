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
  "$MODULE_SESSION"

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

for file in \
  "$LOWERING" \
  "$PORT_OWNER" \
  "$MODULE_LIFECYCLE" \
  "$COMPILER" \
  "$LEGACY_CANDIDATE" \
  "$MODULE_SESSION" \
  "$ROOT_DIR/tools/checks/mirbuilder_inplace_replacement_guard.sh"
do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "file exceeds boundary: ${file#"$ROOT_DIR/"} lines=$lines"
  fi
done

echo "[$TAG] ok"
