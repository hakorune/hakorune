#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)

BIN="$ROOT_DIR/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  echo "Building hakorune (release)..." >&2
  cargo build --release >/dev/null
fi

expect_result7() {
  local label="$1"
  local out_file="$2"
  local rc="$3"

  if [[ "$rc" -eq 7 ]] || rg -q '^Result:\s*7\b' "$out_file"; then
    echo "PASS: $label" >&2
  else
    echo "FAIL: $label (rc=$rc)" >&2
    cat "$out_file" >&2 || true
    exit 1
  fi
}

echo "[Roundtrip] Case A: Ny (source v0) → MIR-Interp (direct bridge)" >&2
TMPNY=$(mktemp)
printf 'return 1+2*3\n' > "$TMPNY"
set +e
NYASH_DISABLE_PLUGINS=1 "$BIN" --backend vm "$TMPNY" > /tmp/nyash-rt-a.out 2>&1
CASE_A_RC=$?
set -e
expect_result7 "Case A (direct bridge)" /tmp/nyash-rt-a.out "$CASE_A_RC"

echo "[Roundtrip] Case B: JSON(v0) file → MIR-Interp" >&2
TMPJSON=$(mktemp)
cat >"$TMPJSON" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Binary","op":"*","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}}]}
JSON
set +e
"$BIN" --json-file "$TMPJSON" > /tmp/nyash-rt-b.out 2>&1
CASE_B_RC=$?
set -e
expect_result7 "Case B (json-file)" /tmp/nyash-rt-b.out "$CASE_B_RC"

echo "All PASS" >&2
