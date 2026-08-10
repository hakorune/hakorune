#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="normal-callable-source-carrier-cutover"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RUNNER="$ROOT_DIR/src/runner/modes/mir.rs"
ROOT_OWNER="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
REQUEST="$ROOT_DIR/src/mir/compiler/normal_default_pipeline.rs"

guard_require_command "$TAG" awk
guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$RUNNER" "$ROOT_OWNER" "$REQUEST"

runner_region="$({
  awk '
    /pub\(crate\) fn execute_mir_mode/ { active = 1 }
    /pub\(crate\) fn execute_mir_json_minimal/ { active = 0 }
    active { print }
  ' "$RUNNER"
})"

expect_runner_token() {
  local token="$1"
  if ! rg -q -F "$token" <<<"$runner_region"; then
    guard_fail "$TAG" "named execute_mir_mode caller missing token: $token"
  fi
}

reject_runner_token() {
  local token="$1"
  if rg -q -F "$token" <<<"$runner_region"; then
    guard_fail "$TAG" "named execute_mir_mode caller restored retired token: $token"
  fi
}

expect_runner_token "parse_normal_callable_program_with_build_config"
expect_runner_token "transform_normal_callable_program_v1"
expect_runner_token "NormalCallableTransformOutcomeV1::SourceBacked"
expect_runner_token "for_mir_mode_callable_source"
expect_runner_token "NormalCallableTransformOutcomeV1::Compatibility"

reject_runner_token "self.parse_source"
reject_runner_token "maybe_expand_and_dump"
reject_runner_token "for_mir_mode(ast"
reject_runner_token "retry"
reject_runner_token "fallback"

guard_expect_fixed_in_file "$TAG" \
  "source: PreparedNormalDefaultProgramSourceV1" \
  "$ROOT_OWNER" \
  "prepared normal root must own one source carrier"
guard_expect_fixed_in_file "$TAG" \
  "Callable(VerifiedFinalCallableProgramSourceV1)" \
  "$ROOT_OWNER" \
  "prepared normal root missing final callable source variant"
guard_expect_fixed_in_file "$TAG" \
  "Compatibility(ASTNode)" \
  "$ROOT_OWNER" \
  "prepared normal root missing typed compatibility variant"
guard_expect_fixed_in_file "$TAG" \
  "for_mir_mode_callable_source" \
  "$REQUEST" \
  "normal compile request missing callable source constructor"

for file in "$RUNNER" "$ROOT_OWNER" "$REQUEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"
