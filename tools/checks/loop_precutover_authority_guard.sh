#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="loop-precutover-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

BUILDER_DIR="$ROOT_DIR/src/mir/builder"
RECURSIVE="$BUILDER_DIR/recursive_child_lowering.rs"
RAW_LOOP="$BUILDER_DIR/raw_loop_child_entry.rs"
RAW_LOOP_PORT="$BUILDER_DIR/raw_loop_child_port.rs"
LOAN_PORT="$BUILDER_DIR/normal_callable_semantic_loan_port.rs"
SOURCE_SCOPE="$BUILDER_DIR/normal_callable_semantic_loan_port/source_scope.rs"
CATALOGED_INSTANCE_SCOPE="$BUILDER_DIR/normal_callable_semantic_loan_port/cataloged_instance_scope.rs"
ROOT_LOWERING="$BUILDER_DIR/program_root_lowering.rs"
DEMAND_ROOT="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe"
ROUTING="$BUILDER_DIR/control_flow/joinir/routing.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$RECURSIVE" "$RAW_LOOP" "$RAW_LOOP_PORT" "$LOAN_PORT" "$SOURCE_SCOPE" "$CATALOGED_INSTANCE_SCOPE" "$ROOT_LOWERING" \
  "$ROUTING"  \


guard_expect_fixed_in_file "$TAG" \
  "NormalCallableSemanticPackageMode::Installed" "$ROOT_LOWERING" \
  "installed package must remain the selected callable ingress"
guard_expect_fixed_in_file "$TAG" \
  "with_selected_lowering_input" "$LOAN_PORT" \
  "selected lowering must borrow the package-owned semantic input"
guard_expect_fixed_in_file "$TAG" \
  "input.semantic()" "$SOURCE_SCOPE" \
  "the package semantic variant must be consumed at the lowering boundary"
guard_expect_fixed_in_file "$TAG" \
  "lower_loop_or_freeze_v1" "$RAW_LOOP" \
  "the pre-cutover legacy physical terminal must remain explicit"
guard_expect_fixed_in_file "$TAG" \
  "try_cf_loop_joinir" "$ROUTING" \
  "the current legacy physical route must remain identifiable"

# The ordinary compatibility AST/JoinIR terminal remains explicit. Both
# compatibility feeders reach the same route entry; route_loop owns its
# current selected winner and physicalizer. The selected Dynamic branch must
# not use either edge; its package adapter handoff remains its sole owner.
compatibility_callers=()
saw_raw_loop_entry=0
saw_raw_loop_port=0
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    "$ROUTING"|*_tests.rs) continue ;;
  esac
  compatibility_callers+=("$file")
  [[ "$file" == "$RAW_LOOP" ]] && saw_raw_loop_entry=1
  [[ "$file" == "$RAW_LOOP_PORT" ]] && saw_raw_loop_port=1
done < <(rg -l --glob '*.rs' -F 'lower_loop_or_freeze_v1(' "$BUILDER_DIR" || true)
if [[ "${#compatibility_callers[@]}" -ne 2 || "$saw_raw_loop_entry" -ne 1 || "$saw_raw_loop_port" -ne 1 ]]; then
  guard_fail "$TAG" "compatibility route-entry callers drifted; expected raw_loop_child_entry.rs and raw_loop_child_port.rs"
fi

guard_expect_fixed_in_file "$TAG" \
  'dynamic-instance-route' "$CATALOGED_INSTANCE_SCOPE" \
  "cataloged instance/Dynamic mismatch must fail before the ordinary route"


for file in "$RECURSIVE" "$RAW_LOOP" "$RAW_LOOP_PORT" "$LOAN_PORT" "$SOURCE_SCOPE" "$CATALOGED_INSTANCE_SCOPE" "$ROOT_LOWERING" "$ROUTING"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "pre-cutover authority file reached 800-line boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (current compatibility/source authority; private selected emitter retired)"
