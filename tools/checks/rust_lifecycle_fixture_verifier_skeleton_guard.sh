#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TOOL="tools/rust_lifecycle/verify_lifecycle_fixture.py"
CARD="docs/development/current/main/phases/phase-296x/296x-1476-HAKO-LIFECYCLE-FIXTURE-VERIFIER-SKELETON-001.md"

test -x "$TOOL"
test -f "$CARD"

python3 "$TOOL" --case binding-context >/tmp/hakorune_lifecycle_binding_verify.out
python3 "$TOOL" --case variable-context >/tmp/hakorune_lifecycle_variable_verify.out
python3 "$TOOL" --case all >/tmp/hakorune_lifecycle_all_verify.out

grep -q "summary=ok" /tmp/hakorune_lifecycle_binding_verify.out
grep -q "summary=ok" /tmp/hakorune_lifecycle_variable_verify.out
grep -q "summary=ok" /tmp/hakorune_lifecycle_all_verify.out

grep -q "fixture_verifier_skeleton_exists=1" "$CARD"
grep -q "binding_context_case_verified=1" "$CARD"
grep -q "variable_context_case_verified=1" "$CARD"
grep -q "rustc_toolchain_integration_started=0" "$CARD"
grep -q "resolver_implementation_started=0" "$CARD"
grep -q "emitter_implementation_started=0" "$CARD"
grep -q "converter_core_changed=0" "$CARD"
grep -q "backend_behavior_changed=0" "$CARD"

if rg -n "import subprocess|os\\.system|Popen\\(|run\\(" "$TOOL"; then
  echo "unexpected process execution API in fixture-only verifier" >&2
  exit 1
fi

cat <<'REPORT'
output_contract=rust-lifecycle-fixture-verifier-skeleton-v0
fixture_verifier_skeleton_exists=1
binding_context_case_verified=1
variable_context_case_verified=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
summary=ok
REPORT
