#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_core_context_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/core_context.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json"
EXE="/tmp/hako_core_context_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/core_context.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::core_context"
assert manifest["pilot_scope"] == "CoreContext_scalar_counters_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["core_context_full_claim"] == 0
assert manifest["claims"]["mirbuilder_wide_claim"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0
assert manifest["excluded_methods"] == [
    "CoreContext::next_value",
    "CoreContext::next_block",
    "CoreContext::peek_next_value",
    "CoreContext::peek_next_block",
]
assert "CoreContextApi.next_binding" in hako
assert "CoreContextApi.next_temp_slot" in hako
assert "CoreContextApi.next_debug_join" in hako
assert "next_value" not in hako
assert "next_block" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_core_context_artifact.mir.json "$ARTIFACT" >/tmp/hako_core_context_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_core_context_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_core_context_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
core_context_scalar_counters_derived_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-core-context-derived-artifact-v0
family_id=hakorune_mir_builder::core_context
pilot_scope=CoreContext_scalar_counters_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
core_context_full_claim=0
mirbuilder_wide_claim=0
generator_object_methods_generated=0
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
