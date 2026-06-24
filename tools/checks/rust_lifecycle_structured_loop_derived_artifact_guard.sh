#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="structured-loop-without-carried-state"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/structured_loop_without_carried_state.hako"
EXE="/tmp/hako_structured_loop_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/structured_loop_without_carried_state.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/structured_loop_without_carried_state.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::structured_loop_without_carried_state"
assert manifest["pilot_scope"] == "StructuredLoop_without_carried_state_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_mirbuilder_crate_claim"] == 0
assert manifest["claims"]["runtime_fallback"] == 0
assert manifest["claims"]["phi_claim"] == 0
assert manifest["claims"]["loop_carried_state_claim"] == 0
assert "loop(i < values.length())" in hako
assert "local i = 0" in hako
assert "BoxHelpers.array_get(values, i)" in hako
assert "TODO" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_structured_loop_artifact.mir.json "$ARTIFACT" >/tmp/hako_structured_loop_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_structured_loop_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_structured_loop_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
structured_loop_without_carried_state_direct_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-structured-loop-derived-artifact-v0
family_id=hakorune_mir_builder::structured_loop_without_carried_state
pilot_scope=StructuredLoop_without_carried_state_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
raw_hako_body=0
break_continue=0
early_return=0
phi_required=0
loop_carried_state=0
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
