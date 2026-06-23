#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_metadata_value_caller_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/metadata_context_value_caller.hako"
EXE="/tmp/hako_metadata_context_value_caller_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/metadata_context_value_caller.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/metadata_context_value_caller.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::metadata_context"
assert manifest["pilot_scope"] == "MetadataContext_value_caller_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_metadata_context_claim"] == 0
assert "value_caller(ctx, value_id): Option<StringBox>" in hako
assert "return ctx.value_origin_callers" not in hako
assert "value_origin_callers(ctx)" not in hako
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
pilot_scope=MetadataContext_value_caller_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
raw_aggregate_return=0
record_value_caller_claim=0
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
