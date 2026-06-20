#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import re
from pathlib import Path

root = Path(".")
doc = (root / "docs/development/current/main/design/promoted-carrier-identity-policy-decision.md").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1442-PROMOTED-CARRIER-IDENTITY-POLICY-DECISION-001.md").read_text()
inventory = (root / "docs/development/current/main/design/promoted-carrier-identity-join-id-design-inventory.md").read_text()
types = (root / "src/mir/join_ir/lowering/carrier_info/types.rs").read_text()
impls = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()
boundary = (root / "src/mir/join_ir/lowering/inline_boundary_builder.rs").read_text()

assert "selected_policy=condition_binding_identity" in doc
assert "join_id_producer_selected=0" in doc
assert "implementation_started=0" in doc
assert "policy_decision_recorded=1" in doc
assert "condition_binding_rewrite_added=0" in doc
assert "trim_route_lowering_still_denied=1" in doc
assert "selected_policy=condition_binding_identity" in card
assert "CarrierVar_join_id_producer_selected=0" in card
assert "do_not_implement_condition_binding_resolution=1" in card
assert "condition_binding_identity_path_present=1" in inventory

assert "pub join_id: Option<ValueId>" in types
assert "pub fn resolve_promoted_join_id" in impls
assert "ParamRole::Condition" in boundary
assert "ConditionBinding" in boundary
assert "join_value" in boundary

def strip_cfg_test_modules(text):
    lines = text.splitlines()
    out = []
    skip = False
    depth = 0
    pending = False
    for line in lines:
        stripped = line.strip()
        if not skip and stripped == "#[cfg(test)]":
            pending = True
            continue
        if pending and stripped.startswith("mod tests"):
            skip = True
            pending = False
            depth = line.count("{") - line.count("}")
            if depth <= 0:
                skip = False
            continue
        if pending:
            out.append("#[cfg(test)]")
            pending = False
        if skip:
            depth += line.count("{") - line.count("}")
            if depth <= 0:
                skip = False
            continue
        out.append(line)
    return "\n".join(out)

rust_files = [
    p for p in (root / "src/mir").rglob("*.rs")
    if p.name != "tests.rs" and "tests" not in p.parts
]
production = "\n".join(strip_cfg_test_modules(p.read_text()) for p in rust_files)
assert "join_id: Some(" not in production
assert not re.search(r"\.join_id\s*=", production)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-promoted-carrier-identity-policy-v0
policy_decision_recorded=1
selected_policy=condition_binding_identity
join_id_producer_added=0
condition_binding_rewrite_added=0
trim_route_lowering_still_denied=1
backend_behavior_changed=0
summary=ok
REPORT
