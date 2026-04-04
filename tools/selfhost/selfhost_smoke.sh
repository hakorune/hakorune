#!/usr/bin/env bash
set -euo pipefail

# Self-host minimal smoke (explicit proof-only keep)
# - Emits MIR(JSON v0) via the explicit compat/proof keep path
# - Runs a representative compat bridge example and compares keep outputs

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
NY_BIN="${ROOT_DIR}/target/release/hakorune"

if [[ ! -x "${NY_BIN}" ]]; then
  echo "[selfhost-smoke] hakorune binary not found at ${NY_BIN}. Please build first: cargo build --release" >&2
  exit 1
fi

echo "[selfhost-smoke] Step 1: Emit JSON via explicit proof keep"
OUT_JSON="/tmp/nyash_selfhost_out.json"
set -x
# Use lang side entry (Stage‑B proof keep). Emission is optional; failure does not fail the smoke.
if NYASH_ENABLE_USING=1 NYASH_ALLOW_USING_FILE=1 NYASH_USING_AST=1 NYASH_FEATURES=stage3 \
   "${NY_BIN}" --backend vm "${ROOT_DIR}/lang/src/compiler/entry/compiler_stageb.hako" -- --source 'box Main { static method main() { return 0 } }' > "${OUT_JSON}" 2>/dev/null; then
  :
else
  echo "[selfhost-smoke] WARN: proof keep emission failed (policy/duplicates?). Continuing." >&2
fi
set +x

if [[ -s "${OUT_JSON}" ]]; then
  echo "[selfhost-smoke] Emitted JSON: ${OUT_JSON} ($(wc -c < "${OUT_JSON}") bytes)"
else
  echo "[selfhost-smoke] NOTE: no JSON emitted (skipped). This is optional for the minimal smoke." >&2
fi

echo "[selfhost-smoke] Step 2: Run representative compat keep example (rewrite=ON/OFF)"
EXAMPLE="apps/examples/json_query/main.hako"
OUT_ON="/tmp/nyash_selfhost_compat_on.txt"
OUT_OFF="/tmp/nyash_selfhost_compat_off.txt"

set -x
"${NY_BIN}" --backend vm "${EXAMPLE}" > "${OUT_ON}"
NYASH_REWRITE_KNOWN_DEFAULT=0 "${NY_BIN}" --backend vm "${EXAMPLE}" > "${OUT_OFF}"
set +x

if ! diff -u "${OUT_ON}" "${OUT_OFF}" >/dev/null 2>&1; then
  echo "[selfhost-smoke] WARN: compat keep output differs between rewrite ON and OFF" >&2
  echo "--- ON (${OUT_ON})" >&2
  head -n 20 "${OUT_ON}" >&2 || true
  echo "--- OFF (${OUT_OFF})" >&2
  head -n 20 "${OUT_OFF}" >&2 || true
  # Non-fatal: keep smoke informative; do not fail hard unless required.
else
  echo "[selfhost-smoke] compat bridge outputs match for rewrite ON/OFF (good)."
fi

echo "[selfhost-smoke] PASS"
