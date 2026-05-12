#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-realloc-same-class"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

REALLOC_PATH="lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako"
OBSERVER="lang/src/hako_alloc/memory/page_map_release_invariant_box.hako"
PAGE_RELEASE="lang/src/hako_alloc/memory/page_map_release_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-realloc-same-class-proof/main.hako"
APP_TEST="apps/mimalloc-realloc-same-class-proof/test.sh"
APP_README="apps/mimalloc-realloc-same-class-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-184-M174-REALLOC-SAME-CLASS-NO-MOVE-PATH.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_realloc_same_class_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_realloc_same_class.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_realloc_same_class.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_realloc_same_class.mir.json"

echo "[$TAG] checking M174 same-class/no-move realloc path"

guard_require_files \
  "$TAG" \
  "$REALLOC_PATH" \
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

guard_expect_in_file "$TAG" 'memory.page_map_realloc_same_class_box = "memory/page_map_realloc_same_class_box.hako"' "$MODULE" "hako module must export the M174 no-move path"
guard_expect_in_file "$TAG" 'box HakoAllocPageMapReallocSameClassPath' "$REALLOC_PATH" "M174 no-move owner must exist"
guard_expect_in_file "$TAG" 'birth\(seam\)' "$REALLOC_PATH" "M174 no-move path must take the M172 seam explicitly"
guard_expect_in_file "$TAG" 'me\.page_map = seam\.page_map' "$REALLOC_PATH" "M174 no-move path must read ownership through the seam page-map"
guard_expect_in_file "$TAG" 'tryReallocSameClass\(ptr, requested_size\)' "$REALLOC_PATH" "M174 must expose the same-class/no-move path"
guard_expect_in_file "$TAG" 'page\.blockIsLive\(block_id\)' "$REALLOC_PATH" "M174 must reject released blocks without calling release"
guard_expect_in_file "$TAG" 'requested_size > page\.block_size' "$REALLOC_PATH" "M174 must reject grow requests that do not fit the current block"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_realloc_same_class_box as HakoAllocPageMapReallocSameClassBox' "$APP" "proof app must import the M174 path"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_release_invariant_box as HakoAllocPageMapReleaseInvariantBox' "$APP" "proof app must observe the frozen M173 handle lifetime contract"
guard_expect_in_file "$TAG" 'seam\.releasePtr\(' "$APP" "proof app must use the M172 seam only to set up released-block evidence"
guard_expect_in_file "$TAG" '293x-184 M174 Realloc Same-Class No-Move Path' "$CARD" "missing M174 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M174 guard"
guard_expect_in_file "$TAG" 'M174 realloc same-class/no-move path' "$PLAN" "plan must retain the M174 row"
guard_expect_in_file "$TAG" 'HakoAllocPageMapReallocSameClassPath' "$ROOT_README" "root README must document the M174 no-move owner"
guard_expect_in_file "$TAG" 'page_map_realloc_same_class_box.hako' "$MEMORY_README" "memory README must document the M174 no-move module"

if rg -n 'init[[:space:]]*\{' "$REALLOC_PATH" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M174 no-move path must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n '\.register\(|releaseLocal\(|\.unregister\(|\.releasePtr\(' "$REALLOC_PATH" >/tmp/"$TAG".owner_leak 2>&1; then
  echo "[$TAG] ERROR: M174 must not take ownership of register/release/unregister execution" >&2
  cat /tmp/"$TAG".owner_leak >&2
  rm -f /tmp/"$TAG".owner_leak
  exit 1
fi
rm -f /tmp/"$TAG".owner_leak

if rg -n 'memcpy|copy_bytes|aligned[A-Z_(]|huge[A-Z_(]|secure[A-Z_(]|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|OSVM|OsVm|provider|hook|replacement|hako_mem_|externcall|alloc_copy|fallback\(|unreserve|release_bytes' \
  "$REALLOC_PATH" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M174 leaked out of same-class/no-move realloc scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-realloc-same-class|HakoAllocPageMapReallocSameClassPath|page_map_realloc_same_class' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M174 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-realloc-same-class-proof$' "$OUT"
grep -q '^setup=1,1,1,1,1,1,1,1$' "$OUT"
grep -q '^same=1,1001,1,2001,1,1,1$' "$OUT"
grep -q '^reject=0,0,0,0,1,1,1$' "$OUT"
grep -q '^deltas=0,0,0,0,0,0$' "$OUT"
grep -q '^path=2,1,1,1,1,4$' "$OUT"
grep -q '^seam=1,1,0,0,0,0$' "$OUT"
grep -q '^page=1,1,0,1,0,0,4$' "$OUT"
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
    "HakoAllocPageMapReallocSameClassPath.birth/1",
    "HakoAllocPageMapReallocSameClassPath.tryReallocSameClass/2",
    "HakoAllocPageMapReleaseObserver.handleIsLive/1",
    "HakoAllocPageMapReleaseSeam.releasePtr/1",
    "HakoAllocPageMap.register/3",
    "ProofCheck.expect/2",
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
for method in ("handleIsLive",):
    require_main_method("HakoAllocPageMapReleaseObserver", method)
for method in ("releasePtr",):
    require_main_method("HakoAllocPageMapReleaseSeam", method)
for method in ("tryReallocSameClass",):
    require_main_method("HakoAllocPageMapReallocSameClassPath", method)
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

require_method_route("HakoAllocPageMapReallocSameClassPath.tryReallocSameClass/2", "HakoAllocPageMap", "lookup", "object_handle")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
