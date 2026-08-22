#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

carrier="src/mir/compiler/normal_source_plan/compatibility_origin.rs"
source_plan="src/mir/compiler/normal_source_plan/mod.rs"
materializer="src/runner/modes/common_util/normal_callable.rs"
request="src/mir/compiler/normal_default_pipeline.rs"
root_lifecycle="src/mir/builder/normal_default_root_catalog_lifecycle.rs"
mir_runner="src/runner/modes/mir.rs"
llvm_runner="src/runner/product/llvm/mir_compiler.rs"
llvm_options="src/runner/product/llvm/compile_options.rs"
module_readme="src/mir/compiler/normal_source_plan/README.md"

for path in "$carrier" "$source_plan" "$materializer" "$request" \
  "$root_lifecycle" "$mir_runner" "$llvm_runner" "$llvm_options" "$module_readme"; do
  test -f "$path"
done

grep -q 'pub(crate) struct NormalCallableCompatibilityOriginV1' "$carrier"
grep -q 'ast: ASTNode' "$carrier"
grep -q 'reason: NormalCallableTransformCompatibilityV1' "$carrier"
grep -q 'lineage: NormalParserSourceLineageV1' "$carrier"
grep -q 'ExpectedProgramRoot' "$carrier"
grep -q 'TypedCompatibility' "$root_lifecycle"
grep -q 'NormalCallableMaterializationOutcomeV1' "$materializer"
grep -q 'NormalCallableCompatibilityOriginV1::issue' "$materializer"
grep -q 'for_mir_mode_compatibility' "$mir_runner"
grep -q 'for_llvm_compatibility' "$llvm_runner"
grep -q 'NormalCallableCompatibilityOriginV1' "$module_readme"

issue_count=0
while IFS= read -r path; do
  [[ "$path" == "$carrier" ]] && continue
  issue_count=$((issue_count + 1))
done < <(rg -l 'NormalCallableCompatibilityOriginV1::issue' src --glob '*.rs' || true)
test "$issue_count" -eq 1

if rg -n 'NormalCallableCompatibilityOriginV1.*Clone|impl Clone for NormalCallableCompatibilityOriginV1' "$carrier"; then
  echo "compatibility origin carrier must remain non-Clone" >&2
  exit 1
fi

if rg -n '^[[:space:]]*(pub\(crate\)[[:space:]]+)?(use|fn|struct|enum|type).*\b(Recipe|Join|FunctionCall|Brand)\b' "$carrier"; then
  echo "transport carrier gained semantic or physical authority" >&2
  exit 1
fi

if rg -n 'unwrap_or\(|unwrap_or_default\(|Option::None|reason:\s*_reason' \
  "$carrier" "$materializer" "$mir_runner" "$llvm_runner" "$llvm_options"; then
  echo "compatibility origin was collapsed through a default or discarded reason" >&2
  exit 1
fi

for path in "$carrier" "$source_plan" "$materializer" "$request" "$root_lifecycle" \
  "$mir_runner" "$llvm_runner" "$llvm_options" "$module_readme"; do
  lines="$(wc -l < "$path")"
  test "$lines" -lt 760
done

echo "callable compatibility source transport guard: PASS"
