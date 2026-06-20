#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

root = Path(".")
base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

impls = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()
scope = (root / "src/mir/join_ir/lowering/scope_manager.rs").read_text()
assert "pub fn resolve_promoted_join_id(&self, original_name: &str) -> Option<ValueId>" in impls
assert ".promoted_body_locals" in impls
assert "format!(\"is_{}\", original_name)" in impls
assert "format!(\"is_{}_match\", original_name)" in impls
assert "if let Some(join_id) = carrier.join_id" in impls
assert "self.carrier_info.resolve_promoted_join_id(name)" in scope

resolver = json.loads((base / "hako-lifecycle-resolver-readonly-diagnostics-v0.json").read_text())
deny = {row["id"]: row for row in resolver["deny"]}
join = deny["CarrierVar.join_id.production_lifecycle"]
assert join["decision"] == "DenyUnresolvedBoundary"
assert join["reason"] == "no_production_Some_ValueId_producer"
assert resolver["claims"]["join_id_dependent_paths_allowed"] is False
assert resolver["claims"]["resolver_selection_owner"] is False

facts = json.loads((base / "promoted-body-locals-producer-facts-v0.json").read_text())
assert "promoted name resolution" in facts["denied_followups"]
assert "join_id producer" in facts["denied_followups"]

plan = json.loads((base / "promoted-body-locals-producer-plan-v0.json").read_text())
assert "promoted name resolution" in plan["denied"]
assert "join_id producer" in plan["denied"]
assert plan["behavior"]["join_id_producer"] is False
assert plan["behavior"]["promoted_name_resolution_claim"] is False
assert plan["behavior"]["general_resolver_implemented"] is False
assert plan["behavior"]["converter_emission_added"] is False

oracle = json.loads((base / "promoted-body-locals-producer-oracle-vectors-v0.json").read_text())
assert "promoted_name_resolution" in oracle["denied_vectors"]
assert "join_id_assignment" in oracle["denied_vectors"]
assert oracle["promotion_scope"]["join_id_producer"] is False
assert oracle["promotion_scope"]["promoted_name_resolution_claim"] is False

doc = Path("docs/development/current/main/design/promoted-name-resolution-deny-closeout.md").read_text()
assert "promoted_name_resolution_closed_as_deny=1" in doc
assert "resolution_allowed=0" in doc
assert "join_id_producer=0" in doc
assert "PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER" in doc
PY

cat <<'REPORT'
output_contract=rust-lifecycle-promoted-name-resolution-deny-v0
resolve_promoted_join_id_requires_join_id=1
scope_manager_consumes_resolution=1
resolver_denies_join_id_production=green
join_id_dependent_paths_allowed=0
producer_fixtures_deny_promoted_name_resolution=green
resolution_allowed=0
join_id_producer=0
implementation_started=0
summary=ok
REPORT
