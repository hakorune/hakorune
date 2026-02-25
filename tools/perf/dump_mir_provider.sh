#!/usr/bin/env bash
set -euo pipefail

# dump_mir_provider.sh — Force provider/selfhost builder to emit MIR(JSON) with verbose diagnostics
# Usage: tools/perf/dump_mir_provider.sh <input.hako> [--out out.json]

INPUT="${1:-}"
OUT=""
shift || true
while [[ $# -gt 0 ]]; do
  case "$1" in
    --out) OUT="$2"; shift 2;;
    -h|--help) echo "Usage: $0 <input.hako> [--out out.json]"; exit 0;;
    *) echo "Unknown arg: $1"; exit 2;;
  esac
done

if [[ -z "$INPUT" || ! -f "$INPUT" ]]; then
  echo "[FAIL] input .hako not found: $INPUT" >&2; exit 2
fi

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[[ -z "$ROOT" ]] && ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

TMP_JSON=$(mktemp --suffix .mir.json)
trap 'rm -f "$TMP_JSON" >/dev/null 2>&1 || true' EXIT

# Try selfhost-first with plugins enabled; print tail on failure
set +e
HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 HAKO_SELFHOST_TRACE=1 \
NYASH_DISABLE_PLUGINS=0 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
"$ROOT/tools/hakorune_emit_mir.sh" "$INPUT" "$TMP_JSON" 2>"$TMP_JSON.err"
rc=$?
set -e
if [[ $rc -ne 0 ]]; then
  echo "[WARN] selfhost-first failed; last 80 lines:" >&2
  tail -n 80 "$TMP_JSON.err" >&2 || true
  echo "[INFO] falling back to provider-first" >&2
  if ! NYASH_DISABLE_PLUGINS=0 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
       "$ROOT/tools/hakorune_emit_mir.sh" "$INPUT" "$TMP_JSON" >/dev/null 2>&1; then
    echo "[FAIL] provider-first emit failed too" >&2
    exit 3
  fi
fi

if [[ -n "$OUT" ]]; then
  cp -f "$TMP_JSON" "$OUT"
  echo "[OK] MIR JSON -> $OUT"
else
  python3 - "$TMP_JSON" <<'PY'
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
fi

