#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64-v0.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3375-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-MAPLOAD-SCALAR-I64-001.md"
ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
GENERATED_MOD="$ROOT/src/mir/generic_method_route_plan/generated/mod.rs"
GENERATOR="$ROOT/tools/rust_lifecycle/generate_mapload_scalar_i64_hako_policy.py"
SHADOW_RS="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"
PLAN_MOD="$ROOT/src/mir/generic_method_route_plan.rs"
HAKO_POLICY="$ROOT/lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_command "$TAG" diff
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$ARTIFACT" "$GENERATED_MOD" "$GENERATOR" \
  "$SHADOW_RS" "$COLLECTION_READ_ROUTES" "$PLAN_MOD" "$HAKO_POLICY" "$TASK_ORDER" "$MANIFEST"

python3 "$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_mapload_scalar_i64.py" --check

TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT
python3 "$GENERATOR" > "$TMP"
diff -u "$ARTIFACT" "$TMP"

python3 - "$FIXTURE" "$CARD" "$ARTIFACT" "$GENERATED_MOD" "$GENERATOR" "$SHADOW_RS" "$COLLECTION_READ_ROUTES" "$PLAN_MOD" "$HAKO_POLICY" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
artifact = Path(sys.argv[3]).read_text(encoding="utf-8")
generated_mod = Path(sys.argv[4]).read_text(encoding="utf-8")
generator = Path(sys.argv[5]).read_text(encoding="utf-8")
shadow_rs = Path(sys.argv[6]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[7]).read_text(encoding="utf-8")
plan_mod = Path(sys.argv[8]).read_text(encoding="utf-8")
hako_policy = Path(sys.argv[9]).read_text(encoding="utf-8")
task_order = Path(sys.argv[10]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[11]).read_text(encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-MAPLOAD-SCALAR-I64-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-004"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeMaploadScalarI64V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(fixture.get("selected_next_card") == next_card, "bad selected next")

input_state = fixture.get("input_state") or {}
need(input_state.get("basis_decision") == "SelectMapLoadScalarI64GeneratedTypedArtifactShadowConsumeImplementation", "basis decision drift")
need(input_state.get("basis_selected_next_card") == token, "basis next drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "GeneratedTypedHakoArtifactShadowConsume", "bad decision kind")
need(decision.get("route_kind") == "MapLoadScalarI64", "bad route kind")
need(decision.get("rust_authority") == "retained", "rust authority must be retained")
need(decision.get("runtime_build_parses_hako_source_text") is False, "runtime text parsing must be false")
need(decision.get("build_rs_hako_compiler_invocation") is False, "build.rs hako invocation must be false")
need(decision.get("selected_next_card") == next_card, "decision next drift")

fields = fixture.get("artifact_fields") or {}
for key, value in {
    "surface": "MapLoadScalarI64Routes",
    "route_kind": "MapLoadScalarI64",
    "core_op": "MapGet",
    "lowering_tier": "WarmDirectAbi",
    "result_class": "ScalarI64OrMissingZeroResult",
    "return_shape": "ScalarI64OrMissingZero",
    "value_demand": "ScalarI64",
    "publication_policy": "NoPublication",
    "effect_class": "read",
    "proof_family": "ScalarI64MapGetStoreFact",
    "role": "classifier_policy_mirror_only",
}.items():
    need(fields.get(key) == value, f"artifact field drift: {key}")
need(len(fields.get("allowed_existing_proofs") or []) == 3, "allowed proof count drift")

claims = fixture.get("claims") or {}
for key in [
    "generated_typed_hako_artifact_shadow_consumed",
    "checked_in_generated_typed_artifact",
    "mapload_fastpath_shadow_consumed",
    "rust_hako_policy_match",
    "generator_check_guard",
    "rust_authority_retained",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
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

for needle in [
    "@generated by tools/rust_lifecycle/generate_mapload_scalar_i64_hako_policy.py",
    "pub(crate) struct HakoMapLoadScalarI64Policy",
    "MAPLOAD_SCALAR_I64_HAKO_POLICY",
    "route_kind: GenericMethodRouteKind::MapLoadScalarI64",
    "core_op: CoreMethodOp::MapGet",
    "lowering_tier: CoreMethodLoweringTier::WarmDirectAbi",
    "return_shape: GenericMethodReturnShape::ScalarI64OrMissingZero",
    "value_demand: GenericMethodValueDemand::ScalarI64",
    "publication_policy: GenericMethodPublicationPolicy::NoPublication",
    "GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape",
    "GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape",
    "GenericMethodRouteProof::MapSetScalarI64CoveredDynamicI64KeyNoEscape",
    'role: "classifier_policy_mirror_only"',
]:
    need(needle in artifact, f"artifact missing: {needle}")

need("pub(super) mod mapload_scalar_i64_hako_policy;" in generated_mod, "generated mod missing MapLoad module")
need("mod generated;" in plan_mod, "route plan module does not include generated module")
need("read_policy_row" in generator and "MapLoad policy row" in generator, "generator does not read policy row")

for forbidden in [
    "include_str!",
    "row.split('|')",
    "map_load_scalar_i64_policy_classifier.hako",
    "map_load_scalar_i64_routes|",
]:
    need(forbidden not in shadow_rs, f"runtime consumer has source-text parser debt: {forbidden}")
for needle in [
    "MAPLOAD_SCALAR_I64_HAKO_POLICY",
    "assert_hako_mapload_scalar_i64_policy_matches_rust(&policy, route_proof)",
    "policy.allowed_proofs.contains(&route_proof)",
    "GenericMethodRouteKind::MapLoadScalarI64",
    "GenericMethodReturnShape::ScalarI64OrMissingZero",
    "GenericMethodPublicationPolicy::NoPublication",
]:
    need(needle in shadow_rs, f"runtime typed artifact consumer missing: {needle}")

need("scalar_known_hako_shadow::mapload_scalar_i64_shadow_consumed_decision" in collection_read_routes, "fast path does not consume MapLoad shadow decision")
need(collection_read_routes.count("mapload_scalar_i64_shadow_consumed_decision") >= 2, "expected both MapLoad scalar branches to consume shadow decision")
need("map_load_scalar_i64_routes|MapLoadScalarI64Routes|MapLoadScalarI64" in hako_policy, "hako source row missing")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
row = rows_by_token.get(token) or {}
need(row.get("card", "").endswith("3375-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-MAPLOAD-SCALAR-I64-001.md"), "manifest card drift")
need(row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64-v0.json"), "manifest fixture drift")
need(row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_mapload_scalar_i64_guard.sh"), "manifest guard drift")

for needle in [
    token,
    next_card,
    "generated_typed_hako_artifact_shadow_consumed = 1",
    "runtime_hako_source_text_parsing = 0",
    "hako_runtime_route_authority = 0",
    "source_selfhost_claim = 0",
]:
    need(needle in card, f"card missing: {needle}")
for needle in [token, next_card, "generated_typed_hako_artifact_shadow_consumed = 1"]:
    need(needle in task_order, f"task-order missing: {needle}")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64-v0")
print("token=" + token)
print("generated_typed_hako_artifact_shadow_consumed=1")
print("checked_in_generated_typed_artifact=1")
print("runtime_hako_source_text_parsing=0")
print("mapload_fastpath_shadow_consumed=1")
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
