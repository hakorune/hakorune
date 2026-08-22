#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

tag="script-direct-static-canonical-source-a-carrier-guard"
carrier=src/mir/compiler/canonical_script_source_a_input.rs
request=src/mir/compiler/canonical_core_source_plan_request.rs
dispatch=src/mir/compiler/canonical_core_dispatch.rs
frontdoor=src/runner/reference/normal_file_vm_frontdoor/script_source_input.rs
plan=src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
card=docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-carrier-i0-2026-08-21.md

for file in "$carrier" "$request" "$dispatch" "$frontdoor" "$plan" "$card"; do
  [[ -f "$file" ]] || { echo "$tag: missing $file" >&2; exit 1; }
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || { echo "$tag: 760-line trigger crossed: $file ($lines)" >&2; exit 1; }
done

rg -q 'enum CanonicalScriptSourceAInputTransportV1' "$carrier"
rg -q 'HandoffReady|DiscardedBeforeA|HandoffConsumed|DispositionTransported' "$carrier"
rg -q 'CanonicalScriptSourceAInputTransportV1' "$request" "$plan" "$dispatch"
rg -q 'into_compiler_transport' "$frontdoor" "$plan"
rg -q 'discard_before_a_consumer' "$dispatch" "$plan"
if rg -n 'script_input:[[:space:]]*_' "$plan"; then
  echo "$tag: silent script-input drop remains" >&2
  exit 1
fi
if rg -n 'HandoffConsumed[[:space:]]*=' "$carrier" "$frontdoor" "$dispatch" | grep -v '=>'; then
  echo "$tag: no-A transport must not issue HandoffConsumed" >&2
  exit 1
fi
if rg -n 'ASTNode|ValueId|MirType|Builder|comp_ctx|Recipe|Join' "$carrier"; then
  echo "$tag: carrier imports semantic or physical authority" >&2
  exit 1
fi
rg -q 'exhaustive|Exhaustive' "$card"
rg -q 'NoSafeSlice' "$card"
rg -q 'HandoffReady' "$card"
rg -q 'DiscardedBeforeA' "$card"
rg -q 'HandoffConsumed' "$card"

echo "$tag: PASS"
