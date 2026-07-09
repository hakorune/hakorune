#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-string-caller-orientation-live-assert-consumer"
source "$ROOT/tools/checks/lib/guard_common.sh"

CARD="$ROOT/docs/development/current/main/phases/phase-296x/3428-MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MODULE="$ROOT/src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW"

python3 - "$CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" <<'PY'
import json
import sys
from pathlib import Path

card = Path(sys.argv[1]).read_text(encoding="utf-8")
task_order = Path(sys.argv[2]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
module = Path(sys.argv[4]).read_text(encoding="utf-8")
shadow = Path(sys.argv[5]).read_text(encoding="utf-8")


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001"
need(token in card, "card token missing")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
need(next_card in task_order, "next task pointer missing")
need("pub(super) fn assert_string_policy_row(policy_row_id: &str)" in module, "assertion signature drift")
need("assert_string_policy_row(policy.policy_row_id);" in shadow, "String live assertion call missing")
need("STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS" in module, "generated contract not consumed")
for forbidden in ["route_kind", "core_op", "receiver_domain", "GenericMethodRouteDecision"]:
    need(f"{forbidden}:" not in module, f"forbidden caller input leaked: {forbidden}")
for forbidden in [
    "caller_orientation_runtime_path", "route_selection_authority_switch",
    "hako_runtime_route_authority", "backend_lowering_authority",
    "runtime_mutation_authority", "publication_execution", "source_selfhost_claim",
]:
    need(forbidden not in module, f"forbidden claim leaked: {forbidden}")
need(module.count("#[test]") == 6, "caller assertion test count drift")
need(module.count("#[should_panic") == 4, "caller assertion rejection test count drift")
need(len(module.splitlines()) < 800, "caller orientation module exceeds 800 lines")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-string-caller-orientation-live-assert-consumer")
print("string_caller_orientation_live_assert_consumer=1")
print("string_three_row_exact=1")
print("assertion_only=1")
print("caller_orientation_runtime_path=0")
print("route_selection_authority_switch=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

cargo test -q caller_orientation
