#!/bin/bash
# phase29bq_joinir_port01_parity_probe_vm.sh
# JIR-PORT-01 contract:
# - Same fixture is lowered through:
#   1) Rust route (`--emit-mir-json`)
#   2) .hako route (`tools/smokes/v2/lib/emit_mir_route.sh --route hako-mainline`)
# - Canonical MIR for function `main` must be identical.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="${SMOKE_NAME_OVERRIDE:-phase29bq_joinir_port01_parity_probe_vm}"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29bq_joinir_port01_if_merge_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
HAKO_ROUTE_EXPECT="${HAKO_ROUTE_EXPECT:-}"

if ! [[ "$RUN_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: timeout must be integer: $RUN_TIMEOUT_SECS"
  exit 2
fi

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $FIXTURE"
  exit 2
fi

EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi

MAIN_SIG_JQ_FILTER="$(cat <<'JQ'
(.functions | map(select(.name=="main")) | if length == 1 then .[0] else error("main missing or duplicated") end) as $m
| {
    name: $m.name,
    blocks: [
      $m.blocks[] | {
        id,
        instructions: [
          .instructions[] | {
            op,
            operation: (.operation // null),
            target: (.target // null),
            then: (.then // null),
            else: (.else // null),
            callee: (.callee // null),
            method: (.method // null),
            argc: ((.args // []) | length),
            incoming_count: ((.incoming // []) | length),
            value_const: (if .op == "const" then (.value // null) else null end)
          }
        ]
      }
    ]
  }
JQ
)"

TMP_RUST_MIR="$(mktemp /tmp/phase29bq_port01_rust_mir.XXXXXX.json)"
TMP_HAKO_MIR="$(mktemp /tmp/phase29bq_port01_hako_mir.XXXXXX.json)"
TMP_RUST_SIG="$(mktemp /tmp/phase29bq_port01_rust_sig.XXXXXX.json)"
TMP_HAKO_SIG="$(mktemp /tmp/phase29bq_port01_hako_sig.XXXXXX.json)"
RUST_LOG="$(mktemp /tmp/phase29bq_port01_rust_log.XXXXXX.log)"
HAKO_LOG="$(mktemp /tmp/phase29bq_port01_hako_log.XXXXXX.log)"
KEEP_DEBUG_ARTIFACTS=0

print_debug_artifacts() {
  echo "[INFO] rust_log=$RUST_LOG"
  echo "[INFO] hako_log=$HAKO_LOG"
  echo "[INFO] rust_mir=$TMP_RUST_MIR"
  echo "[INFO] hako_mir=$TMP_HAKO_MIR"
  echo "[INFO] rust_sig=$TMP_RUST_SIG"
  echo "[INFO] hako_sig=$TMP_HAKO_SIG"
}

retain_debug_artifacts() {
  KEEP_DEBUG_ARTIFACTS=1
  print_debug_artifacts
}

cleanup() {
  if [ "$KEEP_DEBUG_ARTIFACTS" -eq 1 ]; then
    return 0
  fi
  rm -f "$TMP_RUST_MIR" "$TMP_HAKO_MIR" "$TMP_RUST_SIG" "$TMP_HAKO_SIG" "$RUST_LOG" "$HAKO_LOG"
}
trap cleanup EXIT

TIMEOUT_MS=$((RUN_TIMEOUT_SECS * 1000))

COMMON_ENV=(
  NYASH_DISABLE_PLUGINS=1
  HAKO_JOINIR_STRICT=1
  HAKO_JOINIR_PLANNER_REQUIRED=1
  NYASH_JOINIR_STRICT=1
  NYASH_JOINIR_DEV=1
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS"
)

RUST_ROUTE_ENV=(
  NYASH_VM_USE_FALLBACK=0
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0
)

HAKO_ROUTE_ENV=(
)
HAKO_ROUTE_KIND="direct"

if [ "$HAKO_ROUTE_EXPECT" = "selfhost-first" ]; then
  HAKO_ROUTE_KIND="hako-mainline"
else
  HAKO_ROUTE_KIND="direct"
fi

run_route_with_log() {
  local log_path="$1"
  shift
  env "$@" >"$log_path" 2>&1
}

canonicalize_main_signature() {
  local input_json="$1"
  local output_json="$2"
  local route_name="$3"
  if ! jq -e -S "$MAIN_SIG_JQ_FILTER" "$input_json" >"$output_json"; then
    retain_debug_artifacts
    test_fail "$SMOKE_NAME: $route_name route main signature canonicalization failed"
    return 1
  fi
}

set +e
run_route_with_log \
  "$RUST_LOG" \
  "${COMMON_ENV[@]}" \
  "${RUST_ROUTE_ENV[@]}" \
  "$EMIT_ROUTE" --route direct --timeout-secs "$RUN_TIMEOUT_SECS" --out "$TMP_RUST_MIR" --input "$FIXTURE"
rc_rust=$?

run_route_with_log \
  "$HAKO_LOG" \
  "${COMMON_ENV[@]}" \
  "${HAKO_ROUTE_ENV[@]}" \
  "$EMIT_ROUTE" --route "$HAKO_ROUTE_KIND" --timeout-secs "$RUN_TIMEOUT_SECS" --out "$TMP_HAKO_MIR" --input "$FIXTURE"
rc_hako=$?
set -e

if [ "$rc_rust" -eq 124 ] || [ "$rc_hako" -eq 124 ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: timeout (rust=$rc_rust hako=$rc_hako)"
  exit 1
fi

if [ "$rc_rust" -ne 0 ] || [ "$rc_hako" -ne 0 ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: route failed (rust=$rc_rust hako=$rc_hako)"
  exit 1
fi

if [ ! -s "$TMP_RUST_MIR" ] || [ ! -s "$TMP_HAKO_MIR" ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: emitted MIR missing (rust/hako)"
  exit 1
fi

if [ "$HAKO_ROUTE_EXPECT" = "selfhost-first" ]; then
  if ! grep -Fq "[OK] MIR JSON written (selfhost-first):" "$HAKO_LOG"; then
    retain_debug_artifacts
    test_fail "$SMOKE_NAME: hako route did not use selfhost-first path"
    exit 1
  fi
fi

if ! canonicalize_main_signature "$TMP_RUST_MIR" "$TMP_RUST_SIG" "rust"; then
  exit 1
fi

if ! canonicalize_main_signature "$TMP_HAKO_MIR" "$TMP_HAKO_SIG" "hako"; then
  exit 1
fi

if ! jq -e '.blocks[].instructions[] | select(.op=="branch")' "$TMP_RUST_SIG" >/dev/null; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: fixture did not produce branch in rust main signature"
  exit 1
fi

HASH_RUST="$(sha256sum "$TMP_RUST_SIG" | awk '{print $1}')"
HASH_HAKO="$(sha256sum "$TMP_HAKO_SIG" | awk '{print $1}')"

if [ "$HASH_RUST" != "$HASH_HAKO" ]; then
  retain_debug_artifacts
  echo "[INFO] HASH_RUST(main_sig)=$HASH_RUST"
  echo "[INFO] HASH_HAKO(main_sig)=$HASH_HAKO"
  echo "[INFO] DIFF(main signature canonical):"
  diff -u "$TMP_RUST_SIG" "$TMP_HAKO_SIG" | sed -n '1,160p' || true
  test_fail "$SMOKE_NAME: canonical main signature mismatch"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (canonical main signature parity locked)"
