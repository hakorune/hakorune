#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
import sys
from pathlib import Path

sys.path.insert(0, "tools/rust_lifecycle")
from extract_variable_context_simple_map_facts import SOURCE, extract_facts
from mirbuilder_family_artifacts import variable_context_simple_map_spec
from mirbuilder_ordered_map_converter import OrderedMapConversionDeny, compile_variable_context_simple_map_methods
from shared_family_generator import read_json

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-derived-artifact-verifier-result-v0.json").read_text())
recipe = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-behavior-recipe-v0.json").read_text())

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::variable_context"
assert manifest["pilot_scope"] == "VariableContext_simple_map_only"
assert manifest["state"] == "DerivedShadow"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_variable_context_claim"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0

checks = verifier["checks"]
assert checks["selected_body_count"] == "simple_map_methods_only"
assert checks["full_variable_context_claim"] == 0
assert checks["unmapped_thir_nodes"] == 0
assert checks["unmapped_mir_side_effects"] == 0
assert checks["unresolved_call_targets"] == 0
assert checks["unclassified_drop_obligations"] == 0

method_ids = {method["id"] for method in recipe["methods"]}
for method in [
    "VariableContext::lookup",
    "VariableContext::contains",
    "VariableContext::len",
    "VariableContext::is_empty",
    "VariableContext::insert",
    "VariableContext::remove",
]:
    assert method in method_ids

excluded = set(manifest["excluded_methods"])
for method in [
    "VariableContext::variable_map",
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
]:
    assert method in excluded

spec = variable_context_simple_map_spec()
assert spec.box.initializer is None
assert spec.box.initializer_operation == {"kind": "NewOrderedMap"}
assert spec.api_methods
assert all(method.operations is not None for method in spec.api_methods)
assert all(method.operations for method in spec.api_methods)

facts = extract_facts(SOURCE)
plan = read_json(Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-plan-v0.json"))
compile_variable_context_simple_map_methods(facts, plan)
facts["body_facts"][1]["operation"] = "UnexpectedMapGet"
try:
    compile_variable_context_simple_map_methods(facts, plan)
except OrderedMapConversionDeny as exc:
    assert exc.reason == "UnsupportedResolvedCallTarget"
else:
    raise AssertionError("unsupported simple-map body shape must fail closed")
PY

./target/release/hakorune --emit-mir-json /tmp/hako_variable_context_simple_map_artifact.mir.json "$ARTIFACT" >/tmp/hako_variable_context_simple_map_artifact.mir.log 2>&1

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-simple-map-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_simple_map_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
full_variable_context_claim=0
returned_borrow_methods_generated=0
snapshot_restore_methods_generated=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
