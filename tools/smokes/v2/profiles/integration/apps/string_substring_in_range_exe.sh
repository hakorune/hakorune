#!/usr/bin/env bash
# ProductAot successor for the retired CoreDirect substring smoke.
#
# Contract:
# - the checked-in source emits one canonical MIR artifact;
# - pure-first ny-llvmc consumes that same artifact with no harness replay;
# - the linked EXE prints exactly cde and exits 0.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="string_substring_in_range_exe"
APP="$ROOT_DIR/apps/tests/string_substring_in_range_min.hako"
NYLLVMC="${NYASH_NY_LLVM_COMPILER:-$ROOT_DIR/target/release/ny-llvmc}"
NYRT_DIR="${NYASH_EMIT_EXE_NYRT:-$ROOT_DIR/target/release}"
EMIT_ROUTE="$ROOT_DIR/tools/smokes/v2/lib/emit_mir_route.sh"
PREFLIGHT="$ROOT_DIR/tools/checks/pure_first_route_preflight.py"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_DIR="$(mktemp -d /tmp/hakorune_string_substring_exe.XXXXXX)"
MIR_OUT="$TMP_DIR/main.mir.json"
EXE_OUT="$TMP_DIR/main.exe"
EMIT_LOG="$TMP_DIR/emit.log"
EMIT_ERR="$TMP_DIR/emit.err"
PREFLIGHT_LOG="$TMP_DIR/preflight.log"
PREFLIGHT_ERR="$TMP_DIR/preflight.err"
BUILD_LOG="$TMP_DIR/build.log"
BUILD_STDOUT="$TMP_DIR/build.stdout"
RUN_STDOUT="$TMP_DIR/run.stdout"
RUN_STDERR="$TMP_DIR/run.stderr"
EXPECTED="$TMP_DIR/expected.stdout"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

if [ ! -f "$APP" ]; then
  test_fail "$SMOKE_NAME: source fixture missing: $APP"
  exit 1
fi
if [ ! -x "$NYLLVMC" ]; then
  test_fail "$SMOKE_NAME: ny-llvmc missing: $NYLLVMC"
  exit 1
fi
if [ ! -x "$ROOT_DIR/target/release/hakorune" ] && [ -z "${NYASH_BIN:-}" ]; then
  test_fail "$SMOKE_NAME: release hakorune binary missing"
  exit 1
fi
if [ ! -x "$PREFLIGHT" ]; then
  test_fail "$SMOKE_NAME: pure-first preflight missing: $PREFLIGHT"
  exit 1
fi

if ! bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" \
  >"$TMP_DIR/ffi_build.stdout" \
  2>"$TMP_DIR/ffi_build.stderr"; then
  sed -n '1,160p' "$TMP_DIR/ffi_build.stdout" >&2
  sed -n '1,160p' "$TMP_DIR/ffi_build.stderr" >&2
  test_fail "$SMOKE_NAME: LLVM FFI build failed"
  exit 1
fi

if ! bash "$EMIT_ROUTE" --route direct --timeout-secs "$RUN_TIMEOUT_SECS" --out "$MIR_OUT" --input "$APP" >"$EMIT_LOG" 2>"$EMIT_ERR"; then
  sed -n '1,160p' "$EMIT_LOG" >&2
  sed -n '1,160p' "$EMIT_ERR" >&2
  test_fail "$SMOKE_NAME: emit_mir_route.sh --route direct failed"
  exit 1
fi

if ! python3 - "$MIR_OUT" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if payload.get("schema_version") != "1.0":
    raise SystemExit("schema_version is not exact 1.0")
capabilities = payload.get("capabilities")
if not isinstance(capabilities, list) or "callee_typing" not in capabilities:
    raise SystemExit("callee_typing capability is missing")
functions = payload.get("functions")
if not isinstance(functions, list) or not any(item.get("name") == "main" for item in functions if isinstance(item, dict)):
    raise SystemExit("main function is missing")
PY
then
  test_fail "$SMOKE_NAME: emitted artifact is not canonical MIR 1.0 with main"
  exit 1
fi

MIR_SHA_BEFORE="$(sha256sum "$MIR_OUT" | awk '{print $1}')"
if ! python3 "$PREFLIGHT" "$MIR_OUT" >"$PREFLIGHT_LOG" 2>"$PREFLIGHT_ERR"; then
  sed -n '1,160p' "$PREFLIGHT_LOG" >&2
  sed -n '1,160p' "$PREFLIGHT_ERR" >&2
  test_fail "$SMOKE_NAME: pure_first_route_preflight.py rejected the artifact"
  exit 1
fi
MIR_SHA_AFTER_PREFLIGHT="$(sha256sum "$MIR_OUT" | awk '{print $1}')"
if [ "$MIR_SHA_BEFORE" != "$MIR_SHA_AFTER_PREFLIGHT" ]; then
  test_fail "$SMOKE_NAME: preflight changed the MIR artifact"
  exit 1
fi

set +e
NYASH_NY_LLVM_COMPILER="$NYLLVMC" \
  NYASH_LLVM_ROUTE_TRACE=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NYLLVMC" --in "$MIR_OUT" --emit exe --nyrt "$NYRT_DIR" --out "$EXE_OUT" \
  >"$BUILD_STDOUT" 2>"$BUILD_LOG"
BUILD_RC=$?
set -e

if [ "$BUILD_RC" -eq 124 ]; then
  sed -n '1,160p' "$BUILD_LOG" >&2
  test_fail "$SMOKE_NAME: ny-llvmc build timed out"
  exit 1
fi
if [ "$BUILD_RC" -ne 0 ]; then
  sed -n '1,160p' "$BUILD_LOG" >&2
  test_fail "$SMOKE_NAME: pure-first EXE build failed rc=$BUILD_RC"
  exit 1
fi
if [ ! -x "$EXE_OUT" ]; then
  test_fail "$SMOKE_NAME: linked executable is missing or not executable"
  exit 1
fi
MIR_SHA_AFTER_BUILD="$(sha256sum "$MIR_OUT" | awk '{print $1}')"
if [ "$MIR_SHA_BEFORE" != "$MIR_SHA_AFTER_BUILD" ]; then
  test_fail "$SMOKE_NAME: ny-llvmc changed the preflight MIR artifact"
  exit 1
fi
if ! grep -Fq 'owner=boundary recipe=pure-first compat_replay=none' "$BUILD_LOG" && ! grep -Fq 'owner=boundary recipe=pure-first compat_replay=none' "$BUILD_STDOUT"; then
  sed -n '1,160p' "$BUILD_LOG" >&2
  sed -n '1,160p' "$BUILD_STDOUT" >&2
  test_fail "$SMOKE_NAME: boundary pure-first route trace missing"
  exit 1
fi
if grep -Fq '[llvm-route/replay] lane=harness' "$BUILD_LOG" || grep -Fq '[llvm-route/replay] lane=harness' "$BUILD_STDOUT"; then
  test_fail "$SMOKE_NAME: harness replay was used"
  exit 1
fi
if grep -Fq 'unsupported pure shape' "$BUILD_LOG" || grep -Fq 'unsupported pure shape' "$BUILD_STDOUT"; then
  test_fail "$SMOKE_NAME: unsupported pure shape was reported"
  exit 1
fi

printf 'cde\n' >"$EXPECTED"
set +e
NYASH_NYRT_SILENT_RESULT=1 \
  timeout "$RUN_TIMEOUT_SECS" "$EXE_OUT" >"$RUN_STDOUT" 2>"$RUN_STDERR"
RUN_RC=$?
set -e

if [ "$RUN_RC" -eq 124 ]; then
  test_fail "$SMOKE_NAME: executable run timed out"
  exit 1
fi
if [ "$RUN_RC" -ne 0 ]; then
  sed -n '1,160p' "$RUN_STDERR" >&2
  test_fail "$SMOKE_NAME: executable rc=$RUN_RC (expected 0)"
  exit 1
fi
if ! cmp -s "$EXPECTED" "$RUN_STDOUT"; then
  sed -n '1,160p' "$RUN_STDOUT" >&2
  test_fail "$SMOKE_NAME: stdout is not byte-exact cde"
  exit 1
fi
if [ -s "$RUN_STDERR" ]; then
  sed -n '1,160p' "$RUN_STDERR" >&2
  test_fail "$SMOKE_NAME: stderr is not empty"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (exact cde stdout, rc=0, same MIR artifact, boundary pure-first, compat_replay=none)"
