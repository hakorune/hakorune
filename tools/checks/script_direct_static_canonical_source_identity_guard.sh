#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

card="docs/development/current/main/investigations/script-direct-static-call-canonical-source-identity-i0-2026-08-21.md"
files=(
  src/mir/compiler/normal_source_plan/mod.rs
  src/mir/compiler/normal_source_plan/product.rs
  src/mir/compiler/normal_source_plan/rejection.rs
  src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs
  src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
  src/runner/reference/normal_file_vm_frontdoor/source_plan_input_tests.rs
  src/parser/normal_callable_program_source/model.rs
)

for file in "${files[@]}"; do
  [[ -f "$file" ]] || {
    echo "missing source-identity file: $file" >&2
    exit 1
  }
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || {
    echo "760-line split trigger exceeded: $file ($lines)" >&2
    exit 1
  }
done

require() {
  local pattern="$1"
  local file="$2"
  rg -q -- "$pattern" "$file" || {
    echo "missing source-identity contract: $pattern in $file" >&2
    exit 1
  }
}

require 'CanonicalParserSourceHandoffV1' \
  src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs
require 'NormalParserSourceLineageV1::issue' \
  src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs
require 'validate_parser_identity' \
  src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
require 'NormalSourcePlanIdentityFieldV1' \
  src/mir/compiler/normal_source_plan/rejection.rs
require 'SourceAuthorityUnavailable' \
  src/mir/compiler/normal_source_plan/rejection.rs
require 'CompatibilitySourceUnavailable' \
  src/mir/compiler/normal_source_plan/rejection.rs
require 'SourceLineageUnavailable' \
  src/mir/compiler/normal_source_plan/rejection.rs
require 'parser_lineage' \
  src/mir/compiler/normal_source_plan/product.rs

for field in SourceIdentity Digest GrammarProfile Utf8Length ReadCount ParseCount; do
  require "$field" src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
done

for state in NotApplicable CanonicalSourceBacked AstOnlyFixture CompatibilitySource LineageUnavailable IdentityInvalid Transported; do
  require "$state" "$card"
done
require 'NoSafeSlice' "$card"

if rg -n 'parse_from_string|read_to_string|read[[:space:]]*\(' \
  src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs; then
  echo "source-plan identity validation must not reparse or reread bytes" >&2
  exit 1
fi

if rg -n 'NormalParserSourceLineageV1::issue' \
  src/mir/compiler/normal_source_plan \
  --glob '*.rs' --glob '!compatibility_origin.rs' \
  src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs; then
  echo "source-plan identity owners must not issue or reissue parser lineage" >&2
  exit 1
fi

issuer_files="$(rg -l 'NormalParserSourceLineageV1::issue' src --glob '*.rs' || true)"
while IFS= read -r file; do
  [[ -n "$file" ]] || continue
  case "$file" in
    src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs|\
    src/runner/modes/common_util/normal_callable.rs|\
    *tests.rs|*test_support.rs|*compatibility_origin.rs)
      ;;
    *)
      echo "unexpected parser-lineage issuer outside named ingress: $file" >&2
      exit 1
      ;;
  esac
done <<< "$issuer_files"

echo "script direct-static canonical source-identity guard: PASS"
