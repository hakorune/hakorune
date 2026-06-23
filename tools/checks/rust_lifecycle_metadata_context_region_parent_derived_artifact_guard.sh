#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_metadata_region_parent_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/metadata_context_region_parent.hako"
EXE="/tmp/hako_metadata_context_region_parent_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/metadata_context_region_parent.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/metadata_context_region_parent.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::metadata_context"
assert manifest["pilot_scope"] == "MetadataContext_region_parent_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["runtime_fallback"] == 0
assert "current_parent_region(current_region_stack: ArrayBox): Option<i64>" in hako
assert "current_region_stack(ctx)" not in hako
assert "return ctx.current_region_stack" not in hako
assert "ReadView" not in hako
assert "TODO" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_metadata_context_region_parent_artifact.mir.json "$ARTIFACT" >/tmp/hako_metadata_context_region_parent_artifact.mir.log 2>&1

cat <<'REPORT'
output_contract=rust-lifecycle-metadata-context-region-parent-derived-artifact-v0
family_id=hakorune_mir_builder::metadata_context
pilot_scope=MetadataContext_region_parent_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot_claim=0
generated_hako_exe_aot=skipped_pending_boxed_i64_payload
borrow_lowering_decision=ElideToLeafProjection
raw_aggregate_return=0
read_lease_claim=0
boxed_i64_payload_claim=0
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
