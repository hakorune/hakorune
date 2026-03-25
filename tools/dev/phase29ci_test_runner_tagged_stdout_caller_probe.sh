#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

PROG_JSON='{"version":0,"kind":"Program","body":[]}'
TAG_PATTERN_BASIC='\[mirbuilder/min:return.binop.intint\]'
TAG_PATTERN_EXTENDED='\[mirbuilder/(min|registry):return.binop.intint\]'

TMP_FORWARD="$(mktemp --suffix .phase29ci.tagged_stdout.forward)"
TMP_GOOD_STDOUT="$(mktemp --suffix .phase29ci.tagged_stdout.good.stdout)"
TMP_BAD_STDOUT="$(mktemp --suffix .phase29ci.tagged_stdout.bad.stdout)"
TMP_GOOD_SNAPSHOT="$(mktemp --suffix .phase29ci.tagged_stdout.good.snapshot)"
TMP_BAD_SNAPSHOT="$(mktemp --suffix .phase29ci.tagged_stdout.bad.snapshot)"

CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
CALLER_PROBE_SNAPSHOT_PATH=""

cleanup_probe() {
  rm -f \
    "$TMP_FORWARD" \
    "$TMP_GOOD_STDOUT" \
    "$TMP_BAD_STDOUT" \
    "$TMP_GOOD_SNAPSHOT" \
    "$TMP_BAD_SNAPSHOT" 2>/dev/null || true
}
trap cleanup_probe EXIT

cleanup_stdout_file() {
  local tmp_stdout="${1:-}"
  CLEANUP_CALL_COUNT=$((CLEANUP_CALL_COUNT + 1))
  LAST_CLEANED_PATH="$tmp_stdout"
  if [ -n "${CALLER_PROBE_SNAPSHOT_PATH:-}" ] && [ -f "$tmp_stdout" ]; then
    cp "$tmp_stdout" "$CALLER_PROBE_SNAPSHOT_PATH"
  fi
  if [ -n "$tmp_stdout" ]; then
    rm -f "$tmp_stdout" || true
  fi
}

stub_forwarding_runner() {
  local _builder_module="$1"
  local _prog_json="$2"
  local _runner_arg3="$3"
  local _runner_arg4="$4"
  local tmp_stdout="$5"

  printf 'noise-only\n' >"$tmp_stdout"
  return 17
}

stub_good_builder_tag_runner() {
  local _builder_module="$1"
  local _prog_json="$2"
  local _runner_arg3="$3"
  local _runner_arg4="$4"
  local tmp_stdout="$5"

  cat >"$tmp_stdout" <<'EOF'
[mirbuilder/min:return.binop.intint]
[MIR_BEGIN]
{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"ret","value":0}]}]}]}
[MIR_END]
EOF
  return 0
}

stub_bad_registry_tag_runner() {
  local _builder_module="$1"
  local _prog_json="$2"
  local _runner_arg3="$3"
  local _runner_arg4="$4"
  local tmp_stdout="$5"

  printf 'noise-only\n' >"$tmp_stdout"
  return 9
}

if run_stdout_tag_runner_to_file \
  stub_forwarding_runner \
  "hako.mir.builder.min" \
  "$PROG_JSON" \
  "" \
  "" \
  "$TMP_FORWARD"; then
  FORWARD_RC=0
else
  FORWARD_RC=$?
fi

if [ "$FORWARD_RC" -ne 17 ]; then
  echo "[phase29ci/probe] forwarding rc drifted: $FORWARD_RC" >&2
  exit 1
fi
grep -Fx -q 'noise-only' "$TMP_FORWARD" || {
  echo "[phase29ci/probe] forwarding wrapper lost stdout payload" >&2
  exit 1
}
if stdout_file_matches_tagged_mir_contract basic "$TAG_PATTERN_BASIC" "$TMP_FORWARD" 1; then
  echo "[phase29ci/probe] forwarding wrapper started validating tagged stdout" >&2
  exit 1
fi

CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
CALLER_PROBE_SNAPSHOT_PATH="$TMP_GOOD_SNAPSHOT"
if run_stdout_tag_canary \
  stub_good_builder_tag_runner \
  basic \
  "hako.mir.builder.min" \
  "$PROG_JSON" \
  "" \
  0 \
  "$TAG_PATTERN_BASIC" \
  "caller_good" \
  "skip exec" \
  "skip tag" \
  "skip mir" \
  1 \
  0 >"$TMP_GOOD_STDOUT"; then
  GOOD_RC=0
else
  GOOD_RC=$?
fi

if [ "$GOOD_RC" -ne 0 ]; then
  echo "[phase29ci/probe] good caller path rc drifted: $GOOD_RC" >&2
  exit 1
fi
grep -F -q '[PASS] caller_good' "$TMP_GOOD_STDOUT" || {
  echo "[phase29ci/probe] good caller path lost PASS output" >&2
  exit 1
}
if [ "$CLEANUP_CALL_COUNT" -ne 1 ]; then
  echo "[phase29ci/probe] good caller path cleanup count drifted: $CLEANUP_CALL_COUNT" >&2
  exit 1
fi
if [ -e "$LAST_CLEANED_PATH" ]; then
  echo "[phase29ci/probe] good caller path left temp stdout behind" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract basic "$TAG_PATTERN_BASIC" "$TMP_GOOD_SNAPSHOT" 1; then
  echo "[phase29ci/probe] good caller snapshot no longer matches tagged contract" >&2
  exit 1
fi

CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
CALLER_PROBE_SNAPSHOT_PATH="$TMP_BAD_SNAPSHOT"
if run_stdout_tag_canary \
  stub_bad_registry_tag_runner \
  extended \
  "hako.mir.builder" \
  "$PROG_JSON" \
  "return.method.arraymap" \
  0 \
  "$TAG_PATTERN_EXTENDED" \
  "caller_repair" \
  "skip exec" \
  "skip tag" \
  "skip mir" \
  1 \
  0 >"$TMP_BAD_STDOUT"; then
  BAD_RC=0
else
  BAD_RC=$?
fi

if [ "$BAD_RC" -ne 0 ]; then
  echo "[phase29ci/probe] repair caller path rc drifted: $BAD_RC" >&2
  exit 1
fi
grep -F -q '[PASS] caller_repair' "$TMP_BAD_STDOUT" || {
  echo "[phase29ci/probe] repair caller path lost PASS output" >&2
  exit 1
}
if [ "$CLEANUP_CALL_COUNT" -ne 1 ]; then
  echo "[phase29ci/probe] repair caller path cleanup count drifted: $CLEANUP_CALL_COUNT" >&2
  exit 1
fi
if [ -e "$LAST_CLEANED_PATH" ]; then
  echo "[phase29ci/probe] repair caller path left temp stdout behind" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract extended "$TAG_PATTERN_EXTENDED" "$TMP_BAD_SNAPSHOT" 1; then
  echo "[phase29ci/probe] repair caller snapshot is malformed" >&2
  exit 1
fi
BAD_TAG="$(head -n 1 "$TMP_BAD_SNAPSHOT")"
if [ "$BAD_TAG" != "[mirbuilder/registry:return.binop.intint]" ]; then
  echo "[phase29ci/probe] registry caller flavor normalization drifted: $BAD_TAG" >&2
  exit 1
fi

echo "[phase29ci/probe] PASS"
