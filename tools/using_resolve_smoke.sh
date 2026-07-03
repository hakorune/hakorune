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

TMP=$(mktemp --suffix=.hako)
cat >"$TMP" <<'HAKO'
static box Main { main(args) { return 0 } }
HAKO

# Use hako.toml module alias resolution via CLI. File-path using is rejected
# in the current production profile; named modules are the supported surface.
set +e
NYASH_DISABLE_PLUGINS=1 "$BIN" --backend vm "$TMP" \
  --using 'sh_core as Util' > /tmp/hakorune-using-resolve.out 2>&1
rc=$?
set -e

if [ "$rc" -eq 0 ] && rg -q '^RC:[[:space:]]*0([[:space:]]|$)' /tmp/hakorune-using-resolve.out; then
  echo "PASS: using resolve (CLI module alias)" >&2
else
  echo "FAIL: using resolve (CLI module alias, rc=$rc)" >&2
  sed -n '1,120p' /tmp/hakorune-using-resolve.out >&2
  exit 1
fi

echo "All PASS" >&2
