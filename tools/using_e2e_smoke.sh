#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
RUN_SH="$ROOT_DIR/tools/selfhost/run.sh"

APP="$ROOT_DIR/apps/using-e2e/main.hako"
if [ ! -f "$APP" ]; then
  echo "[using-e2e] scaffolding sample..." >&2
  mkdir -p "$ROOT_DIR/apps/using-e2e"
  cat > "$APP" <<'NYCODE'
// using/nyash.link E2E sample (MVP)
using acme.util

static box Main {
  init { }
  main(args) {
    // using line should be accepted when NYASH_ENABLE_USING=1
    return 0
  }
}
NYCODE
fi

set +e
NYASH_DISABLE_PLUGINS=1 NYASH_ENABLE_USING=1 NYASH_CLI_VERBOSE=1 \
  bash "$RUN_SH" --runtime --runtime-route mainline --input "$APP" --timeout-secs 20 \
  > /tmp/nyash-using-e2e.out 2>&1
CODE=$?
set -e
if [ "$CODE" -eq 0 ]; then
  echo "PASS: using/nyash.link E2E (placeholder)" >&2
else
  echo "FAIL: using/nyash.link E2E" >&2; sed -n '1,120p' /tmp/nyash-using-e2e.out; exit 1
fi

echo "All PASS" >&2
