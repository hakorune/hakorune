#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)

BIN="$ROOT_DIR/target/release/hakorune"
if [ ! -x "$BIN" ]; then
  BIN="$ROOT_DIR/target/release/nyash"
fi
if [ ! -x "$BIN" ]; then
  echo "Building hakorune (release)..." >&2
  cargo build --release --features cranelift-jit >/dev/null
fi
if [ -x "$ROOT_DIR/target/release/hakorune" ]; then
  BIN="$ROOT_DIR/target/release/hakorune"
else
  BIN="$ROOT_DIR/target/release/nyash"
fi

echo "[Smoke] Parser v0 JSON pipe → MIR-Interp" >&2
set -o pipefail
set +e
printf '{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}}]}' \
  | "$BIN" --ny-parser-pipe >/tmp/nyash-bridge-smoke.out
PIPE_RC=${PIPESTATUS[1]}
set -e

if [ "$PIPE_RC" -eq 7 ]; then
  echo "PASS: pipe path" >&2
else
  echo "FAIL: pipe path (rc=$PIPE_RC)" >&2; cat /tmp/nyash-bridge-smoke.out; exit 1
fi

echo "[Smoke] --json-file path" >&2
# archive-only evidence: keep this as a compat loader monitor, not a current-facing direct-MIR route
TMPJSON=$(mktemp)
cat >"$TMPJSON" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}}]}
JSON
set +e
"$BIN" --json-file "$TMPJSON" >/tmp/nyash-bridge-smoke2.out
JSON_RC=$?
set -e
if [ "$JSON_RC" -eq 7 ]; then
  echo "PASS: json-file path" >&2
else
  echo "FAIL: json-file path (rc=$JSON_RC)" >&2; cat /tmp/nyash-bridge-smoke2.out; exit 1
fi
echo "All PASS" >&2
