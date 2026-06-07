#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-mimalloc-pre-realloc-release-invariant"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

OBSERVER="lang/src/hako_alloc/memory/page_map_release_invariant_box.hako"
PAGE_RELEASE="lang/src/hako_alloc/memory/page_map_release_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-pre-realloc-release-invariant-proof/main.hako"
APP_TEST="apps/mimalloc-pre-realloc-release-invariant-proof/test.sh"
APP_README="apps/mimalloc-pre-realloc-release-invariant-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-183-M173-PRE-REALLOC-RELEASE-INVARIANT-FREEZE.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_pre_realloc_release_invariant.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_pre_realloc_release_invariant.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_pre_realloc_release_invariant.mir.json"

echo "[$TAG] checking M173 pre-realloc release invariant freeze"

guard_require_files \
  "$TAG" \
  "$OBSERVER" \
  "$PAGE_RELEASE" \
  "$PAGE_MAP" \
  "$PAGE_BOX" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.page_map_release_invariant_box = "memory/page_map_release_invariant_box.hako"' "$MODULE" "hako module must export the M173 observer box"
guard_expect_in_file "$TAG" 'box HakoAllocPageMapReleaseObserver' "$OBSERVER" "M173 observer owner must exist"
guard_expect_in_file "$TAG" 'birth\(seam\)' "$OBSERVER" "M173 observer must take the M172 seam explicitly"
guard_expect_in_file "$TAG" 'me\.page_map = seam\.page_map' "$OBSERVER" "M173 observer must read ownership through the seam page-map"
guard_expect_in_file "$TAG" 'handleIsLive\(ptr\)' "$OBSERVER" "M173 observer must expose handle liveness checks"
guard_expect_in_file "$TAG" 'beginRelease\(ptr\)' "$OBSERVER" "M173 observer must expose pre-release observation"
guard_expect_in_file "$TAG" 'finishRelease\(ptr, result\)' "$OBSERVER" "M173 observer must expose post-release observation"
guard_expect_in_file "$TAG" 'me\.page_map\.lookup\(ptr\)' "$OBSERVER" "M173 observer must observe page-map lookup"
guard_expect_in_file "$TAG" 'observe_count: usize = 0' "$OBSERVER" "observer count must be exact usize storage"
guard_expect_in_file "$TAG" 'success_count: usize = 0' "$OBSERVER" "success count must be exact usize storage"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$OBSERVER" "reject count must be exact usize storage"
guard_expect_in_file "$TAG" 'live_count_before: i64 = 0' "$OBSERVER" "before snapshots must remain signed observers"
guard_expect_in_file "$TAG" 'last_page_id: i64 = -1' "$OBSERVER" "last_page_id sentinel must remain signed"
guard_expect_in_file "$TAG" 'last_block_id: i64 = -1' "$OBSERVER" "last_block_id sentinel must remain signed"
guard_expect_in_file "$TAG" 'last_live_count_delta: i64 = 0' "$OBSERVER" "delta observers must remain signed"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_release_invariant_box as HakoAllocPageMapReleaseInvariantBox' "$APP" "proof app must import the M173 observer box"
guard_expect_in_file "$TAG" 'seam\.releasePtr\(' "$APP" "proof app must keep release execution in the M172 seam"
guard_expect_in_file "$TAG" 'box ProofCheck' "$APP" "M173 proof app must keep labelled proof checks readable"
guard_expect_in_file "$TAG" 'proof\.expect' "$APP" "M173 proof app must use labelled proof expectations"
guard_expect_in_file "$TAG" '293x-183 M173 Pre-Realloc Release Invariant Freeze' "$CARD" "missing M173 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M173 guard"
guard_expect_in_file "$TAG" 'M173 pre-realloc release invariant freeze' "$PLAN" "plan must retain the M173 row"
guard_expect_in_file "$TAG" 'HakoAllocPageMapReleaseObserver' "$ROOT_README" "root README must document the M173 observer"
guard_expect_in_file "$TAG" 'page_map_release_invariant_box.hako' "$MEMORY_README" "memory README must document the M173 observer module"

if rg -n 'init[[:space:]]*\{' "$OBSERVER" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M173 observer must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'HakoAllocUsizeFieldProbe|usize_field_probe' "$OBSERVER" "$APP" >/tmp/"$TAG".usize_probe 2>&1; then
  echo "[$TAG] ERROR: M173 must not depend on the usize probe owner" >&2
  cat /tmp/"$TAG".usize_probe >&2
  rm -f /tmp/"$TAG".usize_probe
  exit 1
fi
rm -f /tmp/"$TAG".usize_probe

if rg -n ': usize' "$APP" >/tmp/"$TAG".usize_app 2>&1; then
  echo "[$TAG] ERROR: M173 proof app must not introduce extra usize locals or fields" >&2
  cat /tmp/"$TAG".usize_app >&2
  rm -f /tmp/"$TAG".usize_app
  exit 1
fi
rm -f /tmp/"$TAG".usize_app

if rg -n '\.register\(|releaseLocal\(|\.unregister\(|\.releasePtr\(' "$OBSERVER" >/tmp/"$TAG".owner_leak 2>&1; then
  echo "[$TAG] ERROR: M173 observer must not take ownership of registration, page release, unregister, or seam execution logic" >&2
  cat /tmp/"$TAG".owner_leak >&2
  rm -f /tmp/"$TAG".owner_leak
  exit 1
fi
rm -f /tmp/"$TAG".owner_leak

if rg -n 'realloc\(|\.realloc\(|reallocate\(|aligned[A-Z_(]|huge[A-Z_(]|secure[A-Z_(]|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|OSVM|OsVm|provider|hook|replacement|hako_mem_|externcall|memcpy|copy_bytes|release_bytes|unreserve' \
  "$OBSERVER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M173 leaked out of pre-realloc release invariant scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden


if rg -n '&&' "$APP" >/tmp/"$TAG".proof_conjunction 2>&1; then
  echo "[$TAG] ERROR: proof app must not regress to a giant && summary condition" >&2
  cat /tmp/"$TAG".proof_conjunction >&2
  rm -f /tmp/"$TAG".proof_conjunction
  exit 1
fi
rm -f /tmp/"$TAG".proof_conjunction

if rg -n 'mimalloc-pre-realloc-release-invariant|HakoAllocPageMapReleaseObserver|page_map_release_invariant' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M173 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-pre-realloc-release-invariant-proof$' "$OUT"
grep -q '^pages=1,1$' "$OUT"
grep -q '^blocks=1,0,0$' "$OUT"
grep -q '^register=1,1,1,1,1$' "$OUT"
grep -q '^observer=6,3,3$' "$OUT"
grep -q '^success=1,1,0,1,1,1,1,1,0$' "$OUT"
grep -q '^reject_released=0,1,0,0,0,0,0$' "$OUT"
grep -q '^reject_unknown=0,0,0,0,0,0$' "$OUT"
grep -q '^reject_stale=0,1,1,0,0,0$' "$OUT"
grep -q '^live=1,1,1,2$' "$OUT"
grep -q '^seam=3,3,1,1,1,3$' "$OUT"
grep -q '^page_state=0,2,1,0,1,1$' "$OUT"
grep -q '^summary=ok$' "$OUT"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --emit-mir-json "$MIR" "$ROOT_DIR/$APP" >/tmp/"$TAG".emit.out 2>/tmp/"$TAG".emit.err

python3 - "$MIR" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocPageMapReleaseObserver.birth/1",
    "HakoAllocPageMapReleaseObserver.beginRelease/1",
    "HakoAllocPageMapReleaseObserver.finishRelease/2",
    "HakoAllocPageMapReleaseObserver.handleIsLive/1",
    "HakoAllocPageMap.register/3",
    "HakoAllocPageMapReleaseSeam.releasePtr/1",
    "ProofCheck.expect/2",
    "ProofCheck.ok/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

unsupported = []
for fn in functions.values():
    for plan in fn.get("metadata", {}).get("lowering_plan", []):
        if plan.get("emit_kind") == "unsupported":
            unsupported.append((fn.get("name"), plan.get("site"), plan.get("symbol"), plan.get("reason")))
if unsupported:
    raise SystemExit(f"unsupported lowering plans remain: {unsupported[:5]}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
observer_plan = plans.get("HakoAllocPageMapReleaseObserver")
if observer_plan is None:
    raise SystemExit("missing typed object plan: HakoAllocPageMapReleaseObserver")

fields = {field.get("name"): field for field in observer_plan.get("fields", [])}
for field_name in ("observe_count", "success_count", "reject_count"):
    field = fields.get(field_name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"release observer {field_name} must be exact usize storage: {field}")

for field_name in (
    "live_count_before",
    "release_count_before",
    "unregister_count_before",
    "page_used_before",
    "local_free_before",
    "last_ptr",
    "last_page_id",
    "last_block_id",
    "last_result",
    "last_entry_live_before",
    "last_lookup_after",
    "last_live_count_delta",
    "last_release_count_delta",
    "last_unregister_count_delta",
    "last_page_used_delta",
    "last_local_free_delta",
):
    field = fields.get(field_name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"release observer {field_name} must remain i64 storage: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_main_method(box_name, name):
    for callee in iter_calls(functions["main"]):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing main method call: {box_name}.{name}")

for method in ("register",):
    require_main_method("HakoAllocPageMap", method)
for method in ("handleIsLive", "beginRelease", "finishRelease"):
    require_main_method("HakoAllocPageMapReleaseObserver", method)
for method in ("releasePtr",):
    require_main_method("HakoAllocPageMapReleaseSeam", method)
for method in ("expect",):
    require_main_method("ProofCheck", method)

def require_method_route(owner_name, box_name, method, ret_shape):
    routes = functions[owner_name].get("metadata", {}).get("lowering_plan", [])
    for route in routes:
        if (
            route.get("route_kind") == "user_box.method"
            and route.get("box_name") == box_name
            and route.get("method") == method
            and route.get("target_body_supported") is True
            and route.get("return_shape") == ret_shape
        ):
            return
    raise SystemExit(f"missing route in {owner_name}: {box_name}.{method} -> {ret_shape}")

require_method_route("HakoAllocPageMapReleaseObserver.handleIsLive/1", "HakoAllocPageMap", "lookup", "object_handle")
require_method_route("HakoAllocPageMapReleaseObserver.beginRelease/1", "HakoAllocPageMap", "lookup", "object_handle")
require_method_route("HakoAllocPageMapReleaseObserver.finishRelease/2", "HakoAllocPageMap", "lookup", "object_handle")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
