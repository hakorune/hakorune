#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="box-compilation-context"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json"
EXE="/tmp/hako_box_compilation_context_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.artifact.json").read_text())
recipe = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-behavior-recipe-v0.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-derived-artifact-verifier-result-v0.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/box_compilation_context.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::context"
assert manifest["pilot_scope"] == "BoxCompilationContext_ctor_is_empty_only"
assert manifest["state"] == "DerivedShadow"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0
assert manifest["claims"]["source_selfhost_claim"] == 0
assert manifest["inputs"]["facts"]["path"].endswith("box-compilation-context-facts-v0.json")
assert manifest["inputs"]["plan"]["path"].endswith("box-compilation-context-plan-v0.json")
assert manifest["inputs"]["oracle"]["path"].endswith("box-compilation-context-oracle-v0.json")
assert manifest["inputs"]["recipe"]["path"].endswith("box-compilation-context-behavior-recipe-v0.json")
assert manifest["inputs"]["verifier"]["path"].endswith("box-compilation-context-derived-artifact-verifier-result-v0.json")
assert manifest["output"]["hako_path"].endswith("box_compilation_context.hako")
assert manifest["excluded_methods"] == ["BoxCompilationContext::size_info"]

assert recipe["kind"] == "HakoBehaviorRecipe"
assert recipe["family_id"] == "hakorune_mir_builder::context"
assert recipe["selected_body_count"] == "constructor_is_empty_only"
assert recipe["pilot_scope"] == "BoxCompilationContext_ctor_is_empty_only"
assert {method["id"] for method in recipe["methods"]} == {"BoxCompilationContext::new", "BoxCompilationContext::is_empty"}

assert verifier["kind"] == "DerivedHakoArtifactVerifierResult"
assert verifier["result"] == "VerifiedHakoFamilyIR"
assert verifier["checks"]["selected_body_count"] == "constructor_is_empty_only"
assert verifier["checks"]["unmapped_thir_nodes"] == 0
assert verifier["checks"]["unresolved_call_targets"] == 0
assert verifier["verified_operations"] == ["DefaultConstruct", "NewOrderedMap", "AllFieldsMapIsEmpty"]

assert "box BoxCompilationContext" in hako
assert "variable_map: OrderedMapBox" in hako
assert "value_origin_newbox: OrderedMapBox" in hako
assert "value_types: OrderedMapBox" in hako
assert "me.variable_map = OrderedMap.create()" in hako
assert "me.value_origin_newbox = OrderedMap.create()" in hako
assert "me.value_types = OrderedMap.create()" in hako
assert "static box BoxCompilationContextApi" in hako
assert "if ctx.variable_map.is_empty() && ctx.value_origin_newbox.is_empty() && ctx.value_types.is_empty() {" in hako
assert "box_compilation_context_derived_artifact=ok" in hako
assert "size_info" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"
./target/release/hakorune --emit-mir-json /tmp/hako_box_compilation_context_artifact.mir.json "$ARTIFACT" >/tmp/hako_box_compilation_context_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_box_compilation_context_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_box_compilation_context_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
box_compilation_context_derived_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-box-compilation-context-derived-artifact-v0
family_id=hakorune_mir_builder::context
pilot_scope=BoxCompilationContext_ctor_is_empty_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe=green
mainline_selected=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
