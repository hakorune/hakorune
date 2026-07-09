#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-collection-caller-orientation-contract-artifact"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-collection-caller-orientation-contract-artifact-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_collection_caller_orientation_contract_artifact.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3423-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001.md"
CONTRACT="$ROOT/lang/src/compiler/lib/collection_scalar_i64_caller_orientation_contract.hako"
POLICY="$ROOT/lang/src/compiler/lib/collection_len_scalar_i64_policy_classifier.hako"
ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/collection_scalar_i64_caller_orientation_contract.rs"
POLICY_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"
GENERATED_MOD="$ROOT/src/mir/generic_method_route_plan/generated/mod.rs"
GENERATOR="$ROOT/tools/rust_lifecycle/generate_collection_scalar_i64_caller_orientation_contract.py"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" diff
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$CONTRACT" "$POLICY" "$ARTIFACT" \
  "$POLICY_ARTIFACT" "$GENERATED_MOD" "$GENERATOR" "$SHADOW" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT
python3 "$GENERATOR" > "$TMP"
diff -u "$ARTIFACT" "$TMP"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$CONTRACT" "$POLICY" "$ARTIFACT" "$POLICY_ARTIFACT" "$GENERATED_MOD" "$SHADOW" "$TASK_ORDER" "$MANIFEST" <<'PY'
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
task_order = Path(sys.argv[10]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[11]).read_text(encoding="utf-8"))


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-CONSUMER-DESIGN-STOP-001"
row_ids = [
    "collection_map_entry_count_scalar_i64_routes",
    "collection_array_slot_len_scalar_i64_routes",
    "collection_string_len_scalar_i64_routes",
    "collection_any_length_scalar_i64_routes",
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

need(fixture.get("token") == token, "token drift")
need(token in card, "card token drift")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
need((fixture.get("decision") or {}).get("selected_next_card") == next_card, "selected next card drift")
need(next_card in task_order, "task chain pointer drift")
contract_data = fixture.get("contract") or {}
need(contract_data.get("policy_row_ids") == row_ids, "contract row set drift")
for key, value in expected.items():
    need(contract_data.get(key) == value, f"fixture contract drift: {key}")
    need(value in artifact, f"artifact contract value missing: {value}")
for row_id in row_ids:
    need(contract.count(row_id) == 1, f"contract row ID count drift: {row_id}")
    need(policy.count(row_id) == 1, f"policy row count drift: {row_id}")
    need(artifact.count(row_id) == 1, f"artifact row ID count drift: {row_id}")
need(sum(line.strip().startswith('"collection_') for line in policy.splitlines()) == 4, "policy row count drift")
need('route_kind: GenericMethodRouteKind::AnyLength' in policy_artifact, "AnyLength policy artifact missing")
need('receiver_domain: "Box"' in policy_artifact, "AnyLength Box policy boundary missing")
need("pub(super) mod collection_scalar_i64_caller_orientation_contract;" in generated_mod, "generated module missing")
need("collection_scalar_i64_hako_route_authority_pilot_decision" in shadow, "Collection Rust oracle missing")
need('Some(receiver_domain)' in shadow, "Collection receiver domain checker missing")
need('GenericMethodRouteKind::AnyLength' in shadow and '"Box"' in shadow, "AnyLength Box guard missing")

for source in (root / "src/mir").rglob("*.rs"):
    if source == artifact_path:
        continue
    need(
        "COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS" not in source.read_text(encoding="utf-8"),
        f"live consumer registered caller contract: {source.relative_to(root)}",
    )

claims = fixture.get("claims") or {}
for key in [
    "collection_caller_orientation_hako_contract_materialized",
    "collection_caller_orientation_generated_typed_artifact",
    "collection_caller_orientation_policy_row_set_verified",
    "collection_caller_orientation_artifact_current",
    "collection_caller_orientation_no_live_consumer_guard",
    "collection_hako_route_decision_authority_retained",
    "collection_rust_oracle_compat_checker_retained",
    "collection_mismatch_fail_fast",
]:
    need(claims.get(key) == 1, f"claim drift: {key}")
for key in [
    "caller_orientation_runtime_path", "caller_runtime_dispatch_authority",
    "route_selection_authority_switch", "hako_runtime_route_authority",
    "scalar_known_hako_runtime_route_authority", "receiver_domain_authority_switch",
    "receiver_domain_widening_authority", "any_length_wildcard_selector",
    "runtime_box_domain_fallback", "backend_lowering_authority",
    "runtime_mutation_authority", "publication_execution",
    "collection_to_scalar_known_wide_authority", "delete_hako_route_decision_authority_pilot",
    "runtime_fallback", "new_backend_route", "new_abi", "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-collection-caller-orientation-contract-artifact")
print("collection_caller_orientation_generated_typed_artifact=1")
print("collection_caller_orientation_policy_row_set_verified=1")
print("collection_caller_orientation_no_live_consumer_guard=1")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
