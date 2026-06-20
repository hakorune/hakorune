#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
import re
from pathlib import Path

source = Path("crates/hakorune_mir_builder/src/variable_context.rs")
definition_lines = []
external_code_uses = []

for root in [Path("crates"), Path("src")]:
    for path in root.rglob("*.rs"):
        text = path.read_text()
        for lineno, line in enumerate(text.splitlines(), 1):
            if "variable_map_mut(" not in line:
                continue
            if path == source and re.search(r"pub\s+fn\s+variable_map_mut\s*\(", line):
                definition_lines.append((path, lineno))
            else:
                external_code_uses.append((path, lineno, line.strip()))

assert len(definition_lines) == 1, definition_lines
assert external_code_uses == [], external_code_uses

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

def load(name):
    return json.loads((base / name).read_text())

facts = [
    load("variable-context-immutable-borrow-facts-v0.json"),
    load("variable-context-snapshot-restore-facts-v0.json"),
]

for doc in facts:
    denied = {
        row["id"]: row["deny_reason"]
        for row in doc.get("denied_methods", [])
    }
    assert denied["VariableContext::variable_map_mut"] == "ReturnedMutableBorrow"

plans = [
    load("variable-context-immutable-borrow-plan-v0.json"),
    load("variable-context-snapshot-restore-plan-v0.json"),
]

for doc in plans:
    assert "VariableContext::variable_map_mut" in set(doc.get("denied", []))
    behavior = doc.get("behavior", {})
    assert behavior.get("general_resolver_implemented") is False
    assert behavior.get("carrier_phi_claim") is False
    assert behavior.get("full_variable_context_claim") is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-mutable-map-deny-v0
external_variable_map_mut_callsite_count=0
deny_reason=ReturnedMutableBorrow
existing_fixtures_keep_variable_map_mut_denied=green
rust_api_changed=0
carrier_PHI_claim=0
general_resolver_implemented=0
full_VariableContext_parity_claim=0
summary=ok
REPORT
