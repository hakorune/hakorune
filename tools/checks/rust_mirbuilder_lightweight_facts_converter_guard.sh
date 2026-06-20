#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family binding-context --check
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family variable-context-simple-map --check

cat <<'REPORT'
output_contract=rust-mirbuilder-lightweight-facts-converter-v0
binding_context=green
variable_context_simple_map=green
summary=ok
REPORT
