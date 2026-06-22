#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_mirbuilder_ordered_map_crate_bundle.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/ordered_map_crate_bundle.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/ordered_map_crate_bundle.artifact.json"
EXE="/tmp/hako_ordered_map_crate_bundle"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/ordered_map_crate_bundle.artifact.json").read_text())
assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::ordered_map_bundle"
assert manifest["pilot_scope"] == "MirBuilder_easy_v0_ordered_map_bundle"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0
assert manifest["generator"]["version"] == "mirbuilder-easy-v0-ordered-map-bundle-v1"
assert manifest["bundle_kind"] == "mirbuilder_easy_v0_ordered_map_bundle"
assert manifest["bundle_members"] == [
    "hakorune_mir_builder::binding_context",
    "hakorune_mir_builder::context",
    "hakorune_mir_builder::variable_context.simple_map",
    "hakorune_mir_builder::variable_context.snapshot_restore",
]
assert manifest["exercised_capabilities"] == [
    "BindingContext.insert_lookup",
    "BoxCompilationContext.new_is_empty",
    "VariableContext.simple_map.insert_lookup",
    "VariableContext.snapshot_owned_projection_call",
]
assert [entry["name"] for entry in manifest["deferred_capabilities"]] == [
    "VariableContext.restore_exe_call",
    "VariableContext.snapshot_mutation_alias_proof_in_bundle",
]
assert len(manifest["source"]["rust_files"]) == 3
assert len(manifest["inputs"]["bundle_members"]) == 4
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_ordered_map_crate_bundle.mir.json "$ARTIFACT" >/tmp/hako_ordered_map_crate_bundle.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_ordered_map_crate_bundle.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_ordered_map_crate_bundle.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
ordered_map_crate_bundle=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-easy-v0-ordered-map-bundle-v1
family_id=hakorune_mir_builder::ordered_map_bundle
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
summary=ok
REPORT
