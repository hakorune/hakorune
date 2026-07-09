#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-string-caller-orientation-contract-artifact"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-string-caller-orientation-contract-artifact-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_string_caller_orientation_contract_artifact.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3422-MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001.md"
CONTRACT="$ROOT/lang/src/compiler/lib/string_scalar_i64_caller_orientation_contract.hako"
POLICY="$ROOT/lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako"
ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/string_scalar_i64_caller_orientation_contract.rs"
POLICY_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs"
GENERATED_MOD="$ROOT/src/mir/generic_method_route_plan/generated/mod.rs"
GENERATOR="$ROOT/tools/rust_lifecycle/generate_string_scalar_i64_caller_orientation_contract.py"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" diff
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$CONTRACT" "$POLICY" "$ARTIFACT" \
  "$POLICY_ARTIFACT" "$GENERATED_MOD" "$GENERATOR" "$SHADOW" "$MANIFEST"

python3 "$TOOL" --check
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT
python3 "$GENERATOR" > "$TMP"
diff -u "$ARTIFACT" "$TMP"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$CONTRACT" "$POLICY" "$ARTIFACT" "$POLICY_ARTIFACT" "$GENERATED_MOD" "$SHADOW" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
card = Path(sys.argv[3]).read_text(encoding="utf-8")
contract = Path(sys.argv[4]).read_text(encoding="utf-8")
policy = Path(sys.argv[5]).read_text(encoding="utf-8")
artifact_path = Path(sys.argv[6])
artifact = artifact_path.read_text(encoding="utf-8")
policy_artifact = Path(sys.argv[7]).read_text(encoding="utf-8")
generated_mod = Path(sys.argv[8]).read_text(encoding="utf-8")
shadow = Path(sys.argv[9]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[10]).read_text(encoding="utf-8"))


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001"
need(fixture.get("token") == token, "token drift")
need(token in card, "card token drift")
need(manifest.get("current_blocker_token") == token, "manifest current blocker drift")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")

row_ids = [
    "string_indexof_scalar_i64_routes",
    "string_lastindexof_scalar_i64_routes",
    "string_contains_scalar_i64_routes",
]
expected = {
    "orientation_kind": "CallerOrientationContractMetadataOnly",
    "scope": "SingleSurface",
    "runtime_consumer": "Forbidden",
    "backend_lowering_consumer": "Forbidden",
    "mutation_consumer": "Forbidden",
    "publication_consumer": "Forbidden",
    "mismatch_policy": "FailFast",
}
contract_data = fixture.get("contract") or {}
need(contract_data.get("policy_row_ids") == row_ids, "contract row set drift")
for key, value in expected.items():
    need(contract_data.get(key) == value, f"fixture contract drift: {key}")
    expected_count = 12 if value == "Forbidden" else 3
    need(contract.count(value) == expected_count, f"contract value count drift: {value}")
    need(value in artifact, f"artifact contract value missing: {value}")
for row_id in row_ids:
    need(contract.count(row_id) == 1, f"contract row ID count drift: {row_id}")
    need(row_id in policy, f"policy row missing: {row_id}")
    need(row_id in artifact, f"artifact row ID missing: {row_id}")
need(
    sum(line.strip().startswith('"string_') and "StringScalarI64Routes" in line for line in policy.splitlines()) == 3,
    "policy surface count drift",
)
need("pub(super) mod string_scalar_i64_caller_orientation_contract;" in generated_mod, "generated module missing")
need("STRING_SEARCH_SCALAR_I64_HAKO_POLICIES" in shadow, "String Rust oracle policy missing")

for forbidden in [
    "STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS",
    "HakoStringCallerOrientationContract",
]:
    need(forbidden not in shadow, f"live route oracle consumed caller contract: {forbidden}")

for source in (root / "src/mir").rglob("*.rs"):
    if source == artifact_path:
        continue
    need(
        "STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS"
        not in source.read_text(encoding="utf-8"),
        f"live consumer registered caller contract: {source.relative_to(root)}",
    )

claims = fixture.get("claims") or {}
for key in [
    "string_caller_orientation_hako_contract_materialized",
    "string_caller_orientation_generated_typed_artifact",
    "string_caller_orientation_policy_row_set_verified",
    "string_caller_orientation_artifact_current",
    "string_caller_orientation_no_live_consumer_guard",
    "string_hako_route_decision_authority_retained",
    "string_rust_oracle_compat_checker_retained",
    "string_mismatch_fail_fast",
]:
    need(claims.get(key) == 1, f"claim drift: {key}")
for key in [
    "caller_orientation_runtime_path",
    "caller_runtime_dispatch_authority",
    "route_selection_authority_switch",
    "hako_runtime_route_authority",
    "scalar_known_hako_runtime_route_authority",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "collection_caller_orientation_authority",
    "delete_hako_route_decision_authority_pilot",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-string-caller-orientation-contract-artifact")
print("string_caller_orientation_hako_contract_materialized=1")
print("string_caller_orientation_generated_typed_artifact=1")
print("string_caller_orientation_policy_row_set_verified=1")
print("string_caller_orientation_no_live_consumer_guard=1")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
