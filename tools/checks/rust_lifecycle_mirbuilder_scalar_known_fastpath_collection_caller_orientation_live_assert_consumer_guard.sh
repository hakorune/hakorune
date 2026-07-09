#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-collection-caller-orientation-live-assert-consumer"
source "$ROOT/tools/checks/lib/guard_common.sh"

CARD="$ROOT/docs/development/current/main/phases/phase-296x/3429-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MODULE="$ROOT/src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
POLICY_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" "$POLICY_ARTIFACT"

python3 - "$CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" "$POLICY_ARTIFACT" <<'PY'
import json
import sys
from pathlib import Path

card = Path(sys.argv[1]).read_text(encoding="utf-8")
task_order = Path(sys.argv[2]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
module = Path(sys.argv[4]).read_text(encoding="utf-8")
shadow = Path(sys.argv[5]).read_text(encoding="utf-8")
policy_artifact = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001"
need(token in card, "card token missing")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
need(next_card in task_order, "next task pointer missing")
need("pub(super) fn assert_collection_policy_row(policy_row_id: &str)" in module, "assertion signature drift")
need("assert_collection_policy_row(policy.policy_row_id);" in shadow, "Collection live assertion call missing")
need("COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS" in module, "generated contract not consumed")
for forbidden in ["route_kind", "core_op", "receiver_domain", "GenericMethodRouteDecision"]:
    need(f"{forbidden}:" not in module, f"forbidden caller input leaked: {forbidden}")
for forbidden in [
    "caller_orientation_runtime_path", "route_selection_authority_switch",
    "receiver_domain_authority_switch", "receiver_domain_widening_authority",
    "any_length_wildcard_selector", "runtime_box_domain_fallback",
    "backend_lowering_authority", "runtime_mutation_authority",
    "publication_execution", "source_selfhost_claim",
]:
    need(forbidden not in module, f"forbidden claim leaked: {forbidden}")
for row_id in [
    "collection_map_entry_count_scalar_i64_routes",
    "collection_array_slot_len_scalar_i64_routes",
    "collection_string_len_scalar_i64_routes",
    "collection_any_length_scalar_i64_routes",
]:
    need(module.count(row_id) == 1, f"Collection row identity drift: {row_id}")
need('route_kind: GenericMethodRouteKind::AnyLength' in policy_artifact, "AnyLength policy row lost")
need('receiver_domain: "Box"' in policy_artifact, "AnyLength Box boundary lost")
for name in [
    "collection_assertion_accepts_all_existing_policy_rows",
    "collection_assertion_rejects_unknown_policy_row",
    "collection_assertion_rejects_metadata_drift",
]:
    need(f"fn {name}(" in module, f"Collection assertion test missing: {name}")
need(len(module.splitlines()) < 800, "caller orientation module exceeds 800 lines")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-collection-caller-orientation-live-assert-consumer")
print("collection_caller_orientation_live_assert_consumer=1")
print("collection_four_row_exact=1")
print("anylength_box_boundary_retained=1")
print("assertion_only=1")
print("caller_orientation_runtime_path=0")
print("receiver_domain_authority_switch=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

cargo test -q caller_orientation
