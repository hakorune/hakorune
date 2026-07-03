#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
HAKORUNE_BIN="$ROOT_DIR/target/release/hakorune"
LEGACY_NYASH_BIN="$ROOT_DIR/target/release/nyash"
if [ -x "$HAKORUNE_BIN" ]; then
  BIN="$HAKORUNE_BIN"
else
  BIN="$LEGACY_NYASH_BIN"
fi

if [ ! -x "$BIN" ]; then
  cargo build --release --features cranelift-jit >/dev/null
  BIN="$HAKORUNE_BIN"
fi

TMP=$(mktemp)
cat >"$TMP" <<'NY'
using "./definitely_missing_12345.hako" as Miss
static box Main { main(args) { return 0 } }
NY

set +e
NYASH_ENABLE_USING=1 NYASH_USING_STRICT=1 "$BIN" --backend vm "$TMP" >/tmp/hakorune-using-strict.out 2>&1
rc=$?
set -e

if [ $rc -ne 0 ]; then
  echo "PASS: strict path missing fails (rc=$rc)" >&2
else
  echo "FAIL: strict path missing did not fail" >&2
  cat /tmp/hakorune-using-strict.out >&2 || true
  exit 1
fi

echo "All PASS" >&2
