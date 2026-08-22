#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

tag="script-direct-static-canonical-source-a-envelope-guard"
envelope=src/mir/compiler/canonical_script_source_plan_envelope.rs
request=src/mir/compiler/canonical_core_source_plan_request.rs
dispatch=src/mir/compiler/canonical_core_dispatch.rs
frontdoor=src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
parser_witness=src/parser/callable_parameter_source/parser_invocation_witness.rs
rows=src/parser/callable_parameter_source/script_source_rows_model.rs
card=docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-source-envelope-i0-2026-08-21.md

for file in "$envelope" "$request" "$dispatch" "$frontdoor" "$parser_witness" "$rows" "$card"; do
  [[ -f "$file" ]] || { echo "$tag: missing $file" >&2; exit 1; }
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || { echo "$tag: 760-line trigger crossed: $file ($lines)" >&2; exit 1; }
done

envelope_lines="$(wc -l < "$envelope" | tr -d '[:space:]')"
(( envelope_lines < 300 )) || {
  echo "$tag: envelope child must stay below 300 lines ($envelope_lines)" >&2
  exit 1
}

rg -q 'CanonicalScriptSourcePlanEnvelopeV1' "$envelope" "$frontdoor" "$dispatch"
rg -q 'CanonicalCoreSourcePlanInputV1' "$request" "$frontdoor" "$dispatch"
rg -q 'from_plan_and_transport' "$request" "$frontdoor"
rg -q 'SourceEnvelopeReady' "$request" "$dispatch" "$card"
rg -q 'Rejected' "$request"
rg -q 'MovedToParallelHandoff' "$rows"
rg -q 'DiscardedBeforeA' "$card" "$dispatch"

if rg -n '^[[:space:]]*(use|pub\([^)]*\)[[:space:]]+use).*\b(ASTNode|ValueId|MirType|Builder|comp_ctx|Recipe|Join)\b' "$envelope"; then
  echo "$tag: envelope imports semantic or physical authority" >&2
  exit 1
fi
if rg -n '^[[:space:]]*script_input[[:space:]]*:' "$request"; then
  echo "$tag: request retains an untyped script_input field" >&2
  exit 1
fi
if rg -n 'Canonical(Core|Script).*HandoffConsumed[[:space:]]*=' src/mir/compiler src/runner/reference/normal_file_vm_frontdoor; then
  echo "$tag: compiler transport must not issue parser/A HandoffConsumed" >&2
  exit 1
fi
if rg -n 'parse_from_string|read_to_string|ASTNode|source_identity.*digest.*pair' "$envelope"; then
  echo "$tag: envelope must not reparse, rescan, or infer identity" >&2
  exit 1
fi

rg -q 'Exhaustive transport state table' "$card"
rg -q 'NoSafeSlice' "$card"
rg -q 'typed Script envelope' "$card"
rg -q 'one move-only' "$card"
rg -q 'SourceEnvelopeReady' "$card"

echo "$tag: PASS"
