#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
ROOT_DIR=$(cd -- "$SCRIPT_DIR/../.." &>/dev/null && pwd)
BIN="$ROOT_DIR/target/release/hakorune"
APP="$ROOT_DIR/tmp/plugin_v2_smoke_main.hako"

echo "[plugin-v2-smoke] building hakorune (release)..." >&2
cargo build --release -j 2 >/dev/null

echo "[plugin-v2-smoke] building FileBox plugin (release)..." >&2
pushd "$ROOT_DIR/plugins/nyash-filebox-plugin" >/dev/null
cargo build --release -j 2 >/dev/null
popd >/dev/null

echo "[plugin-v2-smoke] building PathBox plugin (release)..." >&2
pushd "$ROOT_DIR/plugins/nyash-path-plugin" >/dev/null
cargo build --release -j 2 >/dev/null
popd >/dev/null

echo "[plugin-v2-smoke] building Math/Time plugin (release)..." >&2
pushd "$ROOT_DIR/plugins/nyash-math-plugin" >/dev/null
cargo build --release -j 2 >/dev/null
popd >/dev/null

echo "[plugin-v2-smoke] building Regex plugin (release)..." >&2
pushd "$ROOT_DIR/plugins/nyash-regex-plugin" >/dev/null
cargo build --release -j 2 >/dev/null
popd >/dev/null

echo "[plugin-v2-smoke] building Net plugin (release)..." >&2
pushd "$ROOT_DIR/plugins/nyash-net-plugin" >/dev/null
cargo build --release -j 2 >/dev/null
popd >/dev/null

if [[ ! -x "$BIN" ]]; then
  echo "error: hakorune binary not found at $BIN" >&2
  exit 1
fi

export NYASH_CLI_VERBOSE=${NYASH_CLI_VERBOSE:-1}
export NYASH_DEBUG_PLUGIN=${NYASH_DEBUG_PLUGIN:-1}
unset NYASH_DISABLE_PLUGINS
mkdir -p "$ROOT_DIR/tmp"
cat > "$APP" <<'EOF'
static box Main {
  main() {
    return 0
  }
}
EOF

echo "[plugin-v2-smoke] running explicit compat proof: $BIN --backend vm $APP" >&2
set +e
timeout -s KILL 25s "$BIN" --backend vm "$APP" > /tmp/nyash-plugin-v2-smoke.out 2>&1
code=$?
set -e
echo "--- plugin v2 smoke output ---"
tail -n 80 /tmp/nyash-plugin-v2-smoke.out || true
echo "-------------------------------"

if [[ $code -ne 0 ]]; then
  echo "plugin-v2-smoke: hakorune exited with code $code" >&2
  exit $code
fi

grep -q "\[plugin/init\] plugin host initialized from nyash.toml" /tmp/nyash-plugin-v2-smoke.out || {
  echo "plugin-v2-smoke: missing plugin host init log" >&2; exit 1; }
grep -q "\[plugin/init\] plugin host fully configured" /tmp/nyash-plugin-v2-smoke.out || {
  echo "plugin-v2-smoke: missing plugin host configured log" >&2; exit 1; }
grep -q "\[provider/select:FileBox ring=plugin src=dynamic\]" /tmp/nyash-plugin-v2-smoke.out || {
  echo "plugin-v2-smoke: FileBox provider was not selected from plugin dynamic route" >&2; exit 1; }

echo "plugin-v2-smoke: OK" >&2

FUNC_APP="$ROOT_DIR/apps/tests/plugin_v2_functional.hako"
if [[ "${NYASH_PLUGIN_V2_FUNCTIONAL:-0}" == "1" && -f "$FUNC_APP" ]]; then
  echo "[plugin-v2-smoke] functional explicit compat proof: $BIN --backend vm $FUNC_APP" >&2
  set +e
  timeout -s KILL 25s "$BIN" --backend vm "$FUNC_APP" > /tmp/nyash-plugin-v2-func.out 2>&1
  code=$?
  set -e
  tail -n 50 /tmp/nyash-plugin-v2-func.out || true
  if [[ $code -eq 0 ]]; then
    echo "plugin-v2-functional: OK" >&2
  else
    echo "plugin-v2-functional: hakorune exited with code $code" >&2
    exit $code
  fi
fi

NET_APP="$ROOT_DIR/apps/tests/net_roundtrip.hako"
if [[ "${NYASH_PLUGIN_V2_NET_FUNCTIONAL:-0}" == "1" && -f "$NET_APP" ]]; then
  echo "[plugin-v2-smoke] functional (net) explicit compat proof: $BIN --backend vm $NET_APP" >&2
  set +e
  timeout -s KILL 25s "$BIN" --backend vm "$NET_APP" > /tmp/nyash-plugin-v2-net.out 2>&1
  code=$?
  set -e
  tail -n 60 /tmp/nyash-plugin-v2-net.out || true
  if [[ $code -eq 0 ]]; then
    echo "plugin-v2-net-functional: OK" >&2
  else
    echo "plugin-v2-net-functional: hakorune exited with code $code" >&2
    exit $code
  fi
fi

exit 0
