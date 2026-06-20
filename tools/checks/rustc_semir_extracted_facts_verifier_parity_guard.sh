#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

BINDING_FACTS="$TMP_DIR/binding-context-adapter-facts-v0.json"
VARIABLE_FACTS="$TMP_DIR/variable-context-adapter-facts-v0.json"

python3 tools/rust_lifecycle/extract_binding_context_facts.py --emit-json > "$BINDING_FACTS"
python3 tools/rust_lifecycle/extract_variable_context_facts.py --emit-json > "$VARIABLE_FACTS"

python3 tools/rust_lifecycle/verify_lifecycle_fixture.py \
  --case all \
  --binding-context-facts "$BINDING_FACTS" \
  --variable-context-facts "$VARIABLE_FACTS" >/dev/null

git diff --quiet -- docs/development/current/main/design/fixtures/rust-lifecycle

cat <<'REPORT'
output_contract=rustc-semir-extracted-facts-verifier-parity-v0
extracted_facts_verifier_parity_green=1
binding_context_generated_facts_verified=1
variable_context_generated_facts_verified=1
checked_in_fixtures_unchanged=1
hako_policy_owner=0
backend_behavior_changed=0
summary=ok
REPORT
