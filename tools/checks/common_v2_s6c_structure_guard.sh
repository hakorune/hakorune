#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="common-v2-s6c-structure"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc

files=(
  "$ROOT_DIR/src/mir/loop_recipe_contract/s6c_prephysical_ingress.rs"
  "$ROOT_DIR/src/mir/loop_recipe_contract/s6c_prephysical_ingress_validation.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session_length.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session_segments.rs"
)
guard_require_files "$TAG" "${files[@]}"

for file in "${files[@]}"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source reached the hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

ingress="$ROOT_DIR/src/mir/loop_recipe_contract/s6c_prephysical_ingress.rs"
session="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session.rs"
guard_expect_fixed_in_file "$TAG" 's6c_prephysical_ingress_validation.rs' "$ingress" \
  "ingress must retain the private source-anchor validation child"
guard_expect_fixed_in_file "$TAG" 'common_v2_session_length.rs' "$session" \
  "session must retain the private Length child"
guard_expect_fixed_in_file "$TAG" 'common_v2_session_segments.rs' "$session" \
  "session must retain the private segment child"

echo "[$TAG] ok (one ingress owner, one session owner, files below 800 lines)"
