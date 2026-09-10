#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-artifact-schema-selector-i0-guard"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT_DIR/docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md"
LOADER="$ROOT_DIR/src/runner/json_artifact/mir_loader.rs"
README="$ROOT_DIR/src/runner/json_artifact/README.md"

fail() {
  echo "[$TAG] result_class=current-change failure status=fail: $*" >&2
  exit 1
}

[[ -f "$STATE" && -f "$CARD" && -f "$LOADER" && -f "$README" ]] || fail "required owner missing"

grep -Fq 'current_execution_row = "MIR-ARTIFACT-MIR-JSON-TOPLEVEL-SCHEMA-STOP-I0"' "$STATE" \
  || fail "artifact schema selector is not the selected execution row"
grep -Fq 'MIR-ARTIFACT-MIR-JSON-TOPLEVEL-SCHEMA-STOP-I0' "$CARD" \
  || fail "artifact schema selector row missing from SSOT"

TMP_LOADER="$(mktemp)"
trap 'rm -f "$TMP_LOADER"' EXIT
awk '
  /pub\(super\) fn load_mir_json_to_module/ { in_loader = 1 }
  /pub\(super\) fn parse_direct_mir_json_text/ { in_loader = 0 }
  in_loader { print }
' "$LOADER" >"$TMP_LOADER"
loader_body="$TMP_LOADER"
grep -Fq 'try_parse_v1_to_module(text)' "$loader_body" \
  || fail "artifact loader does not consult the v1 bridge"
if grep -Fq 'text.contains("\"schema_version\"")' "$loader_body"; then
  fail "artifact loader still uses raw schema substring selection"
fi
grep -Fq 'parse_mir_v0_to_module(text)' "$loader_body" \
  || fail "artifact loader lost the no-schema v0 path"
grep -Fq 'load_mir_json_to_module_rejects_escaped_declared_schema_before_v0' "$LOADER" \
  || fail "escaped-key terminal test is missing"
grep -Fq 'parsed top-level v1 bridge' "$README" \
  || fail "artifact README does not name the parsed schema authority"

lines="$(wc -l <"$LOADER")"
(( lines < 760 )) || fail "artifact loader crossed the 760-line split trigger: $lines"

echo "[$TAG] result_class=current-change failure status=pass row=MIR-ARTIFACT-MIR-JSON-TOPLEVEL-SCHEMA-STOP-I0 raw_selector=zero escaped_schema_negative=present v0_absent_path=present lines=$lines"
