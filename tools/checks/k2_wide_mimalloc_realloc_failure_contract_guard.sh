#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-realloc-failure-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

FAILURE_PATH="lang/src/hako_alloc/memory/page_map_realloc_failure_contract_box.hako"
REALLOC_SAME="lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako"
REALLOC_FALLBACK="lang/src/hako_alloc/memory/page_map_realloc_alloc_copy_release_box.hako"
OBSERVER="lang/src/hako_alloc/memory/page_map_release_invariant_box.hako"
PAGE_RELEASE="lang/src/hako_alloc/memory/page_map_release_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/mimalloc-realloc-failure-contract-proof/main.hako"
APP_TEST="apps/mimalloc-realloc-failure-contract-proof/test.sh"
APP_README="apps/mimalloc-realloc-failure-contract-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-186-M176-REALLOC-NEGATIVE-MATRIX-FAILURE-CONTRACT.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_realloc_failure_contract_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_realloc_failure_contract.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_realloc_failure_contract.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_realloc_failure_contract.mir.json"

echo "[$TAG] checking M176 realloc negative matrix / failure contract"

guard_require_files \
  "$TAG" \
  "$FAILURE_PATH" \
  "$REALLOC_SAME" \
  "$REALLOC_FALLBACK" \
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

guard_expect_in_file "$TAG" 'memory.page_map_realloc_failure_contract_box = "memory/page_map_realloc_failure_contract_box.hako"' "$MODULE" "hako module must export the M176 failure-contract owner"
guard_expect_in_file "$TAG" 'box HakoAllocPageMapReallocFailureContract' "$FAILURE_PATH" "M176 failure-contract owner must exist"
guard_expect_in_file "$TAG" 'birth\(same_class_path, fallback_path, seam\)' "$FAILURE_PATH" "M176 failure-contract owner must compose M174, M175, and the shared seam explicitly"
guard_expect_in_file "$TAG" 'tryReallocWithFailureContract\(ptr, requested_size\)' "$FAILURE_PATH" "M176 must expose a single diagnostics wrapper entry"
guard_expect_in_file "$TAG" 'requested_size > me\.last_max_block_size' "$FAILURE_PATH" "M176 must freeze an oversized reject before fallback allocation"
guard_expect_in_file "$TAG" 'me\.same_class_path\.tryReallocSameClass\(ptr, requested_size\)' "$FAILURE_PATH" "M176 must delegate same-class success detection to M174"
guard_expect_in_file "$TAG" 'me\.fallback_path\.tryReallocAllocCopyRelease\(ptr, requested_size\)' "$FAILURE_PATH" "M176 must delegate grow fallback to M175"
guard_expect_in_file "$TAG" 'last_failure_kind = 6' "$FAILURE_PATH" "M176 must keep alloc-fail distinct in the failure matrix"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_realloc_failure_contract_box as HakoAllocPageMapReallocFailureContractBox' "$APP" "proof app must import the M176 failure-contract owner"
guard_expect_in_file "$TAG" '293x-186 M176 Realloc Negative Matrix / Failure Contract' "$CARD" "missing M176 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M176 guard"
guard_expect_in_file "$TAG" 'M176 realloc negative matrix / failure contract' "$PLAN" "plan must retain the M176 row"
guard_expect_in_file "$TAG" 'HakoAllocPageMapReallocFailureContract' "$ROOT_README" "root README must document the M176 failure-contract owner"
guard_expect_in_file "$TAG" 'page_map_realloc_failure_contract_box.hako' "$MEMORY_README" "memory README must document the M176 failure-contract module"

if rg -n 'init[[:space:]]*\{' "$FAILURE_PATH" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M176 failure-contract owner must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n '\.register\(|\.releasePtr\(|\.unregister\(|releaseLocal\(|memcpy|copy_bytes|aligned[A-Z_(]|huge[A-Z_(]|secure[A-Z_(]|provider|hook|hako_mem_|externcall|fallback\(|unreserve|release_bytes' \
  "$FAILURE_PATH" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M176 owner leaked out of diagnostics-only failure-contract scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-realloc-failure-contract|HakoAllocPageMapReallocFailureContract|page_map_realloc_failure_contract' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M176 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-realloc-failure-contract-proof$' "$OUT"
grep -q '^setup=1,1,1,1,1,1$' "$OUT"
grep -q '^same=1,2001$' "$OUT"
grep -q '^move=1,9000,0,1$' "$OUT"
grep -q '^zero=0,1,1$' "$OUT"
grep -q '^oversized=0,2,64,1$' "$OUT"
grep -q '^alloc_fail=0,6,1$' "$OUT"
grep -q '^released=1,0,5,1$' "$OUT"
grep -q '^stale=0,4,1$' "$OUT"
grep -q '^unknown=0,3$' "$OUT"
grep -q '^deltas=1,1,1,0,-1,1,1$' "$OUT"
grep -q '^contract=2,1,1,1,1,1,1,1,1,6,0$' "$OUT"
grep -q '^same_path=1,2,1,1,1,5$' "$OUT"
grep -q '^fallback=1,1,0,1,0,0,0,1$' "$OUT"
grep -q '^seam=1,1,0,0,0,0$' "$OUT"
grep -q '^page=1,1,0,2,0,0,6,5$' "$OUT"
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
    "HakoAllocPageMapReallocFailureContract.birth/3",
    "HakoAllocPageMapReallocFailureContract.tryReallocWithFailureContract/2",
    "HakoAllocPageMapReallocSameClassPath.tryReallocSameClass/2",
    "HakoAllocPageMapReallocAllocCopyReleasePath.tryReallocAllocCopyRelease/2",
    "HakoAllocPageMapReleaseObserver.handleIsLive/1",
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

for method in ("tryReallocWithFailureContract",):
    require_main_method("HakoAllocPageMapReallocFailureContract", method)
for method in ("handleIsLive",):
    require_main_method("HakoAllocPageMapReleaseObserver", method)
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

require_method_route(
    "HakoAllocPageMapReallocFailureContract.tryReallocWithFailureContract/2",
    "HakoAllocPageMapReallocSameClassPath",
    "tryReallocSameClass",
    "scalar_i64",
)
require_method_route(
    "HakoAllocPageMapReallocFailureContract.tryReallocWithFailureContract/2",
    "HakoAllocPageMapReallocAllocCopyReleasePath",
    "tryReallocAllocCopyRelease",
    "scalar_i64",
)
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
