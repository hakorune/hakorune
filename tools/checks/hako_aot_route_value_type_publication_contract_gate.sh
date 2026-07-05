#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-route-value-type-publication-contract-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-route-value-type-publication-contract-v0.json"
POLICY_SRC="$ROOT_DIR/src/mir/route_value_type_publication.rs"
MIR_MOD="$ROOT_DIR/src/mir/mod.rs"
GLOBAL_SRC="$ROOT_DIR/src/mir/global_call_route_plan/value_type_publish.rs"
USERBOX_SRC="$ROOT_DIR/src/mir/user_box_method_route_plan/value_type_publish.rs"
MIR_JSON_SRC="$ROOT_DIR/src/runner/mir_json_emit/emitters/basic.rs"
AOT_REGRESSION="$ROOT_DIR/tools/checks/hako_aot_dynamic_string_eq_and_int_to_str_correctness_gate.sh"
PROGRAMJSON_REGRESSION="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_scan_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" \
  "$FIXTURE" \
  "$POLICY_SRC" \
  "$MIR_MOD" \
  "$GLOBAL_SRC" \
  "$USERBOX_SRC" \
  "$MIR_JSON_SRC" \
  "$AOT_REGRESSION" \
  "$PROGRAMJSON_REGRESSION"

python3 - "$FIXTURE" "$POLICY_SRC" "$MIR_MOD" "$GLOBAL_SRC" "$USERBOX_SRC" "$MIR_JSON_SRC" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, policy_path, mir_mod_path, global_path, userbox_path, mir_json_path = map(Path, sys.argv[1:])
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
policy_src = policy_path.read_text(encoding="utf-8")
mir_mod_src = mir_mod_path.read_text(encoding="utf-8")
global_src = global_path.read_text(encoding="utf-8")
userbox_src = userbox_path.read_text(encoding="utf-8")
mir_json_src = mir_json_path.read_text(encoding="utf-8")

if fixture.get("kind") != "HakoAotRouteValueTypePublicationContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-ROUTE-VALUE-TYPE-PUBLICATION-CONTRACT-001":
    raise SystemExit("bad fixture token")

rows = {
    row["return_shape"]: row
    for row in fixture.get("return_shape_value_type_publication") or []
}
expected_rows = {
    "ScalarI64": ("Integer", "Publish"),
    "scalar_i64_or_missing_zero": ("Integer", "Publish"),
    "string_handle": ("StringBox", "Publish"),
    "object_handle": (None, "DoNotPublishAmbiguous"),
    "mixed_runtime_i64_or_handle": (None, "DoNotPublishAmbiguous"),
}
for shape, (value_type, state) in expected_rows.items():
    row = rows.get(shape)
    if not row:
        raise SystemExit(f"missing return-shape row: {shape}")
    if row.get("published_value_type") != value_type:
        raise SystemExit(f"bad published value type for {shape}")
    if row.get("publication_state") != state:
        raise SystemExit(f"bad publication state for {shape}")

helper_rows = fixture.get("helper_param_type_publication_policy") or []
helper_by_id = {
    (row.get("helper_id"), row.get("param_index")): row
    for row in helper_rows
}
expected_helper_rows = {
    "StringHelpers.to_i64/1": "Integer",
    "StringHelpers.int_to_str/1": "StringBox",
    "BoxHelpers.value_i64/1": "Integer",
    "BoxHelpers.expect_i64/2": "Integer",
    "MirJsonEmitBox._expect_i64/2": "Integer",
    "MirSchemaBox._expect_i64/2": "Integer",
}
for helper_id, result_type in expected_helper_rows.items():
    helper = helper_by_id.get((helper_id, 0))
    if not helper:
        raise SystemExit(f"missing {helper_id} param0 policy")
    if helper.get("policy") != "PolymorphicInputDoNotPublishFromSingleObservation":
        raise SystemExit(f"bad {helper_id} param0 publication policy")
    if "Integer" not in (helper.get("accepted_value_kinds") or []):
        raise SystemExit(f"bad {helper_id} accepted value kinds")
    if helper_id.startswith("StringHelpers.") and "NumericLikeStringBox" not in (
        helper.get("accepted_value_kinds") or []
    ):
        raise SystemExit(f"bad {helper_id} accepted value kinds")
    if helper.get("result_published_value_type") != result_type:
        raise SystemExit(f"bad {helper_id} result published type")

hint = fixture.get("mir_json_hint_policy") or {}
string_compare = hint.get("string_compare") or {}
string_concat = hint.get("string_concat") or {}
for key in [
    "eq_ne_only",
    "one_side_string_like_required",
    "other_side_string_like_or_unknown_required",
    "stringbox_vs_void_null_forbidden",
]:
    if string_compare.get(key) is not True:
        raise SystemExit(f"bad string compare hint: {key}")
if string_concat.get("add_with_any_string_like_operand_emits_stringbox_dst_type") is not True:
    raise SystemExit("bad string concat hint")

required_gates = fixture.get("required_regression_gates") or []
for gate in [
    "tools/checks/hako_aot_dynamic_string_eq_and_int_to_str_correctness_gate.sh",
    "tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_scan_parity_gate.sh",
]:
    if gate not in required_gates:
        raise SystemExit(f"missing regression gate: {gate}")

claims = fixture.get("claims") or {}
for key in [
    "hako_syntax_change",
    "new_hako_library_api",
    "programjson_traversal_capability",
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering_claim",
    "new_backend_route",
    "new_abi",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

required_policy_needles = [
    "pub(crate) enum HelperParamTypePublicationPolicy",
    "pub(crate) fn helper_param_type_publication_policy",
    "pub(crate) fn route_return_shape_value_type",
    'pub(crate) const STRING_HELPERS_TO_I64: &str = "StringHelpers.to_i64/1";',
    'pub(crate) const STRING_HELPERS_INT_TO_STR: &str = "StringHelpers.int_to_str/1";',
    'pub(crate) const BOX_HELPERS_VALUE_I64: &str = "BoxHelpers.value_i64/1";',
    'pub(crate) const BOX_HELPERS_EXPECT_I64: &str = "BoxHelpers.expect_i64/2";',
    'pub(crate) const MIR_JSON_EMIT_BOX_EXPECT_I64: &str = "MirJsonEmitBox._expect_i64/2";',
    'pub(crate) const MIR_SCHEMA_BOX_EXPECT_I64: &str = "MirSchemaBox._expect_i64/2";',
    "POLYMORPHIC_HELPER_PARAM0_INPUTS",
    "PolymorphicInputDoNotPublishFromSingleObservation",
    'Some("scalar_i64_or_missing_zero")',
    'Some("string_handle") | Some("string_handle_or_null")',
    'Some("object_handle") | Some("mixed_runtime_i64_or_handle") | None => None',
]
for needle in required_policy_needles:
    if needle not in policy_src:
        raise SystemExit(f"policy source missing: {needle}")

if "pub mod route_value_type_publication;" not in mir_mod_src:
    raise SystemExit("src/mir/mod.rs must expose route_value_type_publication")

for needle in [
    "helper_param_type_publication_policy",
    "route_return_shape_value_type",
    "HelperParamTypePublicationPolicy",
]:
    if needle not in global_src:
        raise SystemExit(f"global route publisher missing shared policy: {needle}")
if 'target_symbol == "StringHelpers.to_i64/1"' in global_src:
    raise SystemExit("global route publisher must not carry name-specific to_i64 branch")
if "fn value_type_from_return_shape" in global_src:
    raise SystemExit("global route publisher must not keep local return-shape mapper")

for needle in [
    "route_return_shape_value_type",
    "helper_param_type_publication_policy",
    "HelperParamTypePublicationPolicy",
    "publish_generic_route_result_value_types",
    "route.return_shape()?.as_metadata_name()",
]:
    if needle not in userbox_src:
        raise SystemExit(f"user-box route publisher missing shared policy: {needle}")
if "fn value_type_from_return_shape" in userbox_src:
    raise SystemExit("user-box route publisher must not keep local return-shape mapper")

required_mir_json_needles = [
    "matches!(op, BinaryOp::Add)",
    "mir_type_is_string_like(value_types.get(lhs))",
    "mir_type_is_string_like(value_types.get(rhs))",
    'obj["dst_type"] = json!({"kind":"handle","box_type":"StringBox"});',
    "matches!(op, CompareOp::Eq | CompareOp::Ne)",
    "mir_type_allows_string_compare",
    'obj["cmp_kind"] = json!("string");',
]
for needle in required_mir_json_needles:
    if needle not in mir_json_src:
        raise SystemExit(f"MIR JSON emitter missing hint contract needle: {needle}")
PY

cargo test -q route_return_shape_publication_contract --lib
cargo test -q polymorphic_helper_param0_inputs_do_not_publish_from_single_observation --lib
bash "$AOT_REGRESSION"
bash "$PROGRAMJSON_REGRESSION"

cat <<'REPORT'
output_contract=hako-aot-route-value-type-publication-contract-gate-v0
fixture=hako-aot-route-value-type-publication-contract-v0.json
policy_owner=RouteReturnShapeValueTypePublisherV1
helper_policy_owner=HelperParamTypePublicationPolicyV1
scalar_i64_publication=Integer
scalar_i64_or_missing_zero_publication=Integer
string_handle_publication=StringBox
object_handle_publication=DoNotPublishAmbiguous
mixed_runtime_i64_or_handle_publication=DoNotPublishAmbiguous
polymorphic_helper_param0_policy=PolymorphicInputDoNotPublishFromSingleObservation
polymorphic_helper_param0_count=6
mir_json_string_compare_hint=green
mir_json_string_concat_hint=green
aot_dynamic_string_eq_and_int_to_str_regression=green
programjson_loop_body_control_flow_scan_regression=green
hako_syntax_change=0
new_hako_library_api=0
programjson_traversal_capability=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
