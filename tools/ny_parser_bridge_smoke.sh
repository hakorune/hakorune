#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)

TMP_FILES=()
cleanup_tmp() { rm -f "${TMP_FILES[@]}" 2>/dev/null || true; }
trap cleanup_tmp EXIT

HAKORUNE_BIN="$ROOT_DIR/target/release/hakorune"
LEGACY_NYASH_BIN="$ROOT_DIR/target/release/nyash"
BIN="$HAKORUNE_BIN"
if [ ! -x "$BIN" ]; then
  BIN="$LEGACY_NYASH_BIN"
fi
if [ ! -x "$BIN" ]; then
  echo "Building hakorune (release)..." >&2
  cargo build --release --features cranelift-jit >/dev/null
fi
if [ -x "$HAKORUNE_BIN" ]; then
  BIN="$HAKORUNE_BIN"
else
  BIN="$LEGACY_NYASH_BIN"
fi

echo "[Smoke] Parser v0 JSON pipe → MIR-Interp" >&2
set -o pipefail
PIPE_OUT=$(mktemp /tmp/hakorune-bridge-smoke.XXXXXX.out)
TMP_FILES+=("$PIPE_OUT")
set +e
printf '{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}}]}' \
  | "$BIN" --ny-parser-pipe >"$PIPE_OUT"
PIPE_RC=${PIPESTATUS[1]}
set -e

if [ "$PIPE_RC" -eq 7 ]; then
  echo "PASS: pipe path" >&2
else
  echo "FAIL: pipe path (rc=$PIPE_RC)" >&2; cat "$PIPE_OUT"; exit 1
fi

echo "[Smoke] --json-file path" >&2
# archive-only evidence: keep this as a compat loader monitor, not a current-facing direct-MIR route
TMPJSON=$(mktemp)
JSON_OUT=$(mktemp /tmp/hakorune-bridge-smoke-json.XXXXXX.out)
TMP_FILES+=("$TMPJSON" "$JSON_OUT")
cat >"$TMPJSON" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}}]}
JSON
set +e
"$BIN" --json-file "$TMPJSON" >"$JSON_OUT"
JSON_RC=$?
set -e
if [ "$JSON_RC" -eq 7 ]; then
  echo "PASS: json-file path" >&2
else
  echo "FAIL: json-file path (rc=$JSON_RC)" >&2; cat "$JSON_OUT"; exit 1
fi
echo "All PASS" >&2
