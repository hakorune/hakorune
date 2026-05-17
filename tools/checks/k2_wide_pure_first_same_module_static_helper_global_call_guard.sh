#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-pure-first-same-module-static-helper-global-call"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/pure-first-same-module-static-helper-global-call-proof/main.hako"
APP_README="apps/pure-first-same-module-static-helper-global-call-proof/README.md"
APP_TEST="apps/pure-first-same-module-static-helper-global-call-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-622-PURE-FIRST-GLOBAL-CALL-001-SAME-MODULE-STATIC-HELPER-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"

printf '[%s] checking PURE-FIRST-GLOBAL-CALL-001 same-module static helper route\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD" "$INDEX" "$PROOF_MANIFEST"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'PURE-FIRST-GLOBAL-CALL-001' "$CARD" "card must define this route row"
guard_expect_in_file "$TAG" 'PURE-FIRST-GLOBAL-CALL-001' "$PROOF_MANIFEST" "proof manifest must list the compiler sidecar"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list this guard"

if rg -n 'pure-first-same-module-static-helper-global-call-proof|seedScalar|makeReport|readReport' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: focused helper proof leaked into backend .inc matchers" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_pure_first_gcall.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/gcall.mir.json"
exe_out="$tmp_dir/gcall.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'pure-first-same-module-static-helper-global-call-proof' "$vm_log"
rg -F -q 'seed=32' "$vm_log"
rg -F -q 'report=32,42' "$vm_log"
rg -F -q 'total=74' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)

main = next((fn for fn in data.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main")

routes = [
    row for row in main.get("metadata", {}).get("global_call_routes", [])
    if str(row.get("callee_name", "")).startswith("Main.")
]

expected = {
    "Main.seedScalar/2": ("typed_global_call_same_module_scalar_i64", "ScalarI64", None),
    "Main.makeReport/1": (
        "typed_global_call_same_module_object_handle",
        "object_handle",
        "HelperReport",
    ),
}

seen = {}
for route in routes:
    callee = route.get("callee_name")
    if callee not in expected:
        continue
    proof, return_shape, result_box = expected[callee]
    if route.get("tier") != "DirectAbi":
        raise SystemExit(f"{callee}: expected DirectAbi, got {route.get('tier')}")
    if route.get("emit_kind") != "direct_function_call":
        raise SystemExit(f"{callee}: expected direct_function_call")
    if route.get("proof") != proof:
        raise SystemExit(f"{callee}: expected proof {proof}, got {route.get('proof')}")
    if route.get("return_shape") != return_shape:
        raise SystemExit(f"{callee}: expected return_shape {return_shape}, got {route.get('return_shape')}")
    if result_box is not None and route.get("target_result_box_name") != result_box:
        raise SystemExit(f"{callee}: expected target_result_box_name {result_box}, got {route.get('target_result_box_name')}")
    seen[callee] = True

missing = sorted(set(expected) - set(seen))
if missing:
    raise SystemExit(f"missing expected helper routes: {missing}")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'pure-first-same-module-static-helper-global-call-proof' "$run_log"
rg -F -q 'seed=32' "$run_log"
rg -F -q 'report=32,42' "$run_log"
rg -F -q 'total=74' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"
