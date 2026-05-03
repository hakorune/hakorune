#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env >/dev/null || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json"
TMP_HAKO="$(mktemp --suffix .phase29ck_backend_recipe_profile_probe.hako)"
TMP_MIR="$(mktemp --suffix .phase29ck_backend_recipe_profile_probe.json)"
TMP_LOG="$(mktemp --suffix .phase29ck_backend_recipe_profile_probe.log)"
trap 'rm -f "$TMP_HAKO" "$TMP_MIR" "$TMP_LOG"' EXIT

if [ ! -f "$FIXTURE" ]; then
  echo "[FAIL] phase29ck_backend_recipe_profile_probe: fixture missing: $FIXTURE" >&2
  exit 1
fi

apply_patch_hako() {
  cat <<'SRC'
using selfhost.shared.backend.recipe as BackendRecipeBox

static box Main {
  main() {
    local profile = BackendRecipeBox.compile_route_profile("/tmp/method_call_only_small.prebuilt.mir.json")
    if profile == null {
      return 1
    }
    print("route_profile=" + ("" + profile.get("route_profile")))
    print("acceptance_policy=" + ("" + profile.get("acceptance_policy")))
    print("acceptance_case=" + ("" + profile.get("acceptance_case")))
    print("compile_recipe=" + ("" + profile.get("compile_recipe")))
    print("compat_replay=" + ("" + profile.get("compat_replay")))
    return 0
  }
}
SRC
}

apply_patch_hako >"$TMP_HAKO"

set +e
HAKO_MIR_BUILDER_FUNCS=1 \
"$NYASH_BIN" --emit-mir-json "$TMP_MIR" "$TMP_HAKO" >"$TMP_LOG" 2>&1
RC=$?
set -e

if [ "$RC" -ne 0 ]; then
  cat "$TMP_LOG" >&2
  echo "[FAIL] phase29ck_backend_recipe_profile_probe: MIR emit failed (rc=$RC)" >&2
  exit 1
fi

if [ ! -f "$TMP_MIR" ]; then
  echo "[FAIL] phase29ck_backend_recipe_profile_probe: MIR output missing: $TMP_MIR" >&2
  exit 1
fi

for expected in \
  '"name": "BackendRecipeBox.compile_route_profile/1"' \
  '"name": "BackendRecipeBox._acceptance_case_compat_bucket_for/1"' \
  '"name": "BackendRecipeBox.acceptance_case_method_call_only_small_compat_v1/0"' \
  '"value": "method_call_only_small.prebuilt.mir.json"' \
  '"value": "boundary-pure-seed-matrix-v1"' \
  '"value": "method-call-only-small-compat-v1"'
do
  if ! grep -Fq "$expected" "$TMP_MIR"; then
    cat "$TMP_LOG" >&2
    echo "[FAIL] phase29ck_backend_recipe_profile_probe: missing MIR evidence: $expected" >&2
    exit 1
  fi
done

echo "[PASS] phase29ck_backend_recipe_profile_probe"
