#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="single-scalar-loop-carrier"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/single_scalar_loop_carrier.hako"
EXE="/tmp/hako_single_scalar_loop_carrier_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/single_scalar_loop_carrier.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/single_scalar_loop_carrier.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::single_scalar_loop_carrier"
assert manifest["pilot_scope"] == "SingleScalarLoopCarrier_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_mirbuilder_crate_claim"] == 0
assert manifest["claims"]["runtime_fallback"] == 0
assert manifest["claims"]["phi_claim"] == 0
assert manifest["claims"]["multi_carrier_claim"] == 0
assert "local sum = 0" in hako
assert "sum = sum + BoxHelpers.array_get(values, i)" in hako
assert "return sum" in hako
assert "TODO" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_single_scalar_loop_carrier_artifact.mir.json "$ARTIFACT" >/tmp/hako_single_scalar_loop_carrier_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_single_scalar_loop_carrier_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_single_scalar_loop_carrier_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
single_scalar_loop_carrier_direct_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-single-scalar-loop-carrier-derived-artifact-v0
family_id=hakorune_mir_builder::single_scalar_loop_carrier
pilot_scope=SingleScalarLoopCarrier_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
raw_hako_body=0
single_scalar_carrier=1
multi_carrier_claim=0
phi_required=0
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
