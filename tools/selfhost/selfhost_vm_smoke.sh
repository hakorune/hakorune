#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"
APP="$ROOT_DIR/apps/selfhost-minimal/main.hako"

if [ ! -x "$BIN" ]; then
  echo "[selfhost] building hakorune (release)..." >&2
  (cd "$ROOT_DIR" && cargo build --release --bin hakorune >/dev/null)
fi

if [ ! -f "$APP" ]; then
  echo "[selfhost] sample missing: $APP" >&2
  exit 2
fi

echo "[selfhost-vm-smoke] explicit proof-only VM keep" >&2
set +e
NYASH_DISABLE_PLUGINS=1 NYASH_CLI_VERBOSE=1 "$BIN" --backend vm "$APP" \
  >/tmp/nyash-selfhost-minimal.out 2>/tmp/nyash-selfhost-minimal.err
RC=$?
set -e

if [[ "$RC" -eq 0 ]]; then
  echo "PASS: selfhost-minimal (explicit proof-only VM keep)" >&2
else
  echo "FAIL: selfhost-minimal rc=$RC" >&2
  sed -n '1,120p' /tmp/nyash-selfhost-minimal.err
  exit 1
fi

echo "All PASS" >&2
