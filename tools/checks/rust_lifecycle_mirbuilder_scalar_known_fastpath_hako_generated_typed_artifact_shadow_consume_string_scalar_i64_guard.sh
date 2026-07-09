#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-string-scalar-i64-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_string_scalar_i64.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3378-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-STRING-SCALAR-I64-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
BASIS="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-string-scalar-i64-v0.json"
GENERATOR="$ROOT/tools/rust_lifecycle/generate_string_search_scalar_i64_hako_policy.py"
GENERATED="$ROOT/src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
STRING_ROUTES="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"
HAKO="$ROOT/lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-string-scalar-i64"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$BASIS" "$GENERATOR" "$GENERATED" "$SHADOW" "$STRING_ROUTES" "$HAKO"

python3 "$TOOL" --check
python3 "$GENERATOR" | diff -u "$GENERATED" -

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$BASIS" "$GENERATED" "$SHADOW" "$STRING_ROUTES" "$HAKO" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
basis = json.load(open(sys.argv[5], encoding="utf-8"))
generated = Path(sys.argv[6]).read_text(encoding="utf-8")
shadow = Path(sys.argv[7]).read_text(encoding="utf-8")
string_routes = Path(sys.argv[8]).read_text(encoding="utf-8")
hako = Path(sys.argv[9]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-STRING-SCALAR-I64-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-005"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeStringScalarI64V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((basis.get("decision") or {}).get("kind") == "SelectStringScalarI64GeneratedTypedArtifactShadowConsumeImplementation", "basis decision drift")

decision = fixture.get("shadow_consumed_decision") or {}
need(decision.get("surface") == "StringScalarI64Routes", "surface drift")
need(decision.get("route_kind_family") == ["StringIndexOf", "StringLastIndexOf", "StringContains"], "route family drift")
need(decision.get("core_ops") == ["StringIndexOf", "StringLastIndexOf", "StringContains"], "core op drift")
need(decision.get("proof_or_policy_sources") == [
    "IndexOfSurfacePolicy",
    "LastIndexOfSurfacePolicy",
    "ContainsSurfacePolicy",
], "policy source drift")
need(decision.get("return_shape") == "ScalarI64", "return shape drift")
need(decision.get("value_demand") == "ScalarI64", "value demand drift")
need(decision.get("publication_policy") == "NoPublication", "publication drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

implementation = fixture.get("implementation") or {}
for key in [
    "checked_in_generated_typed_artifact",
    "string_fastpath_shadow_consumed",
    "rust_hako_policy_match",
    "rust_authority_retained",
]:
    need(implementation.get(key) is True, f"implementation flag drift: {key}")
need(implementation.get("runtime_hako_source_text_parsing") is False, "runtime source parsing drift")

for needle in [
    "pub(crate) struct HakoStringSearchScalarI64Policy",
    "STRING_SEARCH_SCALAR_I64_HAKO_POLICIES",
    "GenericMethodRouteKind::StringIndexOf",
    "GenericMethodRouteKind::StringLastIndexOf",
    "GenericMethodRouteKind::StringContains",
    "GenericMethodRouteProof::IndexOfSurfacePolicy",
    "GenericMethodRouteProof::LastIndexOfSurfacePolicy",
    "GenericMethodRouteProof::ContainsSurfacePolicy",
]:
    need(needle in generated, f"generated artifact missing {needle}")

need("include_str!" not in shadow, "shadow consumer must not parse .hako source text at runtime")
need("split(\"|\")" not in shadow and ".split('|')" not in shadow, "shadow consumer must not split source text")
need("STRING_SEARCH_SCALAR_I64_HAKO_POLICIES" in shadow, "shadow missing generated table")
need("string_scalar_i64_shadow_consumed_decision" in shadow, "shadow missing String decision")
need(shadow.count("assert_hako_string_scalar_i64_policy_matches_rust") >= 4, "shadow missing String assertions/tests")
need(string_routes.count("string_scalar_i64_shadow_consumed_decision") == 3, "string_routes must shadow-consume all three String routes")
for row in [
    "string_indexof_scalar_i64_routes|StringScalarI64Routes|StringIndexOf",
    "string_lastindexof_scalar_i64_routes|StringScalarI64Routes|StringLastIndexOf",
    "string_contains_scalar_i64_routes|StringScalarI64Routes|StringContains",
]:
    need(row in hako, f"hako source row missing: {row}")

fixture_decision = fixture.get("decision") or {}
need(fixture_decision.get("kind") == "SelectConnectedCloseoutInventoryRerun005", "fixture decision drift")
need(fixture_decision.get("selected_next_card") == next_card, "fixture next drift")

claims = fixture.get("claims") or {}
for key in [
    "generated_typed_hako_artifact_shadow_consumed",
    "checked_in_generated_typed_artifact",
    "string_fastpath_shadow_consumed",
    "rust_hako_policy_match",
    "generator_check_guard",
    "rust_authority_retained",
]:
    need(claims.get(key) == 1, f"claim missing: {key}")
for key in [
    "runtime_hako_source_text_parsing",
    "read_surface_connection_complete",
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "build_rs_hako_compiler_invocation",
    "live_hako_authority",
    "caller_orientation_runtime_path",
    "new_backend_route",
    "new_abi",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3378-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-STRING-SCALAR-I64-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-string-scalar-i64-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_string_scalar_i64_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-string-scalar-i64-v0")
print("generated_typed_hako_artifact_shadow_consumed=1")
print("checked_in_generated_typed_artifact=1")
print("runtime_hako_source_text_parsing=0")
print("string_fastpath_shadow_consumed=1")
print("rust_hako_policy_match=1")
print("generator_check_guard=1")
print("rust_authority_retained=1")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("rust_fastpath_rewired=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
