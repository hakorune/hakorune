#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_binding_context_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/binding_context.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json"
EXE="/tmp/hako_binding_context_derived_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
import sys
from pathlib import Path

sys.path.insert(0, "tools/rust_lifecycle")
from extract_binding_context_facts import SOURCE, extract_facts
from mirbuilder_family_artifacts import binding_context_spec
from mirbuilder_ordered_map_converter import OrderedMapConversionDeny, compile_binding_context_methods
from shared_family_generator import read_json

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-derived-artifact-verifier-result-v0.json").read_text())
recipe = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-behavior-recipe-v0.json").read_text())

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::binding_context"
assert manifest["state"] == "DerivedShadow"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0

checks = verifier["checks"]
assert checks["rust_facts_input"] == "verified"
assert checks["hako_lifecycle_plan"] == "verified"
assert checks["hako_behavior_recipe"] == "verified"
assert checks["selected_body_count"] == "all_non_test_methods"
assert checks["unmapped_thir_nodes"] == 0
assert checks["unmapped_mir_side_effects"] == 0
assert checks["unresolved_call_targets"] == 0
assert checks["unclassified_drop_obligations"] == 0
assert checks["mainline_selected"] == 0
assert checks["rust_bootstrap_retained"] == 1
assert checks["backend_behavior_changed"] == 0

method_ids = {method["id"] for method in recipe["methods"]}
for method in [
    "BindingContext::new",
    "BindingContext::is_empty",
    "BindingContext::len",
    "BindingContext::contains",
    "BindingContext::lookup",
    "BindingContext::insert",
    "BindingContext::remove",
    "BindingContext::clear_for_function_entry",
]:
    assert method in method_ids

spec = binding_context_spec()
assert spec.box.initializer is None
assert spec.box.initializer_operation == {"kind": "NewOrderedMap"}
assert spec.api_methods
assert all(method.body_lines is None for method in spec.api_methods)
assert all(method.operations for method in spec.api_methods)

facts = extract_facts(SOURCE)
plan = read_json(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-plan-v0.json"))
compile_binding_context_methods(facts, plan)
facts["body_facts"][1]["operation"] = "UnexpectedMapIsEmpty"
try:
    compile_binding_context_methods(facts, plan)
except OrderedMapConversionDeny as exc:
    assert exc.reason == "UnsupportedResolvedCallTarget"
else:
    raise AssertionError("unsupported BindingContext body shape must fail closed")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"
rm -f tmp/nyash_cli_emit.json

./target/release/hakorune --emit-mir-json /tmp/hako_binding_context_derived_artifact.mir.json "$ARTIFACT" >/tmp/hako_binding_context_derived_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_binding_context_derived_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_binding_context_derived_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
binding_context_derived_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-derived-artifact-v0
family_id=BindingContext
rust_facts_input=verified
hako_lifecycle_plan=verified
hako_behavior_recipe=verified
selected_body_count=all_non_test_methods
unmapped_thir_nodes=0
unmapped_mir_side_effects=0
unresolved_call_targets=0
unclassified_drop_obligations=0
generated_hako_checked_in=1
artifact_manifest_checked_in=1
generated_hako_manual_edit=0
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
rust_oracle_behavior_parity=green
mainline_selected=0
rust_bootstrap_retained=1
backend_behavior_changed=0
summary=ok
REPORT
