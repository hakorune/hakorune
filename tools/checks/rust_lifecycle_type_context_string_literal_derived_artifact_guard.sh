#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_type_context_string_literal_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/type_context_string_literal.hako"
EXE="/tmp/hako_type_context_string_literal_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/type_context_string_literal.artifact.json").read_text())
facts = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/type-context-string-literal-facts-v0.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/type-context-string-literal-derived-artifact-verifier-result-v0.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/type_context_string_literal.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::type_context"
assert manifest["pilot_scope"] == "TypeContext_string_literals_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_emit_string_claim"] == 0
assert manifest["claims"]["full_map_value_publication_claim"] == 0
assert manifest["claims"]["backend_behavior_changed"] == 0
assert verifier["checks"]["storage_access_normalized"] == 1
assert verifier["checks"]["borrow_lowering_decision"] == "ElideToLeafProjection"
assert verifier["checks"]["order_observed"] == 0
borrow_use = {row["id"]: row for row in facts["borrow_use_facts"]}
assert borrow_use["TypeContext::string_literals.get_cloned"]["consumer_kind"] == "GetClone"
assert borrow_use["TypeContext::string_literals.get_cloned"]["escapes"] is False
assert borrow_use["TypeContext::string_literals.get_cloned"]["order"] == "Unobserved"
assert "string_literal(ctx, value_id): Option<StringBox>" in hako
assert "return ctx.string_literals" not in hako
assert "emit_string" not in hako
assert "TODO" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_type_context_string_literal_artifact.mir.json "$ARTIFACT" >/tmp/hako_type_context_string_literal_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_type_context_string_literal_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_type_context_string_literal_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
type_context_string_literal_direct_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-type-context-string-literal-derived-artifact-v0
family_id=hakorune_mir_builder::type_context
pilot_scope=TypeContext_string_literals_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
raw_aggregate_return=0
storage_access_normalized=1
borrow_lowering_decision=ElideToLeafProjection
order_observed=0
full_emit_string_claim=0
full_map_value_publication_claim=0
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
