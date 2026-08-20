#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

frontdoor=src/runner/reference/normal_file_vm_frontdoor.rs
handoff=src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs
plan_input=src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
plan_product=src/mir/compiler/normal_source_plan/product.rs
parser_mod=src/parser/mod.rs
postpass=src/parser/postpass_envelope.rs
string_entry=src/parser/string_postpass_entry.rs
lineage=src/parser/normal_callable_program_source/model.rs
parser_product=src/parser/callable_parameter_source/product.rs
handoff_product=src/mir/compiler/normal_source_plan/parser_callable_source_handoff.rs

for file in "$frontdoor" "$handoff" "$plan_input" "$plan_product" "$postpass" "$string_entry" "$lineage" "$parser_product" "$handoff_product"; do
  [[ -f "$file" ]] || { echo "missing parser-handoff file: $file" >&2; exit 1; }
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || { echo "760-line split trigger exceeded: $file ($lines)" >&2; exit 1; }
done

[[ "$(wc -l < "$parser_mod")" -eq 762 ]] || {
  echo "parser/mod.rs must remain visibility-only in this row" >&2
  exit 1
}

rg -q 'string_postpass_entry::parse_with_callable_parameter_source' "$frontdoor"
if rg -n 'parse_from_string_with_build_config|parse_from_string_with_source_seal' "$frontdoor"; then
  echo "frontdoor must not issue a second AST/source-seal parser" >&2
  exit 1
fi
rg -q 'parser_source_handoff: CanonicalParserSourceHandoffV1' "$frontdoor"
rg -q 'pub\(crate\) mod postpass_envelope' "$parser_mod"
rg -q 'pub\(crate\) mod string_postpass_entry' "$parser_mod"
rg -q 'pub\(crate\) struct CompletedParserPostpassV1' "$postpass"
rg -q 'pub\(crate\) fn parse_with_callable_parameter_source' "$string_entry"
rg -q 'ParserBacked\(NormalParserCallableSourceHandoffV1\)' "$plan_product"
rg -q 'PreparedNormalSourcePlanInputV1::from_parser_callable_source' "$plan_input"
rg -q 'parser_source_handoff.into_parts' "$plan_input"
rg -q 'pub\(crate\) fn parser_postpass' "$plan_product"
rg -q 'CanonicalParserSourceHandoffV1' "$handoff"
rg -q 'NormalParserSourceLineageV1' "$lineage"
rg -q 'ParserCallableSourceDispositionV1' "$parser_product"
rg -q 'NormalParserCallableSourceHandoffV1' "$handoff_product"

if rg -U -n '#\[derive\([^]]*Clone[^]]*\)\][[:space:]]*pub\(crate\) struct CanonicalParserSourceHandoffV1' "$handoff"; then
  echo "parser source handoff must remain non-Clone" >&2
  exit 1
fi
if rg -n 'ASTNode::.*source|source_path|display_name.*parser|parse_from_string' "$handoff"; then
  echo "handoff child must not reissue or infer parser identity" >&2
  exit 1
fi
if rg -n 'parse_postpass\(' "$frontdoor"; then
  echo "frontdoor must consume the callable-aware parser product" >&2
  exit 1
fi

echo "script direct-static canonical parser source handoff guard: PASS"
