#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

PROG_JSON_TEXT='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":0}}]}'
TMP_RUNNER=""
TMP_DEFAULT="$(mktemp --suffix .phase29ci.builder_envrender.default)"
TMP_REGISTRY="$(mktemp --suffix .phase29ci.builder_envrender.registry)"
TMP_DIAG="$(mktemp --suffix .phase29ci.builder_envrender.diag)"
CURRENT_CAPTURE=""

cleanup() {
  if [ -n "$TMP_RUNNER" ]; then
    cleanup_builder_module_program_json_runner_file "$TMP_RUNNER"
  fi
  rm -f "$TMP_DEFAULT" "$TMP_REGISTRY" "$TMP_DIAG" 2>/dev/null || true
}
trap cleanup EXIT

clear_builder_env_probe_state() {
  unset PROG_JSON \
    NYASH_FEATURES \
    NYASH_ENABLE_USING \
    HAKO_ENABLE_USING \
    NYASH_FAIL_FAST \
    HAKO_PREINCLUDE \
    HAKO_MIR_BUILDER_SKIP_LOOPS \
    HAKO_MIR_BUILDER_REGISTRY_ONLY \
    NYASH_USE_NY_COMPILER \
    HAKO_MIR_BUILDER_DELEGATE \
    HAKO_MIR_BUILDER_INTERNAL \
    HAKO_MIR_BUILDER_REGISTRY \
    HAKO_MIR_BUILDER_DEBUG 2>/dev/null || true
}

execute_builder_module_program_json_runner() {
  local tmp_hako="$1"
  {
    printf 'runner=%s\n' "$tmp_hako"
    printf 'PROG_JSON=%s\n' "${PROG_JSON:-__unset__}"
    printf 'NYASH_FEATURES=%s\n' "${NYASH_FEATURES:-__unset__}"
    printf 'NYASH_ENABLE_USING=%s\n' "${NYASH_ENABLE_USING:-__unset__}"
    printf 'HAKO_ENABLE_USING=%s\n' "${HAKO_ENABLE_USING:-__unset__}"
    printf 'NYASH_FAIL_FAST=%s\n' "${NYASH_FAIL_FAST:-__unset__}"
    printf 'HAKO_PREINCLUDE=%s\n' "${HAKO_PREINCLUDE:-__unset__}"
    printf 'HAKO_MIR_BUILDER_SKIP_LOOPS=%s\n' "${HAKO_MIR_BUILDER_SKIP_LOOPS:-__unset__}"
    printf 'HAKO_MIR_BUILDER_REGISTRY_ONLY=%s\n' "${HAKO_MIR_BUILDER_REGISTRY_ONLY:-__unset__}"
    printf 'NYASH_USE_NY_COMPILER=%s\n' "${NYASH_USE_NY_COMPILER:-__unset__}"
    printf 'HAKO_MIR_BUILDER_DELEGATE=%s\n' "${HAKO_MIR_BUILDER_DELEGATE:-__unset__}"
    printf 'HAKO_MIR_BUILDER_INTERNAL=%s\n' "${HAKO_MIR_BUILDER_INTERNAL:-__unset__}"
    printf 'HAKO_MIR_BUILDER_REGISTRY=%s\n' "${HAKO_MIR_BUILDER_REGISTRY:-__unset__}"
    printf 'HAKO_MIR_BUILDER_DEBUG=%s\n' "${HAKO_MIR_BUILDER_DEBUG:-__unset__}"
  } > "$CURRENT_CAPTURE"
  return 0
}

TMP_RUNNER="$(prepare_builder_module_program_json_runner_context "hako.mir.builder.min")"
if [ ! -s "$TMP_RUNNER" ]; then
  echo "[phase29ci/probe] missing rendered builder runner" >&2
  exit 1
fi

grep -F -q 'using "hako.mir.builder.min" as MirBuilderBox' "$TMP_RUNNER" || {
  echo "[phase29ci/probe] builder runner lost using contract" >&2
  exit 1
}
grep -F -q 'local program_json = env.get("PROG_JSON")' "$TMP_RUNNER" || {
  echo "[phase29ci/probe] builder runner lost PROG_JSON contract" >&2
  exit 1
}
grep -F -q 'MirBuilderBox.emit_from_program_json_v0(program_json, null)' "$TMP_RUNNER" || {
  echo "[phase29ci/probe] builder runner lost emit_from_program_json_v0 contract" >&2
  exit 1
}
grep -F -q '[MIR_BEGIN]' "$TMP_RUNNER" || {
  echo "[phase29ci/probe] builder runner lost MIR begin marker" >&2
  exit 1
}
grep -F -q '[MIR_END]' "$TMP_RUNNER" || {
  echo "[phase29ci/probe] builder runner lost MIR end marker" >&2
  exit 1
}

clear_builder_env_probe_state
CURRENT_CAPTURE="$TMP_DEFAULT"
run_rendered_builder_module_program_json_runner "$TMP_RUNNER" "$PROG_JSON_TEXT" 0 "" 0 0
grep -F -q "runner=$TMP_RUNNER" "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] default route did not execute rendered runner" >&2
  exit 1
}
grep -F -q "PROG_JSON=$PROG_JSON_TEXT" "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] common env did not carry PROG_JSON" >&2
  exit 1
}
grep -F -q 'NYASH_FEATURES=stage3' "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] common env lost stage3 default" >&2
  exit 1
}
grep -F -q 'NYASH_ENABLE_USING=1' "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] common env lost NYASH_ENABLE_USING default" >&2
  exit 1
}
grep -F -q 'HAKO_ENABLE_USING=1' "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] common env lost HAKO_ENABLE_USING default" >&2
  exit 1
}
grep -F -q 'HAKO_PREINCLUDE=__unset__' "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] default route should not set preinclude" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_SKIP_LOOPS=__unset__' "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] default route should not set skip-loops" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_REGISTRY_ONLY=__unset__' "$TMP_DEFAULT" || {
  echo "[phase29ci/probe] default route should not set registry-only" >&2
  exit 1
}

clear_builder_env_probe_state
CURRENT_CAPTURE="$TMP_REGISTRY"
run_rendered_builder_module_program_json_runner "$TMP_RUNNER" "$PROG_JSON_TEXT" 1 "return.method.arraymap" 1 0
grep -F -q 'HAKO_PREINCLUDE=1' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route lost preinclude flag" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_SKIP_LOOPS=__unset__' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route should not set skip-loops" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_REGISTRY_ONLY=return.method.arraymap' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route lost registry-only contract" >&2
  exit 1
}
grep -F -q 'NYASH_USE_NY_COMPILER=0' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route lost NYASH_USE_NY_COMPILER default" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_DELEGATE=0' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route lost delegate default" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_INTERNAL=1' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route lost internal default" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_REGISTRY=1' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route lost registry default" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_DEBUG=1' "$TMP_REGISTRY" || {
  echo "[phase29ci/probe] registry+preinclude route lost debug default" >&2
  exit 1
}

clear_builder_env_probe_state
CURRENT_CAPTURE="$TMP_DIAG"
run_rendered_builder_module_program_json_runner "$TMP_RUNNER" "$PROG_JSON_TEXT" 1 "" 0 1
grep -F -q 'HAKO_PREINCLUDE=__unset__' "$TMP_DIAG" || {
  echo "[phase29ci/probe] diag route should not set preinclude" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_SKIP_LOOPS=1' "$TMP_DIAG" || {
  echo "[phase29ci/probe] diag route lost skip-loops flag" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_REGISTRY_ONLY=__unset__' "$TMP_DIAG" || {
  echo "[phase29ci/probe] diag route should not set registry-only when empty" >&2
  exit 1
}
grep -F -q 'HAKO_MIR_BUILDER_INTERNAL=1' "$TMP_DIAG" || {
  echo "[phase29ci/probe] diag route lost internal default" >&2
  exit 1
}

cleanup_builder_module_program_json_runner_file "$TMP_RUNNER"
TMP_RUNNER=""

echo "[phase29ci/probe] PASS"
