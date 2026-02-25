#!/bin/bash
# phase29bq_hako_mirbuilder_ls0_mir_json_v0_load_store_min_vm.sh
# Contract pin (LS0): mir_json_v0 loader accepts minimal load/store ops.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_ls0_mir_json_v0_load_store_min_${$}"
MJSON="$TMP_DIR/${RUN_ID}_mir.json"

cat >"$MJSON" <<'JSON'
{"functions":[{"name":"main","params":[],"blocks":[{"id":0,"instructions":[{"op":"const","dst":2,"value":{"type":"i64","value":7}},{"op":"store","ptr":1,"value":2},{"op":"load","dst":3,"ptr":1},{"op":"ret","value":3}]}]}]}
JSON

if ! rg -q '"op":"store"' "$MJSON"; then
  echo "[FAIL] hako_mirbuilder ls0 pin: missing store op in fixture json" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

if ! rg -q '"op":"load"' "$MJSON"; then
  echo "[FAIL] hako_mirbuilder ls0 pin: missing load op in fixture json" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

set +e
RAW_OUT="$("$NYASH_BIN" --mir-json-file "$MJSON" 2>&1)"
EXEC_RC=$?
set -e
OUT="$(printf "%s" "$RAW_OUT" | filter_noise || true)"

if [ "$EXEC_RC" -ne 7 ]; then
  echo "[FAIL] hako_mirbuilder ls0 pin: --mir-json-file failed (rc=$EXEC_RC)" >&2
  echo "[FAIL] expected rc=7" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

if [ -n "$OUT" ]; then
  echo "[FAIL] hako_mirbuilder ls0 pin: unexpected stdout (expected='' got='$OUT')" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder ls0 pin (mir_json_v0 load/store): PASS"
