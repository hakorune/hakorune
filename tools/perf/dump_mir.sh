#!/usr/bin/env bash
set -euo pipefail

# dump_mir.sh — Stable helper to emit MIR(JSON) and print a quick histogram
#
# Usage:
#   tools/perf/dump_mir.sh <input.hako> [--out out.json] [--mode {provider|jsonfrag}]
#
# Notes:
#   - provider: 普通の MirBuilder ルート（失敗する環境では自動で jsonfrag にフォールバック）
#   - jsonfrag : ループを while-form に純化した最小 MIR（構造検証用）

INPUT="${1:-}"
OUT=""
MODE="provider"
shift || true
while [[ $# -gt 0 ]]; do
  case "$1" in
    --out) OUT="$2"; shift 2;;
    --mode) MODE="$2"; shift 2;;
    -h|--help) echo "Usage: $0 <input.hako> [--out out.json] [--mode {provider|jsonfrag}]"; exit 0;;
    *) echo "Unknown arg: $1"; exit 2;;
  esac
done

if [[ -z "$INPUT" || ! -f "$INPUT" ]]; then
  echo "[FAIL] input .hako not found: $INPUT" >&2; exit 2
fi

ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel 2>/dev/null || true)"
[[ -z "$ROOT" ]] && ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
EMIT_ROUTE_HELPER="$ROOT/tools/smokes/v2/lib/emit_mir_route.sh"

if [[ ! -x "$EMIT_ROUTE_HELPER" ]]; then
  echo "[FAIL] emit route helper not found: $EMIT_ROUTE_HELPER" >&2
  exit 2
fi

TMP_OUT=$(mktemp --suffix .mir.json)
trap 'rm -f "$TMP_OUT" >/dev/null 2>&1 || true' EXIT

emit_provider() {
  # Provider/selfhost-first with min fallback; keep plugins ON to satisfy core boxes
  set +e
  NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=0 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
  HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_TRY_MIN=1 HAKO_MIR_NORMALIZE_PROVIDER=0 NYASH_JSON_ONLY=1 \
  "$EMIT_ROUTE_HELPER" --route hako-helper --timeout-secs 60 --out "$TMP_OUT" --input "$INPUT" >/dev/null 2>&1
  local rc=$?
  set -e
  return $rc
}

emit_jsonfrag() {
  NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1 \
  HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_MIR_BUILDER_LOOP_JSONFRAG=1 HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
  HAKO_MIR_BUILDER_JSONFRAG_PURIFY=1 NYASH_JSON_ONLY=1 \
  "$EMIT_ROUTE_HELPER" --route hako-helper --timeout-secs 60 --out "$TMP_OUT" --input "$INPUT" >/dev/null
}

if [[ "$MODE" = "provider" ]]; then
  if ! emit_provider; then
    echo "[WARN] provider emit failed; falling back to jsonfrag" >&2
    emit_jsonfrag
  fi
else
  emit_jsonfrag
fi

if [[ -n "$OUT" ]]; then
  cp -f "$TMP_OUT" "$OUT"
  echo "[OK] MIR JSON -> $OUT"
fi

# Print a quick histogram
python3 - "$TMP_OUT" <<'PY'
import json,sys
p=sys.argv[1]
j=json.load(open(p))
for f in j.get('functions',[]):
  print('Function:', f.get('name'))
  for b in (f.get('blocks') or []):
    ops=[(i or {}).get('op') for i in (b.get('instructions') or [])]
    if not ops: continue
    from collections import Counter
    c=Counter(ops)
    print('  bb', b.get('id'), dict(c))
PY
