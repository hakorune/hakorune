#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-caller-orientation-contract-artifact"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-caller-orientation-contract-artifact-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_write_set_mapstore_i64_caller_orientation_contract_artifact.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3432-MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001.md"
CONTRACT="$ROOT/lang/src/compiler/lib/write_set_mapstore_i64_caller_orientation_contract.hako"
POLICY="$ROOT/lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako"
ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_caller_orientation_contract.rs"
POLICY_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
GENERATED_MOD="$ROOT/src/mir/generic_method_route_plan/generated/mod.rs"
GENERATOR="$ROOT/tools/rust_lifecycle/generate_write_set_mapstore_route_policy.py"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" diff
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$CONTRACT" "$POLICY" "$ARTIFACT" "$POLICY_ARTIFACT" "$GENERATED_MOD" "$GENERATOR" "$SHADOW" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT
python3 "$GENERATOR" --artifact i64_caller > "$TMP"
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

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-PUSH-ARRAYAPPENDANY-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001"
row_id = "map_store_i64_set_surface"
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
# This historical artifact card is retained for provenance; current sequencing
# is owned by the active 3456 card and CURRENT_STATE.toml.
data = fixture.get("contract") or {}
need(data.get("policy_row_ids") == [row_id], "contract row set drift")
for key, value in expected.items():
    need(data.get(key) == value, f"fixture contract drift: {key}")
    need(value in artifact, f"artifact contract value missing: {value}")
need(contract.count(row_id) == 1, "contract row ID count drift")
need(policy.count(row_id) == 1, "policy row count drift")
need(artifact.count(row_id) == 1, "artifact row ID count drift")
for value in ["effect_class", "mutation_class"]:
    need(value not in artifact, f"caller artifact copied policy field: {value}")
for value in ["mutate", "MutatesReceiverOrContainer", "ScalarI64"]:
    need(value not in artifact, f"caller artifact copied policy value: {value}")
need("effect_class" in policy_artifact and "mutation_class" in policy_artifact, "policy artifact semantics missing")
need("pub(super) mod write_set_mapstore_i64_caller_orientation_contract;" in generated_mod, "generated module missing")
need("mapstore_i64_hako_route_authority_pilot_decision" in shadow, "MapStoreI64 Rust oracle missing")
for source in (root / "src/mir").rglob("*.rs"):
    if source == artifact_path:
        continue
    need("WRITE_SET_MAPSTORE_I64_CALLER_ORIENTATION_CONTRACT" not in source.read_text(encoding="utf-8"), f"live consumer registered caller contract: {source.relative_to(root)}")
claims = fixture.get("claims") or {}
for key in [
    "mapstore_i64_caller_orientation_hako_contract_materialized",
    "mapstore_i64_caller_orientation_generated_typed_artifact",
    "mapstore_i64_caller_orientation_policy_row_reference_verified",
    "mapstore_i64_caller_orientation_artifact_current",
    "mapstore_i64_caller_orientation_no_live_consumer_guard",
    "mapstore_i64_hako_route_decision_authority_retained",
    "mapstore_i64_rust_oracle_compat_checker_retained",
    "mapstore_i64_mismatch_fail_fast",
]:
    need(claims.get(key) == 1, f"claim drift: {key}")
for key in [
    "mutation_metadata_copied_to_caller_contract", "effect_metadata_copied_to_caller_contract",
    "value_boundary_copied_to_caller_contract", "caller_orientation_live_consumer",
    "caller_orientation_runtime_path", "route_selection_authority_switch", "hako_runtime_route_authority",
    "scalar_known_hako_runtime_route_authority", "write_mutation_authority", "runtime_mutation_authority",
    "publication_execution", "delete_hako_route_decision_authority_pilot", "write_wide_authority",
    "scalar_known_wide_authority", "backend_lowering_authority", "runtime_fallback", "new_backend_route",
    "new_abi", "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-caller-orientation-contract-artifact")
print("mapstore_i64_caller_orientation_generated_typed_artifact=1")
print("mapstore_i64_caller_orientation_no_live_consumer_guard=1")
print("write_mutation_authority=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
