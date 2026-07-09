#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-non-delete-write-policy-row-identity-transport"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-non-delete-write-policy-row-identity-transport-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_non_delete_write_policy_row_identity_transport.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3435-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-POLICY-ROW-IDENTITY-TRANSPORT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" diff
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$ROOT" "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[1])
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
card = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-POLICY-ROW-IDENTITY-TRANSPORT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001"
expected = [
    ("mapstore_i64", "map_store_i64_set_surface"),
    ("push_arrayappendany", "array_append_any_push_surface"),
    ("mapstore_any", "map_store_any_set_surface"),
]
need(fixture.get("token") == token, "token drift")
need(token in card, "card token missing")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
need((fixture.get("decision") or {}).get("next_card") == next_card, "next card drift")
need(next_card in card and next_card in task_order, "next task pointer missing")
surfaces = fixture.get("surfaces") or []
need([(row.get("name"), row.get("policy_row_id")) for row in surfaces] == expected, "three-row identity drift")
need((fixture.get("scope") or {}).get("delete_included") is False, "Delete entered transport scope")
need((fixture.get("scope") or {}).get("caller_contract_semantics_copied") is False, "caller semantics copied")
for row in surfaces:
    source = root / row["policy_source"]
    generator = root / row["generator"]
    artifact = root / row["typed_artifact"]
    source_text = source.read_text(encoding="utf-8")
    artifact_text = artifact.read_text(encoding="utf-8")
    tmp = root / ".tmp-row-identity-transport.rs"
    try:
        generated = subprocess.check_output(["python3", str(generator)], text=True)
        need(generated == artifact_text, f"generated artifact stale: {artifact.relative_to(root)}")
    finally:
        tmp.unlink(missing_ok=True)
    row_id = row["policy_row_id"]
    need(source_text.count(row_id) == 1, f"source row identity drift: {row_id}")
    need(artifact_text.count(f'policy_row_id: "{row_id}"') == 1, f"artifact row identity drift: {row_id}")
    need("route_selection_authority_switch" not in artifact_text, "route claim leaked")
    need("caller_orientation_runtime_path" not in artifact_text, "runtime claim leaked")
claims = fixture.get("claims") or {}
for key in [
    "non_delete_write_policy_row_identity_transport", "mapstore_i64_policy_row_identity_transported",
    "push_arrayappendany_policy_row_identity_transported", "mapstore_any_policy_row_identity_transported",
    "exact_three_row_set_verified",
]:
    need(claims.get(key) == 1, f"claim drift: {key}")
for key in [
    "caller_orientation_runtime_path", "route_selection_authority_switch", "backend_lowering_authority",
    "write_mutation_authority", "runtime_mutation_authority", "publication_execution",
    "delete_hako_route_decision_authority_pilot", "write_wide_authority", "scalar_known_wide_authority",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-non-delete-write-policy-row-identity-transport")
print("non_delete_write_policy_row_identity_transport=1")
print("exact_three_row_set_verified=1")
print("delete_hako_route_decision_authority_pilot=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
