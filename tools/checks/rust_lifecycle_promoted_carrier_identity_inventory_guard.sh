#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import re
from pathlib import Path

root = Path(".")
doc = (root / "docs/development/current/main/design/promoted-carrier-identity-join-id-design-inventory.md").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1440-PROMOTED-CARRIER-IDENTITY-JOIN-ID-DESIGN-INVENTORY-001.md").read_text()
types = (root / "src/mir/join_ir/lowering/carrier_info/types.rs").read_text()
impls = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()
boundary = (root / "src/mir/join_ir/lowering/inline_boundary_builder.rs").read_text()
decision = (root / "docs/development/current/main/design/phi-carrier-join-id-vocabulary-decision.md").read_text()

assert "promoted_carrier_identity_inventory=1" in doc
assert "production_join_id_producer=0" in doc
assert "condition_binding_identity_path_present=1" in doc
assert "selected_implementation=none" in doc
assert "trim_route_lowering_still_denied=1" in doc
assert "do not implement join_id producer in this inventory" in doc
assert "do_not_implement_join_id_producer=1" in card

assert "pub join_id: Option<ValueId>" in types
assert "pub promoted_body_locals: Vec<String>" in types
assert "pub fn resolve_promoted_join_id" in impls
assert "format!(\"is_{}\", original_name)" in impls
assert "format!(\"is_{}_match\", original_name)" in impls
assert "if let Some(join_id) = carrier.join_id" in impls
assert "ParamRole::Condition" in boundary
assert "ConditionBinding" in boundary
assert "join_value" in boundary
assert "production_producer=0" in decision

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
assert "join_id: None" in production
assert "join_id: Some(" not in production
assert not re.search(r"\.join_id\s*=", production)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-promoted-carrier-identity-inventory-v0
promoted_carrier_identity_inventory=1
production_join_id_producer=0
production_join_id_assignment=0
condition_binding_identity_path_present=1
selected_implementation=none
trim_route_lowering_still_denied=1
backend_behavior_changed=0
summary=ok
REPORT
