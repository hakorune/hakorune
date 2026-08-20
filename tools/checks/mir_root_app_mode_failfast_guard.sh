#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

tag="mir-root-app-mode-failfast-guard"
lifecycle="src/mir/builder/raw_expression_dispatch/nonmain_static_box_lifecycle.rs"
tests="src/mir/builder/raw_expression_dispatch/tests.rs"
card="docs/development/current/main/investigations/mir-root-app-mode-undecided-failfast-d0-2026-08-21.md"
readme="src/mir/builder/README.md"

for file in "$lifecycle" "$tests" "$card" "$readme"; do
  test -f "$file"
done

grep -q 'Some(is_app_mode) => Ok(is_app_mode)' "$lifecycle"
grep -q '\[freeze:contract\]\[mir/root-app-mode/undecided\]' "$lifecycle"
grep -q 'prepared_root_app_mode_v1(builder)?' "$lifecycle"
test "$(rg -c 'prepared_root_app_mode_v1\(builder\)\?' "$lifecycle")" -eq 2

if rg -n 'root_is_app_mode\.unwrap_or|root_is_app_mode.*unwrap_or' "$lifecycle"; then
  echo "$tag: implicit root-mode default remains" >&2
  exit 1
fi

grep -q 'raw_nonmain_static_box_app_mode_is_void_without_registration' "$tests"
grep -q 'raw_nonmain_static_box_undecided_mode_freezes_before_registration' "$tests"
grep -q 'normal_nonmain_static_box_undecided_mode_freezes_before_registration' "$tests"
grep -q 'Some(true).*App' "$card"
grep -q 'None.*lifecycle contract error' "$card"
grep -q 'Root app-mode boundary P0' "$readme"

test "$(wc -l < "$lifecycle")" -lt 760
test "$(wc -l < "$tests")" -lt 760
test "$(wc -l < "$readme")" -lt 760

echo "$tag: PASS"
