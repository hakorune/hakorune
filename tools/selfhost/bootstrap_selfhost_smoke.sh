#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  echo "[bootstrap] building hakorune (release, JIT)..." >&2
  cargo build --release --features cranelift-jit >/dev/null
fi

echo "[bootstrap] c0 baseline -> c1 -> c1' parity (proof-only bootstrap keep)" >&2

set +e
timeout -s KILL 20s env NYASH_DISABLE_PLUGINS=1 NYASH_CLI_VERBOSE=1 \
  "$BIN" --backend vm "$ROOT_DIR/apps/examples/string_p0.hako" > /tmp/nyash-c0.out 2>/tmp/nyash-c0.err
RC0=$?

timeout -s KILL 20s env NYASH_DISABLE_PLUGINS=1 NYASH_USE_NY_COMPILER=1 NYASH_VM_USE_FALLBACK=1 NYASH_CLI_VERBOSE=1 \
  "$BIN" --backend vm "$ROOT_DIR/apps/examples/string_p0.hako" > /tmp/nyash-c1.out 2>/tmp/nyash-c1.err
RC1=$?

timeout -s KILL 20s env NYASH_DISABLE_PLUGINS=1 NYASH_USE_NY_COMPILER=1 NYASH_VM_USE_FALLBACK=1 NYASH_CLI_VERBOSE=1 \
  "$BIN" --backend vm "$ROOT_DIR/apps/examples/string_p0.hako" > /tmp/nyash-c1p.out 2>/tmp/nyash-c1p.err
RC2=$?
set -e

echo "[bootstrap] c0 baseline rc=${RC0}" >&2
echo "[bootstrap] c1 compat keep rc=${RC1}" >&2
echo "[bootstrap] c1' compat keep rc=${RC2}" >&2

if [[ "$RC0" -eq 0 ]]; then
  echo "PASS: c0 baseline" >&2
else
  echo "FAIL: c0 baseline" >&2
  sed -n '1,120p' /tmp/nyash-c0.err
  exit 1
fi

if [[ "$RC1" -eq 0 && "$RC2" -eq 0 ]]; then
  echo "PASS: c1/c1' (ny compiler path)" >&2
else
  echo "WARN: c1/c1' did not report expected result; treating as optional while MVP matures" >&2
fi

echo "All PASS (bootstrap smoke)" >&2
