#!/usr/bin/env bash
set -euo pipefail

# Self-host minimal smoke (explicit proof-only keep)
# - Emits MIR(JSON v0) via the explicit compat/proof keep path
# - Runs one representative canonical bridge example

ROOT_DIR=$(cd "$(dirname "$0")/../../.." && pwd)
NY_BIN="${ROOT_DIR}/target/release/hakorune"
PROOF_STAGEB_SCRIPT="${ROOT_DIR}/tools/selfhost/proof/run_stageb_compiler_vm.sh"

if [[ ! -x "${NY_BIN}" ]]; then
  echo "[selfhost-smoke] hakorune binary not found at ${NY_BIN}. Please build first: cargo build --release" >&2
  exit 1
fi

echo "[selfhost-smoke] Step 1: Emit JSON via explicit proof keep"
OUT_JSON="/tmp/nyash_selfhost_out.json"
INLINE_SRC="/tmp/nyash_selfhost_smoke_inline_stageb.hako"
printf '%s\n' 'box Main { static method main() { return 0 } }' > "${INLINE_SRC}"
set -x
# Route mode-B compatibility proof emission through the explicit proof gate script.
# Emission stays optional; failure does not fail the minimal smoke.
if NYASH_SELFHOST_STAGEB_PROOF_ONLY=1 NYASH_ENABLE_USING=1 NYASH_ALLOW_USING_FILE=1 NYASH_USING_AST=1 NYASH_FEATURES=stage3 \
   bash "${PROOF_STAGEB_SCRIPT}" --source-file "${INLINE_SRC}" --route-id "SH-SMOKE-MINIMAL" > "${OUT_JSON}" 2>/dev/null; then
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

echo "[selfhost-smoke] Step 2: Run representative canonical keep example"
EXAMPLE="apps/examples/json_query/main.hako"
OUT_CANONICAL="/tmp/nyash_selfhost_compat_canonical.txt"

set -x
"${NY_BIN}" --backend vm "${EXAMPLE}" > "${OUT_CANONICAL}"
set +x

echo "[selfhost-smoke] canonical bridge example completed ($(wc -c < "${OUT_CANONICAL}") bytes)."

echo "[selfhost-smoke] PASS"
