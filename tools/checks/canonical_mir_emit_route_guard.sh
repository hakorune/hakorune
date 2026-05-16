#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="canonical-mir-emit-route"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

EMIT_ROUTE="tools/smokes/v2/lib/emit_mir_route.sh"
SELFHOST_BUILD="tools/selfhost/selfhost_build.sh"
SELFHOST_DIRECT="tools/selfhost/lib/selfhost_build_direct.sh"
PURE_FIRST_LIB="tools/checks/lib/pure_first_exe_guard.sh"
EXACTNESS_GUARD="tools/checks/pure_first_mir_artifact_exactness_guard.sh"
PREFLIGHT_GUARD="tools/checks/pure_first_route_preflight_guard.sh"
COMPARE="tools/selfhost/lib/mir_canonical_compare.py"
SSOT="docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-453-MIR-EMIT-SSOT-002-CANONICAL-EMIT-WRAPPER.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking canonical MIR emit route"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$EMIT_ROUTE" \
  "$SELFHOST_BUILD" \
  "$SELFHOST_DIRECT" \
  "$PURE_FIRST_LIB" \
  "$EXACTNESS_GUARD" \
  "$PREFLIGHT_GUARD" \
  "$COMPARE" \
  "$SSOT" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'tools/smokes/v2/lib/emit_mir_route.sh' "$SSOT" "SSOT must name the canonical route"
guard_expect_in_file "$TAG" 'tools/smokes/v2/lib/emit_mir_route.sh' "$CARD" "453 card must name the canonical route"
guard_expect_in_file "$TAG" '--backend mir --emit-mir-json' "$EMIT_ROUTE" "direct route must own backend=mir MIR emission"
guard_expect_in_file "$TAG" 'emit_mir_route.sh' "$SELFHOST_DIRECT" "selfhost source MIR emission must use canonical route"
guard_expect_in_file "$TAG" 'emit_mir_route.sh' "$PURE_FIRST_LIB" "pure-first emit helper must use canonical route"
guard_expect_in_file "$TAG" 'emit_mir_route.sh' "$EXACTNESS_GUARD" "exactness guard must use canonical route"
guard_expect_in_file "$TAG" 'emit_mir_route.sh' "$PREFLIGHT_GUARD" "preflight guard must use canonical route"
guard_expect_in_file "$TAG" 'tools/checks/canonical_mir_emit_route_guard.sh' "$INDEX" "check index must list this guard"

if rg -n 'target/debug/hakorune.*--emit-mir-json|--backend mir --emit-mir-json' \
  "$PURE_FIRST_LIB" "$EXACTNESS_GUARD" "$PREFLIGHT_GUARD" >/tmp/"$TAG".direct_emit 2>&1; then
  echo "[$TAG] ERROR: pure-first/check callers must use emit_mir_route.sh for source MIR emission" >&2
  cat /tmp/"$TAG".direct_emit >&2
  rm -f /tmp/"$TAG".direct_emit
  exit 1
fi
rm -f /tmp/"$TAG".direct_emit

tmp_dir="$(mktemp -d /tmp/hakorune_emit_route.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

app="$tmp_dir/min.hako"
route_mir="$tmp_dir/route.mir.json"
selfhost_mir="$tmp_dir/selfhost.mir.json"
diff_summary="$tmp_dir/diff.json"

cat >"$app" <<'HAKO'
static box Main {
  main() {
    local x = 40 + 2
    return x
  }
}
HAKO

cargo build -q --bin hakorune

NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$EMIT_ROUTE" --route direct --out "$route_mir" --input "$app" >/dev/null

NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$SELFHOST_BUILD" --in "$app" --mir-out "$selfhost_mir" >/dev/null 2>"$tmp_dir/selfhost.err"

if ! python3 "$COMPARE" summarize-first-diff "$route_mir" "$selfhost_mir" >"$diff_summary"; then
  echo "[$TAG] ERROR: canonical route and selfhost --mir-out differ" >&2
  cat "$diff_summary" >&2
  sed -n '1,120p' "$tmp_dir/selfhost.err" >&2
  exit 1
fi

echo "[$TAG] ok"
