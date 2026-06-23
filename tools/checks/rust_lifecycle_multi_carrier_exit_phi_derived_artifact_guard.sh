#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_multi_exit_phi_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/multi_carrier_exit_phi.hako"
EXE="/tmp/hako_multi_carrier_exit_phi_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/multi_carrier_exit_phi.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/multi_carrier_exit_phi.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::multi_carrier_exit_phi"
assert manifest["pilot_scope"] == "MultiCarrierExitPhi_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["runtime_fallback"] == 0
assert manifest["claims"]["inferred_phi_claim"] == 0
assert "if exit_kind == 0" in hako
assert "else if exit_kind == 1" in hako
assert "else if exit_kind == 2" in hako
assert "TODO" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_multi_carrier_exit_phi_artifact.mir.json "$ARTIFACT" >/tmp/hako_multi_carrier_exit_phi_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_multi_carrier_exit_phi_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_multi_carrier_exit_phi_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
multi_carrier_exit_phi_direct_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-multi-carrier-exit-phi-derived-artifact-v0
family_id=hakorune_mir_builder::multi_carrier_exit_phi
pilot_scope=MultiCarrierExitPhi_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
raw_hako_body=0
exit_kinds=break,continue,early_return
carrier_count=2
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
