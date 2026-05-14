#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimap012-object-loop-row-a-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-object-loop-row-a-proof/main.hako"
APP_README="apps/mimalloc-object-loop-row-a-proof/README.md"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INV="docs/development/current/main/investigations/mimap012-heavy-object-loop-shape-investigation.md"
INDEX="docs/tools/check-scripts-index.md"

for path in "$APP" "$APP_README" "$PAGE" "$TASKBOARD" "$INV" "$INDEX"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'loop(i < page_count)' "$APP"
rg -F -q 'local page = pages.get(i)' "$APP"
rg -F -q 'if selected < 0' "$APP"
rg -F -q 'page.freeCount() > 0' "$APP"
rg -F -q 'selected_id = page.page_id' "$APP"
rg -F -q 'MIR-ROW-A' "$TASKBOARD"
rg -F -q 'MIR JSON guard passes' "$INV"
rg -F -q 'k2_wide_mimap012_object_loop_row_a_exe_guard.sh' "$INDEX"

if rg -n 'considerPage|last_selected_page|last_selected_kind|dense|OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook|remote' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIR-ROW-A must stay minimal and must not activate substrate/provider/hook behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimap012-object-loop-row-a-proof' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap012_row_a.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/row_a.mir.json"
exe_out="$tmp_dir/row_a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")

text = json.dumps(main, sort_keys=True)
for needle in ("get", "freeCount", "page_id"):
    if needle not in text:
        raise SystemExit(f"missing expected row-a MIR marker: {needle}")

if "considerPage" in text or "last_selected_page" in text:
    raise SystemExit("row-a MIR must not include helper call or nullable selected object field")

print("[mimap012-row-a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimap012-object-loop-row-a-proof' "$run_log"
rg -F -q 'selected=1' "$run_log"
rg -F -q 'selected_id=20' "$run_log"
rg -F -q 'shape=4' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

