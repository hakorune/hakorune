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
assert manifest["generator"]["version"] == "mirbuilder-easy-v0-ordered-map-bundle-v4"
assert manifest["bundle_kind"] == "mirbuilder_easy_v0_ordered_map_bundle"
assert manifest["bundle_contract_model"] == "membership_only_v1"
assert manifest["bundle_members"] == [
    "binding_context",
    "box_compilation_context",
    "core_context",
    "mirbuilder_next_value_id_prepared_state_kernel",
    "variable_context_simple_map",
    "variable_context_snapshot_restore",
]
contract_refs = manifest["bundle_member_contracts"]
assert len(contract_refs) == 6
core_contract = [row for row in contract_refs if row["member"] == "core_context"]
assert len(core_contract) == 1
assert core_contract[0]["family_id"] == "hakorune_mir_builder::core_context"
assert core_contract[0]["contract"]["kind"] == "VerifiedFamilyArtifactContractV1"
assert core_contract[0]["contract"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json"
allocation_contract = [row for row in contract_refs if row["member"] == "mirbuilder_next_value_id_prepared_state_kernel"]
assert len(allocation_contract) == 1
assert allocation_contract[0]["family_id"] == "hakorune_mir_builder::next_value_id_prepared_state_kernel"
assert allocation_contract[0]["contract"]["kind"] == "VerifiedFamilyArtifactContractV1"
assert allocation_contract[0]["contract"]["manifest_path"] == "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.artifact.json"
assert manifest["exercised_capabilities"] == [
    "BindingContext.insert_lookup",
    "BoxCompilationContext.new_is_empty",
    "CoreContext.scalar_counters_and_id_generators",
    "MirBuilderAllocationPolicy.prepared_state_next_value_id",
    "VariableContext.simple_map.insert_lookup",
    "VariableContext.snapshot_owned_projection_set_get",
    "VariableContext.snapshot_restore",
]
assert manifest["deferred_capabilities"] == []
assert len(manifest["source"]["rust_files"]) == 5
assert len(manifest["inputs"]["bundle_members"]) == 6
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
output_contract=rust-lifecycle-mirbuilder-easy-v0-ordered-map-bundle-v4
family_id=hakorune_mir_builder::ordered_map_bundle
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
summary=ok
REPORT
