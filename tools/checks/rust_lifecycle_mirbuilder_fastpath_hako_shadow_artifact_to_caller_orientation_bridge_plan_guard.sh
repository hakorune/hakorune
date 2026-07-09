#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-fastpath-hako-shadow-artifact-to-caller-orientation-bridge-plan-guard"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-fastpath-hako-shadow-artifact-to-caller-orientation-bridge-plan-v0.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3360-MIRBUILDER-FASTPATH-HAKO-SHADOW-ARTIFACT-TO-CALLER-ORIENTATION-BRIDGE-PLAN-001.md"
SHADOW_RS="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
DESCRIPTORS="$ROOT/src/mir/generated/generic_method_route_descriptors.rs"
HAKO_POLICY="$ROOT/lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$SHADOW_RS" "$WRITE_ROUTES" "$DESCRIPTORS" "$HAKO_POLICY" "$TASK_ORDER"

python3 - "$FIXTURE" "$CARD" "$SHADOW_RS" "$WRITE_ROUTES" "$DESCRIPTORS" "$HAKO_POLICY" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
shadow_rs = Path(sys.argv[3]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[4]).read_text(encoding="utf-8")
descriptors = Path(sys.argv[5]).read_text(encoding="utf-8")
hako_policy = Path(sys.argv[6]).read_text(encoding="utf-8")
task_order = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-FASTPATH-HAKO-SHADOW-ARTIFACT-TO-CALLER-ORIENTATION-BRIDGE-PLAN-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-I64-001"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderFastpathHakoShadowArtifactToCallerOrientationBridgePlanV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(fixture.get("selected_next_card") == next_card, "bad selected next")

decision = fixture.get("decision") or {}
need(decision.get("selected_path") == "C_SHADOW_TYPED_ARTIFACT_FIRST_THEN_HAKO_CALLER_ORIENTATION", "bad selected path")
need(decision.get("short_term") == "RustConsumesCheckedInGeneratedTypedHakoArtifactAsShadow", "bad short term")
need(decision.get("long_term") == "HakoCallerOrientationWithRustAsHostOracleCompatChecker", "bad long term")
need(decision.get("bootstrap_policy") == "NoBuildRsHakoruneCompilerInvocationFirst", "bad bootstrap policy")

target = fixture.get("generated_artifact_target") or {}
need(target.get("first_surface") == "SetSurfacePolicy/MapStoreI64", "bad first surface")
need(target.get("target_artifact_kind") == "CheckedInGeneratedTypedRustArtifact", "bad artifact kind")
need(target.get("runtime_build_parses_hako_source_text") is False, "runtime build must not parse hako source text")

claims = fixture.get("claims") or {}
for key in [
    "fastpath_hako_shadow_artifact_to_caller_orientation_bridge_plan",
    "current_include_str_split_connection_debt_recorded",
    "selected_checked_in_generated_typed_artifact_shadow_consume",
    "selected_long_term_hako_caller_orientation",
    "rust_authority_retained",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "build_rs_hako_compiler_invocation",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "new_backend_route",
    "new_abi",
    "runtime_fallback",
    "live_hako_authority",
    "caller_orientation_runtime_path",
    "source_text_parsing_as_authority",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "include_str!",
    "write_set_mapstore_i64_policy_classifier.hako",
    "row.split('|')",
    "find(\"\\\"map_store_i64_set_surface|\")",
]:
    need(needle in shadow_rs, f"current debt marker missing: {needle}")
for needle in [
    "scalar_known_hako_shadow::mapstore_i64_shadow_consumed_decision()",
    "GenericMethodRouteKind::MapStoreI64",
]:
    need(needle in write_routes, f"live shadow consume path missing: {needle}")
for needle in [
    "descriptor_for_route_kind",
    "GenericMethodRouteKind::MapStoreI64",
    "tag: \"map_store_i64\"",
    "value_demand: \"write_any\"",
]:
    need(needle in descriptors, f"generated route descriptor context missing: {needle}")
need("map_store_i64_set_surface|SetSurfacePolicy|MapStoreI64" in hako_policy, "hako policy row missing")

for needle in [
    token,
    next_card,
    "C_SHADOW_TYPED_ARTIFACT_FIRST_THEN_HAKO_CALLER_ORIENTATION",
    "build_rs_hako_compiler_invocation = 0",
    "source_text_parsing_as_authority = 0",
]:
    need(needle in card, f"card missing: {needle}")
for needle in [token, next_card]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-fastpath-hako-shadow-artifact-to-caller-orientation-bridge-plan-guard-v0
token=MIRBUILDER-FASTPATH-HAKO-SHADOW-ARTIFACT-TO-CALLER-ORIENTATION-BRIDGE-PLAN-001
selected_path=C_SHADOW_TYPED_ARTIFACT_FIRST_THEN_HAKO_CALLER_ORIENTATION
current_include_str_split_connection_debt_recorded=1
selected_checked_in_generated_typed_artifact_shadow_consume=1
selected_long_term_hako_caller_orientation=1
build_rs_hako_compiler_invocation=0
rust_authority_retained=1
hako_runtime_route_authority=0
rust_fastpath_rewired=0
route_selection_authority_switch=0
backend_lowering_authority=0
runtime_mutation_authority=0
publication_execution=0
new_backend_route=0
new_abi=0
runtime_fallback=0
live_hako_authority=0
caller_orientation_runtime_path=0
source_text_parsing_as_authority=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-I64-001
summary=ok
REPORT
