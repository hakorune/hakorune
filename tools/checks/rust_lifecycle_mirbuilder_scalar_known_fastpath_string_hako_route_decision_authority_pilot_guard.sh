#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-string-hako-route-decision-authority-pilot"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-string-hako-route-decision-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_string_hako_route_decision_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3393-MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
STRING_ROUTES="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" \
  "$MANIFEST" "$SHADOW_SOURCE" "$STRING_ROUTES"

python3 "$TOOL" --check
cargo test -q scalar_known_hako_shadow

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$STRING_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
shadow_source = Path(sys.argv[5]).read_text(encoding="utf-8")
string_routes = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-AUTHORITY-PILOT-RERUN-001"
authority_fn = "string_scalar_i64_hako_route_authority_pilot_decision"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathStringHakoRouteDecisionAuthorityPilotV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in card, "card missing next token")
need(authority_fn in shadow_source, "authority helper missing")
need(string_routes.count(authority_fn) == 3, "String live routes must call authority helper three times")

implementation = fixture.get("implementation") or {}
need(implementation.get("surface") == "StringScalarI64Routes", "surface drift")
need(implementation.get("authority_function") == authority_fn, "authority function drift")
need(implementation.get("live_route_calls_authority_function") is True, "live route call drift")
need(implementation.get("rust_oracle_compat_checker") is True, "rust oracle drift")
need(implementation.get("mismatch_policy") == "FailFast", "mismatch policy drift")

summary = fixture.get("summary") or {}
for key in [
    "string_hako_route_decision_authority_pilot",
    "string_hako_authority_result_consumed",
    "string_rust_oracle_compat_checker",
    "string_mismatch_fail_fast",
    "string_live_route_calls_authority_pilot",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in ["scalar_known_hako_runtime_route_authority", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "scalar_known_hako_runtime_route_authority",
    "scalar_known_transport_axis_authority_switch",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "caller_orientation_runtime_path",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
need(token in rows_by_token, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-string-hako-route-decision-authority-pilot")
print("string_hako_route_decision_authority_pilot=1")
print("string_rust_oracle_compat_checker=1")
print("string_mismatch_fail_fast=1")
print("scalar_known_hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
