#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-push-write-hako-route-decision-authority-pilot"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-push-write-hako-route-decision-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_push_write_hako_route_decision_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3408-MIRBUILDER-SCALAR-KNOWN-FASTPATH-PUSH-WRITE-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$WRITE_ROUTES"

python3 "$TOOL" --check
cargo test -q scalar_known_hako_shadow

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$WRITE_ROUTES" <<'PY'
import json, sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
shadow_source = Path(sys.argv[5]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[6]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-PUSH-WRITE-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-PUSH-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001"
authority_fn = "write_push_hako_route_authority_pilot_decision"
need(fixture.get("kind") == "MirBuilderScalarKnownFastpathPushWriteHakoRouteDecisionAuthorityPilotV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card and next_token in card, "card token drift")
need(authority_fn in shadow_source and authority_fn in write_routes, "live authority helper missing")
need("Push .hako authority pilot diverged from Rust oracle" in shadow_source, "fail-fast oracle compare missing")
summary = fixture.get("summary") or {}
for key in ["push_hako_route_decision_authority_pilot", "push_hako_authority_result_consumed", "push_live_route_calls_authority_pilot", "push_rust_oracle_compat_checker", "push_mismatch_fail_fast"]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in ["runtime_mutation_authority", "publication_execution", "write_wide_authority", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")
claims = fixture.get("claims") or {}
for key in ["any_write_boundary_opened", "mapstoreany_authority", "mapdeleteany_authority", "runtime_fallback", "new_backend_route", "new_abi"]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
need(token in {row.get("token") for row in manifest.get("rows") or []}, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-push-write-hako-route-decision-authority-pilot")
print("push_hako_route_decision_authority_pilot=1")
print("push_rust_oracle_compat_checker=1")
print("push_mismatch_fail_fast=1")
print("runtime_mutation_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
