#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

flags="src/config/env/builder_flags.rs"
handler="src/mir/builder/method_call_handlers.rs"
tests="src/mir/builder/method_call_handlers_tests.rs"
card="docs/development/current/main/investigations/me-call-arity-failfast-d0-2026-08-21.md"
reference="docs/reference/environment-variables.md"

test -f "$flags"
test -f "$handler"
test -f "$tests"
test -f "$card"
test -f "$reference"

grep -q 'env_bool_default("NYASH_ME_CALL_ARITY_STRICT", true)' "$flags"
grep -q 'validate_prepared_me_arity_before_descent' "$handler"
grep -q 'me_arity_error' "$handler"
grep -q 'NYASH_ME_CALL_ARITY_STRICT=0' "$card"
grep -q 'NYASH_ME_CALL_ARITY_STRICT.*ON' "$reference"

if rg -n 'unwrap_or\(|unwrap_or_default\(' "$handler"; then
  echo "me arity handler introduced a defaulting route" >&2
  exit 1
fi
if rg -n 'ASTNode::MethodCall|ASTNode::Call' "$handler"; then
  echo "me arity row added a second method-call matcher" >&2
  exit 1
fi

test "$(wc -l < "$handler")" -lt 760
test "$(wc -l < "$tests")" -lt 760
test "$(wc -l < "$flags")" -lt 760

echo "me-call arity fail-fast guard: PASS"
