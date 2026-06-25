#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="metadata-context-value-caller"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/metadata_context_value_caller.hako"
EXE="/tmp/hako_metadata_context_value_caller_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/metadata_context_value_caller.artifact.json").read_text())
facts = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/metadata-context-value-caller-facts-v0.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/metadata-context-value-caller-derived-artifact-verifier-result-v0.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/metadata_context_value_caller.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::metadata_context"
assert manifest["pilot_scope"] == "MetadataContext_value_caller_and_origin_fold_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_metadata_context_claim"] == 0
assert verifier["checks"]["storage_access_normalized"] == 1
assert verifier["checks"]["borrow_lowering_decision"] == "ElideToLeafProjection"
assert verifier["checks"]["read_fold_lowering_decision"] == "ElideToReadFold"
assert verifier["checks"]["key_domain_roundtrip"] == "CanonicalI64Text"
assert verifier["checks"]["read_fold_direct_shape_rule"] == "borrow.read_fold"
assert verifier["transport_notes"]["source_storage_transport"] == "ValueIdOrderedMapBox"
assert verifier["transport_notes"]["target_storage_transport"] == "ValueIdOrderedMapBox"
borrow_use = {row["id"]: row for row in facts["borrow_use_facts"]}
assert borrow_use["MetadataContext::value_origin_callers.get_cloned"]["consumer_kind"] == "GetClone"
assert borrow_use["MetadataContext::value_origin_callers.get_cloned"]["escapes"] is False
primary_fold = borrow_use["MetadataContext::value_origin_callers.iter_owned_copy.finalize_module"]
parity_fold = borrow_use["MetadataContext::value_origin_callers.iter_owned_copy.finalize_function"]
assert primary_fold["consumer_kind"] == "ReadOnlyFold"
assert primary_fold["escapes"] is False
assert primary_fold["fold_semantics"]["collision"] == "SourceWins"
assert primary_fold["fold_semantics"]["output_order"] == "KeyAscending(ValueIdOrdV1)"
assert parity_fold["consumer_kind"] == "ReadOnlyFold"
assert parity_fold["parity_only"] is True
assert "value_caller(ctx, value_id): Option<StringBox>" in hako
assert "merge_value_origin_callers(source: ValueIdOrderedMapBox, base: ValueIdOrderedMapBox): ValueIdOrderedMapBox" in hako
assert "local total = source.length()" in hako
assert "local key = source.key_at(i)" in hako
assert "local value = source.value_at(i)" in hako
assert "using apps.lib.collections.value_id_ordered_map as ValueIdOrderedMap" in hako
assert "value_origin_callers: ValueIdOrderedMapBox" in hako
assert "me.value_origin_callers = ValueIdOrderedMap.create()" in hako
assert "local merged = ValueIdOrderedMap.create()" in hako
assert "local merged_clone_total = base.length()" in hako
assert "local merged_clone_key = base.key_at(merged_clone_i)" in hako
assert "local merged_clone_value = base.value_at(merged_clone_i)" in hako
assert "merged.set(merged_clone_key, merged_clone_value)" in hako
assert "merged.set(key, value)" in hako
assert "merged.key_at(0)" in hako
assert "keys_value.push(key)" not in hako
assert "values_value.push(value)" not in hako
assert "return ctx.value_origin_callers\n" not in hako
assert "value_origin_callers(ctx)" not in hako
assert "MapReadFoldOwnedCopy" not in hako
assert "TODO" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_metadata_context_value_caller_artifact.mir.json "$ARTIFACT" >/tmp/hako_metadata_context_value_caller_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_metadata_context_value_caller_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_metadata_context_value_caller_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
metadata_context_value_caller_direct_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-metadata-context-value-caller-derived-artifact-v0
family_id=hakorune_mir_builder::metadata_context
pilot_scope=MetadataContext_value_caller_and_origin_fold_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
raw_aggregate_return=0
storage_access_normalized=1
borrow_lowering_decision=ElideToLeafProjection
read_fold_lowering_decision=ElideToReadFold
key_domain_roundtrip=CanonicalI64Text
record_value_caller_claim=0
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
