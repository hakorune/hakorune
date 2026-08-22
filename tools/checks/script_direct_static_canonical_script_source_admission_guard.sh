#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

issuer=src/parser/callable_parameter_source/canonical_script_source_admission.rs
product=src/parser/callable_parameter_source/product.rs
catalog=src/parser/callable_parameter_source/catalog.rs
postpass=src/parser/postpass_envelope.rs
card=docs/development/current/main/investigations/script-direct-static-call-canonical-script-source-admission-i0-2026-08-21.md

for file in "$issuer" "$product" "$catalog" "$postpass" "$card"; do
  [[ -f "$file" ]] || { echo "[script-admission] missing $file" >&2; exit 1; }
done

for file in "$issuer" "$product" "$catalog" "$postpass"; do
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || {
    echo "[script-admission] 760-line split trigger exceeded: $file ($lines)" >&2
    exit 1
  }
done

rg -q 'issue_canonical_script_cohort' "$product"
rg -q 'CanonicalScriptCohortAdmitted' "$issuer"
rg -q 'program_cohort_for_admission' "$postpass"
rg -q 'parser_brand' "$catalog"
rg -q 'Exhaustive top-level shape table' "$card"
rg -q 'Exhaustive admission state table' "$card"
rg -q 'NoBoxDeclarations' "$card"

if rg -n 'CompletedParserPostpassV1::is_source_backed|completed\.is_source_backed' "$issuer"; then
  echo "[script-admission] old boolean cannot issue the canonical admission" >&2
  exit 1
fi
issuer_impl="$(mktemp)"
trap 'rm -f "$issuer_impl"' EXIT
sed '/^#\[cfg(test)\]/,$d' "$issuer" > "$issuer_impl"
if rg -n 'use .*mir|NormalSourcePlan|comp_ctx|ValueId|MirType|parse_from_string' "$issuer_impl"; then
  echo "[script-admission] parser admission crossed into a downstream authority" >&2
  exit 1
fi
if rg -n 'CanonicalScriptCohortAdmissionV1.*Clone|derive\([^]]*Clone[^]]*\).*CanonicalScriptCohortAdmissionV1' "$issuer"; then
  echo "[script-admission] admission witness must remain non-Clone" >&2
  exit 1
fi
if rg -n '\b_\s*=>' "$issuer_impl"; then
  echo "[script-admission] wildcard shape fallback detected" >&2
  exit 1
fi

echo "script direct-static canonical Script source admission guard: PASS"
