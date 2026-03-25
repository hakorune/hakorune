#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

PROG_JSON='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"get","args":[{"type":"Int","value":0}]}}]}'
TAG_PATTERN='[mirbuilder/registry:return.method.arraymap]'
TMP_CAPTURE_SNAPSHOT="$(mktemp --suffix .phase29ci.registry_tagged.prepare.snapshot)"
TMP_DIAG_STDOUT="$(mktemp --suffix .phase29ci.registry_tagged.diag.stdout)"
TMP_DIAG_SNAPSHOT="$(mktemp --suffix .phase29ci.registry_tagged.diag.snapshot)"
TMP_CAPTURE_ARGS="$(mktemp --suffix .phase29ci.registry_tagged.capture.args)"
TMP_DIAG_ARGS="$(mktemp --suffix .phase29ci.registry_tagged.diag.args)"

CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
CAPTURE_MODE="good"
DIAG_MODE="good"
PROBE_CLEANUP_SNAPSHOT=""

cleanup_probe() {
  rm -f \
    "$TMP_CAPTURE_SNAPSHOT" \
    "$TMP_DIAG_STDOUT" \
    "$TMP_DIAG_SNAPSHOT" \
    "$TMP_CAPTURE_ARGS" \
    "$TMP_DIAG_ARGS" 2>/dev/null || true
}
trap cleanup_probe EXIT

cleanup_stdout_file() {
  local tmp_stdout="${1:-}"
  CLEANUP_CALL_COUNT=$((CLEANUP_CALL_COUNT + 1))
  LAST_CLEANED_PATH="$tmp_stdout"
  if [ -n "${PROBE_CLEANUP_SNAPSHOT:-}" ] && [ -f "$tmp_stdout" ]; then
    cp "$tmp_stdout" "$PROBE_CLEANUP_SNAPSHOT"
  fi
  if [ -n "$tmp_stdout" ]; then
    rm -f "$tmp_stdout" || true
  fi
}

run_registry_builder_module_vm_to_stdout_file() {
  local builder_module="$1"
  local prog_json="$2"
  local registry_only="$3"
  local use_preinclude="${4:-0}"
  local tmp_stdout="$5"

  printf '%s\n' "${builder_module}|${registry_only}|${use_preinclude}|${prog_json}" >"$TMP_CAPTURE_ARGS"
  case "${CAPTURE_MODE}" in
    good)
      cat >"$tmp_stdout" <<'EOF'
[mirbuilder/registry:return.method.arraymap]
[MIR_BEGIN]
{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","method":"get","args":[0],"dst":1},{"op":"ret","value":1}]}]}]}
[MIR_END]
EOF
      return 0
      ;;
    bad_shape)
      printf 'noise-only\n' >"$tmp_stdout"
      return 0
      ;;
    bad_rc)
      printf 'noise-only\n' >"$tmp_stdout"
      return 9
      ;;
  esac

  printf 'unexpected capture mode\n' >&2
  return 99
}

run_program_json_via_registry_builder_module_vm_diag() {
  local builder_module="$1"
  local prog_json="$2"
  local registry_only="${3:-}"

  printf '%s\n' "${builder_module}|${registry_only}|${prog_json}" >"$TMP_DIAG_ARGS"
  case "${DIAG_MODE}" in
    good)
      cat <<'EOF'
[mirbuilder/registry:return.method.arraymap]
[MIR_BEGIN]
{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","method":"get","args":[0],"dst":1},{"op":"ret","value":1}]}]}]}
[MIR_END]
EOF
      return 0
      ;;
    repair)
      printf 'noise-only\n'
      return 7
      ;;
  esac

  printf 'unexpected diag mode\n' >&2
  return 99
}

PREPARED_STDOUT=""
CAPTURE_MODE="good"
CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
prepare_registry_tagged_mir_canary_stdout \
  "$PROG_JSON" \
  "return.method.arraymap" \
  "$TAG_PATTERN" \
  fixed \
  0 \
  PREPARED_STDOUT

if [ ! -f "$PREPARED_STDOUT" ]; then
  echo "[phase29ci/probe] prepare did not return a live stdout snapshot" >&2
  exit 1
fi
CAPTURE_ARGS="$(cat "$TMP_CAPTURE_ARGS")"
if [ "$CAPTURE_ARGS" != "hako.mir.builder|return.method.arraymap|0|$PROG_JSON" ]; then
  echo "[phase29ci/probe] prepare capture args drifted: $CAPTURE_ARGS" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract fixed "$TAG_PATTERN" "$PREPARED_STDOUT" 1; then
  echo "[phase29ci/probe] prepared stdout snapshot is malformed" >&2
  exit 1
fi
cleanup_stdout_file "$PREPARED_STDOUT"

CAPTURE_MODE="bad_shape"
CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
PREPARED_STDOUT=""
if prepare_registry_tagged_mir_canary_stdout \
  "$PROG_JSON" \
  "return.method.arraymap" \
  "$TAG_PATTERN" \
  fixed \
  1 \
  PREPARED_STDOUT; then
  echo "[phase29ci/probe] prepare accepted malformed registry stdout" >&2
  exit 1
fi
if [ "$CLEANUP_CALL_COUNT" -ne 1 ]; then
  echo "[phase29ci/probe] prepare failure cleanup count drifted: $CLEANUP_CALL_COUNT" >&2
  exit 1
fi
if [ -n "$LAST_CLEANED_PATH" ] && [ -e "$LAST_CLEANED_PATH" ]; then
  echo "[phase29ci/probe] prepare failure left temp stdout behind" >&2
  exit 1
fi

DIAG_MODE="repair"
CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
PROBE_CLEANUP_SNAPSHOT="$TMP_DIAG_SNAPSHOT"
if run_registry_builder_diag_canary \
  "hako.mir.builder" \
  "$PROG_JSON" \
  "return.method.arraymap" \
  "$TAG_PATTERN" \
  "registry_diag_probe" \
  fixed >"$TMP_DIAG_STDOUT"; then
  DIAG_RC=0
else
  DIAG_RC=$?
fi
PROBE_CLEANUP_SNAPSHOT=""

if [ "$DIAG_RC" -ne 0 ]; then
  echo "[phase29ci/probe] diag wrapper rc drifted: $DIAG_RC" >&2
  exit 1
fi
DIAG_ARGS="$(cat "$TMP_DIAG_ARGS")"
if [ "$DIAG_ARGS" != "hako.mir.builder|return.method.arraymap|$PROG_JSON" ]; then
  echo "[phase29ci/probe] diag runner args drifted: $DIAG_ARGS" >&2
  exit 1
fi
grep -F -q '[diag] rc=0' "$TMP_DIAG_STDOUT" || {
  echo "[phase29ci/probe] diag wrapper lost normalized rc output" >&2
  exit 1
}
grep -F -q '[PASS] registry_diag_probe' "$TMP_DIAG_STDOUT" || {
  echo "[phase29ci/probe] diag wrapper lost PASS output" >&2
  exit 1
}
if [ "$CLEANUP_CALL_COUNT" -ne 1 ]; then
  echo "[phase29ci/probe] diag wrapper cleanup count drifted: $CLEANUP_CALL_COUNT" >&2
  exit 1
fi
if [ -n "$LAST_CLEANED_PATH" ] && [ -e "$LAST_CLEANED_PATH" ]; then
  echo "[phase29ci/probe] diag wrapper left temp stdout behind" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract fixed "$TAG_PATTERN" "$TMP_DIAG_SNAPSHOT" 1; then
  echo "[phase29ci/probe] diag repaired snapshot is malformed" >&2
  exit 1
fi

echo "[phase29ci/probe] PASS"
