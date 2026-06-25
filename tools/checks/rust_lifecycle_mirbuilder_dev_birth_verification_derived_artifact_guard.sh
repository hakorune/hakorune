#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-dev-birth-verification"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.hako"
EXE="/tmp/hako_mirbuilder_dev_birth_verification"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.hako").read_text()
required = [
    "box DevBirthFunctionShellBox",
    "box DevBirthVerificationResultBox",
    "DevBirthVerificationApi",
    "run(fn_state): DevBirthVerificationResultBox",
    "fn_state.using_is_dev = 0",
    "fn_state.stageb_dev_verify_enabled = 0",
    "fn_state.cli_verbose_enabled = 0",
    "fn_state.guard_conditions = 3",
    "fn_state.verification_steps = 10",
    "result.warnings = 0",
    "result.mutates_function = 0",
    "result.module_function_insertion = 0",
    "result.full_finalize_module = 0",
    "mirbuilder_dev_birth_verification_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing dev birth verification artifact text: {missing}")
for forbidden in [
    "module.add_function",
    "condition_fn",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"dev birth artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::dev_birth_verification":
    raise SystemExit("dev birth verification manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("dev birth verification artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "dev_birth_verification": 1,
    "module_function_insertion": 0,
    "condition_fn_injection": 0,
    "all_functions_phi_materialization": 0,
    "region_stack_pop": 0,
    "slot_registry_release": 0,
    "metadata_publication": 0,
    "semantic_refresh": 0,
    "full_finalize_module": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
    "backend_behavior_changed": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
    "new_canonical_mir_instruction": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"dev birth verification claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "dev_birth_verification_only": 1,
    "entrypoint": "MirBuilder::finalize_module dev birth verification block",
    "function_transport": "MirFunctionPreparedMain",
    "context": "finalize_module",
    "minimal_path_expected_result": "NoErrorReturn",
    "mutation_frame": [],
    "side_effect": "dev_warning_only",
    "module_function_insertion": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"dev birth verification verifier check drift: {key}={checks.get(key)}")
if checks.get("guard_conditions") != [
    "using_is_dev",
    "stageb_dev_verify_enabled",
    "cli_verbose_enabled",
]:
    raise SystemExit("dev birth verification guard condition drift")
if len(checks.get("verification_steps") or []) != 10:
    raise SystemExit("dev birth verification step count drift")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_dev_birth_verification.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_dev_birth_verification.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in dev birth MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "DevBirthVerificationApi.run/1"]
if len(matches) != 1:
    raise SystemExit(f"expected one dev birth route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"dev birth route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"dev birth route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"dev birth route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "DevBirthVerificationResultBox":
    raise SystemExit(f"dev birth result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"dev birth value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "DevBirthVerificationApi.run/1" not in symbols:
    raise SystemExit("missing dev birth same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_dev_birth_verification.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_dev_birth_verification.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_dev_birth_verification_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-dev-birth-verification-derived-artifact-v0
family_id=hakorune_mir_builder::dev_birth_verification
dev_birth_verification_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
dev_birth_verification=1
module_function_insertion=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
