#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-map-release"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_RELEASE="lang/src/hako_alloc/memory/page_map_release_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-page-map-release-proof/main.hako"
APP_TEST="apps/mimalloc-page-map-release-proof/test.sh"
APP_README="apps/mimalloc-page-map-release-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-180-M172-MIMALLOC-PAGE-MAP-BACKED-RELEASE-SEAM.md"
CLEANUP_CARD="docs/development/current/main/phases/phase-293x/293x-182-M172-PROOF-CHECK-CLEANUP.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_map_release_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_page_map_release.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_map_release.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_map_release.mir.json"

echo "[$TAG] checking M172 mimalloc page-map-backed release seam"

guard_require_files \
  "$TAG" \
  "$PAGE_RELEASE" \
  "$PAGE_MAP" \
  "$PAGE_BOX" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$CLEANUP_CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.page_map_release_box = "memory/page_map_release_box.hako"' "$MODULE" "hako module must export M172 release seam"
guard_expect_in_file "$TAG" 'box HakoAllocPageMapReleaseSeam' "$PAGE_RELEASE" "M172 release seam owner must exist"
guard_expect_in_file "$TAG" 'birth\(page_map\)' "$PAGE_RELEASE" "M172 release seam must take the page-map owner explicitly"
guard_expect_in_file "$TAG" 'releasePtr\(ptr\)' "$PAGE_RELEASE" "M172 release seam must expose releasePtr"
guard_expect_in_file "$TAG" 'page_map\.lookup\(ptr\)' "$PAGE_RELEASE" "M172 must resolve pointer ownership through HakoAllocPageMap.lookup"
guard_expect_in_file "$TAG" 'page\.releaseLocal\(entry\.block_id\)' "$PAGE_RELEASE" "M172 must delegate block release to HakoAllocPageModel.releaseLocal"
guard_expect_in_file "$TAG" 'page_map\.unregister\(ptr\)' "$PAGE_RELEASE" "M172 must unregister ownership after page-local release succeeds"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_release_box as HakoAllocPageMapReleaseBox' "$APP" "proof app must import M172 release seam"
guard_expect_in_file "$TAG" 'box ProofCheck' "$APP" "M172 proof app must keep labelled proof checks readable"
guard_expect_in_file "$TAG" 'proof\.expect' "$APP" "M172 proof app must use labelled proof expectations"
guard_expect_in_file "$TAG" 'proof\.ok\(\)' "$APP" "M172 proof app must use the proof helper summary result"
guard_expect_in_file "$TAG" '293x-180 M172 Mimalloc Page-Map-Backed Release Seam' "$CARD" "missing M172 card"
guard_expect_in_file "$TAG" '293x-182 M172 Proof Check Cleanup' "$CLEANUP_CARD" "missing M172 proof cleanup card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M172 guard"
guard_expect_in_file "$TAG" 'M172 page-map-backed release seam' "$PLAN" "plan must retain M172 row"
guard_expect_in_file "$TAG" '293x-182 M172 proof check cleanup' "$PLAN" "plan must retain the proof cleanup row"

if rg -n 'init[[:space:]]*\{' "$PAGE_RELEASE" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M172 release seam must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'registerPtr|\.register\(' "$PAGE_RELEASE" >/tmp/"$TAG".registration_owner 2>&1; then
  echo "[$TAG] ERROR: M172 release seam must not own pointer registration; M171 page_map_box owns register" >&2
  cat /tmp/"$TAG".registration_owner >&2
  rm -f /tmp/"$TAG".registration_owner
  exit 1
fi
rm -f /tmp/"$TAG".registration_owner

if rg -n '&&' "$APP" >/tmp/"$TAG".proof_conjunction 2>&1; then
  echo "[$TAG] ERROR: M172 proof app must not regress to a giant && summary condition" >&2
  cat /tmp/"$TAG".proof_conjunction >&2
  rm -f /tmp/"$TAG".proof_conjunction
  exit 1
fi
rm -f /tmp/"$TAG".proof_conjunction

if rg -n 'realloc|aligned|huge|secure|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|OSVM|OsVm|provider|hook|replacement|hako_mem_|externcall|unreserve|release_bytes|hako_osvm_(unreserve|release)' \
  "$PAGE_RELEASE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M172 leaked out of page-map-backed release scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-page-map-release|HakoAllocPageMapReleaseSeam|page_map_release' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M172 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-page-map-release-proof$' "$OUT"
grep -q '^pages=1,1$' "$OUT"
grep -q '^blocks=1,0,0$' "$OUT"
grep -q '^register=1,1,1,1,1$' "$OUT"
grep -q '^release=1,0,0,0,0,1,1$' "$OUT"
grep -q '^page0=0,2,1,2$' "$OUT"
grep -q '^page1=0,1,1,1$' "$OUT"
grep -q '^seam=2,2,3,3,2,1,1,4$' "$OUT"
grep -q '^map=5,2,5,7,2,3,0$' "$OUT"
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
    "HakoAllocPageMap.register/3",
    "HakoAllocPageMap.lookup/1",
    "HakoAllocPageMap.unregister/1",
    "HakoAllocPageMapReleaseSeam.birth/1",
    "HakoAllocPageMapReleaseSeam.addPage/1",
    "HakoAllocPageMapReleaseSeam.releasePtr/1",
    "HakoAllocPageModel.releaseLocal/1",
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
for method in ("addPage", "releasePtr"):
    require_main_method("HakoAllocPageMapReleaseSeam", method)
for method in ("expect", "ok"):
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

require_method_route("HakoAllocPageMapReleaseSeam.releasePtr/1", "HakoAllocPageMap", "lookup", "object_handle")
require_method_route("HakoAllocPageMapReleaseSeam.releasePtr/1", "HakoAllocPageModel", "releaseLocal", "scalar_i64")
require_method_route("HakoAllocPageMapReleaseSeam.releasePtr/1", "HakoAllocPageMap", "unregister", "scalar_i64")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
