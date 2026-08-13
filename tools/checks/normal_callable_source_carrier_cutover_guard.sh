#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="normal-callable-source-carrier-cutover"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RUNNER="$ROOT_DIR/src/runner/modes/mir.rs"
LLVM_RUNNER="$ROOT_DIR/src/runner/product/llvm/mod.rs"
HELPER="$ROOT_DIR/src/runner/modes/common_util/normal_callable.rs"
ROOT_OWNER="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
REQUEST="$ROOT_DIR/src/mir/compiler/normal_default_pipeline.rs"

guard_require_command "$TAG" awk
guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$RUNNER" "$LLVM_RUNNER" "$HELPER" "$ROOT_OWNER" "$REQUEST"

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

expect_runner_token "materialize_normal_callable_program_v1"
expect_runner_token "NormalCallableTransformOutcomeV1::SourceBacked"
expect_runner_token "for_mir_mode_callable_source"
expect_runner_token "NormalCallableTransformOutcomeV1::Compatibility"
expect_runner_token "for_mir_mode(ast"

reject_runner_token "parse_normal_callable_program_with_build_config"
reject_runner_token "transform_normal_callable_program_v1"
reject_runner_token "retry"
reject_runner_token "fallback"

llvm_region="$({
  awk '
    /pub\(crate\) fn execute_llvm_mode/ { active = 1 }
    active { print }
  ' "$LLVM_RUNNER"
})"
if ! rg -q -F "materialize_normal_callable_program_v1" <<<"$llvm_region"; then
  guard_fail "$TAG" "LLVM mode must use the shared normal-callable materialization helper"
fi
if rg -q -F "normalize_core_pass" <<<"$llvm_region"; then
  guard_fail "$TAG" "LLVM mode must not normalize SourceBacked input outside the shared helper"
fi

guard_expect_fixed_in_file "$TAG" \
  "parse_normal_callable_program_with_build_config" \
  "$HELPER" \
  "shared helper must own the callable parser frontdoor"
guard_expect_fixed_in_file "$TAG" \
  "transform_normal_callable_program_v1" \
  "$HELPER" \
  "shared helper must own the callable transform"
guard_expect_fixed_in_file "$TAG" \
  "normalize_core_pass" \
  "$HELPER" \
  "Compatibility-only normalization must remain in the shared helper"
normalize_count="$(rg -o -F "normalize_core_pass" "$HELPER" | wc -l | tr -d '[:space:]')"
if [[ "$normalize_count" != "1" ]]; then
  guard_fail "$TAG" "shared helper must contain exactly one compatibility normalization call (found $normalize_count)"
fi
source_line="$(rg -n -F "NormalCallableTransformOutcomeV1::SourceBacked(source)" "$HELPER" | cut -d: -f1 | head -n1)"
normalize_line="$(rg -n -F "normalize_core_pass" "$HELPER" | cut -d: -f1 | head -n1)"
if [[ -z "$source_line" || -z "$normalize_line" || "$source_line" -ge "$normalize_line" ]]; then
  guard_fail "$TAG" "SourceBacked preservation must be structurally before Compatibility normalization"
fi
guard_expect_fixed_in_file "$TAG" \
  "for_llvm_callable_source" \
  "$REQUEST" \
  "normal compile request missing LLVM callable source constructor"

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

for file in "$RUNNER" "$LLVM_RUNNER" "$HELPER" "$ROOT_OWNER" "$REQUEST"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"
