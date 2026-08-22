#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

tag="script-cataloged-box-root-partition-i0"
neutral=src/mir/builder/normal_script_neutral_window.rs
neutral_tests=src/mir/builder/normal_script_neutral_window_tests.rs
lookup_tests=src/mir/builder/normal_script_direct_static_lookup_tests.rs
lifecycle_tests=src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs
root_traversal=src/mir/resolved_semantics/shadow/root_traversal.rs
card=docs/development/current/main/investigations/script-cataloged-box-root-partition-i0-2026-08-22.md

for file in "$neutral" "$neutral_tests" "$lookup_tests" "$lifecycle_tests" \
  "$root_traversal" "$card"; do
  [[ -f "$file" ]] || {
    echo "[$tag] missing $file" >&2
    exit 1
  }
done

for file in "$neutral" "$neutral_tests" "$lookup_tests" "$lifecycle_tests"; do
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || {
    echo "[$tag] 760-line split trigger exceeded: $file ($lines)" >&2
    exit 1
  }
done

# The source issuer owns the partition.  Lookup only consumes the existing
# transfer semantic and never learns the AST declaration shape.
rg -q 'CatalogedNonMainStaticBox' "$neutral"
rg -q 'validate_cataloged_static_box_source' "$neutral"
rg -q 'StaticCallableCatalogTransfer' "$neutral"
rg -q 'selected_callable_sources' "$neutral"
rg -q 'SelectedNormalCallableSourceSiteV1::ProgramBoxMethod' "$neutral"
rg -q 'selected_callable_sources\.site' "$neutral"

# A BoxDeclaration must not be repaired or skipped by the observer itself.
if rg -n 'BoxDeclaration' "$root_traversal"; then
  echo "[$tag] root observer mentions BoxDeclaration; source partition leaked downstream" >&2
  exit 1
fi
rg -q 'ScriptRootSemanticDispositionV1::Transferred\(_\)' "$root_traversal"

# Focused evidence pins the real parser-scan shape and the next existing
# blocker after root lookup, without claiming method-body publication.
rg -q 'parser_scan_loop_box_catalog_transfer' "$neutral_tests"
rg -q 'cataloged_box_has_complete_empty_root_coverage' "$lookup_tests"
rg -q 'callable-semantic-lowering/missing-variable-site' "$lifecycle_tests"
rg -q 'SCRIPT-CATALOGED-BOX-ROOT-PARTITION-I0' "$card"
rg -q 'CompleteEmpty' "$card"
rg -q 'no .*publication' "$card"

echo "$tag guard: PASS"
