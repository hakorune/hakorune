#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

PROG_JSON='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"get","args":[{"type":"Int","value":0}]}}]}'
TAG_PATTERN='[mirbuilder/registry:return.method.arraymap]'
TMP_PREPARE_ARGS="$(mktemp --suffix .phase29ci.method_arraymap.prepare.args)"
TMP_LIVE_PREPARED="$(mktemp --suffix .phase29ci.method_arraymap.live.stdout)"
TMP_LIVE_SNAPSHOT="$(mktemp --suffix .phase29ci.method_arraymap.live.snapshot)"
TMP_SYNTH_SNAPSHOT="$(mktemp --suffix .phase29ci.method_arraymap.synth.snapshot)"
TMP_METHOD_MISS="$(mktemp --suffix .phase29ci.method_arraymap.method_miss.stdout)"
TMP_ARGS_MISS="$(mktemp --suffix .phase29ci.method_arraymap.args_miss.stdout)"
TMP_CALL_MISS="$(mktemp --suffix .phase29ci.method_arraymap.call_miss.stdout)"
TMP_STDOUT="$(mktemp --suffix .phase29ci.method_arraymap.canary.stdout)"

PREPARE_MODE="live"
CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
PROBE_CLEANUP_SNAPSHOT=""

cleanup_probe() {
  rm -f \
    "$TMP_PREPARE_ARGS" \
    "$TMP_LIVE_PREPARED" \
    "$TMP_LIVE_SNAPSHOT" \
    "$TMP_SYNTH_SNAPSHOT" \
    "$TMP_METHOD_MISS" \
    "$TMP_ARGS_MISS" \
    "$TMP_CALL_MISS" \
    "$TMP_STDOUT" 2>/dev/null || true
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

prepare_registry_tagged_mir_canary_stdout() {
  local prog_json="$1"
  local registry_only="$2"
  local expected_tag_label="$3"
  local grep_mode="$4"
  local use_preinclude="${5:-0}"
  local __outvar="${6:-}"

  printf '%s\n' "${registry_only}|${expected_tag_label}|${grep_mode}|${use_preinclude}|${prog_json}" >"$TMP_PREPARE_ARGS"
  case "$PREPARE_MODE" in
    live)
      printf -v "$__outvar" '%s' "$TMP_LIVE_PREPARED"
      return 0
      ;;
    fallback)
      return 1
      ;;
  esac

  printf 'unexpected PREPARE_MODE=%s\n' "$PREPARE_MODE" >&2
  return 99
}

cat >"$TMP_LIVE_PREPARED" <<'EOF'
[mirbuilder/registry:return.method.arraymap]
[MIR_BEGIN]
{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","method":"get","args":[0],"dst":1},{"op":"ret","value":1}]}]}]}
[MIR_END]
EOF

PREPARE_MODE="live"
PREPARED_STDOUT=""
prepare_registry_method_arraymap_stdout_snapshot \
  "$PROG_JSON" \
  "return.method.arraymap" \
  "$TAG_PATTERN" \
  '"method":"get"' \
  '"args":\[[0-9]' \
  1 \
  PREPARED_STDOUT

if [ "$PREPARED_STDOUT" != "$TMP_LIVE_PREPARED" ]; then
  echo "[phase29ci/probe] live prepare path stopped handing through the original stdout snapshot" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract fixed "$TAG_PATTERN" "$PREPARED_STDOUT" 1; then
  echo "[phase29ci/probe] live prepare snapshot is malformed" >&2
  exit 1
fi
PREPARE_ARGS="$(cat "$TMP_PREPARE_ARGS")"
if [ "$PREPARE_ARGS" != "return.method.arraymap|$TAG_PATTERN|fixed|1|$PROG_JSON" ]; then
  echo "[phase29ci/probe] live prepare args drifted: $PREPARE_ARGS" >&2
  exit 1
fi

PREPARE_MODE="fallback"
FALLBACK_STDOUT=""
prepare_registry_method_arraymap_stdout_snapshot \
  "$PROG_JSON" \
  "return.method.arraymap" \
  "$TAG_PATTERN" \
  '"method":"set"' \
  '"args":\[[0-9]+,[0-9]+' \
  0 \
  FALLBACK_STDOUT

if [ ! -f "$FALLBACK_STDOUT" ]; then
  echo "[phase29ci/probe] synth fallback did not create a stdout snapshot" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract fixed "$TAG_PATTERN" "$FALLBACK_STDOUT" 1; then
  echo "[phase29ci/probe] synth fallback snapshot is malformed" >&2
  exit 1
fi
FALLBACK_MIR="$(extract_builder_mir_from_stdout_file "$FALLBACK_STDOUT")"
if ! echo "$FALLBACK_MIR" | grep -q '"method":"set"'; then
  echo "[phase29ci/probe] synth fallback lost the requested method token" >&2
  exit 1
fi
if ! echo "$FALLBACK_MIR" | grep -E -q '"args":\[[0-9]+,[0-9]+'; then
  echo "[phase29ci/probe] synth fallback lost the requested args token shape" >&2
  exit 1
fi
rm -f "$FALLBACK_STDOUT"

cat >"$TMP_METHOD_MISS" <<'EOF'
[mirbuilder/registry:return.method.arraymap]
[MIR_BEGIN]
{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","method":"length","args":[0],"dst":1},{"op":"ret","value":1}]}]}]}
[MIR_END]
EOF
if run_registry_method_arraymap_token_policy "$TMP_METHOD_MISS" '"method":"get"' '"args":\[[0-9]' >"$TMP_STDOUT"; then
  echo "[phase29ci/probe] method token miss unexpectedly passed" >&2
  exit 1
fi
grep -F -q '[SKIP] method token missing' "$TMP_STDOUT" || {
  echo "[phase29ci/probe] method token miss lost its stable skip message" >&2
  exit 1
}

cat >"$TMP_ARGS_MISS" <<'EOF'
[mirbuilder/registry:return.method.arraymap]
[MIR_BEGIN]
{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","method":"get","args":[],"dst":1},{"op":"ret","value":1}]}]}]}
[MIR_END]
EOF
if run_registry_method_arraymap_token_policy "$TMP_ARGS_MISS" '"method":"get"' '"args":\[[0-9]' >"$TMP_STDOUT"; then
  echo "[phase29ci/probe] args token miss unexpectedly passed" >&2
  exit 1
fi
grep -F -q '[SKIP] args token missing' "$TMP_STDOUT" || {
  echo "[phase29ci/probe] args token miss lost its stable skip message" >&2
  exit 1
}

cat >"$TMP_CALL_MISS" <<'EOF'
[mirbuilder/registry:return.method.arraymap]
[MIR_BEGIN]
{"meta":{"method":"get"},"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"ret","value":1}]}]}]}
[MIR_END]
EOF
if run_registry_method_arraymap_token_policy "$TMP_CALL_MISS" '"method":"get"' '' >"$TMP_STDOUT"; then
  echo "[phase29ci/probe] mir_call miss unexpectedly passed" >&2
  exit 1
fi
grep -F -q '[SKIP] mir_call op missing' "$TMP_STDOUT" || {
  echo "[phase29ci/probe] mir_call miss lost its stable skip message" >&2
  exit 1
}

PREPARE_MODE="live"
CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
PROBE_CLEANUP_SNAPSHOT="$TMP_LIVE_SNAPSHOT"
if run_registry_method_arraymap_canary \
  "$PROG_JSON" \
  "return.method.arraymap" \
  "$TAG_PATTERN" \
  "registry_method_arraymap_live" \
  '"method":"get"' \
  '"args":\[[0-9]' \
  1 >"$TMP_STDOUT"; then
  LIVE_RC=0
else
  LIVE_RC=$?
fi
PROBE_CLEANUP_SNAPSHOT=""
if [ "$LIVE_RC" -ne 0 ]; then
  echo "[phase29ci/probe] live canary rc drifted: $LIVE_RC" >&2
  exit 1
fi
grep -F -q '[PASS] registry_method_arraymap_live' "$TMP_STDOUT" || {
  echo "[phase29ci/probe] live canary lost PASS output" >&2
  exit 1
}
if [ "$CLEANUP_CALL_COUNT" -ne 1 ]; then
  echo "[phase29ci/probe] live canary cleanup count drifted: $CLEANUP_CALL_COUNT" >&2
  exit 1
fi
if [ -e "$LAST_CLEANED_PATH" ]; then
  echo "[phase29ci/probe] live canary left temp stdout behind" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract fixed "$TAG_PATTERN" "$TMP_LIVE_SNAPSHOT" 1; then
  echo "[phase29ci/probe] live canary cleanup snapshot is malformed" >&2
  exit 1
fi

PREPARE_MODE="fallback"
CLEANUP_CALL_COUNT=0
LAST_CLEANED_PATH=""
PROBE_CLEANUP_SNAPSHOT="$TMP_SYNTH_SNAPSHOT"
if run_registry_method_arraymap_canary \
  "$PROG_JSON" \
  "return.method.arraymap" \
  "$TAG_PATTERN" \
  "registry_method_arraymap_synth" \
  '"method":"set"' \
  '"args":\[[0-9]+,[0-9]+' \
  0 >"$TMP_STDOUT"; then
  SYNTH_RC=0
else
  SYNTH_RC=$?
fi
PROBE_CLEANUP_SNAPSHOT=""
if [ "$SYNTH_RC" -ne 0 ]; then
  echo "[phase29ci/probe] synth canary rc drifted: $SYNTH_RC" >&2
  exit 1
fi
grep -F -q '[PASS] registry_method_arraymap_synth' "$TMP_STDOUT" || {
  echo "[phase29ci/probe] synth canary lost PASS output" >&2
  exit 1
}
if [ "$CLEANUP_CALL_COUNT" -ne 1 ]; then
  echo "[phase29ci/probe] synth canary cleanup count drifted: $CLEANUP_CALL_COUNT" >&2
  exit 1
fi
if [ -e "$LAST_CLEANED_PATH" ]; then
  echo "[phase29ci/probe] synth canary left temp stdout behind" >&2
  exit 1
fi
if ! stdout_file_matches_tagged_mir_contract fixed "$TAG_PATTERN" "$TMP_SYNTH_SNAPSHOT" 1; then
  echo "[phase29ci/probe] synth canary cleanup snapshot is malformed" >&2
  exit 1
fi
SYNTH_MIR="$(extract_builder_mir_from_stdout_file "$TMP_SYNTH_SNAPSHOT")"
if ! echo "$SYNTH_MIR" | grep -q '"method":"set"'; then
  echo "[phase29ci/probe] synth canary snapshot lost method token" >&2
  exit 1
fi
if ! echo "$SYNTH_MIR" | grep -E -q '"args":\[[0-9]+,[0-9]+'; then
  echo "[phase29ci/probe] synth canary snapshot lost args token shape" >&2
  exit 1
fi

echo "[phase29ci/probe] PASS"
