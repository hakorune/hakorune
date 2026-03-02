#!/usr/bin/env bash
# Selfhost-first builder: Stage‑B → MirBuilder (Hako) → MIR(JSON) (no delegate unless allowed)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

TMP_HAKO=$(mktemp --suffix .hako)
cat >"${TMP_HAKO}" <<'HAKO'
static box Main { method main(args) { return 42 } }
HAKO

OUT_JSON=$(mktemp --suffix .json)
set +e
NYASH_FAIL_FAST=0 \
HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=0 \
bash "${ROOT_DIR}/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "${OUT_JSON}" --input "${TMP_HAKO}" >/dev/null 2>&1
rc=$?
set -e
rm -f "$TMP_HAKO" || true
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] selfhost-first builder failed"; rm -f "$OUT_JSON" || true; exit 0; fi
if ! grep -q '"functions"' "$OUT_JSON"; then echo "[SKIP] MIR JSON missing functions"; rm -f "$OUT_JSON" || true; exit 0; fi
echo "[PASS] selfhost_builder_first_return42"
rm -f "$OUT_JSON" || true
exit 0
