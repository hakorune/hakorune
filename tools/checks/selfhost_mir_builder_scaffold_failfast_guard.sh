#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

VERIFY="lang/src/selfhost/mir_builder/verify.hako"
PHI="lang/src/selfhost/mir_builder/phi.hako"
README="lang/src/selfhost/mir_builder/README.md"

if rg -n "always OK|always ok|return 0\\s*//\\s*0=ok|return dst|placeholder" "$VERIFY" "$PHI"; then
  echo "selfhost_mir_builder_scaffold_failfast=fail silent placeholder remains" >&2
  exit 1
fi

if rg -n "always OK|always ok|v0 always ok" "$README"; then
  echo "selfhost_mir_builder_scaffold_failfast=fail README claims verifier success" >&2
  exit 1
fi

rg -q "\\[selfhost/mir_builder/verify:unimplemented\\]" "$VERIFY"
rg -q "return 2" "$VERIFY"
rg -q "\\[selfhost/mir_builder/phi:unimplemented:merge2\\]" "$PHI"
rg -q "\\[selfhost/mir_builder/phi:unimplemented:mergeN\\]" "$PHI"
rg -q "Do not report scaffolded MIR verification as success" "$README"

cat <<'REPORT'
output_contract=selfhost-mir-builder-scaffold-failfast-v0
selfhost_mir_verify_always_ok=0
selfhost_phi_placeholder_return_dst=0
scaffold_calls_fail_fast=1
summary=ok
REPORT
