#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

parent=src/mir/builder/normal_default_root_catalog_lifecycle.rs
child=src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs

test "$(wc -l < "$parent")" -lt 760
test "$(wc -l < "$child")" -lt 760
rg -Fq '#[path = "normal_default_root_catalog_lifecycle_tests.rs"]' "$parent"
rg -Fq 'mod normal_default_root_catalog_lifecycle_tests;' "$parent"
rg -Fq 'fn verified_expansion_disposition_reaches_script_and_app_root_lowering()' "$child"
rg -Fq 'fn actual_string_helpers_general_result_row_reaches_its_first_loop_carrier()' "$child"
if rg -q '^mod tests \{' "$parent"; then
  echo '[normal-default-brand-catalog-lifecycle-split-guard] inline tests returned' >&2
  exit 1
fi

CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust \
  normal_default_root_catalog_lifecycle_tests --lib -- \
  --skip verified_expansion_disposition_reaches_script_and_app_root_lowering \
  --skip source_backed_selected_callable_uses_the_installed_package_port \
  --skip parser_scan_package_reaches_the_existing_physical_blocker_without_fallback \
  --quiet

echo '[normal-default-brand-catalog-lifecycle-split-guard] ok'
