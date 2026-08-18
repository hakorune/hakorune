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
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_substring_callout_materializer.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_text_content_root_admission.rs"
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
guard_expect_fixed_in_file "$TAG" 's6c_substring_callout_materializer.rs' "$session" \
  "session must retain the private canonical V9 materializer child"
content_admission="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_text_content_root_admission.rs"
guard_expect_fixed_in_file "$TAG" 'issue_common_v2_s6c_text_content_root_admission_v1' "$content_admission" \
  "base-root mapping must have one named compiler issuer"
guard_expect_fixed_in_file "$TAG" 'V9 remains a derived slice and never becomes a root' "$content_admission" \
  "V9 must remain outside the base-root namespace"
guard_expect_fixed_in_file "$TAG" 'Subject,' "$content_admission" \
  "base-root roles must include an explicit Subject label"
guard_expect_fixed_in_file "$TAG" 'Needle,' "$content_admission" \
  "base-root roles must include an explicit Needle label"

echo "[$TAG] ok (one ingress owner, one session owner, files below 800 lines)"
