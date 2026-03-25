#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
export NYASH_ROOT="$ROOT"

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

PROG_JSON_TEXT='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":0}}]}'
TMP_CAPTURE_DIRECT="$(mktemp --suffix .phase29ci.stdout_capture.direct)"
TMP_CAPTURE_BUILDER="$(mktemp --suffix .phase29ci.stdout_capture.builder)"
TMP_CAPTURE_REGISTRY="$(mktemp --suffix .phase29ci.stdout_capture.registry)"
TMP_CAPTURE_PREINCLUDE="$(mktemp --suffix .phase29ci.stdout_capture.preinclude)"

cleanup() {
  rm -f \
    "$TMP_CAPTURE_DIRECT" \
    "$TMP_CAPTURE_BUILDER" \
    "$TMP_CAPTURE_REGISTRY" \
    "$TMP_CAPTURE_PREINCLUDE" 2>/dev/null || true
}
trap cleanup EXIT

run_program_json_via_builder_module_vm() {
  local builder_module="$1"
  local prog_json="$2"
  printf 'builder:%s:%s\n' "$builder_module" "$prog_json"
  printf 'builder-stderr\n' >&2
  return 17
}

run_program_json_via_registry_builder_module_vm() {
  local builder_module="$1"
  local prog_json="$2"
  local registry_only="${3:-}"
  printf 'registry:%s:%s:%s\n' "$builder_module" "$prog_json" "$registry_only"
  printf 'registry-stderr\n' >&2
  return 23
}

run_program_json_via_registry_builder_module_vm_with_preinclude() {
  local builder_module="$1"
  local prog_json="$2"
  local registry_only="${3:-}"
  printf 'preinclude:%s:%s:%s\n' "$builder_module" "$prog_json" "$registry_only"
  printf 'preinclude-stderr\n' >&2
  return 29
}

if capture_runner_stdout_to_file \
  run_program_json_via_builder_module_vm \
  "hako.mir.builder.min" \
  "$PROG_JSON_TEXT" \
  "" \
  "" \
  "$TMP_CAPTURE_DIRECT"; then
  DIRECT_RC=0
else
  DIRECT_RC=$?
fi

if [ "$DIRECT_RC" -ne 17 ]; then
  echo "[phase29ci/probe] direct capture rc drifted: $DIRECT_RC" >&2
  exit 1
fi
grep -F -q "builder:hako.mir.builder.min:$PROG_JSON_TEXT" "$TMP_CAPTURE_DIRECT" || {
  echo "[phase29ci/probe] direct capture lost builder stdout" >&2
  exit 1
}
if grep -F -q 'stderr' "$TMP_CAPTURE_DIRECT"; then
  echo "[phase29ci/probe] direct capture leaked stderr into stdout file" >&2
  exit 1
fi

REGISTRY_RUNNER="$(select_registry_builder_module_runner 0)"
if [ "$REGISTRY_RUNNER" != "run_program_json_via_registry_builder_module_vm" ]; then
  echo "[phase29ci/probe] registry runner selection drifted: $REGISTRY_RUNNER" >&2
  exit 1
fi

PREINCLUDE_RUNNER="$(select_registry_builder_module_runner 1)"
if [ "$PREINCLUDE_RUNNER" != "run_program_json_via_registry_builder_module_vm_with_preinclude" ]; then
  echo "[phase29ci/probe] preinclude runner selection drifted: $PREINCLUDE_RUNNER" >&2
  exit 1
fi

if run_builder_module_vm_to_stdout_file "hako.mir.builder.min" "$PROG_JSON_TEXT" "$TMP_CAPTURE_BUILDER"; then
  BUILDER_RC=0
else
  BUILDER_RC=$?
fi
if [ "$BUILDER_RC" -ne 17 ]; then
  echo "[phase29ci/probe] builder wrapper rc drifted: $BUILDER_RC" >&2
  exit 1
fi
grep -F -q "builder:hako.mir.builder.min:$PROG_JSON_TEXT" "$TMP_CAPTURE_BUILDER" || {
  echo "[phase29ci/probe] builder wrapper lost stdout contract" >&2
  exit 1
}

if run_registry_builder_module_vm_to_stdout_file "hako.mir.builder" "$PROG_JSON_TEXT" "return.method.arraymap" 0 "$TMP_CAPTURE_REGISTRY"; then
  REGISTRY_RC=0
else
  REGISTRY_RC=$?
fi
if [ "$REGISTRY_RC" -ne 23 ]; then
  echo "[phase29ci/probe] registry wrapper rc drifted: $REGISTRY_RC" >&2
  exit 1
fi
grep -F -q "registry:hako.mir.builder:$PROG_JSON_TEXT:return.method.arraymap" "$TMP_CAPTURE_REGISTRY" || {
  echo "[phase29ci/probe] registry wrapper lost stdout contract" >&2
  exit 1
}

if run_registry_builder_module_vm_to_stdout_file "hako.mir.builder" "$PROG_JSON_TEXT" "return.method.arraymap" 1 "$TMP_CAPTURE_PREINCLUDE"; then
  PREINCLUDE_RC=0
else
  PREINCLUDE_RC=$?
fi
if [ "$PREINCLUDE_RC" -ne 29 ]; then
  echo "[phase29ci/probe] preinclude wrapper rc drifted: $PREINCLUDE_RC" >&2
  exit 1
fi
grep -F -q "preinclude:hako.mir.builder:$PROG_JSON_TEXT:return.method.arraymap" "$TMP_CAPTURE_PREINCLUDE" || {
  echo "[phase29ci/probe] preinclude wrapper lost stdout contract" >&2
  exit 1
}

echo "[phase29ci/probe] PASS"
